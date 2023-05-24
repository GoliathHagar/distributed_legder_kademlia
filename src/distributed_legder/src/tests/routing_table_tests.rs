use std::sync::Arc;
use std::thread;

use log::debug;

use crate::constants::fixed_sizes::DUMP_STATE_TIMEOUT;
use crate::constants::utils::get_local_ip;
use crate::dht::kademlia::KademliaDHT;
use crate::dht::routing_table::RoutingTable;
use crate::network::client::Client;
use crate::network::datagram::{Datagram, DatagramType};
use crate::network::key::Key;
use crate::network::node::Node;
use crate::network::rpc::Rpc;

#[test]
fn distance_to_self() {
    let node = Node::new(get_local_ip(), 1432);
    let node2 = node.clone();
    /* let mut k = node.id.0;
    k[31] ^= 255u8 ;
    k[30] ^= 4u8 ;*/
    // k[29] = 255u8 ;
    /* let key2 = Key(k);
    let node2 =  Node{
        id: key2.clone(),
        ip:"0.0.0.0".to_string(),
        port:2545
    };*/

    let rt = RoutingTable::new(Arc::new(node.clone()), None);

    let index = rt.node_find_bucket_index(&node2.id);

    debug!(
        "distance = {:?}, ,  index= {}",
        node.clone().id.distance(&node2.id),
        index
    );

    assert_eq!(255, index)
}

#[test]
fn get_k_closest_node() {
    let current = Node::new(get_local_ip(), 1432);
    let contact = Node::new("1.1.1.1".to_string(), 1540);

    let nodes1 = [
        Node::new("0.0.0.0".to_string(), 1432),
        Node::new("0.25.0.0".to_string(), 5665),
        Node::new("127.0.0.1".to_string(), 87),
        Node::new("10.6.0.1".to_string(), 25),
        Node::new("0.0.0.0".to_string(), 78),
        Node::new("1.1.1.1".to_string(), 1540),
        Node::new("2.0.0.0".to_string(), 1540),
        Node::new("8.8.8.8".to_string(), 9635),
        Node::new("8.0.0.8".to_string(), 963),
        Node::new("1.1.8.8".to_string(), 35),
        Node::new("125.0.8.8".to_string(), 635),
        Node::new("123.8.8.8".to_string(), 951),
        Node::new("8.8.47.8".to_string(), 7855),
        Node::new(
            "fg9sdg48fds6f8dg51dfsg4dfgd4f1gdgfdsgfdg45dfg\
        5dfgdggdfgsdfgdfgdfsgdfgsd"
                .to_string(),
            1285,
        ),
        Node::new("2".to_string(), 5665),
        Node::new("3".to_string(), 7835),
        Node::new("4".to_string(), 965),
        Node::new("5".to_string(), 6452),
        Node::new("8.7.8.8".to_string(), 485),
        Node::new("8.4.8.8".to_string(), 986),
        Node::new("8.4.8.8".to_string(), 123),
        Node::new("8.8.0.8".to_string(), 021),
        Node::new("125.125.152.2".to_string(), 021),
        Node::new("rest".to_string(), 021),
        Node::new("test".to_string(), 021),
        Node::new("ap".to_string(), 021),
        Node::new("dummy".to_string(), 021),
        Node::new("8".to_string(), 021),
    ]
        .to_vec();

    let mut rt = RoutingTable::new(Arc::new(current), None);

    for x in nodes1 {
        rt.update(x, None);
    }

    let cl = rt.get_closest_nodes(&contact.id, 5);
    for b in 0..rt.buckets.len() {
        println!("{} = {:?}", b, rt.buckets[b]);
    }
    for x in cl.clone() {
        println!(
            "closest {:?} {:?} {}",
            x.0,
            Key { 0: x.1 },
            RoutingTable::new(Arc::new(contact.clone()), None).node_find_bucket_index(&x.0.id)
        )
    }

    assert_eq!(cl.len(), 5)
}

#[test]
fn routing_table_building() {
    let btp = Node::new(get_local_ip(), 1543);

    let c = Node::new(get_local_ip(), 1430);

    let boot_stap_node = Arc::new(KademliaDHT::new(btp.clone(), None));
    let contact1 = Arc::new(KademliaDHT::new(
        c.clone(),
        Some(btp.clone()),
    ));

    let contact2 = Arc::new(KademliaDHT::new(
        Node::new(get_local_ip(), 1431),
        Some(btp.clone()),
    ));

    let contact4 = Arc::new(KademliaDHT::new(
        Node::new(get_local_ip(), 1434),
        Some(btp.clone()),
    ));

    let contact3 = Arc::new(KademliaDHT::new(
        Node::new(get_local_ip(), 1433),
        Some(btp.clone()),
    ));


    let client = Client::new(contact1.service.clone());
    let ac1 = contact1.clone();
    let ac2 =contact2.clone();
    let ac3 = contact3.clone();
    let ac4 = contact4.clone();

    let t0 = boot_stap_node.clone().init(Some("state_dumps/test-bootstrap-1.json".to_string()));
    let t1 = contact1.clone().init(Some("state_dumps/test-contact-1.json".to_string()));
    let t2 = contact2.clone().init(Some("state_dumps/test-contact-2.json".to_string()));
    let t3 = contact3.clone().init(Some("state_dumps/test-contact-3.json".to_string()));
    let t4 = contact4.clone().init(Some("state_dumps/test-contact-4.json".to_string()));

    if let Ok(l) = contact1.routing_table.lock() {
        let cls = l.get_closest_nodes(
            &Node::new(get_local_ip(), 1543).id,
            20,
        );
        for x in cls.clone() {
            println!("closest {:?} {:?}", x.0, Key { 0: x.1 })
        }
    };
    //
    ac1.put("key11".to_string(), "va gv".to_string());
    ac2.put("key21".to_string(), "va kholll".to_string());
    ac3.put("key31".to_string(), "val jkfssgdl".to_string());
    thread::sleep(std::time::Duration::from_millis(5*DUMP_STATE_TIMEOUT));
    ac4.clone().put("key41".to_string(), "vall".to_string());
    ac4.get("key21".to_string());

    client.clone().datagram_request(Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: c.get_address(),
        destination: format!(
            "{}:{}",
            get_local_ip(),
            1433
        ),
        data: Rpc::Ping,
    });

    client.clone().datagram_request(Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: c.get_address(),
        destination: format!(
            "{}:{}",
            get_local_ip(),
            1434
        ),
        data: Rpc::Ping,
    });

    client.clone().datagram_request(Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: c.get_address(),
        destination: format!(
            "{}:{}",
            get_local_ip(),
            1431
        ),
        data: Rpc::Ping,
    });

  client.clone().datagram_request(Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: c.get_address(),
        destination: format!(
            "{}:{}",
            get_local_ip(),
            1543
        ),
        data: Rpc::Ping,
    });

    client.clone().datagram_request(Datagram {
        data_type: DatagramType::KILL,
        token_id: Key::new("test".to_string()),
        source: c.get_address(),
        destination: format!(
            "{}:{}",
            get_local_ip(),
            1430
        ),
        data: Rpc::Ping,
    });

    t0.join().expect("t0");
    t1.join().expect("t1");
    t2.join().expect("t2");
    t3.join().expect("t3");
    t4.join().expect("t4");
}
