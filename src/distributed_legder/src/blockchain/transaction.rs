use serde::{Serialize, Deserialize};
use ring::digest;
use ring::rand::SystemRandom;
use ring::signature::{self, KeyPair};

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
    pub signature: Option<String>,
}

impl Transaction {
    /// Creates a new transaction with the specified details.
    pub fn new(sender: String, recipient: String, amount: f64, signature: String) -> Self {

        let id = Transaction::calculate_id(&sender, &recipient, amount);
        Transaction {
            id,
            sender,
            recipient,
            amount,
            signature: None,
        }
    }

    /// Calculates the ID of the transaction as a hash of the sender, recipient, and amount.
    fn calculate_id(sender: &str, recipient: &str, amount: f64) -> String {
        let mut hasher = Sha256::new();
        let data = format!("{}{}{}", sender, recipient, amount);
        hasher.update(data.as_bytes());
        let hash = hasher.finalize();
        hex::encode(hash)
    }

    /// Signs the transaction based on the hash of the ID, sender, recipient, and amount.
    pub fn sign(&mut self) {
        let data = format!("{}{}{}{}", self.id, self.sender, self.recipient, self.amount);
        let signature = Transaction::calculate_signature(&data);
        self.signature = Some(signature);
    }

    /// Calculates the signature of the transaction data.
    fn calculate_signature(data: &str) -> String {
        // Generate a new key pair
        let rng = SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();

        // Sign the data
        let signature = key_pair.sign(data.as_bytes());

        // Convert the signature to a hexadecimal string
        hex::encode(signature.as_ref())
    }
}