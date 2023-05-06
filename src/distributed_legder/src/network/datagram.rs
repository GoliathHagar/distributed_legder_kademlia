use crate::network::rpc::Rpc;
use crate::network::key::Key;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum DatagramType {
    REQUEST,
    RESPONSE,
    KILL,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Datagram {
    pub data_type: DatagramType,
    pub token_id: Key,
    pub source: String,
    pub destination: String,
    pub data: Rpc,
}

impl Datagram {
    pub fn extract_src_ip_port(self) -> Option<(String, u16)> {
        let info: Vec<&str> = self.source.split(":").collect();

        if info.len() < 2 {
            return None;
        }

        if let Ok(p) = info[1].parse() {
            return Some((info[0].to_string(), p));
        }

        None
    }
}
