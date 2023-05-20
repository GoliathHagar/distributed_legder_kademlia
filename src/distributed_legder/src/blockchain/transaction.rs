use serde::{Serialize, Deserialize};

/// Represents a transaction in the blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// The unique ID of the transaction.
    pub id: String,

    /// The sender of the transaction.
    pub sender: String,

    /// The recipient of the transaction.
    pub recipient: String,

    /// The amount of the transaction.
    pub amount: f64,

    /// The signature of the transaction signed by the auction owner.
    pub signature: String,
}


