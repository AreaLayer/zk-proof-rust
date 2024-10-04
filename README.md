# ZK Proof Rust 🌀

ZK Proof Library for Coinjoin transactions

## About

The **ZK Proof Coinjoin Library** is a Rust-based implementation designed to facilitate privacy-preserving Coinjoin transactions. Utilizing advanced cryptographic techniques, including zero-knowledge proofs (zk-SNARKs), this library aims to enhance the privacy and security of Bitcoin transactions by enabling multiple users to combine their coins into a single transaction without revealing their individual inputs.

### Key Features

- **Zero-Knowledge Proofs**: Implement zk-SNARKs to ensure that transaction details remain confidential while still allowing verification of the transaction's validity.
- **Peer-to-Peer Communication**: Facilitate secure communication between participants in a Coinjoin transaction, ensuring that user identities and transaction details are protected.
- **SegWit and Taproot Compatibility**: Support modern Bitcoin features like Segregated Witness (SegWit) and Taproot, enhancing flexibility and transaction efficiency.

### Use Cases

This library is suitable for developers looking to implement Coinjoin functionality in their Bitcoin applications, enabling users to enhance their privacy during transactions. It can be utilized in wallets, payment processors, and other Bitcoin-related applications.

To get started with the ZK Proof Coinjoin Library, check out the [Installation](###installation) section for setup instructions and examples on how to use the library in your projects.

### Installation

You can use crates
```rust
[dependencies]
zk_coinjoin_lib = "1.0.0-beta"
```

## Roadmap

- [x] Add core logic
- [x] Rust Bitcoin compatible
- [ ] Crates
- [ ] Official Beta release in 2025
