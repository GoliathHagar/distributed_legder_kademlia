// BYTES 8 BITS
pub const BYTE: usize = 8;

// 256 bits --> 32 bytes
// 160 bits --> 20 bytes (require alter hasher in utils.rs and key.rs)
pub const KEY_SIZE: usize = 32;

// number entries in a list
// 256 bits --> 32 bytes
// 160 bits --> 20 bytes (require alter hasher in utils.rs and key.rs)
pub const K_BUCKET_SIZE: usize = 32;

pub const ALPHA: usize = 3;

// a list for each bit of the node ID
// 32 bytes * 8 --> 256 bit
pub const N_BUCKETS: usize = KEY_SIZE * BYTE;

// buffer size used for streaming UDP
pub const UDP_STREAMING_BUFFER_SIZE: usize = 8192;

// response timeout 5000ms
pub const RESPONSE_TIMEOUT: u64 = 8000;
pub const BOOTSTRAP_TIMEOUT: u64 = 3600000;

// Republish store values timeout  = 60s
pub const REPUBLISH_TIMEOUT: u64 = 60 * 60000;

// state save disk timeout 10000ms
pub const DUMP_STATE_TIMEOUT: u64 = 6000;

//Trust and reputation
pub const ENABLE_THRUST_MECHANISM: bool = false;
pub const BALANCE_FACTOR: f64 = 0.65;
pub const WEIGHT_REPUTATION : f64 = 0.2;
pub const WEIGHT_RISK : f64 = 1.0- WEIGHT_REPUTATION as f64;

//Blockchain
pub const BLOCKCHAIN_VERSION : u8 = 1;
pub const BLOCKCHAIN_MINING_DIFFICULTY : usize = 10;
pub const  ZEROS_HASH : &str = "0000000000000000000000000000000000000000000000000000000000000000";