// src/lib.rs

pub mod proof; // Module for zero-knowledge proofspub mod transaction; 
pub mod utils;
pub mod transaction; // Module for Coinjoin transaction logic


use utils::generate_nonce;

use crate::proof::{generate_proof, verify_proof, ZKProof};

// Public function to create a Coinjoin transaction with ZK proofs
pub fn create_coinjoin_transaction(/* parameters */) -> Result<(transaction::CoinjoinTransaction, ZKProof), String> {
    // Create the Coinjoin transaction
    let transaction = transaction::CoinjoinTransaction::new(/* parameters */);

    // Generate the ZK proof
    let proof = generate_nonce();
    let proof = generate_proof();
    
    Ok((transaction, proof))
}

// Public function to verify a Coinjoin transaction's proof
use bellman::groth16::prepare_verifying_key;
pub fn verify_coinjoin_transaction(transaction: &transaction::CoinjoinTransaction, proof: &ZKProof) -> bool {
    // Prepare the verifying key
    let vk = prepare_verifying_key(&proof.vk);
    // Verify the proof
    let result = vk.verify(&proof.inputs, &proof.outputs);
    result
}

