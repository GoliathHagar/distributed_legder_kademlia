use std::sync::Arc;

use crate::blockchain::blockchain::Blockchain;
use crate::blockchain::consensus::ConsensusAlgorithm;
use crate::constants::fixed_sizes::RESPONSE_TIMEOUT;
use crate::dht::kademlia::KademliaDHT;
use crate::network::node::Node;

struct BlockchainHandler {
    pub(self) blockchain: Arc<Blockchain>,
    pub(self) kademlia: Arc<KademliaDHT>,
    pub(self) is_bootstrap: bool,
}

impl BlockchainHandler {
    pub fn new(consensus: ConsensusAlgorithm, node: Node, bootstrap: Option<Node>) -> BlockchainHandler {
        let kad = Arc::new(KademliaDHT::new(node, bootstrap.clone()));
        let chain = Arc::new(Blockchain::new(consensus));

        Self {
            blockchain: chain,
            kademlia: kad,
            is_bootstrap: bootstrap.is_some(),
        }
    }

    pub fn start(self, dump_path: &str) -> std::thread::JoinHandle<()> {
        let network_thread = self.kademlia.clone().init(Some(dump_path.to_string()));
        std::thread::sleep(std::time::Duration::from_millis(RESPONSE_TIMEOUT));

        self.handel_broadcast_blocks();

        network_thread
    }

    fn handel_broadcast_blocks(self) {
        let kad = self.kademlia.clone();
        let blk = self.blockchain.clone();

        std::thread::spawn(move || loop {
            let payload = kad.clone().multicast_subscriber();

            blk.is_valid();

            //todo task on block chain

            println!("ok fui dormir {:?}", payload);
        });
    }
}