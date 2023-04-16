use crate::constants::utils::calculate_sha256;
use crate::network::key::Key;

#[test]
fn test_key_hash_generator() {
    let given_string: String = "test".to_string();

    let expect = calculate_sha256(&given_string);

    assert_eq!(expect, Key::new(given_string).0.to_vec())
}
