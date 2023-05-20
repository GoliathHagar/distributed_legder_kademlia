use crate::blockchain::block::Block;

pub struct Miner{

}
impl Miner {
    fn proof_of_work(self, block: Block) -> u128 {
        let mut nonce = block.header.nonce.clone();

        while !self.valid_proof(block.clone(), nonce) {
            nonce += 1;
        }

        nonce
    }


    fn valid_proof(&self, block: Block, nounce: u128) -> bool {
        let mining_block = block.header.clone();

        block

    }


}