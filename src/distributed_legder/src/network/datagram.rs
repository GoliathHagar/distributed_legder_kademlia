use crate::network::key::Key;
use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum DatagramType{
    REQUEST,
    RESPONSE,
    KILL
}


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Datagram {
    pub data_type : DatagramType,
    pub token_id: String,
    pub source : String,
    pub destination: String,
    pub data:String
}

