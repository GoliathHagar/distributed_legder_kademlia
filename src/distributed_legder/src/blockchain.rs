
pub struct Block {
    index: u64,            // Index of the block in the blockchain
    timestamp: u64,        // Time when the block was created
    data: String,          // Data stored in the block
    previous_hash: String, // Hash of the previous block in the blockchain
    hash: String,          // Hash of the current block
}

use crypto_hash::{Digest, Sha256};

fn calculate_hash(block: &Block) -> String {
    let mut hasher = Sha256::new();
    let data = format!("{}{}{}{}", block.index, block.timestamp, block.data, block.previous_hash);
    hasher.update(data.as_bytes());
    let hash = hasher.finalize();
    hex::encode(hash)
}

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: 0,
            data: "Genesis block".to_string(),
            previous_hash: "0".to_string(),
            hash: "".to_string(),
        };
        let hash = calculate_hash(&genesis_block);
        let mut blocks = Vec::new();
        let genesis_block = Block { hash, ..genesis_block };
        blocks.push(genesis_block);
        Self { blocks }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.blocks.last().unwrap();
        let index = self.blocks.len() as u32;
        let timestamp = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let previous_hash = previous_block.hash.clone();
        let block = Block { index, timestamp, data, previous_hash, hash: "".to_string() };
        let hash = calculate_hash(&block);
        let block = Block { hash, ..block };
        self.blocks.push(block);
    }

    fn is_valid(&self) -> bool {
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
}

