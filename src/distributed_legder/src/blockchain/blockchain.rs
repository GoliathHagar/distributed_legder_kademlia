use std::collections::HashMap;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use crate::blockchain::consensus::ConsensusAlgorithm;
use std::convert::TryInto;
use sha1::Digest;
use crate::constants::fixed_sizes::BLOCKCHAIN_VERSION;
use crate::constants::utils::{calculate_block_hash, calculate_sha256, get_timestamp_now};

pub struct Blockchain {
    pub blocks: Vec<Block>, // valid by pow/pos blocks
    current_transactions: Vec<Transaction>,
    consensus_algorithm: ConsensusAlgorithm,
}

impl Blockchain {
    pub fn new(consensus_algorithm: ConsensusAlgorithm) -> Self {
        let mut  genesis_block = Block::new(
            0,
            "0".to_string(),
            "0".to_string(),
            Vec::new(),
        );

        let hash = calculate_block_hash(&genesis_block);
        genesis_block.header.hash = hash;

        let blocks = Vec::from([genesis_block]);

        Self {
            blocks,
            current_transactions: Vec::new(),
            consensus_algorithm,
        }
    }

    pub fn create_block(&mut self) {
        if self.current_transactions.is_empty() { return; }

        let previous_block = self.blocks.last().unwrap();
        let index = previous_block.header.index + 1;

        let timestamp = get_timestamp_now();

        let previous_hash = previous_block.header.hash.clone();

        let mut block = Block::new(
            index,
            previous_hash,
            "".to_string(),
            self.current_transactions.clone()
        );

        let hash = calculate_block_hash(&block);
        block.header.hash = hash;
        self.blocks.push(block);
        self.current_transactions = Vec::new();
    }

    pub fn add_block(&mut self, block : Block) {

    }

    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn is_valid(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if i > 0 {
                let previous_block = &self.blocks[i - 1];
                if block.header.previous_hash != previous_block.header.hash {
                    return false;
                }
            }

            let hash = calculate_block_hash(block);
            if hash != block.header.hash {
                return false;
            }
        }
        true
    }

   /* /// Add a new transaction to the transaction pool
    pub fn add_transaction(&mut self, sender: String, recipient: String, amount: f64) {
        let transaction = Transaction {
            sender,
            recipient,
            amount,
        };
        self.current_transactions.push(transaction);
    }*/

    fn valid_proof(self, last_proof: u128, proof: u128) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let mut hasher = Sha256::new();
        hasher.update(guess.as_bytes());
        let guess_hash = hasher.finalize();
        guess_hash.starts_with(&[0, 0, 0, 0])
    }

  /*  /// Mine a new block
    pub fn mine_block(&mut self, miner_address: String) -> Result<(), String> {
        let last_block = self.blocks.last().clone().unwrap();
        let proof = match self.consensus_algorithm {
            ConsensusAlgorithm::ProofOfWork => self.proof_of_work(last_block.proof as u128)?,
            ConsensusAlgorithm::DelegatedProofOfStake => self.delegated_proof_of_stake()?,
        };
        let previous_hash = calculate_hash(&last_block);

        let new_block = Block {
            index: last_block.index + 1,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            nonce: 0,
            transactions: self.current_transactions.clone(),
            proof: proof.try_into().unwrap(),
            previous_hash,
            hash: String::new(),
            payload: "".to_string(),
        };

        // Calculate the hash of the new block and update the block with it
        let hash = calculate_hash(&new_block);
        let new_block = Block { hash, ..new_block };

        // Add the new block to the blockchain and clear the transaction pool
        self.blocks.push(new_block);
        self.current_transactions = vec![];

        Ok(())
    }
*/
    fn delegated_proof_of_stake(&self) -> Result<u128, String> {
        // Logic for delegated proof of stake

        Ok(0)
    }

    pub fn set_consensus_algorithm(&mut self, consensus_algorithm: ConsensusAlgorithm) {
        self.consensus_algorithm = consensus_algorithm;
    }

}
