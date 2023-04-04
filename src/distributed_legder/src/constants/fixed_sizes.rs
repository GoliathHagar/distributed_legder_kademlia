// BYTES 8 BITS
const  BYTE :usize = 8;

// 256 bits --> 32 bytes
pub const KEY_SIZE: usize = 32;

// number entries in a list
pub const K_BUCKET_SIZE: usize = 20;

pub const ALPHA: usize = 3;

// a list for each bit of the node ID
// 32 bytes * 8 --> 256 bit
pub  const N_BUCKETS: usize = KEY_SIZE * BYTE;

// buffer size used for streaming UDP
pub const UDP_STREAMING_BUFFER_SIZE :usize = 8192 ;

// response timeout 5000ms
pub const RESPONSE_TIMEOUT: u64 = 8000;

//enable skademlia thrust security mecanism
pub const ENABLE_SECURITY: bool = false;