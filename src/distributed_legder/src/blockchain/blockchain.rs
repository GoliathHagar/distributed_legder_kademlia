use std::sync::{Arc, Mutex};

use log::error;
use sha1::Digest;
use sha2::Sha256;

use crate::blockchain::block::Block;
use crate::blockchain::consensus::ConsensusAlgorithm;
use crate::blockchain::miner::Miner;
use crate::blockchain::transaction::Transaction;
use crate::constants::blockchain_node_type::BlockchainNodeType;
use crate::constants::fixed_sizes::ZEROS_HASH;
use crate::constants::utils::calculate_block_hash;

pub struct Blockchain {
    pub(self) blocks: Arc<Mutex<Vec<Block>>>,
    // valid by pow/pos blocks
    pub(self) current_transactions: Arc<Mutex<Vec<Transaction>>>,
    pub(self) miner: Arc<Miner>,
    pub consensus_algorithm: ConsensusAlgorithm,
    pub(self) node_type: BlockchainNodeType,
}

impl Blockchain {
    pub fn new(consensus_algorithm: ConsensusAlgorithm, node_type: BlockchainNodeType) -> Self {
        Self {
            blocks: Arc::new(Mutex::new(Vec::new())),
            current_transactions: Arc::new(Mutex::new(Vec::new())),
            miner: Arc::new(Miner::new(consensus_algorithm)),
            consensus_algorithm,
            node_type,
        }
    }

    pub fn init(self) -> Option<Block> {
        if self.node_type == BlockchainNodeType::Bootstrap {
            let mut genesis_block = Block::new(
                0,
                "0".to_string(),
                "0".to_string(),
                Vec::new(),
            );
            genesis_block.header.timestamp = 0;

            let hash = calculate_block_hash(&genesis_block);
            genesis_block.header.hash = hash;

            let nonce = self.miner.mine_block(genesis_block.clone());
            genesis_block.header.nonce = nonce;

            let mut blocks = match self.blocks.lock() {
                Ok(sv) => sv,
                Err(e) => {
                    error!("Failed to acquire lock on blocks");
                    panic!("{}", e.to_string());
                }
            };

            blocks.push(genesis_block.clone());

            return Some(genesis_block);
        }
        None
    }

    pub fn create_block(self: Arc<Self>) {
        let mut transactions = match self.current_transactions.lock() {
            Ok(sv) => sv,
            Err(_) => {
                error!("Failed to acquire lock on transactions");
                return;
            }
        };

        let blocks = match self.blocks.lock() {
            Ok(sv) => sv,
            Err(_) => {
                error!("Failed to acquire lock on blocks");
                return;
            }
        };

        if transactions.is_empty() { return; }

        let previous_block = blocks.last().unwrap();
        let index = previous_block.header.index + 1;

        let previous_hash = previous_block.header.hash.clone();

        let mut block = Block::new(
            index,
            previous_hash,
            "".to_string(),
            transactions.clone(),
        );

        let hash = calculate_block_hash(&block);
        block.header.hash = hash;
        transactions.clear();
    }

    pub fn add_block(self: Arc<Self>, block: Block) -> bool {
        let mut blocks = match self.blocks.lock() {
            Ok(sv) => sv,
            Err(_) => {
                error!("Failed to acquire lock on blocks");
                return false;
            }
        };

        if blocks.is_empty()
            && block.header.previous_hash.eq(ZEROS_HASH) && block.is_valid() {
            blocks.push(block);

            return true;
        } else {
            let prv_hsh = blocks.last().unwrap().clone().header.hash;

            if block.header.previous_hash == prv_hsh && block.is_valid() {
                blocks.push(block);

                return true;
            }
            else {
                //
                //todo: ask the network the previous(kademlia);
            }

        }

        false
    }

    pub fn block_count(self: Arc<Self>) -> usize {
        let blocks = match self.blocks.lock() {
            Ok(sv) => sv,
            Err(e) => {
                error!("Failed to acquire lock on blocks");
                panic!("{}", e.to_string());
            }
        };

        blocks.len()
    }

    pub fn is_chain_valid(self: Arc<Self>) -> bool {
        let blocks = match self.blocks.lock() {
            Ok(sv) => sv,
            Err(e) => {
                error!("Failed to acquire lock on blocks");
                panic!("{}", e.to_string());
            }
        };

        for (i, block) in blocks.iter().enumerate() {
            if i > 0 {
                let previous_block = &blocks[i - 1];
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

    /// Add a new transaction to the transaction pool
    pub fn add_transaction(self: Arc<Self>, transaction: Transaction) {
        // Add the transaction to the current_transactions list
        let mut current_transactions = match self.current_transactions.lock() {
            Ok(sv) => sv,
            Err(_) => {
                error!("Failed to acquire lock on transactions");
                return;
            }
        };

        // Sign the transaction
        let mut tst = transaction.clone();
        tst.sign();

        current_transactions.push(tst.clone());

        // Add the transaction to the last block if it exists
        //TOdo: transacitons limites reached create blocl
        /*let blocks = match self.blocks.lock() {
            Ok(sv) => sv,
            Err(_) => {
                error!("Failed to acquire lock on blocks");
                return;
            }
        };
        if let Some(last_block) = blocks.last_mut() {
            last_block.transactions.push(transaction);
        }*/
    }

    // Mine a new block
    pub fn mine_block(self: Arc<Self>, block: Block) -> Block {
        let mut blk = block.clone();

        let nonce = self.miner.mine_block(block);

        blk.header.nonce = nonce;

        blk.header.hash = calculate_block_hash(&blk);

        blk
    }

    fn delegated_proof_of_stake(self: Arc<Self>) -> Result<u128, String> {
        // Logic for delegated proof of stake

        Ok(0)
    }

    pub fn notify_miner(self: Arc<Self>, hash: String) {
        self.miner.set_mined_hash(hash);
    }
}
