use sha2::{Sha256, Digest};
use hex::encode;
use crate::blockchain::block::Block;

pub fn calculate_hash(block: &Block) -> String {
    let mut hasher = Sha256::new();
    let data = format!(
        "{}{}{}{}{}",
        block.index, block.timestamp, block.nonce, block.data, block.previous_hash
    );
    hasher.update(data.as_bytes());
    let hash = hasher.finalize();
    hex::encode(hash)
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: 0,
            nonce: 0,
            data: "Genesis block".to_string(),
            previous_hash: "0".to_string(),
            hash: "".to_string(),
        };
        let hash = calculate_hash(&genesis_block);
        let genesis_block = Block { hash, ..genesis_block };
        let blocks = vec![genesis_block];
        Self { blocks }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.blocks.last().unwrap();
        let index = self.blocks.len() as u64;
        let timestamp = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let previous_hash = previous_block.hash.clone();
        let block = Block { index, timestamp, nonce: previous_block.nonce + 1, data, previous_hash, hash: "".to_string() };
        let data = serde_json::to_string(&block).unwrap();
        let hash = calculate_hash(&block);
        let block = Block { hash, ..block };
        self.blocks.push(block);
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
}