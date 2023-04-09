use std::cmp::Ordering;
use std::sync::{Arc};
use log::{debug, error};
use crate::network::node::Node;
use crate::constants::fixed_sizes::{ENABLE_SECURITY, K_BUCKET_SIZE, KEY_SIZE, N_BUCKETS};
use crate::dht::rpc::Rpc;
use crate::network::client::Client;
use crate::network::key::Key;
use crate::network::rpc_socket::RpcSocket;

#[derive(Debug)]
pub struct Bucket{
    pub nodes : Vec<Node>,
    pub size: usize
}

#[derive(Debug)]
pub struct RoutingTable{
    pub node: Node,
    pub buckets: Vec<Bucket>,
}

#[derive(Debug, Eq, Hash, Clone)]
pub struct RoutingDistance( pub Node, pub [u8; KEY_SIZE]);
/*

A k-bucket with index i stores contacts whose ids
have a distance between 2^i and 2^i+1 to the own id`

*/
impl Bucket {
    pub fn new() -> Bucket {
        Self{
            nodes: Vec::new(),
            size:   K_BUCKET_SIZE
        }
    }

}

impl RoutingTable{
    pub  fn new(node: Node, bootstrap: Option<Node>) -> RoutingTable {
        let mut buckets: Vec<Bucket> = Vec::new();

        for _ in  0..N_BUCKETS {
            buckets.push(Bucket::new())
        }

        let mut rt = Self{
            buckets,
            node
        };

        // node.id.
        if let Some(bootstrap) = bootstrap{ rt.update(bootstrap, None)}

        rt
    }

    pub fn node_find_bucket_index(&self, key: &Key) -> usize {
        // given a bucket j, we are guaranteed that
        //  2^j <= distance(node, contact) < 2^(j+1)
        // a node with distance d will be put in the k-bucket with index i=⌊logd⌋

        let dst = self.node.id.clone().distance(key);
        let thrust : usize = if ENABLE_SECURITY { key.thrust()} else { 0 };

        for i in 0..KEY_SIZE {
            for j in (0..8).rev(){
                let bit = dst[i] & (0x01 << j);
                //println!("(i: {} ,j: {} , index: {}, dst {:#010b} , bit {} )", i,j,i*8 + 7-j, dst[i], bit.clone() );
                if bit != 0 {
                    debug!("(i: {} ,j: {} , index: {}, dst {:#010b} , bit {} )", i,j,i*8 + 7-j, dst[i], bit.clone() );
                    return  ((i*8 + 7- j) + thrust)%256;
                }
                // else if i == KEY_SIZE -1 && j==0 { return thrust%256 } //distance to self
            }
        }

        ((KEY_SIZE*8 -1) + thrust)%256
    }

    pub fn get_closest_nodes(&self, key: &Key, capacity : usize) -> Vec<RoutingDistance>{

        let mut closests : Vec<RoutingDistance> = Vec::with_capacity(capacity);

        let mut bucket_index : usize = self.node_find_bucket_index(key);
        let mut bucket_index_reverse = if bucket_index  > 0 {bucket_index -1} else { 0 };

        //search forward (closests)
        while closests.len() < capacity && bucket_index < self.buckets.len() -1 {
            for nd in &self.buckets[bucket_index].nodes {
                closests.push(RoutingDistance(nd.clone(), nd.id.distance(&key)))
            }

            bucket_index += 1;

        }

        //search backwards (farthest)
        while closests.len() < capacity &&  bucket_index_reverse >= 0 {
            for nd in &self.buckets[bucket_index_reverse].nodes {
                closests.push(RoutingDistance(nd.clone(), nd.id.distance(&key) ))
            }

            bucket_index_reverse -= 1;
        }

        closests.sort_by(
            |a, b| a.1.cmp(&b.1)
        );

        closests.truncate(capacity);

        closests

    }

    pub fn remove(&mut self, node: &Node) {
        let bucket_idx : usize = self.node_find_bucket_index(&node.id);

        if let Some(i) = self.buckets[bucket_idx]
            .nodes
            .iter()
            .position(|x| x.id == node.id)
        {
            self.buckets[bucket_idx].nodes.remove(i);
        } else {
            error!("Tried to remove non-existing entry");
        }
    }

    pub fn update(&mut self, node: Node,  rpc: Option<Arc<RpcSocket> >){
        let index = self.node_find_bucket_index(&node.id);

        if self.buckets[index].nodes.len() < self.buckets[index].size {
            let node_idx = self.buckets[index].nodes.iter()
                .position(|x| x.id == node.id);

            match node_idx {
                Some(i) => {
                    self.buckets[index].nodes.remove(i);
                    self.buckets[index].nodes.push(node);
                }
                None => {
                    self.buckets[index].nodes.push(node);
                }
            }
        }
        else {
            if rpc.is_some() {
                let nd = self.buckets[index].nodes[0].clone();

                match Client::new(rpc.unwrap()).make_call(Rpc::Ping, nd.clone()).recv().unwrap() {
                    Some(_) =>  {
                        let add_front = self.buckets[index].nodes.remove(0);
                        self.buckets[index].nodes.push(add_front);
                    },
                    None => {
                        error!("Failed to contact node {:?}", nd.id);

                        self.buckets[index].nodes.remove(0);
                        self.buckets[index].nodes.push(node);
                    }
                };
            }
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