// src/lib.rs

pub mod proof; // Module for zero-knowledge proofspub mod transaction; 
pub mod utils;
pub mod transaction; // Module for Coinjoin transaction logic
pub mod bitcoin;


use std::f64::consts::E;

use utils::generate_nonce;

use crate::proof::{generate_proof, ZKProof};

// Public function to create a Coinjoin transaction with ZK proofs
pub fn create_coinjoin_transaction(/* parameters */) -> Result<(transaction::CoinjoinTransaction, ZKProof<E>), String> {
    // Create the Coinjoin transaction
    let transaction = transaction::CoinjoinTransaction::new(/* parameters */);

    // Generate the ZK proof
    let nonce: Vec<u8> = generate_nonce();
    let proof: Result<ZKProof<_>, Box<dyn std::error::Error>> = generate_proof(&transaction, &nonce);
    
    Ok((transaction, proof?))
}// Public function to verify a Coinjoin transaction's proof
use bellman::groth16::prepare_verifying_key;
pub fn verify_coinjoin_transaction(transaction: &transaction::CoinjoinTransaction, proof: &ZKProof<E>) -> bool {
    // Prepare the verifying key
    let vk = prepare_verifying_key(&proof.get_vk());
    // Verify the proof
    let result = vk.verify(&proof.get_inputs(), &proof.get_proof());
    result
}
