use log::warn;
use serde::{Deserialize, Serialize};

use crate::auctions::bid::Bid;
use crate::constants::utils::{calculate_signature, get_timestamp_now};
use crate::network::key::Key;
use crate::network::node::Node;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Auction {
    pub auction_id: Key,
    pub auctioneer_node_id: Key,
    pub auction_name: String,
    pub minimum_bid: f64,
    // in minutes
    pub initial_ts: u64,
    pub auction_duration: usize,
    pub auctioneer_pk: String,
    pub signature: String,

    pub bids: Vec<Bid>,
}

impl Auction {
    pub fn new(auction_name: String, auctioneer_node_id: Key, minimum_bid: f64, initial_ts: u64,
               auction_duration: usize, auctioneer: String, signature: String) -> Self {
        let auction_id = Key::new(format!("{}@{}", auction_name, initial_ts));

        Auction {
            auction_id,
            auctioneer_node_id,
            auction_name,
            minimum_bid,
            auction_duration,
            initial_ts,
            auctioneer_pk: auctioneer,
            signature,
            bids: Vec::new(),
        }
    }

    pub fn with_defaults(auction_name: String, auctioneer_node_id: Key, minimum_bid: f64,
                         auction_duration: usize, auctioneer: String) -> Self {
        let initial_ts = get_timestamp_now();
        let auction_id = Key::new(format!("{}@{}", auction_name, initial_ts));

        Auction {
            auction_id,
            auctioneer_node_id,
            auction_name,
            minimum_bid,
            auction_duration,
            initial_ts,
            auctioneer_pk: auctioneer,
            signature: "".to_string(),
            bids: Vec::new()
        }
    }

    pub fn get_final_ts(&self) -> u64 {
        self.initial_ts + (self.auction_duration * 60_000_000_000) as u64
    }

    pub fn initialize_new_auction(node: Node, auction_name: String, duration_in_millis: usize, min_bid: f64, owner_keys: String) -> Self {
        let auction = Auction::with_defaults(auction_name,
                                             node.id.clone(), min_bid, duration_in_millis, owner_keys.clone());

        let signature = calculate_signature(owner_keys.as_str());

        Auction {
            signature,
            ..auction
        }
    }

    pub fn add_bid(&mut self, bid: Bid) {
        let current_time = get_timestamp_now();

        if current_time <= self.get_final_ts() && self.minimum_bid <= bid.amount {
            self.bids.push(bid);
        } else {
            warn!("Auction is closed. Cannot add bid or Bid amount less them minimum bid");
        }
    }
}
