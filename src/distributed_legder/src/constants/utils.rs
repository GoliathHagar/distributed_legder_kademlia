use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::net::UdpSocket;

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
