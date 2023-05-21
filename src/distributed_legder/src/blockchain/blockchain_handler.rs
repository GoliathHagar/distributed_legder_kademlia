use std::collections::HashMap;
use std::ops::Index;
use std::sync::{Arc, Mutex};

use log::{debug, error, info};

use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;
use crate::blockchain::consensus::ConsensusAlgorithm;
use crate::blockchain::miner::Miner;
use crate::constants::blockchain_node_type::BlockchainNodeType;
use crate::constants::fixed_sizes::RESPONSE_TIMEOUT;
use crate::constants::multicast_info_type::MulticastInfoType;
use crate::constants::utils::block_to_string;
use crate::dht::kademlia::KademliaDHT;
use crate::network::node::Node;
use crate::network::rpc::Rpc;

pub struct BlockchainHandler {
    pub(self) blockchain: Arc<Blockchain>,
    pub(self) kademlia: Arc<KademliaDHT>,
    pub(self) node_type: BlockchainNodeType,
}

impl BlockchainHandler {
    pub fn new(consensus: ConsensusAlgorithm, node: Node, node_type: BlockchainNodeType, bootstrap: Option<Node>) -> BlockchainHandler {
        let kad = Arc::new(KademliaDHT::new(node, bootstrap.clone()));
        let chain = Arc::new(Blockchain::new(consensus, node_type.clone()));
        Self {
            blockchain: chain,
            kademlia: kad,
            node_type,
        }
    }

    pub fn start(self, dump_path: &str) -> std::thread::JoinHandle<()> {
        let network_thread = self.kademlia.clone().init(Some(dump_path.to_string()));
        let block = self.blockchain.clone().init();

        if let Some(b) = block {
            let data = block_to_string(b);
            self.kademlia.clone().broadcast_info((b.header.hash, MulticastInfoType::Miscellaneous, data))
        }

        self.handel_broadcast_blocks();

        network_thread
    }

    fn handel_broadcast_blocks(self) {
        let kad = self.kademlia.clone();
        let mut blk = self.blockchain.clone();

        std::thread::spawn(move || loop {
            let payload = kad.clone().multicast_subscriber();
            info!("Blockchain received payload {:?}", payload);

            match payload {
                Rpc::Multicasting(id, payload_type, p) => {
                    if payload_type == MulticastInfoType::Block {
                        let mut block: Block = match serde_json::from_str(&p) {
                            Ok(d) => d,
                            Err(e) => {
                                error!("Unable to decode block string payload: {}", e.to_string());
                                debug!("Payload unknown [{}]", p.trim_end());
                                continue;
                            }
                        };

                        if self.node_type == BlockchainNodeType::Bootstrap && blk.clone().is_chain_valid() {
                            blk.clone().add_block(block);
                        } else if self.node_type == BlockchainNodeType::Miner {
                            if block.is_valid() {
                                mining.index(&block.header.hash).;
                            } else {
                                let bch = self.blockchain.clone();
                            }
                        }


                        //todo task on block chain
                    }
                }
                _ => { continue; }
            };
        });
    }

    fn worker_nine(self, block: Block) {
        let bch = self.blockchain.clone();
        let kad = self.kademlia.clone();
        let mine = std::thread::spawn(
            move || {
                let blk = bch.mine_block(block);

                bch.add_block(blk);

                kad.broadcast_info(block.header.hash, MulticastInfoType::Block)
            }
        );
    }
}