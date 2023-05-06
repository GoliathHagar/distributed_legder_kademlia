use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;

/*#[test]
fn test_calculate_hash() {
    let block = Block {
        index: 1,
        timestamp: 123456,
        data: "test data".to_string(),
        previous_hash: "previous hash".to_string(),
        hash: "".to_string(),
    };
    let expected_hash = "d95922c8f049ae1c2d05a7e3cbfa3819b7e9dbaae601a27d1e2a1a7f934dbb08".to_string();
    let hash = calculate_hash(&block);
    assert_eq!(hash, expected_hash);
}
*/
#[test]
fn test_add_block() {
    let mut blockchain = Blockchain::new();
    let data = "test data".to_string();
    blockchain.add_block(data.clone());
    assert_eq!(blockchain.blocks.len(), 2);
    let block = blockchain.blocks.last().unwrap();
    assert_eq!(block.index, 1);
    assert_eq!(block.data, data);
    assert_eq!(block.previous_hash, blockchain.blocks.first().unwrap().hash);
}

#[test]
fn test_is_valid() {
    let mut blockchain = Blockchain::new();
    assert!(blockchain.is_valid());
    let data = "test data".to_string();
    blockchain.add_block(data.clone());
    assert!(blockchain.is_valid());
    let mut block = blockchain.blocks.last_mut().unwrap();
    block.data = "modified data".to_string();
    assert!(!blockchain.is_valid());
    block.data = data.clone();
    assert!(blockchain.is_valid());
    block.previous_hash = "invalid previous hash".to_string();
    assert!(!blockchain.is_valid());
}