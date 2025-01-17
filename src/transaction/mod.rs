/// An error type for handling transaction-related issues.
#[derive(Debug)]
pub struct TransactionError {
    message: String,
}

impl TransactionError {
    /// Creates a new `TransactionError` with a custom message.
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TransactionError {}

/// A struct representing a transaction input.
#[derive(Debug, Clone)]
pub struct Input {
    pub previous_tx: Vec<u8>, // Reference to a previous transaction
    pub index: usize,         // Output index in the referenced transaction
}

/// A struct representing a transaction output.
#[derive(Debug, Clone)]
pub struct Output {
    pub recipient: Vec<u8>, // Address or public key of the recipient
    pub amount: u64,        // Amount of currency in the output
}

/// A struct representing a transaction.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub inputs: Vec<Input>,   // List of inputs
    pub outputs: Vec<Output>, // List of outputs
}

impl Transaction {
    /// Creates a new transaction.
    pub fn new(inputs: Vec<Input>, outputs: Vec<Output>) -> Self {
        Self { inputs, outputs }
    }

    /// Verifies that the transaction is valid.
    pub fn verify(&self) -> Result<(), TransactionError> {
        // Example verification logic
        if self.inputs.is_empty() {
            return Err(TransactionError::new("Transaction must have at least one input"));
        }
        if self.outputs.is_empty() {
            return Err(TransactionError::new("Transaction must have at least one output"));
        }
        if self.outputs.iter().any(|output| output.amount == 0) {
            return Err(TransactionError::new("Output amounts must be greater than zero"));
        }
        println!("Transaction is valid!");
        Ok(())
    }

    /// Serializes the transaction into a vector of bytes.
    pub fn serialize(&self) -> Result<Vec<u8>, TransactionError> {
        let mut serialized = Vec::new();
        for input in &self.inputs {
            serialized.extend_from_slice(&input.previous_tx);
            serialized.extend_from_slice(&input.index.to_le_bytes());
        }
        for output in &self.outputs {
            serialized.extend_from_slice(&output.recipient);
            serialized.extend_from_slice(&output.amount.to_le_bytes());
        }
        Ok(serialized)
    }

    /// Deserializes a transaction from a vector of bytes.
    pub fn deserialize(data: &[u8]) -> Result<Self, TransactionError> {
        let mut offset = 0;

        // Example logic to reconstruct the transaction (input/output sizes are fixed for simplicity)
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        // Deserialize inputs
        while offset + 36 <= data.len() {
            let previous_tx = data[offset..offset + 32].to_vec();
            offset += 32;
            let index = u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]);
            offset += 4;
            inputs.push(Input { previous_tx, index: index.try_into().unwrap() });
        }

        // Deserialize outputs
        while offset + 40 <= data.len() {
            let recipient = data[offset..offset + 32].to_vec();
            offset += 32;
            let amount = u64::from_le_bytes([
                data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
                data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7],
            ]);
            offset += 8;
            outputs.push(Output { recipient, amount });
        }

        if offset != data.len() {
            return Err(TransactionError::new("Invalid data length"));
        }

        Ok(Self { inputs, outputs })
    }
}