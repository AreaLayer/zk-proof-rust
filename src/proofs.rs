// src/proofs.rs

use bellman::groth16::create_random_proof;
use bellman::groth16::Proof;
use bitcoin::secp256k1::VerifyOnlyPreallocated;
use serde::Deserialize;
use serde::Serialize;
use rand::rngs::OsRng;

#[derive(Serialize, Deserialize)]
pub struct ZKProof {
    proof: Proof<Eq::Engine>,
    public_inputs: Vec<u8>, // Modify this structure based on your needs
}

/// Generate a ZK proof 
pub fn generate_proof(circuit: ExampleCircuit, vk: &VerifyingKey<Bls12>) -> Result<ZKProof, String> {
    let mut rng = OsRng;  // Use OsRng for secure randomness

    // Create random proof
    let proof = create_random_proof(circuit, vk, &mut rng)
        .map_err(|e| format!("Error generating proof: {}", e))?;

    // Define public inputs (replace with your actual public inputs)
    let public_inputs: Vec<Fr> = vec![];

    Ok(ZKProof {
        proof,
        public_inputs,
    });
}
/// Verify a ZK proof
pub fn verify_proof_fn(proof: &ZKProof, vk: &VerifyingKey<Bls12>) -> Result<bool, String> {
    // Verify the proof using the verify_proof function (2 arguments)
    let is_valid = verify_proof(vk, &proof.proof)
        .map_err(|e| format!("Error verifying proof: {}", e))?;

    Ok(is_valid)
}