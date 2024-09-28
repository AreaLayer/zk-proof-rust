// src/utils.rs

use rand::Rng;
use serde::{Serialize, Deserialize};
use serde_json::data::Value;

// Function to generate a random nonce
pub fn generate_nonce() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let nonce: Vec<u8> = (0..32).map(|_| rng.gen()).collect(); // Generate 32 bytes
    nonce
}

// Function to serialize an object to JSON
pub fn serialize_to_json<T: Serialize>(obj: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(obj)
}// Function to deserialize JSON data into a generic type
pub fn deserialize_from_json<T: for<'de> Deserialize<'de>>(json_data: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(json_data)
}// Function to generate a random nonce
// Function to generate a random nonce (renamed to avoid duplicate definition)
pub fn generate_random_nonce() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let nonce: Vec<u8> = (0..32).map(|_| rng.gen()).collect(); // Generate 32 bytes
    nonce
}