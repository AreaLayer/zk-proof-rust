/// An error type for handling proof-related issues.
#[derive(Debug)]
pub struct ProofError {
    message: String,
}

impl ProofError {
    /// Creates a new `ProofError` with a custom message.
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

impl std::fmt::Display for ProofError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ProofError {}

/// A custom proof struct for demonstration purposes.
pub struct MyProof {
    a: Vec<u8>,
    b: Vec<u8>,
}

impl MyProof {
    /// Constructor for `MyProof`.
    pub fn new(a: Vec<u8>, b: Vec<u8>) -> Self {
        Self { a, b }
    }
}

/// A trait defining the behavior required for a proof.
pub trait MyProofTrait {
    fn verify(&self) -> Result<(), ProofError>;
    fn serialize(&self) -> Result<Vec<u8>, ProofError>;
    fn deserialize(data: &[u8]) -> Result<Self, ProofError>
    where
        Self: Sized;
}

impl MyProofTrait for MyProof {
    fn verify(&self) -> Result<(), ProofError> {
        // Verification logic (example)
        println!("Verifying MyProof...");
        Ok(())
    }

    fn serialize(&self) -> Result<Vec<u8>, ProofError> {
        // Example serialization logic
        let mut serialized = Vec::new();
        serialized.extend_from_slice(&self.a);
        serialized.extend_from_slice(&self.b);
        Ok(serialized)
    }

    fn deserialize(data: &[u8]) -> Result<Self, ProofError> {
        // Example deserialization logic
        if data.len() < 2 {
            return Err(ProofError::new("Data too short to deserialize"));
        }
        let mid = data.len() / 2;
        let (a, b) = data.split_at(mid);
        Ok(Self {
            a: a.to_vec(),
            b: b.to_vec(),
        })
    }
}
