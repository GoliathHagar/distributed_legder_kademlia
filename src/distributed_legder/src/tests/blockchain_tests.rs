use crate::blockchain::block::Block;
use crate::blockchain::consensus::ConsensusAlgorithm;
use crate::blockchain::miner::Miner;
use crate::blockchain::transaction::Transaction;
use crate::constants::fixed_sizes::ZEROS_HASH;
use crate::constants::utils::calculate_block_hash;

//use crate::blockchain::blockchain::calculate_hash;

#[test]
fn test_mining_pow_block_and_validate() {
    // Create a sample block
    let mut block = Block::new(
        0,
        ZEROS_HASH.to_string(),
        "".to_string(),
        Vec::from([
            Transaction::new("sad".to_string(), "dpr".to_string(), 100000.0),
            Transaction::new("sad2".to_string(), "dpr2".to_string(), 100000.0)]),
    );
    println!("initial {:?}", block);

    block.header.timestamp=0;

    let miner = Miner::new(ConsensusAlgorithm::ProofOfWork);

    let nonce = miner.mine_block(block.clone());
    // Calculate the block's hash and convert it to a hexadecimal string

    block.header.nonce = nonce;

    let string_hash = calculate_block_hash(&block);
    let binary_hash = hex::decode(string_hash.clone()).unwrap().iter()
        .map(|byte| format!("{:08b}-", byte))
        .collect::<String>();

    // Verify the block's validity
    let is_valid = block.is_valid();

    println!("Last {:?}", block);

    println!("Nonce {} Hash {} bin {}", nonce, string_hash, binary_hash);
    // Assert that the block is valid
    assert_eq!(is_valid, true)
}
