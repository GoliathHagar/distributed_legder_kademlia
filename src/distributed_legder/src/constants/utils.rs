use std::net::UdpSocket;

use chrono::Utc;
use log::error;
use sha1::Sha1;
use sha2::{Digest, Sha256};

use crate::blockchain::block::Block;

pub fn get_timestamp_now() -> u64 {
    Utc::now().timestamp_nanos() as u64
}

pub fn calculate_block_hash(block: &Block) -> String {
    let blk_hdr = block.header.clone();
    let data = format!(
        "{}{}{}{}{}{}",
        blk_hdr.index, blk_hdr.version, blk_hdr.timestamp, blk_hdr.previous_hash,
        blk_hdr.merkle_root, blk_hdr.nonce);

    let hash = calculate_sha256(&data);
    hex::encode(hash)
}

pub fn calculate_sha256(value: &String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());

    hasher.finalize().to_vec()
}

pub fn calculate_sha1(value: &String) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(value.as_bytes());

    hasher.finalize().to_vec()
}

pub fn block_to_string(block: Block) -> String {
    match serde_json::to_string(&block) {
        Ok(d) => d,
        Err(e) => {
            error!("Unable to serialize message");
            panic!("{}", e.to_string());
        }
    }
}

pub fn get_local_ip() -> String {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return "127.0.0.1".to_string(),
    };

    match socket.connect("1.1.1.1:80") {
        Ok(()) => (),
        Err(_) => return "127.0.0.1".to_string(),
    };

    return match socket.local_addr() {
        Ok(addr) => addr.ip().to_string(),
        Err(_) => "127.0.0.1".to_string(),
    };
}
