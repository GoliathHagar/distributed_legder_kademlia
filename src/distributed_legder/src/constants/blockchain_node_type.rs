use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum BlockchainNodeType {
    Bootstrap,
    Miner,

}