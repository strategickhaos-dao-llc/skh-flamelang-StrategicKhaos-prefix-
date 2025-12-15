//! Storage utilities for FlameVault encrypted secrets

use crate::flamevault::{encryption, FlameVaultError, FlameVaultResult};
use std::fs;
use std::path::PathBuf;

/// Store an encrypted secret
pub fn store_secret(
    vault_path: &PathBuf,
    key: &[u8],
    name: &str,
    value: &str,
) -> FlameVaultResult<()> {
    // Encrypt the secret
    let encrypted = encryption::encrypt(key, value)?;
    
    // Encode as base64 for storage
    let encoded = encryption::encode_base64(&encrypted);
    
    // Write to file
    let secret_file = vault_path.join(format!("{}.enc", name));
    fs::write(&secret_file, encoded)
        .map_err(|e| FlameVaultError::Storage(format!("Failed to write secret: {}", e)))?;
    
    eprintln!("🔥 Encrypted: {} → {:?}", name, secret_file);
    
    Ok(())
}

/// Retrieve and decrypt a secret
pub fn retrieve_secret(vault_path: &PathBuf, key: &[u8], name: &str) -> FlameVaultResult<String> {
    let secret_file = vault_path.join(format!("{}.enc", name));
    
    if !secret_file.exists() {
        return Err(FlameVaultError::SecretNotFound(name.to_string()));
    }
    
    // Read encoded data
    let encoded = fs::read_to_string(&secret_file)
        .map_err(|e| FlameVaultError::Storage(format!("Failed to read secret: {}", e)))?;
    
    // Decode from base64
    let encrypted = encryption::decode_base64(encoded.trim())?;
    
    // Decrypt
    encryption::decrypt(key, &encrypted)
}

/// List all stored secrets
pub fn list_secrets(vault_path: &PathBuf) -> FlameVaultResult<Vec<String>> {
    if !vault_path.exists() {
        return Ok(Vec::new());
    }
    
    let entries = fs::read_dir(vault_path)
        .map_err(|e| FlameVaultError::Storage(format!("Failed to read vault directory: {}", e)))?;
    
    let mut secrets = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "enc" {
                    if let Some(name) = path.file_stem() {
                        secrets.push(name.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    
    Ok(secrets)
}

/// Remove a secret
pub fn remove_secret(vault_path: &PathBuf, name: &str) -> FlameVaultResult<()> {
    let secret_file = vault_path.join(format!("{}.enc", name));
    
    if !secret_file.exists() {
        return Err(FlameVaultError::SecretNotFound(name.to_string()));
    }
    
    fs::remove_file(&secret_file)
        .map_err(|e| FlameVaultError::Storage(format!("Failed to remove secret: {}", e)))?;
    
    eprintln!("🗑️ Removed secret: {}", name);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flamevault::encryption;
    use std::env;
    
    #[test]
    fn test_secret_storage_roundtrip() {
        let temp_dir = env::temp_dir().join("flamevault_storage_test");
        std::fs::create_dir_all(&temp_dir).unwrap();
        
        let key = encryption::derive_machine_key().unwrap();
        let name = "TEST_SECRET";
        let value = "test_value_12345";
        
        // Store
        let store_result = store_secret(&temp_dir, &key, name, value);
        assert!(store_result.is_ok());
        
        // Retrieve
        let retrieved = retrieve_secret(&temp_dir, &key, name);
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap(), value);
        
        // List
        let secrets = list_secrets(&temp_dir).unwrap();
        assert!(secrets.contains(&name.to_string()));
        
        // Remove
        let remove_result = remove_secret(&temp_dir, name);
        assert!(remove_result.is_ok());
        
        // Verify removed
        let retrieved_after = retrieve_secret(&temp_dir, &key, name);
        assert!(retrieved_after.is_err());
        
        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
