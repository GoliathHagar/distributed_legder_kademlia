use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Index;
use std::sync::Arc;

use log::{debug, error};

use crate::constants::fixed_sizes::{K_BUCKET_SIZE, KEY_SIZE, N_BUCKETS};
use crate::network::client::Client;
use crate::network::key::Key;
use crate::network::node::{Node, ThrustAndReputation};
use crate::network::rpc::Rpc;
use crate::network::rpc_socket::RpcSocket;

#[derive(Debug)]
pub struct Bucket {
    pub nodes: Vec<Node>,
    pub size: usize,
}

#[derive(Debug)]
pub struct RoutingTable {
    pub node: Arc<Node>,
    pub buckets: Vec<Bucket>,
    pub reputation : HashMap<Key, ThrustAndReputation>
}

#[derive(Debug, Eq, Hash, Clone)]
pub struct RoutingDistance(pub Node, pub [u8; KEY_SIZE]);
/*

A k-bucket with index i stores contacts whose ids
have a distance between 2^i and 2^i+1 to the own id`

*/
impl Bucket {
    pub fn new() -> Bucket {
        Self {
            nodes: Vec::new(),
            size: K_BUCKET_SIZE,
        }
    }
}

impl RoutingTable {
    pub fn new(node: Arc<Node>, bootstrap: Option<Node>) -> RoutingTable {
        let mut buckets: Vec<Bucket> = Vec::new();
        let reputation = HashMap::new();

        for _ in 0..N_BUCKETS {
            buckets.push(Bucket::new())
        }

        let mut rt = Self { buckets, node, reputation};

        if let Some(bootstrap) = bootstrap {
            rt.update(bootstrap, None)
        }

        rt
    }

    pub fn node_find_bucket_index(&self, key: &Key) -> usize {
        // given a bucket j, we are guaranteed that
        //  2^j <= distance(node, contact) < 2^(j+1)
        // a node with distance d will be put in the k-bucket with index i=⌊logd⌋

        let dst = self.clone().node.id.distance(key);

        for i in 0..KEY_SIZE {
            for j in (0..8).rev() {
                let bit = dst[i] & (0x01 << j);
                //println!("(i: {} ,j: {} , index: {}, dst {:#010b} , bit {} )", i,j,i*8 + 7-j, dst[i], bit.clone() );
                if bit != 0 {
                    debug!("(i: {} ,j: {} , index: {}, dst {:#010b} , bit {} )",
                        i, j, i * 8 + 7 - j, dst[i], bit.clone());

                    return i * 8 + 7 - j;
                }
                // else if i == KEY_SIZE -1 && j==0 { return thrust%256 } //distance to self
            }
        }

        KEY_SIZE * 8 - 1
    }

    pub fn get_closest_nodes(&self, key: &Key, capacity: usize) -> Vec<RoutingDistance> {
        let mut closest: Vec<RoutingDistance> = Vec::with_capacity(capacity);

        let mut bucket_index: usize = self.node_find_bucket_index(key);
        let mut bucket_index_reverse = bucket_index;

        //search forward (closests)
        while closest.len() < capacity && bucket_index < self.buckets.len() - 1 {
            for nd in &self.buckets[bucket_index].nodes {
                if nd.id.clone() == key.clone() { continue; }
                closest.push(RoutingDistance(nd.clone(), nd.id.distance(&key)))
            }

            bucket_index += 1;
        }

        //search backwards (farthest)
        while closest.len() < capacity && bucket_index_reverse > 0 {
            bucket_index_reverse -= 1;

            for n in &self.buckets[bucket_index_reverse].nodes {
                if n.id.clone() == key.clone() { continue; }
                closest.push(RoutingDistance(n.clone(), n.id.distance(&key)))
            }
        }

        closest.sort_by(|a, b| a.1.cmp(&b.1));

        closest.truncate(capacity);

        closest
    }

    pub fn remove(&mut self, node: &Node) {
        let bucket_idx: usize = self.node_find_bucket_index(&node.clone().id);

        if let Some(i) = self.buckets[bucket_idx]
            .nodes
            .iter()
            .position(|x| x.id == node.clone().id)
        {
           self.buckets[bucket_idx].nodes.remove(i);
        } else {
            error!("Tried to remove non-existing entry, {}", node.clone().get_address());
        }

        self.update_reputation(node.clone().id, false);

    }

    pub fn update_reputation(&mut self, key: Key, status : bool){
        if self.reputation.contains_key(&key.clone()) {
            self.reputation.insert(key.clone(), self.reputation.index(&key).clone().update_interaction(status));
        }
        else {
            self.reputation.insert(key, ThrustAndReputation::new().update_interaction(status));
        }
    }

    pub fn update(&mut self, update_node: Node, rpc: Option<Arc<RpcSocket>>) {
        let index = self.node_find_bucket_index(&update_node.id);

        if self.buckets[index].nodes.len() < K_BUCKET_SIZE {
            let node_idx = self.buckets[index].nodes.iter()
                .position(|x| x.id == update_node.clone().id);

            match node_idx {
                Some(i) => {
                    self.buckets[index].nodes.remove(i);
                    self.buckets[index].nodes.push(update_node.clone());
                    debug!("{:?}  push to routing table {:?} at {}", self.node.clone(), update_node.clone(), index);
                }
                None => {
                    self.buckets[index].nodes.push(update_node.clone());
                    debug!("{:?}  push to routing table {:?} at {}", self.node.clone(), update_node.clone(), index);
                }
            }

        }
        else if rpc.is_some(){
            let nd = self.buckets[index].nodes[0].clone();

            match Client::new(rpc.unwrap()).make_call(Rpc::Ping,nd.clone()).recv() {
                Ok(pong) => if let Some(_) = pong {
                    let add_front = self.buckets[index].nodes.remove(0);
                    self.buckets[index].nodes.push(add_front.clone());

                    debug!("{:?}  push to routing table {:?} at {}", self.node.clone(), update_node.clone(), index);
                }
                ,
                Err(_) => {
                    error!("Failed to contact node {:?}", nd.id);

                    self.buckets[index].nodes.remove(0);
                    self.buckets[index].nodes.push(update_node.clone());
                }
            };

        }

        self.update_reputation(update_node.clone().id, true);
    }
}

impl PartialEq for RoutingDistance {
    fn eq(&self, other: &Self) -> bool {
        let mut equal = true;
        let mut i = 0;
        while equal && i < KEY_SIZE {
            if self.1[i] != other.1[i] {
                equal = false;
            }

            i += 1;
        }

        equal
    }
}

impl PartialOrd for RoutingDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.1.cmp(&self.1))
    }
}

impl Ord for RoutingDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}
