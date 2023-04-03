use std::fmt::{Binary, Debug, Error, Formatter};
use std::iter::{Map, Zip};
use std::slice::Iter;
use log::error;
use serde::{Deserialize, Serialize};
use crate::constants::fixed_sizes::KEY_SIZE;
use crate::constants::utils::calculate_sha256;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Key(
    #[serde(with = "hex_serde")]
    pub [u8; KEY_SIZE]
);

impl Key {
    pub fn new(input: String) -> Self {
        // we know that the hash output is going to be 256 bits = 32 bytes
        let result = calculate_sha256(&input);
        let mut hash = [0; KEY_SIZE];

        for i in 0..result.len() {
            hash[i] = result[i];
        }

        Self(hash)
    }

    pub fn distance(&self, key: &Key) -> [u8; KEY_SIZE]{
        let mut d = [0; KEY_SIZE];
        for i in 0..KEY_SIZE {
            d[i] = &self.0[i] ^ key.0[i];
        }

        d
    }

}

//to pretty print as toString()
impl Debug for Key {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for x in &self.0 {
            if let Err(_) = write!(f, "{:X}", x){
                error!("Failed to format contents of Key")
            }
        }
        Ok(())
    }
}

impl Binary for Key {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for x in &self.0 {
            if let Err(_) =  write!(f, "{:b}", x){
                error!("Failed to format contents of Key to Binary")
            }
        }
        Ok(())
    }
}