use serde::{Serialize, Deserialize};
use crate::blockchain::transaction::Transaction;

/// Represents a block in the blockchain.
#[derive(Serialize, Deserialize)]
pub struct Block {
    /// Index of the block in the blockchain.
    pub index: u64,

    /// Time when the block was created.
    pub timestamp: u64,

    /// Data stored in the block.
    pub data: String,

    /// Hash of the previous block in the blockchain.
    pub previous_hash: String,

    /// Transactions stored in the block.
    pub transactions: Vec<Transaction>,

    /// Proof of work for the block.
    pub proof: u64,

    /// Hash of the current block.
    pub hash: String,

    pub nonce: u64,
}
