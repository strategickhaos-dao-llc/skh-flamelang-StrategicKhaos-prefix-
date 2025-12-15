//! # FlameVault Cryptographic Module
//! 
//! Quantum-resistant polymorphic encryption engine leveraging FlameLang's
//! 5-layer transformation pipeline as encryption stages.
//! 
//! © 2025 Strategickhaos DAO LLC

pub mod flamevault;
pub mod quantum;
pub mod dna_encoding;

pub use flamevault::{FlameVault, FlameVaultBlock, EncryptionResult};
pub use quantum::{QuantumResistantKeyPair, KeyExchange, SignatureScheme};
pub use dna_encoding::{DnaEncoder, DnaKeyDerivation};
