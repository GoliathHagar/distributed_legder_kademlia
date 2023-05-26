use std::sync::Arc;

use distributed_legder::auctions::auction_ui::AuctionUI;
use distributed_legder::blockchain::blockchain_handler::BlockchainHandler;
use distributed_legder::blockchain::consensus::ConsensusAlgorithm;
use distributed_legder::constants::blockchain_node_type::BlockchainNodeType;
use distributed_legder::constants::utils::get_local_ip;
use distributed_legder::network::node::Node;

fn main() {
    env_logger::init();

    let node = Node::new(get_local_ip(), 1432);
    //let node_boot = Node::new("192.168.43.9".to_string(), 1432);

    let blockchain = BlockchainHandler::new(
        ConsensusAlgorithm::ProofOfWork,
        node.clone(),
        BlockchainNodeType::Bootstrap,
        None,
    );

    let tb = blockchain.clone().start("state_dumps/self.json");

    let aution = Arc::new(AuctionUI::new(node));

    //aution.init(Arc::new(blockchain.clone()));
    aution.start(Arc::new(blockchain), "state_dumps/self.json");


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


}