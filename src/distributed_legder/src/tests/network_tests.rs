use std::sync::Arc;
use std::thread;

use crate::blockchain::block::Block;
use crate::blockchain::consensus::ConsensusAlgorithm;
use crate::blockchain::miner::Miner;
use crate::constants::fixed_sizes::DUMP_STATE_TIMEOUT;
use crate::constants::multicast_info_type::MulticastInfoType;
use crate::constants::utils::{block_to_string, calculate_block_hash, get_local_ip};
use crate::dht::kademlia::KademliaDHT;
use crate::network::client::Client;
use crate::network::datagram::{Datagram, DatagramType};
use crate::network::key::Key;
use crate::network::node::Node;
use crate::network::rpc::Rpc;

#[test]
fn two_way_handshake_ping_pong() {
    let data = &Datagram {
        data_type: DatagramType::REQUEST,
        token_id: Key::new("test".to_string()),
        source: "127.0.0.1:1234".to_string(),
        destination: "127.0.0.1:8000".to_string(),
        data: Rpc::Ping,
    };
    let kill = &Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: "127.0.0.1:1234".to_string(),
        destination: "127.0.0.1:8000".to_string(),
        data: Rpc::Ping,
    };

    let kill2 = &Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: "127.0.0.1:8000".to_string(),
        destination: "127.0.0.1:1234".to_string(),
        data: Rpc::Ping,
    };

    println!("{:?}", data);

    let current_node = Node::new("127.0.0.1".to_string(), 1234);
    let remote_node = Node::new("127.0.0.1".to_string(), 8000);

    let kad = Arc::new(KademliaDHT::new(current_node.clone(), None));
    let kad2 = Arc::new(KademliaDHT::new(remote_node.clone(), None));

    let threa1 = kad.clone().init(None);
    let threa2 = kad2.clone().init(None);

    let client = Client::new(kad.service.clone());
    let client2 = Client::new(kad2.service.clone());

    let rec: Datagram = client
        .clone()
        .make_call(Rpc::Ping, remote_node.clone())
        .recv()
        .unwrap()
        .unwrap();

    client.datagram_request(kill.clone());
    client2.datagram_request(kill2.clone());

    threa1.join().expect("thead 1 dead");
    threa2.join().expect("thread 2 dead");

    assert_eq!(rec.data, Rpc::Pong);
    assert_eq!(rec.source, remote_node.get_address());
    assert_eq!(rec.destination, current_node.get_address());
}

#[test]
fn test_no_response() {
    /*let mut data = &Datagram {
            data_type: DatagramType::REQUEST,
            token_id: "test".to_string(),
            source: "127.0.0.1:8080".to_string(),
            destination: "127.0.0.1:12345".to_string(),
            data: Rpc::Ping
        };
        let kill = &Datagram {
            data_type: DatagramType::KILL,
            token_id: Key::new("test".to_string()),
            source: "127.0.0.1:8080".to_string(),
            destination: "127.0.0.1:8080".to_string(),
            data: Rpc::Ping
        };
    */
    let current_node = Node::new("127.0.0.1".to_string(), 8080);

    let kad = Arc::new(KademliaDHT::new(current_node.clone(), None));

    let client = Client::new(kad.service.clone());

    let rec: Option<Datagram> = client
        .clone()
        .make_call(Rpc::Ping, Node::new("127.0.0.1".to_string(), 8080))
        .recv()
        .unwrap();

    assert_eq!(rec, None);
}

#[test]
fn test_find_value_not_store() {
    /*let mut data = &Datagram {
        data_type: DatagramType::REQUEST,
        token_id: "test".to_string(),
        source: "127.0.0.1:8080".to_string(),
        destination: "127.0.0.1:12345".to_string(),
        data: Rpc::Ping
    };*/
    let kill = &Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: "127.0.0.1:8081".to_string(),
        destination: "127.0.0.1:8081".to_string(),
        data: Rpc::Ping,
    };

    let current_node = Node::new("127.0.0.1".to_string(), 8081);

    let kad = Arc::new(KademliaDHT::new(current_node.clone(), None));

    let threa1 = kad.clone().init(None);

    let client = Client::new(kad.service.clone());

    let rec: Option<Datagram> = client
        .clone()
        .make_call(
            Rpc::FindValue("test".to_string()),
            current_node,
        )
        .recv()
        .unwrap();

    client.datagram_request(kill.clone());

    threa1.join().expect("thead 1 dead");

    let data = match rec.unwrap().data {
        Rpc::FindValueReply(k, v) => {
            Some((k,v))
        }
        _ => None
    };

    assert_eq!(None, data );
}
#[test]
fn test_find_value_store_successful() {
    /*let mut data = &Datagram {
        data_type: DatagramType::REQUEST,
        token_id: "test".to_string(),
        source: "127.0.0.1:8080".to_string(),
        destination: "127.0.0.1:12345".to_string(),
        data: Rpc::Ping
    };*/
    let kill = &Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: "127.0.0.1:8082".to_string(),
        destination: "127.0.0.1:8082".to_string(),
        data: Rpc::Ping,
    };

    let current_node = Node::new("127.0.0.1".to_string(), 8082);

    let kad = Arc::new(KademliaDHT::new(current_node.clone(), None));

    let key = "test".to_string();

    let value = "this is a test".to_string();


    let threa1 = kad.clone().init(None);

    kad.clone().put(key.clone(), value.clone());

    let client = Client::new(kad.service.clone());

    let rec: Option<Datagram> = client
        .clone()
        .make_call(
            Rpc::FindValue(key.clone()),
            current_node,
        )
        .recv()
        .unwrap();

    client.datagram_request(kill.clone());

    threa1.join().expect("thead 1 dead");

    let data = rec.unwrap().data;

    assert!(
          (
                Rpc::FindNodeReply(Vec::new()) == data
          ) || (
            Rpc::FindValueReply(key.clone(), value.clone()) == data
          )
    )


}
#[test]
fn test_store() {
    /*let mut data = &Datagram {
        data_type: DatagramType::REQUEST,
        token_id: "test".to_string(),
        source: "127.0.0.1:8080".to_string(),
        destination: "127.0.0.1:12345".to_string(),
        data: Rpc::Ping
    };*/
    let kill = &Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: "127.0.0.1:8083".to_string(),
        destination: "127.0.0.1:8083".to_string(),
        data: Rpc::Ping,
    };

    let current_node = Node::new("127.0.0.1".to_string(), 8083);

    let kad = Arc::new(KademliaDHT::new(current_node.clone(), None));

    let threa1 = kad.clone().init(None);

    let client = Client::new(kad.service.clone());

    let rec: Option<Datagram> = client
        .clone()
        .make_call(
            Rpc::Store("test".to_string(), "this is a test".to_string()),
            Node::new("127.0.0.1".to_string(), 8083),
        )
        .recv()
        .unwrap();

    client.datagram_request(kill.clone());

    threa1.join().expect("thead 1 dead");

    assert_eq!(rec.unwrap().data, Rpc::Pong);
}


#[test]
fn test_broadcast_nodes() {

    let btp = Node::new(get_local_ip(), 1432);

    // let boot_stap_node = KademliaDHT::new(btp.clone(), None);
    let c = Node::new(get_local_ip(), 4441);
    let c1 = Node::new(get_local_ip(), 8542);

    let kill = &Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: c.get_address(),
        destination: c1.get_address(),
        data: Rpc::Ping,
    };

    let kill1 = &Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: c.get_address(),
        destination: btp.get_address(),
        data: Rpc::Ping,
    };

    let kill_self = &Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: c.get_address(),
        destination: c.get_address(),
        data: Rpc::Ping,
    };

    let bt =Arc::new( KademliaDHT::new(
        btp.clone(),
        None,
    ));

    let contact1 = Arc::new(KademliaDHT::new(
        c.clone(),
        Some(btp.clone()),
    ));

    let contact2 = Arc::new(KademliaDHT::new(
        c1.clone(),
        Some(btp.clone()),
    ));


    let client = Client::new(contact1.service.clone());

    let c1c = contact1.clone();
    let c2c = contact2.clone();

    //
    let t0 = bt.clone().init(Some("state_dumps/test-network-boot-1.json".to_string()));
    let t1 = contact1.init(Some("state_dumps/test-network-1.json".to_string()));
    let t2 = contact2.init(Some("state_dumps/test-network-2.json".to_string()));

    let mut block = Block::new(1, "0000343260d0fda774e64673fb0ce7b66eb3942bfdbcd9bc24019f48cb70ba48".to_string(), "".to_string(), Vec::new());

    let nonce = Arc::new(Miner::new(ConsensusAlgorithm::ProofOfWork)).mine_block(block.clone());
    block.header.nonce = nonce;
    let id = calculate_block_hash(&block);
    block.header.hash = id.clone();

    let expected = (id, MulticastInfoType::Block, block_to_string(block));

    c1c.clone().put(expected.clone().0, expected.clone().2);

    c1c.broadcast_info((expected.clone().0, expected.clone().1, expected.clone().2));

    let binfo = c2c.multicast_subscriber();


    thread::sleep(std::time::Duration::from_millis(DUMP_STATE_TIMEOUT));

    client.clone().datagram_request(kill.clone());
    //
    client.clone().datagram_request(kill1.clone());
    client.datagram_request(kill_self.clone());

    t1.join().expect("thead 1 dead");
    t2.join().expect("thead 1 dead");

    //
    t0.join().expect("thread 2 dead");

    assert_eq!(Rpc::Multicasting(expected.0, expected.1, expected.2), binfo);
}