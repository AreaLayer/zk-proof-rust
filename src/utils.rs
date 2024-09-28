// src/utils.rs

use rand::Rng;
use serde::{Serialize, Deserialize};
use  serde_json::data::Value;

// Function to generate a random nonce
pub fn generate_nonce() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let nonce: Vec<u8> = (0..32).map(|_| rng.gen()).collect(); // Generate 32 bytes
    nonce
}

// Function to serialize an object to JSON
pub fn serialize_to_json<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(data)
}

// Function to deserialize JSON to an object
pub fn deserialize_from_json<T: for<'de> Deserialize<'de>>(data: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(data)
}
