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

#[cfg(test)]
mod tests {
    use super::*;
    use bellman::groth16::Proof;
    use bellman::groth16::VerifyingKey;
    use bellman::groth16::{prepare_verifying_key, verify_proof};
    use bellman::pairing::bn256::{Bn256, Fr};
    use create::Proofs;

    fn test () {
        let proofs = Proofs();
        let proof = proofs[0].proof();
        let public_inputs = proofs[0].public_inputs();
        let verifying_key = VerifyingKey::<Bn256>::from_bytes(&vec![0; 64]).unwrap();
        let mut prepared_vk = prepare_verifying_key(&verifying_key);
        let mut rng = rand::thread_rng();
        let input = vec![Fr::random(&mut rng); 10];
        let proof = Proof::<Bn256>::from_bytes(&vec![0; 128]).unwrap();
    }
}