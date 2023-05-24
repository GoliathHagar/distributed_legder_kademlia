use std::net::UdpSocket;

use chrono::Utc;
use log::error;
use ring::rand::SystemRandom;
use ring::signature;
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
    value_to_string::<Block>(block)
}

pub fn string_to_block(bl: String) -> Block {
    string_to_value::<Block>(bl)
}

pub fn string_to_value<T: serde::de::DeserializeOwned>(value: String) -> T {
    match serde_json::from_str(&value) {
        Ok(data) => data,
        Err(e) => {
            error!("Unable to decode block string payload: {}", e.to_string());
            panic!("{}", e.to_string());
        }
    }
}

pub fn value_to_string<T: serde::Serialize>(value: T) -> String {
    match serde_json::to_string(&value) {
        Ok(serialized) => serialized,
        Err(e) => {
            error!("Unable to serialize value");
            panic!("{}", e.to_string());
        }
    }
}


/// Calculates the signature of the transaction data.
pub fn calculate_signature(data: &str) -> String {
    // Generate a new key pair
    let rng = SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();

    // Sign the data
    let signature = key_pair.sign(data.as_bytes());

    // Convert the signature to a hexadecimal string
    hex::encode(signature.as_ref())
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
