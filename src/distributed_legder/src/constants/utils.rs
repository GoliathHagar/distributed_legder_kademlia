use std::net::UdpSocket;
use sha2::{Sha256, Digest};

pub fn calculate_sha256(value : &String ) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());

    hasher.finalize().to_vec()
}

pub fn get_local_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("1.1.1.1:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}