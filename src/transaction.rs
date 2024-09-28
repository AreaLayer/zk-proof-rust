use std::process::Output;

use serde::Serialize;

#[derive(Serialize)]
pub struct CoinjoinTransaction {
    inputs: Vec<bellman::Index>,  // Define Input struct
    outputs: Vec<Output>, // Define Output struct
    // Add other necessary fields
}

