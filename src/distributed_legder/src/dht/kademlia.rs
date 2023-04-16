use crate::constants::fixed_sizes::{ALPHA, DUMP_STATE_TIMEOUT, K_BUCKET_SIZE, REPUBLISH_TIMEOUT};
use crate::dht::routing_table::{Bucket, RoutingDistance, RoutingTable};
use crate::dht::rpc::Rpc;
use crate::network::client::Client;
use crate::network::datagram::{Datagram, DatagramType};
use crate::network::key::Key;
use crate::network::node::Node;
use crate::network::rpc_socket::RpcSocket;
use crate::network::server::Server;
use log::{error, info};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::create_dir_all;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone, Debug)]
pub struct KademliaDHT {
    pub routing_table: Arc<Mutex<RoutingTable>>,
    pub store_values: Arc<Mutex<HashMap<String, String>>>,
    pub service: Arc<RpcSocket>,
    pub node: Node,
}

impl KademliaDHT {
    pub fn new(node: Node, bootstrap_node: Option<Node>) -> KademliaDHT {
        let routing = RoutingTable::new(node.clone(), bootstrap_node); //Todo: Routing Table
        let rpc = RpcSocket::new(node.clone());

        info!("Node id [{:?}] created read to start", node.id);

        Self {
            routing_table: Arc::new(Mutex::new(routing)),
            store_values: Arc::new(Mutex::new(HashMap::new())),
            service: Arc::new(rpc),
            node,
        }
    }

    pub fn init(self, path: Option<String>) -> JoinHandle<()> {
        let kdl = Arc::new(self.clone());
        let ss =   self.start_server();

        let kad = kdl.clone();
        thread::spawn(move || loop {

            thread::sleep(std::time::Duration::from_millis(REPUBLISH_TIMEOUT));
            kad.clone().republish();
        });

        //dump state
        if let Some(pd) = path{
            let copy_self = kdl.clone();
            let p_dir = pd.clone();

            thread::spawn(move || loop {
                thread::sleep(std::time::Duration::from_millis(DUMP_STATE_TIMEOUT));
                copy_self.dump_state(p_dir.as_str())
            });

            info!("Dump State thead started");

        }

        ss
    }

    fn start_server(self) -> JoinHandle<()> {
        let kdl = Arc::new(self.clone());
        let server = Server::new(kdl.clone());
        let server_thread = server.start_service();

        kdl.node_lookup(&self.node.id);

        server_thread
    }

    fn republish(self: Arc<Self>) {
        let store_value = match self.store_values.lock() {
            Ok(sv) => sv,
            Err(_) => {
                error!("Failed to acquire lock on Store Values");
                return;
            }
        };

        for (key, value) in &*store_value {
            self.clone().put(key.to_string(), value.to_string());
        }
    }

    pub fn handle_request(app: Arc<KademliaDHT>, req: Datagram) -> Option<Datagram> {
        let payload: Datagram = req.clone();
        let proto = app.clone();

        let resp: Option<Datagram> = match req.data {
            Rpc::Ping => proto.ping_reply(payload),
            Rpc::FindNode(_) => proto.find_node_reply(payload),
            Rpc::FindValue(_) => proto.find_value_reply(payload),
            Rpc::Store(_, _) => proto.store_reply(payload),
            _ => None, //reply payload ignored
        };

        if let Some(v) = req.extract_src_ip_port() {
            let mut routes = match app.routing_table.lock() {
                Ok(rt) => rt,
                Err(_) => {
                    error!("Failed to acquire lock on Routing Table");
                    return resp;
                }
            };

            routes.update(Node::new(v.0, v.1), Some(app.service.clone()));
        }

        resp
    }

    pub(self) fn ping_reply(self: Arc<Self>, payload: Datagram) -> Option<Datagram> {
        Some(Datagram {
            token_id: payload.token_id,
            source: payload.source,
            destination: payload.destination,
            data_type: DatagramType::RESPONSE,
            data: Rpc::Pong,
        })
    }

    pub(self) fn find_node_reply(self: Arc<Self>, payload: Datagram) -> Option<Datagram> {
        let routes = match self.routing_table.lock() {
            Ok(rt) => rt,
            Err(_) => {
                error!("Failed to acquire lock on Routing Table");
                return None;
            }
        };

        if let Rpc::FindNode(key) = payload.data.clone() {
            let res = routes.get_closest_nodes(&key, K_BUCKET_SIZE);
            let closest_nodes: Vec<Node> = res.iter().map(|n| n.0.clone()).collect();

            return Some(Datagram {
                token_id: payload.token_id,
                source: payload.source,
                destination: payload.destination,
                data_type: DatagramType::RESPONSE,
                data: Rpc::FindNodeReply(closest_nodes),
            });
        }

        None
    }

    pub(self) fn find_value_reply(self: Arc<Self>, payload: Datagram) -> Option<Datagram> {
        //Todo: find_value -> se não tiver o Nó envia os k Nós mais próximos do valor??
        if let Rpc::FindValue(k) = payload.data.clone() {
            let store_value = match self.store_values.lock() {
                Ok(sv) => sv,
                Err(_) => {
                    error!("Failed to acquire lock on Store Values");
                    return None;
                }
            };

            let value = store_value.get(k.as_str());

            return if value.is_some() {
                Some(Datagram {
                    token_id: payload.token_id,
                    source: payload.source,
                    destination: payload.destination,
                    data_type: DatagramType::RESPONSE,
                    data: Rpc::FindValueReply(k, value?.to_string()),
                })
            } else {
                drop(store_value);
                let mut data = payload.clone();

                data.data = Rpc::FindNode(Key::new(k));

                self.find_node_reply(data)
            };
        }

        None
    }

    pub(self) fn store_reply(self: Arc<Self>, payload: Datagram) -> Option<Datagram> {
        //Todo: store
        if let Rpc::Store(k, value) = payload.data {
            let mut store_value = match self.store_values.lock() {
                Ok(sv) => sv,
                Err(_) => {
                    error!("Failed to acquire lock on Store Values");
                    return None;
                }
            };

            store_value.insert(k.clone(), value.clone());

            return Some(Datagram {
                token_id: payload.token_id,
                source: payload.source,
                destination: payload.destination,
                data_type: DatagramType::RESPONSE,
                data: Rpc::Pong,
            });
        }
        None
    }

    pub(self) fn node_lookup(self: Arc<Self>, key: &Key) -> Vec<RoutingDistance> {
        let mut nodes = Vec::new();

        // nodes visited
        let mut queried: HashSet<RoutingDistance> = HashSet::new();

        let routes = match self.routing_table.lock() {
            Ok(rt) => rt,
            Err(_) => {
                error!("Failed to acquire lock on Routing Table");
                return nodes;
            }
        };

        // nodes to visit
        let mut to_query = BinaryHeap::from(routes.get_closest_nodes(key, K_BUCKET_SIZE));
        drop(routes);

        for nd in &to_query {
            queried.insert(nd.clone());
        }

        while !to_query.is_empty() {
            let mut jobs: Vec<JoinHandle<Option<Vec<RoutingDistance>>>> = Vec::new();

            let mut queries: Vec<RoutingDistance> = Vec::new();
            let mut results: Vec<Option<Vec<RoutingDistance>>> = Vec::new();

            for _ in 0..ALPHA {
                match to_query.pop() {
                    Some(entry) => {
                        queries.push(entry);
                    }
                    None => {
                        break;
                    }
                }
            }

            for &RoutingDistance(ref node, _) in &queries {
                let no = node.clone();
                let k = key.clone();
                let kad = self.clone();

                jobs.push(thread::spawn(move || kad.find_node(k, no)));
            }

            for job in jobs {
                if let Ok(j) = job.join() {
                    results.push(j);
                } else {
                    error!("Node Lookup failed to join thread while visiting nodes")
                }
            }

            for (r, q) in results.into_iter().zip(queries) {
                if let Some(nds) = r {
                    nodes.push(q);

                    for nd in nds {
                        if queried.insert(nd.clone()) {
                            to_query.push(nd)
                        }
                    }
                }
            }
        }

        nodes.sort_by(|a, b| a.1.cmp(&b.1));

        nodes.truncate(K_BUCKET_SIZE);

        nodes
    }

    pub(self) fn value_lookup(
        self: Arc<Self>,
        value_key: String,
    ) -> (Option<String>, Vec<RoutingDistance>) {
        let mut nodes = Vec::new();
        let key_search = Key::new(value_key.clone()); // search value nodes 'closer' to key
        let mut queried: HashSet<RoutingDistance> = HashSet::new(); // nodes visited

        let routes = match self.routing_table.lock() {
            Ok(rt) => rt,
            Err(_) => {
                error!("Failed to acquire lock on Routing Table");
                return (None, nodes);
            }
        };

        // nodes to visit
        let mut to_query = BinaryHeap::from(routes.get_closest_nodes(&key_search, K_BUCKET_SIZE));
        drop(routes);

        for nd in &to_query {
            queried.insert(nd.clone());
        }

        while !to_query.is_empty() {
            let mut jobs: Vec<JoinHandle<Option<Rpc>>> = Vec::new();

            let mut queries: Vec<RoutingDistance> = Vec::new();
            let mut results: Vec<Option<Rpc>> = Vec::new();

            for _ in 0..ALPHA {
                match to_query.pop() {
                    Some(entry) => {
                        queries.push(entry);
                    }
                    None => {
                        break;
                    }
                }
            }

            for &RoutingDistance(ref node, _) in &queries {
                let no = node.clone();
                let k_val = value_key.clone();
                let kad = self.clone();

                jobs.push(thread::spawn(move || kad.find_value(k_val, no)));
            }

            for job in jobs {
                if let Ok(j) = job.join() {
                    results.push(j);
                } else {
                    error!("Value Lookup failed to join thread while visiting nodes")
                }
            }

            for (r, q) in results.into_iter().zip(queries) {
                if let Some(rpc) = r {
                    match rpc {
                        Rpc::FindValueReply(_, v) => {
                            nodes.sort_by(|a, b| a.1.cmp(&b.1));
                            nodes.truncate(K_BUCKET_SIZE);

                            return (Some(v), nodes);
                        }

                        Rpc::FindNodeReply(nds) => {
                            nodes.push(q);

                            for n in nds {
                                let nd = RoutingDistance(n.clone(), n.id.distance(&key_search));
                                if queried.insert(nd.clone()) {
                                    to_query.push(nd)
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        nodes.sort_by(|a, b| a.1.cmp(&b.1));

        nodes.truncate(K_BUCKET_SIZE);

        (None, nodes)
    }

    fn ping(self: Arc<Self>, destination: Node) -> bool {
        let client = Client::new(self.service.clone());

        let pong = client.make_call(Rpc::Ping, destination.clone()).recv();

        let mut routes = match self.routing_table.lock() {
            Ok(rt) => rt,
            Err(_) => {
                error!("Failed to acquire lock on Routing Table");
                return false;
            }
        };

        return match pong {
            Ok(Some(datag)) => {
                if let Rpc::Pong = datag.data {
                    routes.update(destination, Some(self.service.clone()));
                    return true;
                }

                false
            }
            _ => {
                routes.remove(&destination);
                false
            }
        };
    }
    fn store(self: Arc<Self>, payload: (String, String), destination: Node) -> bool {
        let client = Client::new(self.service.clone());

        let stored_pong = client
            .make_call(Rpc::Store(payload.0, payload.1), destination.clone())
            .recv();

        let mut routes = match self.routing_table.lock() {
            Ok(rt) => rt,
            Err(_) => {
                error!("Failed to acquire lock on Routing Table");
                return false;
            }
        };

        return match stored_pong {
            Ok(Some(payload)) => {
                if let Rpc::Pong = payload.data {
                    routes.update(destination, Some(self.service.clone()));
                    return true;
                }

                false
            }
            _ => {
                routes.remove(&destination);
                false
            }
        };
    }
    fn find_node(self: Arc<Self>, key: Key, destination: Node) -> Option<Vec<RoutingDistance>> {
        let nodes: Vec<RoutingDistance> = Vec::new();

        let client = Client::new(self.service.clone());

        let value = client
            .make_call(Rpc::FindNode(key), destination.clone())
            .recv();

        let mut routes = match self.routing_table.lock() {
            Ok(rt) => rt,
            Err(_) => {
                error!("Failed to acquire lock on Routing Table");
                return Some(nodes);
            }
        };

        if let Ok(Some(d)) = value {
            if let Rpc::FindNodeReply(n) = d.data {
                routes.update(destination, Some(self.service.clone()));

                let ret: Vec<RoutingDistance> = n
                    .iter()
                    .map(|a| RoutingDistance(a.clone(), self.node.id.distance(&a.id)))
                    .collect();

                return Some(ret);
            }

            None
        } else {
            routes.remove(&destination);
            None
        }
    }
    fn find_value(self: Arc<Self>, key: String, destination: Node) -> Option<Rpc> {
        let client = Client::new(self.service.clone());

        let value = client
            .make_call(Rpc::FindValue(key), destination.clone())
            .recv();

        let mut routes = match self.routing_table.lock() {
            Ok(rt) => rt,
            Err(_) => {
                error!("Failed to acquire lock on Routing Table");
                return None;
            }
        };

        return match value {
            Ok(Some(dg)) => {
                routes.update(destination, Some(self.service.clone()));

                Some(dg.data)
            }
            _ => {
                routes.remove(&destination);
                None
            }
        };
    }

    pub fn put(self: Arc<Self>, key: String, value: String) {
        let candidates = self.clone().node_lookup(&Key::new(key.clone()));

        for RoutingDistance(node, _) in candidates {
            let kad = self.clone();
            let payload = (key.clone(), value.clone());

            thread::spawn(move || kad.store(payload, node));
        }
    }

    pub fn get(self: Arc<Self>, key: String) -> Option<String> {
        let (value, mut nodes) = self.clone().value_lookup(key.clone());

        value.map(|val| {
            if let Some(RoutingDistance(contact, _)) = nodes.pop() {
                self.store((key.clone(), val.clone()), contact);
            } else {
                self.clone()
                    .store((key.clone(), val.clone()), self.node.clone());
            }

            val
        })
    }

    fn dump_state(&self, path: &str) {
        if let Err(_) = create_dir_all("state_dumps") {
            error!("Unable to create state dumps diretory");
            return;
        }

        let routes = match self.routing_table.lock() {
            Ok(rt) => rt,
            Err(_) => {
                error!("Failed to acquire lock on Routing Table");
                return;
            }
        };

        let stored = match self.store_values.lock() {
            Ok(s) => s,
            Err(_) => {
                error!("Failed to acquire lock on Stored values");
                return;
            }
        };

        let flattened: Vec<&Bucket> = routes.buckets.iter().collect();

        let mut parsed_buckets = Vec::new();

        for kb in flattened {
            for n in &kb.nodes {
                let bucket = serde_json::json!(
                    {
                        "nodes": {
                            "ip": n.ip,
                            "port": n.port,
                            "id": format!("{:?}", n.id),
                        },
                        "size": kb.size,
                     }
                );
                parsed_buckets.push(bucket);
            }
        }

        let mut parsed_store = Vec::new();
        // parse store
        for (k, v) in &*stored {
            let obj = serde_json::json!({ k: v });
            parsed_store.push(obj);
        }

        let json = serde_json::json!(
            {
                "node": {
                    "ip": self.node.ip,
                    "port": self.node.port,
                    "id": format!("{:?}", self.node.id),
                },
                "routes": {
                    /*"node": {
                        "ip": routes.node.ip,
                        "port": routes.node.port,
                        "id": format!("{:?}", routes.node.id),
                    },*/
                    "kbuckets": parsed_buckets,
                },
                "store": parsed_store,
                "rpc": {
                    "socket": format!("{:?}", self.service.socket),
                    "awaiting_response": format!("{:?}", self.service.awaiting_response.lock().unwrap()),
                    /*"node": {
                        "ip": self.service.node.ip,
                        "port": self.service.node.port,
                        "id": format!("{:?}", self.service.node.id),
                    },*/
                }
            }
        );

        let mut file = match std::fs::File::create(path) {
            Ok(f) => f,
            Err(_) => {
                error!("Unable to create dump file");
                return;
            }
        };

        if let Err(_) = file.write_all(&json.to_string().as_bytes()) {
            error!("Unable to create dump file Json");
        }

        let mut diagram = match std::fs::File::create(format!("{}.plantuml", path)) {
            Ok(d) => d,
            Err(_) => {
                error!("Unable to create dump file Diagram");
                return;
            }
        };

        if let Err(_) = diagram.write_all("@startjson\n".to_string().as_bytes()) {
            error!("Unable to write to dump file");
            return;
        }

        if let Err(_) = diagram.write_all(&json.to_string().as_bytes()) {
            error!("Unable to write to dump file");
            return;
        }

        if let Err(_) = diagram.write_all("\n@endjson".to_string().as_bytes()) {
            error!("Unable to write to dump file");
            return;
        }
    }
}
