use distributed_legder::constants::utils::get_local_ip;
use distributed_legder::dht::kademlia::KademliaDHT;
use distributed_legder::network::rpc::Rpc;
use distributed_legder::network::datagram::{Datagram, DatagramType};
use distributed_legder::network::key::Key;
use distributed_legder::network::node::Node;
use log::{debug, info};
use std::sync::Arc;
use std::thread;
use distributed_legder::constants::fixed_sizes::DUMP_STATE_TIMEOUT;
//use distributed_legder::blockchain::blockchain::Blockchain;
//use distributed_legder::blockchain::consensus::ConsensusAlgorithm;

fn main() {

    /*
    let data = Datagram {
        data_type: DatagramType::REQUEST,
        token_id: Key::new("test".to_string()),
        source: "YourIP".to_string(),
        destination: format!("{}:{}", get_local_ip().unwrap_or_default(), 1432),
        data: Rpc::Ping,
    };
*/
    env_logger::init();

    let current_node = Node::new(get_local_ip().unwrap_or("0.0.0.0".to_string()), 1432);

    let bootstrap_node = Node::new(get_local_ip().unwrap_or("192.168.153.8".to_string()), 1432);

    //info!("{}", serde_json::to_string(&data).unwrap());

    let kad = KademliaDHT::new(current_node.clone(), Some(bootstrap_node.clone()));

    let kadc = kad.clone();

    let threa1 = kad.init(Some("state_dumps/self.json".to_string()));

    thread::sleep(std::time::Duration::from_millis(2*DUMP_STATE_TIMEOUT));
    Arc::new(kadc.clone()).put(
        "test00".to_string(),
        "It works, this value was stored successifully".to_string(),
    );
    thread::sleep(std::time::Duration::from_millis(DUMP_STATE_TIMEOUT));

    Arc::new(kadc.clone()).put(
        "claudia".to_string(),
        "It works, this value was stored successifully".to_string(),
    );

    debug!("Done initial setup");

    threa1.join().expect("thead: dead");


/*
    // Create a new blockchain with the desired consensus algorithm
    let mut blockchain = Blockchain::new(ConsensusAlgorithm::ProofOfWork);

    // Add transactions to the blockchain
    blockchain.add_transaction("sender1".to_string(), "recipient1".to_string(), 1.0);
    blockchain.add_transaction("sender2".to_string(), "recipient2".to_string(), 2.0);

    // Mine a new block
    blockchain.mine_block("miner_address".to_string()).unwrap();

    // Switch to a different consensus algorithm
    blockchain.set_consensus_algorithm(ConsensusAlgorithm::DelegatedProofOfStake);

    // Add more transactions and mine a new block using the new consensus algorithm
    blockchain.add_transaction("sender3".to_string(), "recipient3".to_string(), 3.0);
    blockchain.add_transaction("sender4".to_string(), "recipient4".to_string(), 4.0);
    blockchain.mine_block("miner_address".to_string()).unwrap();

    // Verify the validity of the blockchain
    println!("Blockchain is valid: {}", blockchain.is_valid());
*/

}