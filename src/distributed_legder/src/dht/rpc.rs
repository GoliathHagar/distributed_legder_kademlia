use serde::{Deserialize, Serialize};
use crate::network::key::Key;
use crate::network::node::Node;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Rpc{
    Ping,
    Store(String, String),
    FindNode(Key),
    FindValue(Key),

    Pong,
    FindNodeReply(Vec<Node>),
    FindValueReply(Key, String),
}