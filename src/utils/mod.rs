use bitcoin::{consensus::encode::deserialize, hashes::hex::FromHex, Transaction, TxIn, TxOut};

#[derive(Debug)]
pub enum Error {
    InvalidInput,
    InvalidOutput,
    ParseError(String),
}

pub fn validate_inputs(inputs: &[TxIn]) -> Result<(), Error> {
    for input in inputs {
        if input.witness.is_empty() {
            // Check if witness is empty. For SegWit inputs, this would indicate an invalid input.
            return Err(Error::InvalidInput);
        }
    }
    Ok(())
}

pub fn validate_outputs(outputs: &[TxOut]) -> Result<(), Error> {
    for output in outputs {
        if output.script_pubkey.is_empty() {
            // Check if the output's script_pubkey is empty, indicating an invalid output.
            return Err(Error::InvalidOutput);
        }
    }
    Ok(())
}

pub fn validate_transaction(tx: &Transaction) -> Result<(), Error> {
    validate_inputs(&tx.input)?;
    validate_outputs(&tx.output)?;
    Ok(())
}

pub fn validate_transaction_hex(tx_hex: &str) -> Result<(), Error> {
    let tx_bytes = Vec::from_hex(tx_hex)
        .map_err(|e| Error::ParseError(format!("Hex decode error: {}", e)))?;
    let tx: Transaction = deserialize(&tx_bytes)
        .map_err(|e| Error::ParseError(format!("Transaction deserialize error: {}", e)))?;
    validate_transaction(&tx)
}

pub fn validate_transaction_bytes(tx_bytes: &[u8]) -> Result<(), Error> {
    let tx: Transaction = deserialize(tx_bytes)
        .map_err(|e| Error::ParseError(format!("Transaction deserialize error: {}", e)))?;
    validate_transaction(&tx)
}
