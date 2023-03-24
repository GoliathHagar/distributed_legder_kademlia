use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use log::{debug, info};
use crate::dht::routing_table::RoutingTable;
use crate::dht::rpc::Rpc;
use crate::dht::rpc::Rpc::Pong;
use crate::network::client::Client;
use crate::network::datagram::{Datagram, DatagramType};
use crate::network::node::Node;
use crate::network::rpc_socket::RpcSocket;
use crate::network::server::Server;

#[derive(Clone, Debug)]
pub struct KademliaDHT{
    pub routing_table: Arc<Mutex<RoutingTable>>,
    pub store_values: Arc<Mutex<HashMap<String, String>>>,
    pub service: Arc<RpcSocket>,
    pub node: Node,
}

impl KademliaDHT{
    pub fn new(node: Node, bootstrap_node: Option<Node>) -> KademliaDHT {
        let routing = RoutingTable::new(node.clone(), bootstrap_node); //Todo: Routing Table
        let rpc = RpcSocket::new(node.clone());

        info!("Node id [{:?}] created read to start", node.id);

        Self{
            routing_table: Arc::new(Mutex::new(routing)),
            store_values: Arc::new(Mutex::new(HashMap::new())),
            service: Arc::new(rpc),
            node
        }

    }

    pub fn init(self) -> JoinHandle<()> {

        self.start_server()
    }


    fn start_server(self) -> JoinHandle<()> {
        let server = Server::new(Arc::new(self));
        let server_thread = server.start_service();

        server_thread

    }


    pub fn handle_request(app: Arc<KademliaDHT>, req: Datagram) -> Option<Datagram> {

        let payload : Datagram = req.clone();
        debug!("node {:?}", app.node);

        let resp: Option<Datagram> = match req.data {
            Rpc::Ping =>{
                app.ping(payload)
            }
            Rpc::FindNode(k) =>{
                app.find_node(payload)
            }
            Rpc::FindValue(k) =>{
                app.find_value(payload)
            }
            Rpc::Store(k,v) =>{
                app.store(payload)
            }
            _ => { None } //reply payload ignored
        };

        resp
    }

    pub fn ping(self : Arc<Self>, payload: Datagram) -> Option<Datagram> {
        //todo: complete GOLIATHHAGAR

        Some(
            Datagram{
                token_id: payload.token_id,
                source: payload.source,
                destination: payload.destination,
                data_type : DatagramType::RESPONSE,
                data: Pong
            }
        )
    }

    pub(self) fn find_node(self : Arc<Self>, payload: Datagram) -> Option<Datagram> {
        //Todo: find_node
        Some(payload)
    }

    pub(self) fn find_value(self : Arc<Self>, payload: Datagram) -> Option<Datagram> {
        //Todo: find_value
        Some(payload)
    }

    pub(self) fn store(self : Arc<Self>, payload: Datagram) -> Option<Datagram> {
        //Todo: store
        Some(payload)
    }

}

