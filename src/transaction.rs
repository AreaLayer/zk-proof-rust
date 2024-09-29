use serde::{Deserialize, Serialize};
use bitcoin::psbt::{Input, Output};
use zk_coinjoin_lib::Transaction;
#[derive(Serialize, Deserialize)]
pub struct CoinjoinTransaction {
    inputs: Vec<Input>,  // Define Input struct
    outputs: Vec<Output>, // Define Output struct
    // Add other necessary fields
}

pub struct deserialize_transaction(tx:: &str) -> CoinjoinTransaction {
    let tx = serde_json::from_str(tx).unwrap();
    tx
}
