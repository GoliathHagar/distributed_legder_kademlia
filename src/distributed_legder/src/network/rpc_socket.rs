use crate::network::datagram::Datagram;
use crate::network::key::Key;
use crate::network::node::Node;
use log::error;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::{mpsc, Arc, Mutex};

#[derive(Clone, Debug)]
pub struct RpcSocket {
    pub node: Arc<Node>,
    pub socket: Arc<UdpSocket>,
    pub awaiting_response: Arc<Mutex<HashMap<Key, mpsc::Sender<Option<Datagram>>>>>,
}

impl RpcSocket {
    pub fn new(node: Node) -> Self {
        let host = node.get_address();

        let socket = match UdpSocket::bind(&host) {
            Ok(s) => s,
            Err(e) => {
                error!("UdpSocket to specified addr {} -> {}", host, e.to_string());
                panic!("{}", e.to_string())
            }
        };

        Self {
            node: Arc::new(node),
            socket: Arc::new(socket),
            awaiting_response: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
