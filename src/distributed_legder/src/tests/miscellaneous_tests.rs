use std::thread;
use crate::constants::utils::{calculate_sha256, get_timestamp_now};
use crate::network::key::Key;

#[test]
fn test_key_hash_generator() {
    let given_string: String = "test".to_string();

    let expect = calculate_sha256(&given_string);

    assert_eq!(expect, Key::new(given_string).0.to_vec())
}

#[test]
fn test_timestamp_generator() {
    let millis = 1000000;

    let given : u64 = get_timestamp_now();

    thread::sleep(std::time::Duration::from_nanos(millis));

    let expected = get_timestamp_now();

    let res = expected - given;
    println!("given {} expected {} res {}",given, expected, res);

    assert!(res > millis)
}