use crate::constants::utils::calculate_sha256;

pub struct Bid {
    bid_id: String,
    user_node_id: String,
    auction_id: String,
    user_public_key: String,
    encrypted_bid: String,
}

impl Bid {
    pub fn new(user_node_id: String, user_public_key: String, encrypted_bid: String, auction_id: String) -> Bid {
        let bid_id = hex::encode(calculate_sha256(&auction_id.clone()));

        Bid {
            bid_id,
            user_node_id,
            auction_id,
            user_public_key,
            encrypted_bid,
        }
    }
}