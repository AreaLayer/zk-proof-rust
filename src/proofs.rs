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
