use std::borrow::Borrow;
use std::sync::Arc;
use log::{error, info, Level, log_enabled, warn};
use distributed_legder::dht::kademlia::KademliaDHT;
use distributed_legder::network::datagram::{Datagram, DatagramType};
use distributed_legder::network::key::Key;
use distributed_legder::network::node::Node;




fn main() {


   let current_node = Node::new("0.0.0.0".to_string(),1234);
    //let remote_node = Node::new("192.168.1.86".to_string(),8000);

    let kad =KademliaDHT::new(current_node.clone(),None);

    let threa1 = kad.start_server();

    threa1.join().expect("thead: dead");

}

/*let st = &Datagram {
        data_type: DatagramType::RESPONSE,
        token_id: "srer".to_string(),
        source: "sadsdf".to_string(),
        destination: "192.168.99.180:42556".to_string(),
        data: "sersfdarfeqgvbgeb".to_string()
    };
    let sd = Datagram {
        data_type: DatagramType::RESPONSE,
        token_id: "srer".to_string(),
        source: "sadsdf".to_string(),
        destination: "192.168.99.180:42556".to_string(),
        data: "sersfdarfeqgvbgeb".to_string()
    };

    let d = serde_json::to_string(st.clone()).unwrap();
    println!("Hello, world! {:?}", d);*/