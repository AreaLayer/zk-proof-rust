use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CoinjoinTransaction {
    inputs: Vec<Input>,  // Define Input struct
    outputs: Vec<Output>, // Define Output struct
    // Add other necessary fields
}
