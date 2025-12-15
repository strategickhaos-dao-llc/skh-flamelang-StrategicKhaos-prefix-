//! Encryption utilities for FlameVault
//! 
//! Uses AES-256-GCM for authenticated encryption with machine-bound keys

use crate::flamevault::{FlameVaultError, FlameVaultResult};
use sha2::Digest;
use std::env;

/// Derive a machine-bound encryption key
/// 
/// The key is derived from:
/// - Computer/hostname
/// - STRAT_HEMISPHERE environment variable
/// - STRAT_DEVICE_ID environment variable
/// - A constant salt "STRATEGICKHAOS"
pub fn derive_machine_key() -> FlameVaultResult<Vec<u8>> {
    let hostname = hostname::get()
        .map_err(|e| FlameVaultError::Encryption(format!("Failed to get hostname: {}", e)))?
        .to_string_lossy()
        .to_string();
    
    let hemisphere = env::var("STRAT_HEMISPHERE").unwrap_or_else(|_| "UNKNOWN".to_string());
    let device_id = env::var("STRAT_DEVICE_ID").unwrap_or_else(|_| "UNKNOWN".to_string());
    
    let key_material = format!("{}|{}|{}|STRATEGICKHAOS", hostname, hemisphere, device_id);
    
    // Use SHA256 to derive a 32-byte key
    let hash = sha2::Sha256::digest(key_material.as_bytes());
    Ok(hash.to_vec())
}

/// Encrypt data using AES-256-GCM with the provided key
pub fn encrypt(key: &[u8], plaintext: &str) -> FlameVaultResult<Vec<u8>> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    
    if key.len() != 32 {
        return Err(FlameVaultError::Encryption(
            "Key must be 32 bytes for AES-256".into()
        ));
    }
    
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| FlameVaultError::Encryption(format!("Failed to create cipher: {}", e)))?;
    
    // Generate a random nonce (12 bytes for GCM)
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| FlameVaultError::Encryption(format!("Encryption failed: {}", e)))?;
    
    // Prepend nonce to ciphertext for storage
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

/// Decrypt data using AES-256-GCM with the provided key
pub fn decrypt(key: &[u8], ciphertext: &[u8]) -> FlameVaultResult<String> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    
    if key.len() != 32 {
        return Err(FlameVaultError::Decryption(
            "Key must be 32 bytes for AES-256".into()
        ));
    }
    
    if ciphertext.len() < 12 {
        return Err(FlameVaultError::Decryption(
            "Ciphertext too short, missing nonce".into()
        ));
    }
    
    // Extract nonce (first 12 bytes)
    let nonce = Nonce::from_slice(&ciphertext[..12]);
    let encrypted_data = &ciphertext[12..];
    
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| FlameVaultError::Decryption(format!("Failed to create cipher: {}", e)))?;
    
    let plaintext_bytes = cipher
        .decrypt(nonce, encrypted_data)
        .map_err(|e| FlameVaultError::Decryption(format!("Decryption failed: {}", e)))?;
    
    String::from_utf8(plaintext_bytes)
        .map_err(|e| FlameVaultError::Decryption(format!("Invalid UTF-8: {}", e)))
}

/// Encode binary data as base64
pub fn encode_base64(data: &[u8]) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, data)
}

/// Decode base64 to binary data
pub fn decode_base64(data: &str) -> FlameVaultResult<Vec<u8>> {
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, data)
        .map_err(|e| FlameVaultError::Decryption(format!("Base64 decode error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_machine_key_derivation() {
        let key = derive_machine_key();
        assert!(key.is_ok());
        let key = key.unwrap();
        assert_eq!(key.len(), 32); // SHA256 produces 32 bytes
    }
    
    #[test]
    fn test_encryption_decryption() {
        let key = derive_machine_key().unwrap();
        let plaintext = "This is a secret message!";
        
        let encrypted = encrypt(&key, plaintext).unwrap();
        let decrypted = decrypt(&key, &encrypted).unwrap();
        
        assert_eq!(plaintext, decrypted);
    }
    
    #[test]
    fn test_base64_roundtrip() {
        let data = b"Hello, World!";
        let encoded = encode_base64(data);
        let decoded = decode_base64(&encoded).unwrap();
        assert_eq!(data, decoded.as_slice());
    }
}
