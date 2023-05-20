use crate::blockchain::block::Block;

pub struct Miner{

}
impl Miner {
    pub fn proof_of_work(self, block: Block) -> u128 {
        let mut nonce = block.header.nonce.clone();

        while !self.valid_proof(block.clone(), nonce) {
            nonce += 1;
        }

        nonce
    }


    fn valid_proof(&self, block: Block, nonce: u128) -> bool {
        let mut mining_block = block.clone();

        mining_block.header.nonce = nonce;

        mining_block.is_valid()
    }


}