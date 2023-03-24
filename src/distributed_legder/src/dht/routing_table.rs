use crate::network::node::Node;
use crate::constants::fixed_sizes::{K_BUCKET_SIZE, KEY_SIZE, N_BUCKETS};

#[derive(Debug)]
pub struct Bucket{
    pub nodes : Vec<Node>,
    pub size: usize
}

#[derive(Debug)]
pub struct RoutingTable{
    pub node: Node,
    pub buckets: Vec<Bucket>
}

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

        let rt = Self{
            node,
            buckets
        };

        rt
    }
}


