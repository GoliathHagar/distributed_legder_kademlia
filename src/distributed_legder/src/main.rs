use std::sync::Arc;
use std::thread;

use log::{debug, info};

use distributed_legder::blockchain::blockchain_handler::BlockchainHandler;
use distributed_legder::blockchain::consensus::ConsensusAlgorithm;
use distributed_legder::constants::blockchain_node_type::BlockchainNodeType;
use distributed_legder::constants::fixed_sizes::DUMP_STATE_TIMEOUT;
use distributed_legder::constants::utils::get_local_ip;
use distributed_legder::dht::kademlia::KademliaDHT;
use distributed_legder::network::datagram::{Datagram, DatagramType};
use distributed_legder::network::key::Key;
use distributed_legder::network::node::Node;
use distributed_legder::network::rpc::Rpc;

fn main() {
    env_logger::init();

    let node = Node::new(get_local_ip(), 1432);

    let blockchain = BlockchainHandler::new(
        ConsensusAlgorithm::ProofOfWork,
        node,
        BlockchainNodeType::Bootstrap,
        None,
    );

    let tb = blockchain.start("state_dumps/self.json");
    tb.join().expect("TODO: panic message");
    /*   let new_node = Node::new(get_local_ip(), 1422);

       let kad = Arc::new(KademliaDHT::new(current_node.clone(), None));

       let kadc = kad.clone();

       let threa1 = kad.init(Some("state_dumps/self.json".to_string()));

       thread::sleep(std::time::Duration::from_millis(2*DUMP_STATE_TIMEOUT));
       kadc.clone().put(
           "test00".to_string(),
           "It works, this value was stored successifully".to_string(),
       );
       thread::sleep(std::time::Duration::from_millis(DUMP_STATE_TIMEOUT));

       kadc.clone().put(
           "claudia".to_string(),
           "It works, this value was stored successifully".to_string(),
       );

       debug!("Done initial setup");
       let kad2 = Arc::new(KademliaDHT::new(new_node.clone(), Some(current_node)));

       let t2 = kad2.clone().init(Some("state_dumps/self-node.json".to_string()));

       debug!("Done initial setup");

       kad2.clone().broadcast_info(("main".to_string(), "test".to_string(), "this is a test".to_string()));

       threa1.join().expect("thead: dead");
       t2.join().expect("thead: dead");*/



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