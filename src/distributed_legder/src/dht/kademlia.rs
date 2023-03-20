use crate::network::node::Node;

pub struct KademliaDHT{
    routing_table : String,
}

impl KademliaDHT{
    pub fn new(node: Node) -> KademliaDHT {
        Self{
            routing_table: "rest".to_string()
        }
    }
    pub fn handle_request(&self, req: &String) -> String {

        format!("This is a test request handled for resquest :{}",req)
    }

}

