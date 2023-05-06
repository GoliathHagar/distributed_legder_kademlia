use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Block {
    pub index: u64,            // Index of the block in the blockchain
    pub timestamp: u64,        // Time when the block was created
    pub nonce: u64,        // Time when the block was created
    pub data: String,          // Data stored in the block
    pub previous_hash: String, // Hash of the previous block in the blockchain
    pub hash: String,          // Hash of the current block
}
