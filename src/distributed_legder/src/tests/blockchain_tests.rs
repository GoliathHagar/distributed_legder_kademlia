use std::sync::Arc;

use crate::auctions::auction::Auction;
use crate::auctions::bid::Bid;
use crate::blockchain::block::Block;
use crate::blockchain::blockchain_handler::BlockchainHandler;
use crate::blockchain::consensus::ConsensusAlgorithm;
use crate::blockchain::miner::Miner;
use crate::blockchain::transaction::Transaction;
use crate::constants::blockchain_node_type::BlockchainNodeType;
use crate::constants::fixed_sizes::ZEROS_HASH;
use crate::constants::multicast_info_type::MulticastInfoType;
use crate::constants::utils::{calculate_block_hash, get_local_ip, value_to_string};
use crate::network::client::Client;
use crate::network::datagram::{Datagram, DatagramType};
use crate::network::key::Key;
use crate::network::node::Node;
use crate::network::rpc::Rpc;

#[test]
fn test_mining_pow_block_and_validate() {
    // Create a sample block
    let mut block = Block::new(
        0,
        ZEROS_HASH.to_string(),
        "".to_string(),
        Vec::from([
            Transaction::new("sad".to_string(), "dpr".to_string(), 100000.0),
            Transaction::new("sad2".to_string(), "dpr2".to_string(), 100000.0)]),
    );
    println!("initial {:?}", block);

    block.header.timestamp=0;

    let miner = Arc::new(Miner::new(ConsensusAlgorithm::ProofOfWork));

    let nonce = miner.mine_block(block.clone());
    // Calculate the block's hash and convert it to a hexadecimal string

    block.header.nonce = nonce;

    let string_hash = calculate_block_hash(&block);
    let binary_hash = hex::decode(string_hash.clone()).unwrap().iter()
        .map(|byte| format!("{:08b}-", byte))
        .collect::<String>();

    // Verify the block's validity
    let is_valid = block.is_valid();

    println!("Last {:?}", block);

    println!("Nonce {} Hash {} bin {}", nonce, string_hash, binary_hash);
    // Assert that the block is valid
    assert_eq!(is_valid, true)
}

#[ignore]
#[test]
fn test_aution_and_bid_on_miner_depend_on_local_running_server() {
    let node = Node::new(get_local_ip(), 9876);
    let boot = Node::new(get_local_ip(), 1432);

    let blockchain = BlockchainHandler::new(
        ConsensusAlgorithm::ProofOfWork,
        node.clone(),
        BlockchainNodeType::Miner,
        Some(boot),
    );

    let tb = blockchain.clone().start("state_dumps/test-miner.json");


    let kill_self = Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: node.get_address(),
        destination: node.get_address(),
        data: Rpc::Ping,
    };

    let client = Client::new(blockchain.kademlia.service.clone());
    let aution = Auction::initialize_new_auction(
        node.clone(),
        "this is an test auction".to_string(), 2, 5.0, "this a suppost key".to_string());
    blockchain.kademlia.clone().broadcast_info((format!("{:?}", aution.auction_id.clone()),
                                                MulticastInfoType::Auction,
                                                value_to_string::<Auction>(aution.clone())));

    std::thread::sleep(std::time::Duration::from_secs(10));
    let bid = Bid::new(
        node.id.clone(),
        "This is a public key".to_string(),
        54.0,
        aution.auction_id.clone(),
    );
    blockchain.kademlia.broadcast_info((format!("{:?}", bid.bid_id.clone()),
                                        MulticastInfoType::Bid,
                                        value_to_string::<Bid>(bid)));


    std::thread::sleep(std::time::Duration::from_secs(20));
    client.datagram_request(kill_self.clone());
    tb.join().expect("TODO: panic message");
}
