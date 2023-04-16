use crate::dht::kademlia::KademliaDHT;
use crate::dht::rpc::Rpc;
use crate::network::client::Client;
use crate::network::datagram::{Datagram, DatagramType};
use crate::network::key::Key;
use crate::network::node::Node;
use std::sync::Arc;

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

    let kad = KademliaDHT::new(current_node.clone(), None);
    let kad2 = KademliaDHT::new(remote_node.clone(), None);

    let threa1 = kad.clone().init();
    let threa2 = kad2.clone().init();

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

    let kad = KademliaDHT::new(current_node.clone(), None);

    let threa1 = kad.clone().init();

    let client = Client::new(kad.service.clone());

    let rec: Option<Datagram> = client
        .clone()
        .make_call(
            Rpc::FindValue("test".to_string()),
            Node::new("127.0.0.1".to_string(), 8081),
        )
        .recv()
        .unwrap();

    client.datagram_request(kill.clone());

    threa1.join().expect("thead 1 dead");

    assert_eq!(rec, None);
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

    let kad = KademliaDHT::new(current_node.clone(), None);

    let key = "test".to_string();

    let value = "this is a test".to_string();

    Arc::new(kad.clone()).put(key.clone(), value.clone());

    let threa1 = kad.clone().init();

    let client = Client::new(kad.service.clone());

    let rec: Option<Datagram> = client
        .clone()
        .make_call(
            Rpc::FindValue("test".to_string()),
            Node::new("127.0.0.1".to_string(), 8082),
        )
        .recv()
        .unwrap();

    client.datagram_request(kill.clone());

    threa1.join().expect("thead 1 dead");

    assert_eq!(
        rec.unwrap().data,
        Rpc::FindValueReply(key.clone(), value.clone())
    );
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

    let kad = KademliaDHT::new(current_node.clone(), None);

    let threa1 = kad.clone().init();

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

    assert_eq!(rec, None);
}
