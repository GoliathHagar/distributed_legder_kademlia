use crate::blockchain::block::Block;
use crate::blockchain::consensus::ConsensusAlgorithm;

pub struct Miner{
    consensus: ConsensusAlgorithm
}
impl Miner {
    pub fn new(consensus: ConsensusAlgorithm) -> Miner {
        Self {
            consensus
        }
    }

    pub fn mine_block(self, block: Block) -> u128 {
        return match self.consensus {
            ConsensusAlgorithm::ProofOfWork => {
                self.proof_of_work(block)
            }
            ConsensusAlgorithm::DelegatedProofOfStake => {
                self.proof_of_stake(block)
            }
        }
    }

    fn proof_of_work(self, block: Block) -> u128 {
        let mut nonce = block.header.nonce.clone();

        while !self.valid_proof(block.clone(), nonce) {
            nonce += 1;
        }

        nonce
    }

    fn proof_of_stake(self, block: Block) -> u128 {
        let mut nonce = block.header.nonce.clone();

        nonce
    }


    fn valid_proof(&self, block: Block, nonce: u128) -> bool {
        let mut mining_block = block.clone();

        mining_block.header.nonce = nonce;

        mining_block.is_valid()
    }


}