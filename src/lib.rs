//! # FlamevVault v0.1.0
//! 
//! Honeypot + Encrypted Secrets Engine - INV-080
//!
//! © 2025 Strategickhaos DAO LLC

use std::fmt;

/// FlamevVault error type
#[derive(Debug)]
pub enum FlameVaultError {
    Encryption(String),
    Decryption(String),
    KeyDerivation(String),
    Serialization(String),
    FileSystem(String),
    InvalidInput(String),
}

impl fmt::Display for FlameVaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlameVaultError::Encryption(msg) => write!(f, "Encryption error: {}", msg),
            FlameVaultError::Decryption(msg) => write!(f, "Decryption error: {}", msg),
            FlameVaultError::KeyDerivation(msg) => write!(f, "Key derivation error: {}", msg),
            FlameVaultError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            FlameVaultError::FileSystem(msg) => write!(f, "File system error: {}", msg),
            FlameVaultError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for FlameVaultError {}

pub type FlameVaultResult<T> = Result<T, FlameVaultError>;
