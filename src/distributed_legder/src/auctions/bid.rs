pub struct Bid {
    bid_id: String,
    user_node_id: String,
    auction_id: String,
    user_public_key: String,
    encrypted_bid: String,
}

impl Bid {
    pub fn new(user_node_id: String, user_public_key: String, encrypted_bid: String, auction_id: String) -> Bid {

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
        let node_id = String::from_utf8_lossy(&node_id).to_string();

        let shared_secret = Standards::generate_shared_secret(&auction.auctioneer_public_key, &key_pair.private_key);

        let amount_bytes = amount.to_ne_bytes();

        let mut wrapped = RefReadBuffer::new(&amount_bytes);

        let mut cipher_text = Vec::new();

        let mut output = RefWriteBuffer::new(&mut cipher_text);

        let mut encryptor = ecb_encryptor_generic(KeySize256, &shared_secret, NoPadding);

        encryptor.encrypt(&mut wrapped, &mut output, true).unwrap();

        Bid::new(node_id, key_pair.public_key.clone(), cipher_text, auction.auction_id.clone())
    }

    fn calculate_hash(user_node_id: &str, auction_id: &str, user_public_key: &str, encrypted_bid: &str) -> String {
        let mut hasher = Sha3::sha3_256();

        hasher.input(user_node_id.as_bytes());
        hasher.input(auction_id.as_bytes());
        hasher.input(user_public_key.as_bytes());
        hasher.input(encrypted_bid.as_bytes());

        let mut hash = vec![0u8; hasher.output_bytes()];

        hasher.result(&mut hash);

        hex::encode(hash)
    }
}
