use crate::constants::fixed_sizes::{ENABLE_THRUST_MECHANISM, KEY_SIZE, K_BUCKET_SIZE, N_BUCKETS, BALANCE_FACTOR};
use crate::network::rpc::Rpc;
use crate::network::client::Client;
use crate::network::key::Key;
use crate::network::node::{Node, ThrustAndReputation};
use crate::network::rpc_socket::RpcSocket;
use log::{debug, error};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Deref, Index};
use std::sync::{Arc, Mutex};

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
        let mut reputation = HashMap::new();

        for _ in 0..N_BUCKETS {
            buckets.push(Bucket::new())
        }

        let mut rt = Self { buckets, node, reputation};

        // node.id.
        if let Some(bootstrap) = bootstrap {
            rt.update(bootstrap, None)
        }

        rt
    }

    pub fn node_find_bucket_index(&self, key: &Key) -> usize {
        // given a bucket j, we are guaranteed that
        //  2^j <= distance(node, contact) < 2^(j+1)
        // a node with distance d will be put in the k-bucket with index i=⌊logd⌋
        let mut bucket_index = KEY_SIZE * 8 - 1;

        let dst = self.clone().node.id.distance(key);

        for i in 0..KEY_SIZE {
            for j in (0..8).rev() {
                let bit = dst[i] & (0x01 << j);
                //println!("(i: {} ,j: {} , index: {}, dst {:#010b} , bit {} )", i,j,i*8 + 7-j, dst[i], bit.clone() );
                if bit != 0 {
                    debug!("(i: {} ,j: {} , index: {}, dst {:#010b} , bit {} )",
                        i, j, i * 8 + 7 - j, dst[i], bit.clone());

                    bucket_index = i * 8 + 7 - j;
                }
                // else if i == KEY_SIZE -1 && j==0 { return thrust%256 } //distance to self
            }
        }



        bucket_index
    }

    pub fn node_find_bucket_index_thrust(&self, key: &Key) -> usize {
        let bucket_index = self.node_find_bucket_index(key);

        let node : Option<&Node> = self.buckets[bucket_index]
            .nodes.iter().find(
            |nd| nd.id.0 == key.0 );

        if ENABLE_THRUST_MECHANISM {
            if let Some(n) = node{
                //nd = od × b + (1 − b) ×1/t
                return ((bucket_index as f64 )* BALANCE_FACTOR + (1.0 - BALANCE_FACTOR)/key.thrust(self.reputation.index(key))) as usize
            }
        }

       bucket_index

    }

    pub fn get_closest_nodes(&self, key: &Key, capacity: usize) -> Vec<RoutingDistance> {
        let mut closest: Vec<RoutingDistance> = Vec::with_capacity(capacity);

        let mut bucket_index: usize = self.node_find_bucket_index(key);
        let mut bucket_index_reverse = bucket_index;

        //search forward (closests)
        while closest.len() < capacity && bucket_index < self.buckets.len() - 1 {
            for nd in &self.buckets[bucket_index].nodes {
                closest.push(RoutingDistance(nd.clone(), nd.id.distance(&key)))
            }

            bucket_index += 1;
        }

        //search backwards (farthest)
        while closest.len() < capacity && bucket_index_reverse > 0 {
            bucket_index_reverse -= 1;

            for n in &self.buckets[bucket_index_reverse].nodes {
                closest.push(RoutingDistance(n.clone(), n.id.distance(&key)))
            }
        }

        closest.sort_by(|a, b| a.1.cmp(&b.1));

        closest.truncate(capacity);

        closest
    }

    pub fn remove(&mut self, node: &Node) {
        let bucket_idx: usize = self.node_find_bucket_index(&node.id);

        if let Some(i) = self.buckets[bucket_idx]
            .nodes
            .iter()
            .position(|x| x.id == node.id)
        {
            let nod = self.buckets[bucket_idx].nodes.remove(i);

            if self.reputation.contains_key(&node.clone().id){
                let a = self.reputation.index(&node.clone().id).clone().update_interaction(false);
                self.reputation.insert(node.clone().id, a);
            }
            else {
                self.reputation.insert(node.clone().id, ThrustAndReputation::new().update_interaction(false));
            }

        } else {
            if self.reputation.contains_key(&node.clone().id){
                let a = self.reputation.index(&node.clone().id).clone().update_interaction(false);
                self.reputation.insert(node.clone().id, a);
            }
            else {
                self.reputation.insert(node.clone().id, ThrustAndReputation::new().update_interaction(false));
            }

            error!("Tried to remove non-existing entry, {}", node.get_address());
        }

    }

    pub fn update_reputation(&mut self, node : Node, rpc: Option<Arc<RpcSocket>>){

        if !self.reputation.contains_key(&node.clone().id) {
            self.reputation.insert(node.clone().id,ThrustAndReputation::new().update_interaction(true));
        }

        let index = self.node_find_bucket_index(&node.id);

        if self.buckets[index].nodes.len() < self.buckets[index].size {
            let node_idx = self.buckets[index]
                .nodes
                .iter()
                .position(|x| x.id == node.id);

            match node_idx {
                Some(i) => {
                    let no = self.buckets[index].nodes.remove(i);

                    let rpt = self.reputation.index(&no.clone().id);
                    self.reputation.insert(no.clone().id,rpt.clone().update_interaction(true));

                    self.buckets[index].nodes.push(no);
                }
                None => {
                    let rpt = self.reputation.index(&node.clone().id);
                    self.reputation.insert(node.clone().id,rpt.clone().update_interaction(true));

                    self.buckets[index].nodes.push(node.clone());
                }
            }
        } else {
            if rpc.is_some() {
                let nd = self.buckets[index].nodes[0].clone();

                match Client::new(rpc.unwrap())
                    .make_call(Rpc::Ping, nd.clone())
                    .recv()
                    .unwrap()
                {
                    Some(_) => {
                        let add_front = self.buckets[index].nodes.remove(0);

                        let nr = self.reputation.index(&add_front.clone().id);
                        self.reputation.insert(add_front.clone().id,nr.clone().update_interaction(true));

                        self.buckets[index].nodes.push(add_front);
                    }
                    None => {
                        error!("Failed to contact node {:?}", nd.id);
                        self.buckets[index].nodes.remove(0);

                        self.reputation.insert(node.clone().id, ThrustAndReputation::new()
                            .update_interaction(true));

                        self.buckets[index].nodes.push(node.clone());
                    }
                };
            }
        }
    }

    pub fn update(&mut self, node: Node, rpc: Option<Arc<RpcSocket>>) {
        if !self.reputation.contains_key(&node.clone().id) {
            self.reputation.insert(node.clone().id,ThrustAndReputation::new().update_interaction(true));
        }

        let index = self.node_find_bucket_index(&node.id);

        if self.buckets[index].nodes.len() < self.buckets[index].size {
            let node_idx = self.buckets[index].nodes.iter()
                .position(|x| x.id == node.clone().id);

            match node_idx {
                Some(i) => {
                    self.buckets[index].nodes.remove(i);
                    self.buckets[index].nodes.push(node.clone());
                }
                None => {
                    self.buckets[index].nodes.push(node.clone());
                }
            }

            let nr = self.reputation.index(&node.clone().id);
            self.reputation.insert(node.clone().id,nr.clone().update_interaction(true));

        }
        else if rpc.is_some(){
            let nd = self.buckets[index].nodes[0].clone();

            match Client::new(rpc.unwrap()).make_call(Rpc::Ping,nd.clone()).recv() {
                Ok(pong) =>  if let Some(p) = pong {
                    let add_front = self.buckets[index].nodes.remove(0);
                    self.buckets[index].nodes.push(add_front.clone());

                    let nr = self.reputation.index(&add_front.clone().id);
                    self.reputation.insert(add_front.clone().id,nr.clone().update_interaction(true));
                }
                ,
                Err(_) => {
                    error!("Failed to contact node {:?}", nd.id);

                    self.buckets[index].nodes.remove(0);
                    self.buckets[index].nodes.push(node.clone());

                    let nr = self.reputation.index(&node.clone().id);
                    self.reputation.insert(node.clone().id,nr.clone().update_interaction(true));
                }
            };

        }


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
