use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::{Arc, mpsc, Mutex};
use log::error;
use crate::network::datagram::Datagram;
use crate::network::node::Node;

#[derive(Clone, Debug)]
pub struct RpcSocket{
    pub node : Arc<Node>,
    pub socket: Arc<UdpSocket>,
    pub await_response: Arc<Mutex<HashMap<String, mpsc::Sender<Option<Datagram>>>>>,
}

impl RpcSocket {
    pub fn new(node : Node) -> Self {
        let host = node.get_address();

        let socket = match UdpSocket::bind(&host){
            Ok(s) =>s,
            Err(e) =>{
                error!("Error while binding UdpSocket to specified addr {}", host);
                panic!("{}", e.to_string())
            }
        };


        Self{
            node: Arc::new(node),
            socket: Arc::new(socket),
            await_response: Arc::new(Mutex::new(HashMap::new()))
        }

    }
}

