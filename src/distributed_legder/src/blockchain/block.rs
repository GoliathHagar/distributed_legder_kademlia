use serde::{Serialize, Deserialize};
use crate::blockchain::transaction::Transaction;
use crate::constants::utils::{calculate_sha256, get_timestamp_now};

/// Represents a block in the blockchain.


#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader{
    // Index of the block in the blockchain.
    pub index: u64,

    /// Data stored in the block.
    pub version : u8,

    // Time when the block was created.
    pub timestamp: u64,

    /// Hash of the previous block in the blockchain.
    pub previous_hash: String,

    /// Hash of the current block.
    pub hash: String,

    /// Hash of the current block.
    pub merkle_root: String,

    /// Proof of work for the block.
    pub nonce: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub header : BlockHeader,

    /// Transactions stored in the block.
    pub transactions: Vec<Transaction>,
}
impl BlockHeader{
    pub fn new(index: u64, version : u8, previous_hash: String,
               hash: String, merkle_root: String, nonce: u128, ) -> BlockHeader {
        Self{
            index,
            version,
            timestamp: get_timestamp_now(),
            previous_hash,
            hash,
            merkle_root,
            nonce,
        }
    }
}

impl Block {
    pub fn new(index: u64, version : u8, previous_hash: String,
               hash: String, merkle_root: String, nonce: u128, transactions: Vec<Transaction> ) -> Block {
        Self{
            header: BlockHeader::new( index: index,
                                      version: version,
                                      timestamp: get_timestamp_now(),
                                      previous_hash,
                                      hash,
                                      nonce,),
            transactions
        }
    }

    fn calculate_merkle_tree(transations : Vec<Transaction>) -> String {

        let data = "".to_string();

        hex::encode(calculate_sha256(&data))
    }
}
