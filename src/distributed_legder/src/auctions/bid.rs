use serde::{Deserialize, Serialize};

use crate::blockchain::transaction::Transaction;
use crate::constants::utils::get_timestamp_now;
use crate::network::key::Key;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bid {
    pub bid_id: Key,
    pub user_node_id: Key,
    pub auction_id: Key,
    pub user_public_key: String,
    pub bid_time: u64,
    pub amount: f64,
}

impl Bid {
    pub fn new(user_node_id: Key, user_public_key: String, bid_amount: f64, auction_id: Key) -> Bid {
        let bid_id = Key::new(format!("{:?}@{}", auction_id, get_timestamp_now()));

        Bid {
            bid_id,
            user_node_id,
            auction_id,
            user_public_key,
            bid_time: get_timestamp_now(),
            amount: bid_amount,
        }
    }

    pub fn to_transaction(self) -> Transaction {
        Transaction::new(format!("{:?}", self.user_node_id),
                         format!("{:?}", self.user_node_id),
                         self.amount,
        )
    }
}