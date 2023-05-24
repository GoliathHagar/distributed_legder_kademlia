use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum MulticastInfoType {
    Block,
    Auction,
    Bid,
    Miscellaneous,
}