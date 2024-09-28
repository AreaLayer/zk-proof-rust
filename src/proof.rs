use bellman::groth16::{create_random_proof, PreparedVerifyingKey};
use rand::thread_rng;
use std::error::Error;
use pairing::Engine;
use pairing::group::ff::PrimeField; // Correct import

#[derive(Debug)]
pub struct ZKProof<E: Engine> {
    proof: bellman::groth16::Proof<E>,
    public_inputs: Vec<Vec<u8>>, // Adjust based on your needs
}

pub fn generate_proof<E: Engine>(
    circuit: impl bellman::Circuit<E>,
    pk: &bellman::groth16::Parameters<E>,
) -> Result<ZKProof<E>, Box<dyn Error>>
where
    E::Fr: PrimeField + PrimeFieldBits, // Add PrimeFieldBits trait bound
{
    // Random number generator for proof generation
    let rng = &mut thread_rng();

    // Generate the proof using the Bellman `create_random_proof`
    let proof = create_random_proof(circuit, pk, rng)?;

    // Populate public_inputs based on the circuit's public inputs
    // For simplicity, assuming the circuit's public inputs are accessible
    let public_inputs = vec![]; // Replace with actual public input conversion logic

    Ok(ZKProof::<E> {
        proof,
        public_inputs,
    })
pub fn verify_zk_proof<E: Engine + pairing::MultiMillerLoop>(
    zk_proof: &ZKProof<E>,
    vk: &PreparedVerifyingKey<E>,
) -> Result<bool, Box<dyn Error>> 
where
    E::Fr: PrimeField, // Ensure field element trait is correctly bounded
{
    // Convert public inputs from Vec<Vec<u8>> to the format expected by `bellman::groth16::verify_proof`
    let public_inputs: Vec<E::Fr> = zk_proof
        .public_inputs
        .iter()
        .map(|input| {
            // Convert Vec<u8> to E::Fr::Repr, then to E::Fr
            let mut repr = <<E as Engine>::Fr as PrimeField>::Repr::default();
            repr.as_mut().copy_from_slice(input); // Assuming input length matches Repr size
            E::Fr::from_repr(repr).map_err(|_| "Invalid field element representation")
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Verify the proof
    Ok(bellman::groth16::verify_proof(vk, &zk_proof.proof, &public_inputs)?)
}