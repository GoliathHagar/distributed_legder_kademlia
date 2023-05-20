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

impl Transaction {
    /// Creates a new transaction with the specified details.
    pub fn new(id: String, sender: String, recipient: String, amount: f64, signature: String) -> Self {
        Transaction {
            id,
            sender,
            recipient,
            amount,
            signature,
        }
    }
}