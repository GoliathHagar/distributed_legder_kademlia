use distributed_legder::constants::utils::get_local_ip;
use distributed_legder::dht::kademlia::KademliaDHT;
use distributed_legder::network::rpc::Rpc;
use distributed_legder::network::datagram::{Datagram, DatagramType};
use distributed_legder::network::key::Key;
use distributed_legder::network::node::Node;
use log::{debug, info};
use std::sync::Arc;

fn main() {
    //let b = Block();

    let data = Datagram {
        data_type: DatagramType::REQUEST,
        token_id: Key::new("test".to_string()),
        source: "YourIP".to_string(),
        destination: format!("{}:{}", get_local_ip().unwrap_or_default(), 1432),
        data: Rpc::Ping,
    };
    env_logger::init();


    let current_node = Node::new(get_local_ip().unwrap_or("0.0.0.0".to_string()), 1432);
    /*let remote_node = Node::new(get_local_ip().unwrap_or("0.0.0.0".to_string()),8000);
    let remote_node2 = Node::new(get_local_ip().unwrap_or("0.0.0.0".to_string()),8001);*/
    let bootstrap_node = Node::new(get_local_ip().unwrap_or("0.0.0.0".to_string()), 8080);

    info!("{}", serde_json::to_string(&data).unwrap());
    /*
    data.data=Rpc::FindNode(Key::new("t".to_string()));
    info!("{}",serde_json::to_string(&data).unwrap());

    data.data=Rpc::FindValue("test".to_string());
    info!("{}",serde_json::to_string(&data).unwrap());

    data.data=Rpc::Store("k".to_string(), "v".to_string());
    info!("{}",serde_json::to_string(&data).unwrap());*/

    //let kad_bootstrap = KademliaDHT::new(bootstrap_node.clone(),None);
    let kad = KademliaDHT::new(current_node.clone(), Some(bootstrap_node.clone()));
    // let kad_remote = KademliaDHT::new(remote_node.clone(),Some(bootstrap_node.clone()));
    // let kad_remote2 = KademliaDHT::new(remote_node2.clone(),Some(bootstrap_node.clone()));
    let kadc = kad.clone();
    // let kadb = kad_bootstrap.clone();
    // let kadr = kad_remote.clone();
    // let kadr2 = kad_remote2.clone();

    //let threa3 = kad_bootstrap.init();

    //sleep(Duration::from_millis(1000));
    let threa1 = kad.init(Some("state_dumps/self.json".to_string()));
    //let threa2 = kad_remote.init();

    Arc::new(kadc.clone()).put(
        "test00".to_string(),
        "It works, this value was stored successifully".to_string(),
    );
    /*
    Arc::new(kadc.clone()).put("test".to_string(),"works".to_string());
    Arc::new(kadc.clone()).put("test2".to_string(), "works 2".to_string());
    sleep(Duration::from_millis(2000));
    Arc::new(kadr.clone()).put("test-remote".to_string(), "remote value stores".to_string());
    let val = Arc::new(kadr.clone()).get("test".to_string());
    let val2 = Arc::new(kadc.clone()).get("test-remote".to_string());

    debug!("value = {}, {}", val.unwrap_or("Not Found".to_string()), val2.unwrap_or("Not Found 2".to_string()));


    let t0 = kad_remote2.init();

    sleep(Duration::from_millis(6000));

    Arc::new(kadb).dump_state("state_dumps/boot.json");
    Arc::new(kadr).dump_state("state_dumps/remote.json");
    Arc::new(kadr2).dump_state("state_dumps/remote2.json");*/

    debug!("Done initial setup");

    threa1.join().expect("thead: dead"); /*
                                         threa2.join().expect("thead: dead");
                                         threa3.join().expect("thead: dead");
                                         t0.join().expect("thead: dead");*/
}

/*
   let mut blockchain = Blockchain::new();
   blockchain.add_block("This is the first block".to_string());
   blockchain.add_block("This is the second block".to_string());
   blockchain.add_block("This is the third block".to_string());

   println!("Is the blockchain valid? {}", blockchain.is_valid());
*/
