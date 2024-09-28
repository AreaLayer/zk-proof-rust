use std::process::Output;

#[derive(Debug)]
pub struct CoinjoinTransaction {
    inputs: Vec<bellman::Index>,  // Define Input struct
    outputs: Vec<Output>, // Define Output struct
    // Add other necessary fields
}

