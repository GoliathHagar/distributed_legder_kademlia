

use sha2::{Sha256, Digest};

pub fn calculate_sha256(value : &String ) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());

    hasher.finalize().to_vec()
}