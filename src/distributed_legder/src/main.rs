use std::borrow::Borrow;
use std::sync::Arc;
use log::{error, info, Level, log_enabled, warn};
use distributed_legder::dht::kademlia::KademliaDHT;
use distributed_legder::dht::rpc::Rpc;
use distributed_legder::network::datagram::{Datagram, DatagramType};
use distributed_legder::network::key::Key;
use distributed_legder::network::node::Node;




fn main() {
    let mut data = &Datagram {
        data_type: DatagramType::REQUEST,
        token_id: "test".to_string(),
        source: "127.0.0.1:1234".to_string(),
        destination: "127.0.0.1:8000".to_string(),
        data: Rpc::Ping
    };
    env_logger::init();

   let current_node = Node::new("0.0.0.0".to_string(),1234);
    //let remote_node = Node::new("192.168.1.86".to_string(),8000);
    info!("{}",serde_json::to_string(data).unwrap());

    let kad = KademliaDHT::new(current_node.clone(),None);

   let threa1 = kad.init();

    threa1.join().expect("thead: dead");

}
