use std::convert::TryInto;
use std::io::{self, BufRead, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use ring::signature::KeyPair;

use crate::blockchain::transaction::Transaction;
use crate::constants::fixed_sizes::KEY_SIZE;
use crate::constants::utils::calculate_signature;
use crate::network::key::Key;
use crate::network::node::Node;

pub struct Auction {
    auction_id: [u8; KEY_SIZE],
    auctioneer_node_id: String,
    auction_name: String,
    minimum_bid: f32,
    auction_duration: u64,
    // in minutes
    initial_ts: u64,
    auctioneer_pk: String,
    signature: String,
}

impl Auction {
    pub fn new(auction_name: String, auctioneer_node_id: String, minimum_bid: f32, initial_ts: u64,
               auction_duration: u64, auctioneer: String, signature: String) -> Self {
        let auction_id = Auction::generate_id(&auction_name, initial_ts);

        Auction {
            auction_id,
            auctioneer_node_id,
            auction_name,
            minimum_bid,
            auction_duration,
            initial_ts,
            auctioneer_pk: auctioneer,
            signature,
        }
    }

    pub fn with_defaults(auction_name: String, auctioneer_node_id: String, minimum_bid: f32,
                         auction_duration: u64, auctioneer: String) -> Self {
        let initial_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let auction_id = Auction::generate_id(&auction_name, initial_ts);

        Auction {
            auction_id,
            auctioneer_node_id,
            auction_name,
            minimum_bid,
            auction_duration,
            initial_ts,
            auctioneer_pk: auctioneer,
            signature: "".to_string(),
        }
    }

    pub fn get_final_ts(&self) -> u64 {
        self.initial_ts + self.auction_duration
    }

    fn generate_id(auction_name: &str, initial_ts: u64) -> [u8; 32] {
        Key::new(format!("{}:{}", auction_name, initial_ts)).0
    }

    pub fn initialize_new_auction(node: Node, auction_name: String, duration_in_millis: u64, min_bid: f32, owner_keys: String) -> Self {
        let auction = Auction::with_defaults(auction_name,
                                             format!("{:?}", node.id.clone()), min_bid, duration_in_millis, owner_keys.clone());

        let signature = calculate_signature(owner_keys.as_str());

        Auction {
            signature,
            ..auction
        }
    }
}

pub struct AuctionUI {
    stdin: io::Stdin,
    max_rows: i32,
}

impl AuctionUI {
    pub fn new() -> AuctionUI {
        AuctionUI {
            stdin: io::stdin(),
            max_rows: 3,
        }
    }

    pub fn main_menu(&self) {
        println!("1) New auction");
        println!("2) Join auction");
        println!("3) Exit");
        print!("Choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        self.stdin.read_line(&mut choice).unwrap();

        let mut choice_parsed = choice.trim().parse().unwrap_or(0);

        while choice_parsed <= 0 || choice_parsed > self.max_rows {
            print!("Bad option, choose again: ");
            io::stdout().flush().unwrap();

            self.stdin.read_line(&mut choice).unwrap();
            choice_parsed = choice.trim().parse().unwrap_or(0);
        }

        match choice_parsed {
            1 => { self.new_auction_menu() },
            2 => { self.ongoing_auction() },
            3 => {},
            _ => {}
        }
    }

    pub fn new_auction_menu(&self) {
        println!("** MENU TO INITIALIZE AN AUCTION **\n");

        print!("Name of item: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        self.stdin.read_line(&mut name).unwrap();

        print!("Open bid: ");
        io::stdout().flush().unwrap();
        let mut opening_bid = String::new();
        self.stdin.read_line(&mut opening_bid).unwrap();
        let opening_bid = opening_bid.trim().parse().unwrap_or(0);

        print!("Minimum bid: ");
        io::stdout().flush().unwrap();
        let mut minimum_bid = String::new();
        self.stdin.read_line(&mut minimum_bid).unwrap();
        let minimum_bid = minimum_bid.trim().parse().unwrap_or(0);

        print!("Auction duration (in min): ");
        io::stdout().flush().unwrap();
        let mut auction_duration = String::new();
        self.stdin.read_line(&mut auction_duration).unwrap();
        let auction_duration = auction_duration.trim().parse().unwrap_or(0);

        // TODO: Create a new auction with the provided details
    }

    pub fn ongoing_auction(&self) {
        println!("** LIST OF CURRENT AUCTIONS **\n");
        let auctions: Vec<Auction> = Vec::new();
        // TODO: Retrieve the list of auctions from the blockchain

        let mut cont = 1;
        for auction in &auctions {
            println!("{} ) {}", cont, auction.auction_name);
            cont += 1;
        }

        print!("\nChoose: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        self.stdin.read_line(&mut choice).unwrap();
        let mut choice = choice.trim().parse().unwrap_or(0);

        while choice <= 0 || choice >= cont {
            print!("Bad option, choose again: ");
            io::stdout().flush().unwrap();
            let mut choice = String::new();
            self.stdin.read_line(&mut choice).unwrap();
            let choice = choice.trim().parse().unwrap_or(0);
        }

        self.bid_auction(&auctions[choice - 1]);
    }

    pub fn bid_auction(&self, chosen_auction: &Auction) {
        println!("** MENU TO BID THE ITEM **\n");

        println!("Name of the item: {}", chosen_auction.auction_name);
        println!("Starting bid of the item: {}", chosen_auction.minimum_bid);

        let initial_date = chosen_auction.initial_ts;
        let final_date = chosen_auction.get_final_ts();
        println!("Start of auction: {}", initial_date);
        println!("End of auction: {}", final_date);

        print!("\nPlease, choose a bid amount: ");
        io::stdout().flush().unwrap();

        let mut amount = String::new();
        self.stdin.read_line(&mut amount).unwrap();
        let amount = amount.trim().parse().unwrap_or(0.0);

        // TODO: Create a new bid with the chosen auction and bid amount
    }
}
