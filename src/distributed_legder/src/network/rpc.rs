use crate::network::key::Key;
use crate::network::node::Node;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Rpc {
    Ping,
    Store(String, String),
    FindNode(Key),
    FindValue(String),

    Multicasting(String, String),

    Pong,
    FindNodeReply(Vec<Node>),
    FindValueReply(String, String),
}
