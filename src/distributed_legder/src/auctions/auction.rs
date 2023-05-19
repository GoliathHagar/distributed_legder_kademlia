/*

pub struct Auction {
    auction_id: [u8; 20],
    auctioneer_node_id: Vec<u8>,
    auction_name: String,
    minimum_bid: f32,
    auction_duration: u64, // in minutes
    initial_ts: u64,
    auctioneer_pk: Vec<u8>,
    signature: Option<Vec<u8>>,
}

impl Auction {
    pub fn new(
        auction_name: String,
        auctioneer_node_id: Vec<u8>,
        minimum_bid: f32,
        auction_duration: u64,
        auctioneer_pk: Vec<u8>,
        signature: Option<Vec<u8>>,
    ) -> Self {
        let initial_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to obtain system time")
            .as_secs();

        let auction_id = Self::generate_id(&auction_name, initial_ts);

        Auction {
            auction_id,
            auctioneer_node_id,
            auction_name,
            minimum_bid,
            auction_duration,
            initial_ts,
            auctioneer_pk,
            signature,
        }
    }

    pub fn get_final_ts(&self) -> u64 {
        self.initial_ts + self.auction_duration
    }

    fn generate_id(auction_name: &str, initial_ts: u64) -> [u8; 20] {
        let ts = initial_ts.to_string();
        let merge = [auction_name.as_bytes(), ts.as_bytes()].concat();
        let mut hasher = Sha1::new();
        hasher.input(&merge);
        let result = hasher.result();

        let mut auction_id = [0u8; 20];
        auction_id.copy_from_slice(&result[..20]);

        auction_id
    }

    pub fn add_to_signature(&self, signature: &mut Signature) {
        signature.update(self.auction_name.as_bytes());
        signature.update(&self.auction_id);
        signature.update(&self.auctioneer_node_id);

        let mut auction = Vec::new();
        auction.extend_from_slice(&self.minimum_bid.to_be_bytes());
        auction.extend_from_slice(&self.initial_ts.to_be_bytes());
        auction.extend_from_slice(&self.auction_duration.to_be_bytes());

        signature.update(&auction);
    }

    pub fn initialize_new_auction(
        node: &P2PNode,
        auction_name: String,
        duration_in_millis: u64,
        min_bid: f32,
        owner_keys: &KeyPair,
    ) -> Self {
        let auction = Auction::new(
            auction_name,
            node.node_id.clone(),
            min_bid,
            duration_in_millis,
            owner_keys.public_key.clone(),
            None,
        );

        let signature = Signable::calculate_signature_of(&auction, &owner_keys.private_key);
        auction.signature = Some(signature);

        auction
    }
}
*/