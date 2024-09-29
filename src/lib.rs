// src/lib.rs

pub mod proofs;
pub mod transaction;
pub mod utils;

use bitcoin::{secp256k1::{Secp256k1, SecretKey}, util::address::Address, Network, PrivateKey, PublicKey};
use reqwest::blocking::Client;
use transaction::{create_p2wpkh_script, create_p2tr_script};
use proofs::{generate_proof, verify_proof, ZKProof};
use utils::{serialize_to_json, generate_nonce};

/// Create and broadcast a Coinjoin transaction using Taproot or SegWit with ZK Proofs
pub fn create_and_broadcast_transaction(
    client: &Client,
    is_taproot: bool,
    private_key: &PrivateKey,
    recipient_address: &str,
    amount: u64,
    proof_data: &ZKProof,
) -> Result<String, String> {
    let secp = Secp256k1::new();

    // Derive public key from private key
    let pub_key = PublicKey::from_private_key(&secp, private_key);

    // Generate address (Taproot or SegWit based on flag)
    let address = if is_taproot {
        Address::p2tr(&secp, pub_key, None, Network::Bitcoin)
    } else {
        Address::p2wpkh(&pub_key, Network::Bitcoin)
    };

    // Create the transaction
    let transaction = transaction::create_coinjoin_transaction(
        &address, recipient_address, amount, proof_data
    );

    // Serialize transaction to JSON
    let serialized_tx = serialize_to_json(&transaction)
        .map_err(|e| format!("Serialization error: {}", e))?;

    // Send the transaction using Reqwest client
    let response = client
        .post("https://your-bitcoin-node.com/sendrawtransaction")
        .body(serialized_tx)
        .send()
        .map_err(|e| format!("HTTP request error: {}", e))?;

    // Return transaction ID or error
    response.text().map_err(|e| format!("Response error: {}", e))
}
