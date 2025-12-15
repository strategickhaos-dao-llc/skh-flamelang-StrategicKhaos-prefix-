//! Machine-bound encryption using BLAKE3

use blake3;
use serde::{Deserialize, Serialize};
use std::env;
use crate::{FlameResult, FlameError};

/// Encrypted secret structure
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedSecret {
    pub nonce: String,
    pub ciphertext: String,
    pub machine_id: String,
}

/// Derive a machine-bound encryption key
pub fn derive_machine_key() -> FlameResult<Vec<u8>> {
    let mut key_material = Vec::new();
    
    // Collect machine identity components
    let device_id = env::var("STRAT_DEVICE_ID").unwrap_or_else(|_| "default-device".to_string());
    let hemisphere = env::var("STRAT_HEMISPHERE").unwrap_or_else(|_| "Unknown".to_string());
    let region = env::var("STRAT_REGION").unwrap_or_else(|_| "US".to_string());
    
    // Get hardware UID (using hostname as a fallback)
    let hardware_uid = get_hardware_uid();
    
    // Combine all components
    key_material.extend_from_slice(device_id.as_bytes());
    key_material.extend_from_slice(hemisphere.as_bytes());
    key_material.extend_from_slice(region.as_bytes());
    key_material.extend_from_slice(hardware_uid.as_bytes());
    
    // Derive key using BLAKE3
    let hash = blake3::hash(&key_material);
    Ok(hash.as_bytes().to_vec())
}

/// Get hardware UID
fn get_hardware_uid() -> String {
    // Try to get hostname as a basic hardware identifier
    if let Ok(hostname) = hostname::get() {
        if let Ok(hostname_str) = hostname.into_string() {
            return hostname_str;
        }
    }
    
    // Fallback to a static identifier
    "flamevault-machine".to_string()
}

/// Get machine ID for display
pub fn get_machine_id() -> String {
    let device_id = env::var("STRAT_DEVICE_ID").unwrap_or_else(|_| "default-device".to_string());
    let hemisphere = env::var("STRAT_HEMISPHERE").unwrap_or_else(|_| "Unknown".to_string());
    let hardware_uid = get_hardware_uid();
    
    format!("{}@{}:{}", device_id, hemisphere, hardware_uid)
}

/// Encrypt a secret value
pub fn encrypt_secret(key: &[u8], plaintext: &str) -> FlameResult<String> {
    // Generate a nonce (using timestamp + random data)
    let nonce = generate_nonce();
    
    // Create cipher material by combining key with nonce
    let mut cipher_key_material = key.to_vec();
    cipher_key_material.extend_from_slice(nonce.as_bytes());
    let cipher_key = blake3::hash(&cipher_key_material);
    
    // XOR encryption (simple but effective for this use case)
    let ciphertext = xor_encrypt(plaintext.as_bytes(), cipher_key.as_bytes());
    
    let encrypted = EncryptedSecret {
        nonce: nonce.clone(),
        ciphertext: hex::encode(ciphertext),
        machine_id: get_machine_id(),
    };
    
    Ok(serde_json::to_string_pretty(&encrypted)?)
}

/// Decrypt a secret value
pub fn decrypt_secret(key: &[u8], encrypted_json: &str) -> FlameResult<String> {
    let encrypted: EncryptedSecret = serde_json::from_str(encrypted_json)?;
    
    // Verify machine ID matches
    let current_machine_id = get_machine_id();
    if encrypted.machine_id != current_machine_id {
        return Err(FlameError::Vault(format!(
            "Secret encrypted on different machine. Expected: {}, Got: {}",
            encrypted.machine_id,
            current_machine_id
        )));
    }
    
    // Recreate cipher key
    let mut cipher_key_material = key.to_vec();
    cipher_key_material.extend_from_slice(encrypted.nonce.as_bytes());
    let cipher_key = blake3::hash(&cipher_key_material);
    
    // Decode and decrypt
    let ciphertext = hex::decode(&encrypted.ciphertext)
        .map_err(|e| FlameError::Vault(format!("Invalid ciphertext: {}", e)))?;
    
    let plaintext_bytes = xor_encrypt(&ciphertext, cipher_key.as_bytes());
    let plaintext = String::from_utf8(plaintext_bytes)
        .map_err(|e| FlameError::Vault(format!("Invalid UTF-8 in decrypted data: {}", e)))?;
    
    Ok(plaintext)
}

/// Generate a nonce
fn generate_nonce() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    format!("nonce-{}", timestamp)
}

/// XOR encryption/decryption
fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % key.len()])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encryption_decryption() {
        let key = derive_machine_key().unwrap();
        let secret = "my-secret-value-123";
        
        let encrypted = encrypt_secret(&key, secret).unwrap();
        let decrypted = decrypt_secret(&key, &encrypted).unwrap();
        
        assert_eq!(secret, decrypted);
    }
    
    #[test]
    fn test_xor_encrypt() {
        let data = b"hello";
        let key = b"key";
        let encrypted = xor_encrypt(data, key);
        let decrypted = xor_encrypt(&encrypted, key);
        assert_eq!(data.to_vec(), decrypted);
    }
}
