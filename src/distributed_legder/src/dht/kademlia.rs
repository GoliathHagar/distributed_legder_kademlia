use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use log::info;
use crate::dht::routing_table::RoutingTable;
use crate::network::datagram::Datagram;
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
        let routing = RoutingTable; //Todo: Routing Table
        let rpc = RpcSocket::new(node.clone());

        info!("Node id [{:?}] created read to start", node.id);

        Self{
            routing_table: Arc::new(Mutex::new(routing)),
            store_values: Arc::new(Mutex::new(HashMap::new())),
            service: Arc::new(rpc),
            node
        }

    }

    pub fn start_server(self) -> JoinHandle<()> {
        let server = Server::new(Arc::new(self));
        let server_thread = server.start_service();

        server_thread

    }
    pub fn handle_request(app: Arc<KademliaDHT>, req: Datagram) -> Datagram {

        println!("node {:?}", app.node);

        req
    }

}

