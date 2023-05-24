use serde::{Deserialize, Serialize};

use crate::blockchain::transaction::Transaction;
use crate::constants::fixed_sizes::{BLOCKCHAIN_MINING_DIFFICULTY, BLOCKCHAIN_VERSION};
use crate::constants::utils::{calculate_block_hash, calculate_sha256, get_timestamp_now};

/// Represents a block in the blockchain.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BlockHeader {
    // Index of the block in the blockchain.
    pub index: u64,

    /// Data stored in the block.
    pub version: u8,

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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

   /*
    fn calculate_merkle_tree(transations : Vec<Transaction>) -> String {

        let data = "".to_string();

        hex::encode(calculate_sha256(&data))
    }
    */

    fn calculate_merkle_tree(transactions: Vec<Transaction>) -> String {
        if transactions.is_empty() { return "".to_string(); }

        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|transaction| transaction.id.clone())
            .collect();

        while hashes.len() > 1 {
            let mut next_level: Vec<String> = Vec::new();

            for i in (0..hashes.len()).step_by(2) {
                let left = &hashes[i];
                let right = if i + 1 < hashes.len() {
                    &hashes[i + 1]
                } else {
                    left
                };

                let combined = format!("{}{}", left, right);
                let hash = hex::encode(calculate_sha256(&combined));
                next_level.push(hash);
            }

            hashes = next_level;
        }

        return if !hashes.is_empty() {
             hashes[0].clone()
        }
        else { "".to_string() }
    }

    pub fn validate_merkle_tree(self) -> bool {
        let mt =  Block::calculate_merkle_tree(self.transactions);

        self.header.merkle_root.eq(&mt)
    }


    pub fn is_valid(&self) -> bool {
        let string_hash = calculate_block_hash(self);

        let decoded_hash = hex::decode(string_hash).unwrap();
        let bytes_needed = (BLOCKCHAIN_MINING_DIFFICULTY + 7) / 8;

        // Check if the first bytes_needed bytes are all zeros
        decoded_hash[..bytes_needed].iter().all(|&byte| byte == 0)

    }

}

impl PartialEq<Self> for Block {
    fn eq(&self, other: &Self) -> bool {
        let mut eq = true;

        if self.transactions.len() == other.transactions.len() {
            for i in 0..self.transactions.len() {
                eq = eq && (self.transactions[i] == other.transactions[i])
            }

            self.header == other.header && eq
        } else {
            return false;
        }
    }
}