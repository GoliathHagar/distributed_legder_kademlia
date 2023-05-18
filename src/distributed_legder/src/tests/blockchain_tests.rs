use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;
//use crate::blockchain::blockchain::calculate_hash;

/*
#[test]
fn  test_new_blockchain() {
    let blockchain = Blockchain::new();
    assert_eq!(blockchain.blocks.len(), 1);
    assert_eq!(blockchain.blocks[0].index, 0);
    assert_eq!(blockchain.blocks[0].data, "Genesis block".to_string());
    assert_eq!(blockchain.blocks[0].previous_hash, "0".to_string());
    assert_ne!(blockchain.blocks[0].hash, "".to_string());
}

#[test]
fn test_add_block() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Hello, world!".to_string());
    assert_eq!(blockchain.blocks.len(), 2);
    assert_eq!(blockchain.blocks[1].index, 1);
    assert_eq!(blockchain.blocks[1].data, "Hello, world!".to_string());
    assert_eq!(blockchain.blocks[1].previous_hash, blockchain.blocks[0].hash);
    assert_ne!(blockchain.blocks[1].hash, "".to_string());
}

#[test]
fn test_is_valid() {
    let mut blockchain = Blockchain::new();
    assert!(blockchain.is_valid());

    blockchain.add_block("Hello, world!".to_string());
    assert!(blockchain.is_valid());

    // attempt to tamper with the blockchain
    blockchain.blocks[1].data = "Hacked!".to_string();
    assert!(!blockchain.is_valid());

    blockchain.blocks[1].data = "Hello, world!".to_string();
    blockchain.blocks[1].previous_hash = "0000000000000000000000000000000000000000000000000000000000000000".to_string();
    assert!(!blockchain.is_valid());
}

#[test]
fn test_calculate_hash() {
    let block = block {
        index: 0,
        timestamp: 0,
        nonce: 0,
        data: "genesis block".to_string(),
        transactions: vec![],
        previous_hash: "0".to_string(),
        hash: "".to_string(),
        proof: 0,
    };
    let expected_hash = "7e240de74fb1ed08fa08d38063f6a6a91462a815".to_string();
    let actual_hash = calculate_hash(&block);
    assert_eq!(actual_hash, expected_hash);
}
*/