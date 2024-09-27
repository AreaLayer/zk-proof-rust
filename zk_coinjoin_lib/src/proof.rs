use bellman::{groth16::{create_random_proof, prepare_verifying_key, verify_proof, Proof, PreparedVerifyingKey}, Engine};
use rand::thread_rng;
use std::error::Error;
use serde::Deserialize;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct ZKProof<E: bellman::Engine> {
    proof: Proof<E>,
    public_inputs: Vec<Vec<u8>>, // Adjust based on your needs
}

pub fn generate_proof<E: Engine>(
    circuit: impl bellman::Circuit<E>, 
    pk: &bellman::groth16::ProvingKey<E>
) -> Result<ZKProof<E>, Box<dyn Error>> {
    // Random number generator for proof generation
    let rng = &mut thread_rng();

    // Generate the proof using the bellman `create_random_proof`
    let proof = create_random_proof(circuit, pk, rng)?;

    // Populate public_inputs based on the circuit's public inputs
    // For simplicity, assuming the circuit's public inputs are accessible
    let public_inputs = vec![]; // Replace with actual public input conversion logic

    Ok(ZKProof::<E> {
        proof,
        public_inputs,
    })
}
pub fn verify_zk_proof<E: Engine>(
    zk_proof: &ZKProof<E>,
    vk: &PreparedVerifyingKey<E>
) -> Result<bool, Box<dyn Error>> {
    // Convert public inputs from Vec<Vec<u8>> to the format expected by the `bellman::groth16::verify_proof`
    let public_inputs: Vec<E::Fr> = zk_proof
        .public_inputs
        .iter()
        .map(|input| {
            // Convert Vec<u8> to Fr (the field element type)
            E::Fr::from_repr(E::Fr::Repr::from(input.as_slice()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Verify the proof
    Ok(bellman::groth16::verify_proof(vk, &zk_proof.proof, &public_inputs)?)
}