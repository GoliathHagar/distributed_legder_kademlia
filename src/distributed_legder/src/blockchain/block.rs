use serde::{Serialize, Deserialize};
use crate::blockchain::transaction::Transaction;
use crate::constants::fixed_sizes::BLOCKCHAIN_VERSION;
use crate::constants::utils::{calculate_sha256, get_timestamp_now};

/// Represents a block in the blockchain.


#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn new(index: u64, previous_hash: String,
               hash: String, transactions: Vec<Transaction> ) -> Block {
        Self{
            header: BlockHeader::new( index,
                                      BLOCKCHAIN_VERSION,
                                      previous_hash,
                                      hash,
                                      Block::calculate_merkle_tree(transactions.clone()),
                                      0,),
            transactions
        }
    }

    fn calculate_merkle_tree(transations : Vec<Transaction>) -> String {

        let data = "".to_string();

        hex::encode(calculate_sha256(&data))
    }

    pub fn validate_merkle_tree(self) -> bool {
        let mt =  Block::calculate_merkle_tree(self.transactions);

        self.header.merkle_root.eq(&mt)
    }

    pub fn is_valid(&self){

    }

}
