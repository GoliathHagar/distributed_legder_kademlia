use std::collections::{LinkedList, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::auctions::{Auction, Bid};
use crate::blockchain::Block;
use crate::blockchain::transactions::ScriptPubKey;
use crate::blockchain::BlockChainHandler;
use crate::messages::MessageHandler;
use crate::messages::messagetypes::{AuctionMessage, BidMessage, RequestPaymentMessage};
use crate::p2p::kademlia::{P2PNode, StoredKeyMetadata};
use crate::p2p::nodeoperations::{ContentLookupOperation, StoreOperation};
use crate::p2p::kademlia::operations::Operation;
use crate::p2p::node::NodeID;
use crate::util::{ByteWrapper, Hex, Pair};
use std::collections::HashMap;
use std::cmp::Ordering;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

pub struct AuctionHandler {
    node: P2PNode,
    chain_handler: BlockChainHandler,
    message_handler: MessageHandler,
    our_active_auctions: Arc<Mutex<LinkedList<Auction>>>,
    active_auctions: Arc<Mutex<LinkedList<Vec<u8>>>>,
    our_active_bids: Arc<Mutex<HashMap<ByteWrapper, LinkedList<Vec<u8>>>>>,
    active_bids: Arc<Mutex<HashMap<ByteWrapper, LinkedList<Vec<u8>>>>>,
    auction_waiting_for_payment: Arc<Mutex<HashMap<ByteWrapper, LinkedList<Bid>>>>,
    current_bid_index_for_auc: Arc<Mutex<HashMap<ByteWrapper, usize>>>,
    payment_keys: Arc<Mutex<HashMap<ByteWrapper, KeyPair>>>,
    pending_payments: Arc<Mutex<VecDeque<Pair<Pair<Vec<u8>, u64>, Vec<u8>>>>>,
}

impl AuctionHandler {
    pub fn new(
        node: P2PNode,
        message_handler: MessageHandler,
        chain_handler: BlockChainHandler,
    ) -> Self {
        Self {
            node,
            chain_handler,
            message_handler,
            our_active_auctions: Arc::new(Mutex::new(LinkedList::new())),
            active_auctions: Arc::new(Mutex::new(LinkedList::new())),
            our_active_bids: Arc::new(Mutex::new(HashMap::new())),
            active_bids: Arc::new(Mutex::new(HashMap::new())),
            auction_waiting_for_payment: Arc::new(Mutex::new(HashMap::new())),
            current_bid_index_for_auc: Arc::new(Mutex::new(HashMap::new())),
            payment_keys: Arc::new(Mutex::new(HashMap::new())),
            pending_payments: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn handle_request_active_auctions(&self, requesting_node_id: Vec<u8>) {
        let our_active_auctions = self.our_active_auctions.lock().unwrap();
        for our_active_auction in our_active_auctions.iter() {
            self.message_handler
                .send_message(requesting_node_id.clone(), &AuctionMessage::new(our_active_auction.get_auction_id()));
        }
    }

    pub fn start_new_auction(&self, auction: Auction) {
        let message = self.message_handler.serialize_object(&auction);

        let stored_key_metadata = StoredKeyMetadata::new(
            auction.get_auction_id(),
            message,
            auction.get_auctioneer_node_id(),
        );

        let store_operation = StoreOperation::new(self.node.clone(), stored_key_metadata);

        store_operation.execute();

        let mut our_active_auctions = self.our_active_auctions.lock().unwrap();
        our_active_auctions.push_back(auction.clone());

        let mut active_auctions = self.active_auctions.lock().unwrap();
        active_auctions.push_back(auction.get_auction_id());

        self.message_handler.publish_message(&AuctionMessage::new(auction.get_auction_id()));
    }

    pub fn handle_new_auction_started(&self, auction_id: Vec<u8>) {
        let mut active_auctions = self.active_auctions.lock().unwrap();
        active_auctions.push_back(auction_id);
    }

    pub fn handle_new_bid(&self, bid: Vec<u8>) {
        let node = self.node.clone();
        let message_handler = self.message_handler.clone();
        let active_bids = self.active_bids.clone();
        let wrapped_bid = ByteWrapper::new(bid.clone());

        let content_lookup_operation = ContentLookupOperation::new(node, bid.clone(), move |response| {
            let bid_obj = message_handler.deserialize_object(response, Bid::new()).unwrap();

            let mut active_bids = active_bids.lock().unwrap();

            if !active_bids.contains_key(&wrapped_bid) {
                active_bids.insert(wrapped_bid, LinkedList::new());
            }

            if let Some(bids) = active_bids.get_mut(&wrapped_bid) {
                bids.push_back(bid.clone());
            }
        });

        content_lookup_operation.execute();
    }

    pub fn publish_bid(&self, bid: Bid) {
        let message = self.message_handler.serialize_object(&bid);

        let stored_key_metadata = StoredKeyMetadata::new(
            bid.get_bid_id(),
            message,
            bid.get_user_node_id(),
        );

        let store_operation = StoreOperation::new(self.node.clone(), stored_key_metadata);

        store_operation.execute();

        self.append_our_bid(bid.clone());
        self.append_global_bid(bid.clone());

        self.message_handler
            .publish_message(&BidMessage::new(bid.get_bid_id()));
    }

    pub fn auction_ended(&self, auction_id: Vec<u8>) {
        let mut active_auctions = self.active_auctions.lock().unwrap();

        if let Some(index) = active_auctions.iter().position(|id| *id == auction_id) {
            active_auctions.remove(index);
        }

        let wrapped_id = ByteWrapper::new(auction_id.clone());

        let mut active_bids = self.active_bids.lock().unwrap();
        active_bids.remove(&wrapped_id);

        let mut our_active_bids = self.our_active_bids.lock().unwrap();

        if let Some(bid_ids) = our_active_bids.remove(&wrapped_id) {
            for bid_id in bid_ids {
                self.node.delete_value(&bid_id);
            }
        }
    }

    fn append_global_bid(&self, bid: Bid) {
        let wrapped_auction_id = ByteWrapper::new(bid.get_auction_id());

        let mut active_bids = self.active_bids.lock().unwrap();

        if !active_bids.contains_key(&wrapped_auction_id) {
            active_bids.insert(wrapped_auction_id, LinkedList::new());
        }

        if let Some(bids) = active_bids.get_mut(&wrapped_auction_id) {
            bids.push_back(bid.get_bid_id());
        }
    }

    fn append_our_bid(&self, bid: Bid) {
        let wrapped_auction_id = ByteWrapper::new(bid.get_auction_id());

        let mut our_active_bids = self.our_active_bids.lock().unwrap();

        if !our_active_bids.contains_key(&wrapped_auction_id) {
            our_active_bids.insert(wrapped_auction_id, LinkedList::new());
        }

        if let Some(bids) = our_active_bids.get_mut(&wrapped_auction_id) {
            bids.push_back(bid.get_bid_id());
        }
    }

    pub fn finish_auction(&self, auction: Auction, pair: KeyPair) {
        if auction.get_auctioneer_pk() != pair.get_public().encode() {
            println!("Wrong keypair for the auction you want to end!");
            return;
        }

        let mut waiting = AtomicInteger::new();

        let bids = Arc::new(Mutex::new(LinkedList::new()));

        let wrapped_auction_id = ByteWrapper::new(auction.get_auction_id());

        let node = self.node.clone();
        let message_handler = self.message_handler.clone();
        let our_active_bids = self.our_active_bids.clone();

        let mut active_bids = self.active_bids.lock().unwrap();

        if let Some(bid_ids) = active_bids.get(&wrapped_auction_id) {
            for bid_id in bid_ids {
                waiting.increment();

                let content_lookup_operation = ContentLookupOperation::new(node.clone(), bid_id.clone(), move |result| {
                    let bid_obj = message_handler.deserialize_object(result, Bid::new()).unwrap();

                    let mut bids = bids.lock().unwrap();
                    bids.push_back(bid_obj.clone());

                    if waiting.decrement() == 0 {
                        self.finished_loading_bids(auction.clone(), bids.clone(), pair.clone());
                    }
                });

                content_lookup_operation.execute();
            }
        }
    }

    fn finished_loading_bids(&self, auction: Auction, bids: Arc<Mutex<LinkedList<Bid>>>, auction_owner: KeyPair) {
        let mut bids = bids.lock().unwrap();
        bids.sort_by(|a, b| b.get_bid_amount(&auction_owner).partial_cmp(&a.get_bid_amount(&auction_owner)).unwrap());

        let wrapped_auction_id = ByteWrapper::new(auction.get_auction_id());

        let payment_keys = self.payment_keys.lock().unwrap();
        payment_keys.insert(wrapped_auction_id, auction_owner.clone());

        let mut current_bid_index_for_auc = self.current_bid_index_for_auc.lock().unwrap();
        current_bid_index_for_auc.insert(wrapped_auction_id, 0);

        let auction_waiting_for_payment = self.auction_waiting_for_payment.clone();
        let message_handler = self.message_handler.clone();

        let first_bid = bids.front().unwrap().clone();

        let payment_message = RequestPaymentMessage::new(
            first_bid.get_auction_id(),
            first_bid.get_bid_id(),
            Standards::calculate_hashed_public_key_from(auction_owner.get_public()),
        );

        let signed_by_auctioneer = Signable::calculate_signature_of(&payment_message, auction_owner.get_private());
        payment_message.set_signed_by_auctioneer(signed_by_auctioneer);

        let user_node_id = first_bid.get_user_node_id();
        let node = self.node.clone();

        let request_payment_operation = move || {
            let node = node.clone();
            let user_node_id = user_node_id.clone();
            let payment_message = payment_message.clone();

            let request_payment_callback = move |response: Vec<u8>| {
                let tx_id = response;

                let best_current_chain = self.chain_handler.get_best_current_chain();

                if let Some(best_current_chain) = best_current_chain {
                    let latest_valid_block = best_current_chain.get_latest_valid_block();
                    let block_number = latest_valid_block.get_header().get_block_number();

                    let mut pending_payments = self.pending_payments.lock().unwrap();
                    pending_payments.push_back((tx_id, block_number, first_bid.get_auction_id()));
                } else {
                    panic!("Cannot accept a bid with no blockchain!");
                }
            };

            let content_lookup_operation = ContentLookupOperation::new(node, user_node_id.clone(), move |response| {
                let address_obj = message_handler.deserialize_object(response, Address::new()).unwrap();

                let payment_tx = payment_message.clone().to_transaction();

                payment_tx.send_to(&address_obj);

                let tx_id = payment_tx.get_tx_id();

                self.node.store_value(&tx_id, payment_tx.to_bytes());
                self.node.store_value(&payment_tx.get_data(), tx_id.clone());

                let message = message_handler.serialize_object(&payment_message);

                let stored_key_metadata = StoredKeyMetadata::new(
                    payment_message.get_payment_id(),
                    message,
                    user_node_id.clone(),
                );

                let store_operation = StoreOperation::new(node.clone(), stored_key_metadata);

                store_operation.execute();

                request_payment_callback(tx_id.clone());
            });

            content_lookup_operation.execute();
        };

        self.node.find_node_by_id(user_node_id.clone(), request_payment_operation);
    }

    pub fn handle_payment_confirmation(&self, payment_id: Vec<u8>) {
        let wrapped_payment_id = ByteWrapper::new(payment_id.clone());

        let payment_keys = self.payment_keys.lock().unwrap();
        let auction_owner = payment_keys.get(&wrapped_payment_id).unwrap().clone();

        let auction_waiting_for_payment = self.auction_waiting_for_payment.clone();

        let mut auction_waiting_for_payment = auction_waiting_for_payment.lock().unwrap();

        if let Some(bids) = auction_waiting_for_payment.remove(&wrapped_payment_id) {
            let mut current_bid_index_for_auc = self.current_bid_index_for_auc.lock().unwrap();

            for bid in bids {
                let mut current_index = current_bid_index_for_auc.get_mut(&wrapped_payment_id).unwrap();
                *current_index += 1;

                if let Some(bid) = current_index {
                    let auction_id = bid.get_auction_id();
                    let wrapped_auction_id = ByteWrapper::new(auction_id.clone());

                    let mut active_bids = self.active_bids.lock().unwrap();

                    if let Some(bids) = active_bids.get_mut(&wrapped_auction_id) {
                        if let Some(index) = bids.iter().position(|bid_id| *bid_id == bid.get_bid_id()) {
                            bids.remove(index);
                        }
                    }

                    let mut our_active_bids = self.our_active_bids.lock().unwrap();

                    if let Some(bids) = our_active_bids.get_mut(&wrapped_auction_id) {
                        if let Some(index) = bids.iter().position(|bid_id| *bid_id == bid.get_bid_id()) {
                            bids.remove(index);
                        }
                    }
                }
            }
        }
    }

    pub fn handle_payment_failure(&self, payment_id: Vec<u8>) {
        let wrapped_payment_id = ByteWrapper::new(payment_id.clone());

        let payment_keys = self.payment_keys.lock().unwrap();
        let auction_owner = payment_keys.get(&wrapped_payment_id).unwrap().clone();

        let auction_waiting_for_payment = self.auction_waiting_for_payment.clone();

        let mut auction_waiting_for_payment = auction_waiting_for_payment.lock().unwrap();

        if let Some(bids) = auction_waiting_for_payment.remove(&wrapped_payment_id) {
            let mut current_bid_index_for_auc = self.current_bid_index_for_auc.lock().unwrap();

            for bid in bids {
                let mut current_index = current_bid_index_for_auc.get_mut(&wrapped_payment_id).unwrap();
                *current_index += 1;

                if let Some(bid) = current_index {
                    let auction_id = bid.get_auction_id();
                    let wrapped_auction_id = ByteWrapper::new(auction_id.clone());

                    let mut active_bids = self.active_bids.lock().unwrap();

                    if let Some(bids) = active_bids.get_mut(&wrapped_auction_id) {
                        if let Some(index) = bids.iter().position(|bid_id| *bid_id == bid.get_bid_id()) {
                            bids.remove(index);
                        }
                    }

                    let mut our_active_bids = self.our_active_bids.lock().unwrap();

                    if let Some(bids) = our_active_bids.get_mut(&wrapped_auction_id) {
                        if let Some(index) = bids.iter().position(|bid_id| *bid_id == bid.get_bid_id()) {
                            bids.remove(index);
                        }
                    }
                }
            }
        }
    }

    pub fn handle_pending_payments(&self) {
        let mut pending_payments = self.pending_payments.lock().unwrap();

        let best_current_chain = self.chain_handler.get_best_current_chain();

        if let Some(best_current_chain) = best_current_chain {
            let latest_valid_block = best_current_chain.get_latest_valid_block();
            let block_number = latest_valid_block.get_header().get_block_number();

            let mut remove_indices = Vec::new();

            for (index, (tx_id, payment_block_number, auction_id)) in pending_payments.iter().enumerate() {
                if *payment_block_number + self.payment_confirmation_blocks <= block_number {
                    self.node.delete_value(tx_id);
                    self.node.delete_value(auction_id);

                    remove_indices.push(index);
                }
            }

            for index in remove_indices.iter().rev() {
                pending_payments.remove(*index);
            }
        }
    }

    pub fn handle_finished_auctions(&self) {
        let active_auctions = self.active_auctions.lock().unwrap();

        for auction_id in active_auctions.iter() {
            let wrapped_auction_id = ByteWrapper::new(auction_id.clone());

            let mut active_bids = self.active_bids.lock().unwrap();

            if !active_bids.contains_key(&wrapped_auction_id) {
                let our_active_bids = self.our_active_bids.lock().unwrap();

                if let Some(bid_ids) = our_active_bids.get(&wrapped_auction_id) {
                    if bid_ids.is_empty() {
                        self.auction_ended(auction_id.clone());
                    }
                } else {
                    self.auction_ended(auction_id.clone());
                }
            }
        }
    }
}

impl AuctionHandler for AuctionService {
    fn on_auction_started(&self, auction: Auction) {
        self.start_auction(auction);
    }

    fn on_new_bid(&self, bid: Bid) {
        self.handle_new_bid(bid.get_bid_id());
    }

    fn on_auction_ended(&self, auction_id: Vec<u8>) {
        self.auction_ended(auction_id);
    }

    fn on_payment_confirmation(&self, payment_id: Vec<u8>) {
        self.handle_payment_confirmation(payment_id);
    }

    fn on_payment_failure(&self, payment_id: Vec<u8>) {
        self.handle_payment_failure(payment_id);
    }

    fn on_pending_payments(&self) {
        self.handle_pending_payments();
    }

    fn on_finished_auctions(&self) {
        self.handle_finished_auctions();
    }
}
