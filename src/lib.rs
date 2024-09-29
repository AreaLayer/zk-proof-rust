// src/lib.rs

pub mod proof; // Module for zero-knowledge proofspub mod transaction; 
pub mod utils;
pub mod transaction; // Module for Coinjoin transaction logic
pub mod bitcoin;

use spawn_zk_snarks::keygen::KeyPair;
use spawn_zk_snarks::proof::Proof;
use spawn_zk_snarks::proof::Generate_proof;
use spawn_zk_snarks::keygen::KeyPair;

use crate::CoinjoinTransaction;
use crate::Verify_CoinjoinTransaction;