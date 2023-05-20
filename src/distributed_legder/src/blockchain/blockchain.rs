use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use crate::blockchain::consensus::ConsensusAlgorithm;
use std::convert::TryInto;
use sha1::Digest;
use crate::constants::utils::calculate_sha256;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    current_transactions: Vec<Transaction>,
    consensus_algorithm: ConsensusAlgorithm,
}

/*
pub fn calculate_hash(block: &Block) -> String {
    let data = format!(
        "{}{}{}{}{}",
        block.index, block.timestamp, block.proof, block.transactions.len(), block.previous_hash
    );

    let hash = calculate_sha256(&data);
    hex::encode(hash)
}

impl Blockchain {
    pub fn new(consensus_algorithm: ConsensusAlgorithm) -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: 0,
            nonce: 0,
            transactions: Vec::new(),
            proof: 0,
            previous_hash: "0".to_string(),
            hash: "".to_string(),
            payload: "".to_string(),
        };
        let hash = calculate_hash(&genesis_block);
        let genesis_block = Block { hash, ..genesis_block };
        let blocks = Vec::from(genesis_block);
        Self {
            blocks,
            current_transactions: vec![],
            consensus_algorithm,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.blocks.last().unwrap();
        let index = self.blocks.len() as u64;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let previous_hash = previous_block.hash.clone();
        let block = Block {
            index,
            timestamp,
            nonce: 0,
            proof: previous_block.proof + 1,
            transactions: self.current_transactions.clone(),
            previous_hash,
            hash: "".to_string(),
            payload: data,
        };
        let hash = calculate_hash(&block);
        let block = Block { hash, ..block };
        self.blocks.push(block);
        self.current_transactions = vec![];
    }

    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn is_valid(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if i > 0 {
                let previous_block = &self.blocks[i - 1];
                if block.previous_hash != previous_block.hash {
                    return false;
                }
            }
            let hash = calculate_hash(block);
            if hash != block.hash {
                return false;
            }
        }
        true
    }

    /// Add a new transaction to the transaction pool
    pub fn add_transaction(&mut self, sender: String, recipient: String, amount: f64) {
        let transaction = Transaction {
            sender,
            recipient,
            amount,
        };
        self.current_transactions.push(transaction);
    }

    /// Mine a new block
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

    fn proof_of_work(&self, last_proof: u128) -> Result<u128, String> {
        let mut proof = 0;
        while !self.valid_proof(last_proof, proof) {
            proof += 1;
        }
        Ok(proof)
    }

    fn valid_proof(&self, last_proof: u128, proof: u128) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let mut hasher = Sha256::new();
        hasher.update(guess.as_bytes());
        let guess_hash = hasher.finalize();
        guess_hash.starts_with(&[0, 0, 0, 0])
    }

    fn delegated_proof_of_stake(&self) -> Result<u128, String> {
        // Logic for delegated proof of stake

        Ok(0)
    }

    pub fn set_consensus_algorithm(&mut self, consensus_algorithm: ConsensusAlgorithm) {
        self.consensus_algorithm = consensus_algorithm;
    }

}
*/