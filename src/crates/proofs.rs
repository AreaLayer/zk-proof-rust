// src/proofs.rs

use crate::Proofs;
use bellman::groth16::Proof;
use serde::Deserialize;
use serde::Serialize;

pub (crate) fn Proofs() -> Vec<ZKProof> {
    vec![]
}
#[derive(Serialize, Deserialize)]
pub struct ZKProof {
    proof: Proof<Eq::Engine>,
    public_inputs: Vec<u8>, // Modify this structure based on your needs
}

impl ZKProof {
    pub fn new(proof: Proof<Eq::Engine>, public_inputs: Vec<u8>) -> Self {
        ZKProof {
            proof,
            public_inputs,
        }
    }
    pub fn proof(&self) -> &Proof<Eq::Engine> {
        &self.proof
    }

    pub fn public_inputs(&self) -> &Vec<u8> {
        &self.public_inputs
    }
}