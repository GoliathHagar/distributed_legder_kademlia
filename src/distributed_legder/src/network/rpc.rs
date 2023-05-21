use serde::{Deserialize, Serialize};

use crate::constants::multicast_info_type::MulticastInfoType;
use crate::network::key::Key;
use crate::network::node::Node;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Rpc {
    Ping,
    Store(String, String),
    FindNode(Key),
    FindValue(String),

    Multicasting(String, MulticastInfoType, String), //id, type_info, serialized info

    Pong,
    FindNodeReply(Vec<Node>),
    FindValueReply(String, String),
}
