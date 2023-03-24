use std::fmt::{Debug};
use crate::network::key::Key;
use serde::{Serialize,Deserialize};
use crate::dht::rpc::Rpc;


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum DatagramType{
    REQUEST,
    RESPONSE,
    KILL
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Datagram {
    pub data_type : DatagramType,
    pub token_id: Key,
    pub source : String,
    pub destination: String,
    pub data: Rpc
}

