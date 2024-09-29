// src/lib.rs

pub mod proof; // Module for zero-knowledge proofspub mod transaction; 
pub mod utils;
pub mod transaction; // Module for Coinjoin transaction logic
pub mod bitcoin;

use spawn_zk_snarks::keygen::KeyPair;
use spawn_zk_snarks::keygen::KeyPair;
use spawn_zk_snarks::proof::{Proof, generate_proof};  // Function name may need to be in snake_case

use crate::CoinjoinTransaction;
use crate::Verify_CoinjoinTransaction;