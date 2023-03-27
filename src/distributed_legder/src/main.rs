use std::borrow::Borrow;
use std::sync::Arc;
use log::{error, info, Level, log_enabled, warn};
use distributed_legder::constants::utils::get_local_ip;
use distributed_legder::dht::kademlia::KademliaDHT;
use distributed_legder::dht::rpc::Rpc;
use distributed_legder::network::datagram::{Datagram, DatagramType};
use distributed_legder::network::key::Key;
use distributed_legder::network::node::Node;




fn main() {
   let data = &Datagram {
        data_type: DatagramType::REQUEST,
        token_id: Key::new("test".to_string()),
        source: "192.168.1.112:56639".to_string(),
        destination: format!("{}:{}",get_local_ip().unwrap_or_default(),1432),
        data: Rpc::FindValue("test".to_string())
   };
    env_logger::init();

   let current_node = Node::new(get_local_ip().unwrap_or("0.0.0.0".to_string()),1432);
    //let remote_node = Node::new("192.168.1.86".to_string(),8000);
   info!("{}",serde_json::to_string(data).unwrap());

    let kad = KademliaDHT::new(current_node.clone(),None);

    let threa1 = kad.init();



    threa1.join().expect("thead: dead");


}
