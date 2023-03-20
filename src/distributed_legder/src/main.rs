
use distributed_legder::dht::kademlia::KademliaDHT;
use distributed_legder::network::node::Node;
use distributed_legder::network::udp_communitations::{Client, Server};


fn main() {
    println!("Hello, world!");
    let current_node = Node::new("0.0.0.0".to_string(),1234);
    let remote_node = Node::new("192.168.1.86".to_string(),8000);

    let service = Server::new(current_node.clone());
    let kad = KademliaDHT::new(current_node.clone());

    let threa1 = Server::start_service(service.clone(), kad);


    for i in 0..10{
        Client::make_request(service.clone().socket, remote_node.clone(),
                             format!("{}",i))
    }


    threa1.join().expect("thead: dead");

}
