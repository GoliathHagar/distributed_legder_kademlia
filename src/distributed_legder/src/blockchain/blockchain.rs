use std::fs::create_dir_all;
use std::io::Write;
use std::sync::{Arc, Mutex};

use log::{debug, error, info, warn};

use crate::blockchain::block::Block;
use crate::blockchain::consensus::ConsensusAlgorithm;
use crate::blockchain::miner::Miner;
use crate::blockchain::transaction::Transaction;
use crate::constants::blockchain_node_type::BlockchainNodeType;
use crate::constants::fixed_sizes::DUMP_STATE_TIMEOUT;
use crate::constants::utils::calculate_block_hash;

#[derive(Clone, Debug)]
pub struct Blockchain {
    // valid by pow/pos blocks
    pub(self) blocks: Arc<Mutex<Vec<Block>>>,
    pub(self) current_transactions: Arc<Mutex<Vec<Transaction>>>,
    pub(self) miner: Arc<Miner>,
    pub consensus_algorithm: ConsensusAlgorithm,
    pub(self) node_type: BlockchainNodeType,
}

impl Blockchain {
    pub fn new(consensus_algorithm: ConsensusAlgorithm, node_type: BlockchainNodeType) -> Self {
        info!("Blockchain initiating with consensus {:?}", consensus_algorithm);
        Self {
            blocks: Arc::new(Mutex::new(Vec::new())),
            current_transactions: Arc::new(Mutex::new(Vec::new())),
            miner: Arc::new(Miner::new(consensus_algorithm)),
            consensus_algorithm,
            node_type,
        }
    }

    pub fn init(self: Arc<Self>, path: &str) -> Option<Block> {
        let mut res = None;
        if self.node_type == BlockchainNodeType::Bootstrap {
            info!("Blockchain initiating instantiating genesis block",);
            let mut genesis_block = Block::new(
                0,
                "0".to_string(),
                "0".to_string(),
                Vec::from([Transaction::new("".to_string(), "".to_string(), 0.0)]),
            );
            genesis_block.header.timestamp = 0;

            let hash = calculate_block_hash(&genesis_block);
            genesis_block.header.hash = hash;

            let nonce = self.miner.clone().mine_block(genesis_block.clone());
            genesis_block.header.nonce = nonce;
            genesis_block.header.hash = calculate_block_hash(&genesis_block);

            let mut blocks = match self.blocks.lock() {
                Ok(sv) => sv,
                Err(e) => {
                    error!("Failed to acquire lock on blocks");
                    panic!("{}", e.to_string());
                }
            };

            blocks.push(genesis_block.clone());


            info!("Blockchain started genesis block {:?}",genesis_block);

            res = Some(genesis_block);
        }
        //dump state
        let copy_self = self.clone();
        let p_dir = (path.to_owned() + ".blockchain.json").to_string();

        info!("Blockchain dump state started");
        std::thread::spawn(move || loop {
            copy_self.dump_state(p_dir.as_str());
            std::thread::sleep(std::time::Duration::from_millis(DUMP_STATE_TIMEOUT));
        });


        res
    }

    pub fn create_block(self: Arc<Self>) -> Option<Block> {
        let mut transactions = match self.current_transactions.lock() {
            Ok(sv) => sv,
            Err(_) => {
                error!("Failed to acquire lock on transactions");
                return None;
            }
        };

        let blocks = match self.blocks.lock() {
            Ok(sv) => sv,
            Err(_) => {
                error!("Failed to acquire lock on blocks");
                return None;
            }
        };

        if transactions.is_empty() { return None; }

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

        Some(block)
    }

    pub fn add_block(self: Arc<Self>, block: Block) -> bool {
        let mut blocks = match self.blocks.lock() {
            Ok(sv) => sv,
            Err(_) => {
                error!("Failed to acquire lock on blocks");
                return false;
            }
        };

        debug!("Blockchain blocks {:?}", blocks.clone());

        if blocks.is_empty() && block.is_valid() {
            blocks.push(block);

            return true;
        } else {
            let prv_hsh = blocks.last().unwrap().clone().header.hash;

            if block.header.previous_hash.eq(&prv_hsh) && block.is_valid() {
                blocks.push(block);

                return true;
            } else {
                warn!("Block validation failed");
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

    }

    // Mine a new block
    pub fn mine_block(self: Arc<Self>, block: Block) -> Block {
        let mut blk = block.clone();

        let nonce = self.miner.clone().mine_block(block);

        blk.header.nonce = nonce;

        blk.header.hash = calculate_block_hash(&blk);

        blk
    }

    #[allow(dead_code)]
    fn delegated_proof_of_stake(self: Arc<Self>) -> Result<u128, String> {
        // Logic for delegated proof of stake

        Ok(0)
    }

    pub fn notify_miner(self: Arc<Self>, hash: String) {
        self.miner.clone().set_mined_hash(hash);
    }

    fn dump_state(&self, path: &str) {
        if let Err(_) = create_dir_all("state_dumps") {
            error!("Unable to create state dumps diretory");
            return;
        }

        let transactions = match self.current_transactions.lock() {
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

        let mut parsed_blocks = Vec::new();

        let flattened: Vec<Block> = blocks.clone();
        let t: Vec<Transaction> = transactions.clone();

        for kb in flattened {
            let bucket = serde_json::json!(
                {
                    "block": {
                        "header": kb.header,
                        "transactions": kb.transactions,
                    }
                }
            );

            parsed_blocks.push(bucket);
        }

        let json = serde_json::json!(
            {
                "blockchain": {
                    "blocks":parsed_blocks ,
                    "pendent_translations": t
                },
                "consensus_algorithm": self.consensus_algorithm
            }
        );


        let mut file = match std::fs::File::create(path) {
            Ok(f) => f,
            Err(_) => {
                error!("Unable to create dump file");
                return;
            }
        };

        if let Err(_) = file.write_all(&json.to_string().as_bytes()) {
            error!("Unable to create dump file Json");
        }

        let mut diagram = match std::fs::File::create(format!("{}.plantuml", path)) {
            Ok(d) => d,
            Err(_) => {
                error!("Unable to create dump file Diagram");
                return;
            }
        };

        if let Err(_) = diagram.write_all("@startjson\n".to_string().as_bytes()) {
            error!("Unable to write to dump file");
            return;
        }

        if let Err(_) = diagram.write_all(&json.to_string().as_bytes()) {
            error!("Unable to write to dump file");
            return;
        }

        if let Err(_) = diagram.write_all("\n@endjson".to_string().as_bytes()) {
            error!("Unable to write to dump file");
            return;
        }
    }
}
