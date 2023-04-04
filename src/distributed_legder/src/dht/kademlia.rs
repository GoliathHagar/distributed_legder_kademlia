use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use log::{debug, error, info};
use crate::constants::fixed_sizes::K_BUCKET_SIZE;
use crate::dht::routing_table::RoutingTable;
use crate::dht::rpc::Rpc;
use crate::network::datagram::{Datagram, DatagramType};
use crate::network::key::Key;
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
                app.ping_reply(payload)
            }
            Rpc::FindNode(k) =>{
                app.find_node_reply(payload)
            }
            Rpc::FindValue(k) =>{
                app.find_value_reply(payload)
            }
            Rpc::Store(k,v) =>{
                app.store_reply(payload)
            }
            _ => { None } //reply payload ignored
        };

        resp
    }

    pub fn ping_reply(self : Arc<Self>, payload: Datagram) -> Option<Datagram> {
        Some(
            Datagram{
                token_id: payload.token_id,
                source: payload.source,
                destination: payload.destination,
                data_type : DatagramType::RESPONSE,
                data: Rpc::Pong
            }
        )
    }

    pub(self) fn find_node_reply(self : Arc<Self>, payload: Datagram) -> Option<Datagram> {

        let routes = match self.routing_table.lock() {
            Ok(rt) => rt,
            Err(s) => {
                error!("Failed to acquire lock on Routing Table");
                return None;
            }
        };

        if let Rpc::FindNode(key) = payload.data.clone() {

            let res = routes.get_closest_nodes(&key, K_BUCKET_SIZE);
            let closest_nodes : Vec<Node> = res.iter().map(|n | n.0.clone()).collect();

           return  Some(
                 Datagram{
                     token_id: payload.token_id,
                     source: payload.source,
                     destination: payload.destination,
                     data_type : DatagramType::RESPONSE,
                     data: Rpc::FindNodeReply(closest_nodes)
                 }
            )
        }

        None
    }

    pub(self) fn find_value_reply(self : Arc<Self>, payload: Datagram) -> Option<Datagram> {
        //Todo: find_value -> se não tiver o Nó envia os k Nós mais próximos do valor??


        let store_value = match self.store_values.lock() {
            Ok(sv) => sv,
            Err(s) => {
                error!("Failed to acquire lock on Store Values");
                return None;
            }
        };


        if let Rpc::FindValue(k) = payload.data.clone() {
            let value = store_value.get(k.as_str());

            return if value.is_some() {
                Some(
                    Datagram {
                        token_id: payload.token_id,
                        source: payload.source,
                        destination: payload.destination,
                        data_type: DatagramType::RESPONSE,
                        data: Rpc::FindValueReply(k, value?.to_string())
                    }
                )
            } else {
                drop(store_value);
                let mut data = payload.clone();

                data.data = Rpc::FindNode(Key::new(k));

                self.find_node_reply(data)
            }
        }

        None

    }

    pub(self) fn store_reply(self : Arc<Self>, payload: Datagram) -> Option<Datagram> {
        //Todo: store
        let mut store_value = match self.store_values.lock() {
            Ok(sv) => sv,
            Err(s) => {
                error!("Failed to acquire lock on Store Values");
                return None;
            }
        };


        if let Rpc::Store(k, value) = payload.data {
            store_value.insert(k.clone(), value.clone());

            return Some(
                Datagram {
                    token_id: payload.token_id,
                    source: payload.source,
                    destination: payload.destination,
                    data_type: DatagramType::RESPONSE,
                    data: Rpc::Pong
                }
            );
        }
        None
    }

    pub(self) fn node_lookup(self : Arc<Self>, key : &Key) -> Vec<Node> {
        let mut nodes = Vec::new();

        // nodes visited
        let mut queried  = HashSet::new();

        let routes = match self.routing_table.lock() {
            Ok(rt) => rt,
            Err(s) => {
                error!("Failed to acquire lock on Routing Table");
                return nodes;
            }
        };

        // nodes to visit
        let mut to_query = BinaryHeap::from(routes.get_closest_nodes(key, K_BUCKET_SIZE));
        drop(routes);



        nodes
    }


    pub fn store_value(self : Arc<Self>, key : String, value : String) -> bool {
        let mut store_value = match self.store_values.lock() {
            Ok(sv) => sv,
            Err(s) => {
                error!("Failed to acquire lock on Store Values");
                return false;
            }
        };
        store_value.insert(key,value);

        true
    }

}

