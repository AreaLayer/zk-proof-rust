// src/circuit.rs

use std::vec;

use crate::Circuit;
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bellman::gadgets::multipack;
use bitcoin::secp256k1::{PublicKey, Scalar, SecretKey};


// Main function pub crate circuit;
pub (crate) fn circuit() -> Circuit<String> {
    // Create a new circuit
    let circuit = Circuit::new();
}
// Main fucntion to create the ZK circuit
pub fn create_circuit(public_key: PublicKey, secret_key: SecretKey, message: [u8; 32]) -> Circuit<String> {
    let circuit = KeyOwnershipCircuit {
        public_key: Some(public_key),
        secret_key: Some(secret_key),
        message: Some(message),
        private_key: None,
    };
/// ZK circuit to prove that the public key corresponds to the private key
pub struct KeyOwnershipCircuit {
    pub public_key: Option<PublicKey>,
    pub secret_key: Option<SecretKey>,
    pub private_key: Option<SecretKey>,
    pub message: Option<[u8; 32]>, // The message to sign (typically the transaction hash)
};

impl<CS: ConstraintSystem<Scalar>, T> Circuit<String> for KeyOwnershipCircuit {
    fn synthesize(self, _cs: &mut CS) -> Result<(), SynthesisError> {
        // Public inputs: public key, message (transaction hash)
        let _pub_key_var = multipack::bytes_to_bits_le(&self.public_key.unwrap().serialize()[..]);
        let _message_var = multipack::bytes_to_bits_le(&self.message.unwrap());
        let _vec: Vec<T> = vec![];

        // Witness inputs: private key
        let _priv_key_var = multipack::bytes_to_bits_le(&self.private_key.unwrap()[..]);
        Ok(())
    }
}
}
#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1::{PublicKey, SecretKey};
    use bitcoin::util::hash::Hash;
    use crate::Circuit;
    use bellman::{Circuit, ConstraintSystem, SynthesisError};

    fn test_key_ownership_circuit() {
        // Create a new circuit
        let circuit = Circuit::new();
        let public_key = PublicKey::from_slice(&[1; 33]).unwrap();
    }
}