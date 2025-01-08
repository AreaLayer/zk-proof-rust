# ZK Proof Rust 🌀

[![Bitcoin-only](https://img.shields.io/badge/bitcoin-only-FF9900?logo=bitcoin)](https://twentyone.world)
[![LN](https://img.shields.io/badge/lightning-792EE5?logo=lightning)](https://mempool.space/lightning)

**Beta software, use with caution for  production**

ZK Proof Library for Coinjoin transactions

## About

The **ZK Proof Coinjoin Library** is a Rust-based implementation designed to facilitate privacy-preserving Coinjoin transactions. Utilizing advanced cryptographic techniques, including zero-knowledge proofs (zk-SNARKs), this library aims to enhance the privacy and security of Bitcoin transactions by enabling multiple users to combine their coins into a single transaction without revealing their individual inputs.

### Key Features

- **Zero-Knowledge Proofs**: Implement zk-SNARKs to ensure that transaction details remain confidential while still allowing verification of the transaction's validity.
- **Peer-to-Peer Communication**: Facilitate secure communication between participants in a Coinjoin transaction, ensuring that user identities and transaction details are protected.
- **SegWit and Taproot Compatibility**: Support modern Bitcoin features like Segregated Witness (SegWit) and Taproot, enhancing flexibility and transaction efficiency.
- **Rust Bitcoin Compatibility**: Integrate seamlessly with the Rust Bitcoin library, enabling seamless integration with the Bitcoin ecosystem.
- **Transaction Validation**: Validate transactions to ensure they meet the necessary criteria for Coinjoin transactions.
- **MPC Support**: Support for Multi-Party Computation (MPC) to enhance privacy and security in Coinjoin transactions.
- **Lightning Network Integration**: Enable integration with the Lightning Network for enhanced privacy and scalability by LDK
- **Electrum Client**: Integrate with Electrum client to enable secure and private transactions by BDK Electrum

### Use Cases

This library is suitable for developers looking to implement Coinjoin functionality in their Bitcoin applications, enabling users to enhance their privacy during transactions. It can be utilized in wallets, payment processors, and other Bitcoin-related applications.

### Installation

To start using the ZK Proof SDK, add it to your Cargo.toml:

```Cargo.toml
[dependencies]
zk_coinjoin_lib = "1.0.0-beta"
```

## Roadmap

- [ ] Official Beta release in 2025
