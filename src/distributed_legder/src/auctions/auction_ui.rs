use std::collections::HashMap;
use std::fs::create_dir_all;
use std::io;
use std::io::Write;
use std::ops::Index;
use std::sync::{Arc, Mutex};

use log::{debug, error, info};

use crate::auctions::auction::Auction;
use crate::auctions::bid::Bid;
use crate::blockchain::blockchain_handler::BlockchainHandler;
use crate::constants::fixed_sizes::DUMP_STATE_TIMEOUT;
use crate::constants::multicast_info_type::MulticastInfoType;
use crate::constants::utils::{block_to_string, get_timestamp_now, string_to_value, value_to_string};
use crate::network::key::Key;
use crate::network::node::Node;

#[derive(Debug)]
pub struct AuctionUI {
    stdin: io::Stdin,
    max_rows: i32,
    node: Node,
    auctions: Arc<Mutex<HashMap<Key, Auction>>>,
}

impl AuctionUI {
    pub fn new(node: Node) -> AuctionUI {
        AuctionUI {
            stdin: io::stdin(),
            max_rows: 3,
            node,
            auctions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn init(self, bch: Arc<BlockchainHandler>, path: &str) {
        let auctions = self.auctions.clone();
        let chain_handler = bch.clone();

        let auctions_clone = auctions.clone();

        std::thread::spawn(move || loop {
            let obj = chain_handler.receiver_subscription.lock()
                .unwrap().recv().unwrap();

            debug!("Packet received {:?}",obj);

            match obj.0 {
                MulticastInfoType::Auction => {
                    let auction: Auction = string_to_value::<Auction>(obj.1);

                    let mut ats = auctions.lock().unwrap();
                    ats.insert(auction.auction_id.clone(), auction.clone());

                    drop(ats);
                }
                MulticastInfoType::Bid => {
                    let bid: Bid = string_to_value::<Bid>(obj.1);

                    let mut ats = auctions.lock().unwrap();

                    if ats.contains_key(&bid.auction_id) {
                        let mut a = ats.index(&bid.auction_id).clone();
                        a.add_bid(bid.clone());

                        ats.insert(bid.auction_id.clone(), a.clone());
                    } else {
                        error!("Auction not Present in node");
                    }

                    drop(ats);
                }
                _ => {}
            }
        });

        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(2 * 60));
            let auctions = match auctions_clone.lock() {
                Ok(aut) => aut,
                Err(e) => {
                    error!("Unable to acquire mutex on auctions");
                    panic!("{}", e.to_string())
                }
            };

            for auction in auctions.values() {
                if get_timestamp_now() >= auction.get_final_ts() && !auction.bids.is_empty() {
                    info!("Auction '{}' is close creating transaction.", auction.auction_name);
                    let mut bigger_bid: Option<Bid> = None;

                    for bid in &auction.bids {
                        if !bigger_bid.is_some() { bigger_bid = Some(bid.clone()) } else if bigger_bid.clone().unwrap().amount < bid.amount {
                            bigger_bid = Some(bid.clone());
                        }
                    }
                    if let Some(bid) = bigger_bid {
                        let block = bch.clone().add_transaction_create_block(
                            bid.clone().to_transaction()
                        );

                        if let Some(b) = block {
                            bch.kademlia.clone().broadcast_info(
                                (b.header.hash.clone(), MulticastInfoType::Block, block_to_string(b.clone()))
                            );
                        }
                    }
                } else {
                    info!("Auction '{}' is still open.", auction.auction_name);
                }
            }

            drop(auctions);
        });

        let new_path = path.to_owned() + ".auction.json";
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(DUMP_STATE_TIMEOUT));
            self.dump_state(new_path.as_str());
        });
    }

    pub fn start(&self, blockchain_handler: Arc<BlockchainHandler>, path: &str) {
        let cloned_auction_ui = AuctionUI {
            stdin: io::stdin(),
            max_rows: self.max_rows,
            node: self.node.clone(),
            auctions: self.auctions.clone(),
        };

        cloned_auction_ui.init(blockchain_handler.clone(), path);

        let mut option = self.main_menu();

        while option != 3 {
            let map_auctions = match self.auctions.lock() {
                Ok(aut) => aut,
                Err(e) => {
                    error!("Unable to acquire mutex on auctions");
                    panic!("{}", e.to_string())
                }
            };

            let auctions: Vec<Auction> = map_auctions.values().cloned().collect();

            drop(map_auctions);

            match option {
                1 => {
                    let au = self.new_auction_menu();

                    let mut map_auctions = match self.auctions.lock() {
                        Ok(aut) => aut,
                        Err(e) => {
                            error!("Unable to acquire mutex on auctions");
                            panic!("{}", e.to_string())
                        }
                    };

                    map_auctions.insert(au.auction_id.clone(), au.clone());

                    drop(map_auctions);

                    blockchain_handler.kademlia.clone().broadcast_info(
                        (format!("{:?}", au.auction_id), MulticastInfoType::Auction,
                         value_to_string::<Auction>(au))
                    );
                },
                2 => {
                    let bids = self.ongoing_auction(auctions);
                    if let Some(bid) = bids {
                        let mut map_auctions = match self.auctions.lock() {
                            Ok(aut) => aut,
                            Err(e) => {
                                error!("Unable to acquire mutex on auctions");
                                panic!("{}", e.to_string())
                            }
                        };
                        let mut a = map_auctions.index(&bid.auction_id.clone()).clone();
                        a.add_bid(bid.clone());

                        map_auctions.insert(bid.auction_id.clone(), a.clone());

                        drop(map_auctions);

                        blockchain_handler.kademlia.clone().broadcast_info(
                            (format!("{:?}", bid.bid_id.clone()), MulticastInfoType::Bid,
                             value_to_string::<Bid>(bid.clone()))
                        );
                    }
                },
                _ => {}
            };


            option = self.main_menu();
        }
    }

    pub fn main_menu(&self) -> i32 {
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

        choice_parsed
    }

    pub fn new_auction_menu(&self) -> Auction {
        println!("** MENU TO INITIALIZE AN AUCTION **\n");

        print!("Name of item: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        self.stdin.read_line(&mut name).unwrap();

        print!("Open / Minimum bid: ");
        io::stdout().flush().unwrap();
        let mut opening_bid = String::new();
        self.stdin.read_line(&mut opening_bid).unwrap();
        let opening_bid = opening_bid.trim().parse().unwrap_or(0.0);

        print!("Auction duration (in min): ");
        io::stdout().flush().unwrap();
        let mut auction_duration = String::new();
        self.stdin.read_line(&mut auction_duration).unwrap();
        let auction_duration = auction_duration.trim().parse().unwrap_or(0);

        Auction::initialize_new_auction(
            self.node.clone(),
            name,
            auction_duration,
            opening_bid,
            "a possible key".to_string(),
        )
    }

    pub fn ongoing_auction(&self, auctions: Vec<Auction>) -> Option<Bid> {
        println!("** LIST OF CURRENT AUCTIONS **\n");

        let mut count = 1;
        println!("0 ) Exit");
        for auction in &auctions {
            println!("{} ) {}", count, auction.auction_name);
            count += 1;
        }

        print!("\nChoose: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        self.stdin.read_line(&mut choice).unwrap();
        let mut choice_parsed = choice.trim().parse().unwrap_or(0);

        while choice_parsed >= auctions.len() + 1 {
            print!("Bad option, choose again: ");
            io::stdout().flush().unwrap();
            let mut choice = String::new();
            self.stdin.read_line(&mut choice).unwrap();

            choice_parsed = choice.trim().parse().unwrap_or(0);
        }

        if choice_parsed == 0 { return None }
        Some(self.bid_auction(&auctions[choice_parsed - 1]))
    }

    pub fn bid_auction(&self, chosen_auction: &Auction) -> Bid {
        println!("** MENU TO BID THE ITEM **\n");

        println!("Name of the item: {}", chosen_auction.auction_name);
        println!("Starting bid of the item: {}", chosen_auction.minimum_bid);
        println!("Currently time: {}", get_timestamp_now());

        let initial_date = chosen_auction.initial_ts;
        let final_date = chosen_auction.get_final_ts();
        println!("Start of auction: {}", initial_date);
        println!("End of auction: {}", final_date);

        print!("\nPlease, choose a bid amount: ");
        io::stdout().flush().unwrap();

        let mut amount = String::new();
        self.stdin.read_line(&mut amount).unwrap();
        let amount = amount.trim().parse().unwrap_or(0.0);

        Bid::new(
            self.node.id.clone(),
            "some pk".to_string(),
            amount,
            chosen_auction.auction_id.clone(),
        )
    }

    fn dump_state(&self, path: &str) {
        if let Err(_) = create_dir_all("state_dumps") {
            error!("Unable to create state dumps diretory");
            return;
        }

        let map_auctions = match self.auctions.lock() {
            Ok(aut) => aut,
            Err(e) => {
                error!("Unable to acquire mutex on auctions");
                panic!("{}", e.to_string())
            }
        };

        let mut parsed_auctions = Vec::new();
        let mut flatted_auctions = Vec::new();

        for auction in map_auctions.values() {
            flatted_auctions.push(auction.clone());
        }

        for kb in flatted_auctions {
            let auction = serde_json::json!(
                {
                    "auction": {
                        "id": kb.auction_id,
                        "name":  kb.auction_name,
                        "minimum_bid":  kb.minimum_bid,
                        "initial_ts":  kb.initial_ts,
                        "auctioneer_node_id":  kb.auctioneer_node_id,
                        "auction_duration":  kb.auction_duration,
                        "auctioneer_pk":  kb.auctioneer_pk,
                        "signature":  kb.signature,
                        "bids":  kb.bids,
                    }
                }
            );

            parsed_auctions.push(auction);
        }


        let json = serde_json::json!(
            {
                "auctions":parsed_auctions ,
                "current_timestamp": get_timestamp_now()
            }
        );


        let mut file = match std::fs::File::create(path) {
            Ok(f) => f,
            Err(_) => {
                error!("Unable to create dump file");
                return;
            }
        };

        if let Err(_) = file.write_all(&json.to_string().as_bytes()) {
            error!("Unable to create dump file Json");
        }

        let mut diagram = match std::fs::File::create(format!("{}.plantuml", path)) {
            Ok(d) => d,
            Err(_) => {
                error!("Unable to create dump file Diagram");
                return;
            }
        };

        if let Err(_) = diagram.write_all("@startjson\n".to_string().as_bytes()) {
            error!("Unable to write to dump file");
            return;
        }

        if let Err(_) = diagram.write_all(&json.to_string().as_bytes()) {
            error!("Unable to write to dump file");
            return;
        }

        if let Err(_) = diagram.write_all("\n@endjson".to_string().as_bytes()) {
            error!("Unable to write to dump file");
            return;
        }
    }
}
