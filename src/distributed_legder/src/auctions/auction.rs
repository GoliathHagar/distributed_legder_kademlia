use std::io::{self, BufRead};

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

    pub fn main_menu(&self) -> i32 {
        println!("1) New auction");
        println!("2) Find auctions");
        println!("3) Exit");
        print!("Choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        self.stdin.read_line(&mut choice).unwrap();

        let choice = choice.trim().parse().unwrap_or(0);

        while choice <= 0 || choice > self.max_rows {
            print!("Bad option, choose again: ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            self.stdin.read_line(&mut choice).unwrap();
            let choice = choice.trim().parse().unwrap_or(0);
        }

        choice
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
            println!("{} ) {}", cont, auction.get_auction_name());
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

        println!("Name of the item: {}", chosen_auction.get_auction_name());
        println!("Starting bid of the item: {}", chosen_auction.get_minimum_bid());

        let initial_date = chosen_auction.get_initial_ts();
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
