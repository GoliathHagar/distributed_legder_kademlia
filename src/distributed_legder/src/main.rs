use std::borrow::Borrow;
use std::sync::Arc;
use log::{error, info, Level, log_enabled, warn};
use distributed_legder::dht::kademlia::KademliaDHT;
use distributed_legder::network::datagram::{Datagram, DatagramType};
use distributed_legder::network::key::Key;
use distributed_legder::network::node::Node;




fn main() {
    env_logger::init();

   let current_node = Node::new("0.0.0.0".to_string(),1234);
    //let remote_node = Node::new("192.168.1.86".to_string(),8000);

    let kad = KademliaDHT::new(current_node.clone(),None);

    let threa1 = kad.start_server();

    threa1.join().expect("thead: dead");

}
