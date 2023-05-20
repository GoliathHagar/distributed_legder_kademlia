use crypto::aes::KeySize::KeySize256;
use crypto::aes::ecb_decryptor_generic;
use crypto::aes::ecb_encryptor_generic;
use crypto::padding::NoPadding;
use crypto::symmetriccipher::Decryptor;
use crypto::symmetriccipher::Encryptor;
use rust_crypto::digest::Digest;
use rust_crypto::sha3::Sha3;
use std::time::SystemTime;
use std::io::{self, BufRead};

pub struct Bid {
    bid_id: Vec<u8>,
    user_node_id: Vec<u8>,
    auction_id: Vec<u8>,
    user_public_key: Vec<u8>,
    encrypted_bid: Vec<u8>,
}

impl Bid {
    pub fn new(user_node_id: Vec<u8>, user_public_key: Vec<u8>, encrypted_bid: Vec<u8>, auction_id: Vec<u8>) -> Bid {
        let bid_id = Bid::calculate_hash(&user_node_id, &auction_id, &user_public_key, &encrypted_bid);

        Bid {
            bid_id,
            user_node_id,
            auction_id,
            user_public_key,
            encrypted_bid,
        }
    }

    pub fn get_bid_amount(&self, auction_owner: KeyPair) -> f32 {
        let shared_secret = Standards::generate_shared_secret(&self.user_public_key, &auction_owner.private_key);

        let decrypted = Standards::decrypt_with_prefix_iv(&self.encrypted_bid, &shared_secret);

        let mut wrapped = RefReadBuffer::new(&decrypted);

        let mut amount_bytes = [0u8; 4];

        let mut output = RefWriteBuffer::new(&mut amount_bytes);

        let mut decryptor = ecb_decryptor_generic(KeySize256, &shared_secret, NoPadding);

        decryptor.decrypt(&mut wrapped, &mut output, true).unwrap();

        let amount = f32::from_ne_bytes(amount_bytes);

        amount
    }

    pub fn initialize_bid_for(auction: &Auction, node_id: Vec<u8>, key_pair: &KeyPair, amount: f32) -> Bid {
        let shared_secret = Standards::generate_shared_secret(&auction.auctioneer_public_key, &key_pair.private_key);

        let amount_bytes = amount.to_ne_bytes();

        let mut wrapped = RefReadBuffer::new(&amount_bytes);

        let mut cipher_text = Vec::new();

        let mut output = RefWriteBuffer::new(&mut cipher_text);

        let mut encryptor = ecb_encryptor_generic(KeySize256, &shared_secret, NoPadding);

        encryptor.encrypt(&mut wrapped, &mut output, true).unwrap();

        Bid::new(node_id, key_pair.public_key.clone(), cipher_text, auction.auction_id.clone())
    }

    fn calculate_hash(user_node_id: &[u8], auction_id: &[u8], user_public_key: &[u8], encrypted_bid: &[u8]) -> Vec<u8> {
        let mut hasher = Sha3::sha3_256();

        hasher.input(user_node_id);
        hasher.input(auction_id);
        hasher.input(user_public_key);
        hasher.input(encrypted_bid);

        let mut hash = vec![0u8; hasher.output_bytes()];

        hasher.result(&mut hash);

        hash
    }
}
