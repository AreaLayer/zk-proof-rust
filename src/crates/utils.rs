// src/utils.rs

use crate::Util;
use serde::Serialize;
use rand::Rng;


pub (crate) fn get_util_name() -> String {
    "Utils".to_string()
}
/// Serialize an object to JSON
pub fn serialize_to_json<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(data)
}

/// Generate a random nonce
pub fn generate_nonce() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let nonce: Vec<u8> = (0..32).map(|_| rng.gen()).collect(); // Generate 32 random bytes
    nonce
}
