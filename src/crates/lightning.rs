//! lightning_support: A crate for integrating Lightning Network in Rust

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightningData {
    pub invoice: String,          // Lightning Network invoice for payments
    pub preimage_hash: Vec<u8>,   // Hash of the preimage for HTLC resolution
    pub htlc_signature: Vec<u8>,  // HTLC signature ensuring secure payment routing
}

impl LightningData {
    pub fn new(invoice: String, preimage_hash: Vec<u8>, htlc_signature: Vec<u8>) -> Self {
        Self {
            invoice,
            preimage_hash,
            htlc_signature,
        }
    }

    pub fn validate_invoice(&self) -> bool {
        // Simple validation: invoice string is not empty
        !self.invoice.is_empty()
    }
}

// Example function for generating a Lightning invoice
pub fn generate_invoice(amount_sats: u64, description: &str) -> LightningData {
    let invoice = format!("lnbc{}n1...", amount_sats);
    let preimage_hash = vec![0u8; 32]; // Dummy preimage hash
    let htlc_signature = vec![0u8; 64]; // Dummy signature

    LightningData::new(invoice, preimage_hash, htlc_signature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lightning_data() {
        let lightning_data = LightningData::new(
            "lnbc2500n1...".to_string(),
            vec![0u8; 32],
            vec![0u8; 64],
        );
        assert!(lightning_data.validate_invoice());
    }

    #[test]
    fn test_generate_invoice() {
        let invoice = generate_invoice(2500, "Test invoice");
        assert!(!invoice.invoice.is_empty());
    }
}
