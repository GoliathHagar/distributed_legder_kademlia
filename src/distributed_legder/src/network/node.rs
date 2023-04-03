use crate::network::key::Key;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize )]
pub struct Node {
    pub ip: String,
    pub port: u16,
    pub id: Key,
}

struct  ThrustAndReputation{
    total_interation : u32,
    successfully_interation: u32
}

impl Node{

    pub fn new(ip: String, port: u16) -> Self {
        let node_id = format!("{}:{}", ip, port);
        let id = Key::new(node_id);

        Node { ip, port, id }
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn thrust(&self) -> usize {

        //nd = od × b + (1 − b) ×1/t
        0
    }
}

