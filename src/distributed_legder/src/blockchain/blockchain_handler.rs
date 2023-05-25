use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

use log::{error, info};

use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;
use crate::blockchain::consensus::ConsensusAlgorithm;
use crate::blockchain::transaction::Transaction;
use crate::constants::blockchain_node_type::BlockchainNodeType;
use crate::constants::multicast_info_type::MulticastInfoType;
use crate::constants::utils::{block_to_string, string_to_block};
use crate::dht::kademlia::KademliaDHT;
use crate::network::node::Node;
use crate::network::rpc::Rpc;

#[derive(Clone, Debug)]
pub struct BlockchainHandler {
    pub(self) blockchain: Arc<Blockchain>,
    pub kademlia: Arc<KademliaDHT>,
    pub(self) node_type: BlockchainNodeType,
    pub receiver_subscription: Arc<Mutex<Receiver<(MulticastInfoType, String)>>>,
    pub(self) sender_subscription: Arc<Mutex<Sender<(MulticastInfoType, String)>>>,
}

impl BlockchainHandler {
    pub fn new(consensus: ConsensusAlgorithm, node: Node, node_type: BlockchainNodeType, bootstrap: Option<Node>) -> BlockchainHandler {
        let kad = Arc::new(KademliaDHT::new(node, bootstrap.clone()));
        let chain = Arc::new(Blockchain::new(consensus, node_type.clone()));
        let (sender, receiver) = mpsc::channel();
        Self {
            blockchain: chain,
            kademlia: kad,
            node_type,
            receiver_subscription: Arc::new(Mutex::new(receiver)),
            sender_subscription: Arc::new(Mutex::new(sender)),
        }
    }

    pub fn add_transaction_create_block(&self, transaction: Transaction) -> Option<Block> {
        self.blockchain.clone().add_transaction(transaction);
        self.blockchain.clone().create_block()
    }

    pub fn add_transaction_to_blockchain(self, transaction: Transaction) {
        self.blockchain.add_transaction(transaction)
    }


    pub fn start(self, dump_path: &str) -> std::thread::JoinHandle<()> {
        let network_thread = self.kademlia.clone().init(Some(dump_path.to_string()));
        let block = self.blockchain.clone().init(dump_path);

        if let Some(b) = block {
            let data = block_to_string(b.clone());
            self.kademlia.clone().broadcast_info((b.header.hash, MulticastInfoType::Miscellaneous, data))
        }

        self.handel_broadcast_blocks();

        network_thread
    }

    fn handel_broadcast_blocks(self) {
        let kad = self.kademlia.clone();
        let blockchain = self.blockchain.clone();

        info!("Blockchain started awaiting blocks ...");

        std::thread::spawn(move || loop {
            let payload = kad.clone().multicast_subscriber();
            info!("Blockchain received payload {:?}", payload);

            match payload {
                Rpc::Multicasting(id, payload_type, p) => {
                    if payload_type == MulticastInfoType::Block {
                        let block: Block = string_to_block(p);

                        if self.clone().node_type == BlockchainNodeType::Bootstrap && blockchain.clone().is_chain_valid() {
                            info!("Blockchain received block {:?}", block.clone());

                            if !blockchain.clone().add_block(block.clone()) && block.is_valid() {
                                let mut found = Vec::new();
                                let mut bl = block.clone();
                                let mut searching = true;

                                while searching {
                                    if let Some(b) = kad.clone().get(bl.header.previous_hash.clone()) {
                                        let new = string_to_block(b);

                                        if blockchain.clone().add_block(new.clone()) {
                                            searching = false;
                                        }

                                        bl = new.clone();
                                        found.push(new);
                                    } else {
                                        searching = false;
                                        error!("Previous block not found in network {:?}", bl);
                                    }
                                    for b in found.iter() {
                                        blockchain.clone().add_block(b.clone());
                                    }
                                }
                                found.reverse();
                            }
                        } else if self.clone().node_type == BlockchainNodeType::Miner || self.clone().node_type == BlockchainNodeType::Bootstrap {
                            if block.is_valid() {
                                blockchain.clone().notify_miner(id)
                            } else {
                                self.clone().worker_nine(block);
                            }
                        }
                    } else if payload_type == MulticastInfoType::Bid {
                        self.sender_subscription.lock().unwrap()
                            .send((MulticastInfoType::Bid, p)).expect("Unable to send received Bid");
                    } else if payload_type == MulticastInfoType::Auction {
                        self.sender_subscription.lock().unwrap()
                            .send((MulticastInfoType::Auction, p)).expect("Unable to send received Auction");
                    }
                }
                _ => { continue; }
            };
        });
    }

    fn worker_nine(self, block: Block) {
        let bch = self.blockchain.clone();
        let kad = self.kademlia.clone();
        std::thread::spawn(
            move || {
                let blk = bch.clone().mine_block(block.clone());

                bch.add_block(blk.clone());

                kad.broadcast_info(
                    (block.header.hash, MulticastInfoType::Block, block_to_string(blk))
                );
            }
        );
    }
}