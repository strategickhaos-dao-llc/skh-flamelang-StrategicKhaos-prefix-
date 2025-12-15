//! FlameVault: Honeypot + Encrypted Secrets Engine
//! 
//! INV-080: Strategickhaos DAO LLC
//! 
//! Provides a dual-layer security system:
//! - Honeypot layer: Monitored fake keys that trigger alerts
//! - Encrypted layer: Machine-bound encrypted storage for real secrets

pub mod encryption;
pub mod honeypot;
pub mod storage;

use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlameVaultError {
    #[error("Encryption error: {0}")]
    Encryption(String),
    
    #[error("Decryption error: {0}")]
    Decryption(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Secret not found: {0}")]
    SecretNotFound(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Honeypot triggered: {0}")]
    HoneypotTriggered(String),
}

pub type FlameVaultResult<T> = Result<T, FlameVaultError>;

/// FlameVault instance managing secrets and honeypots
pub struct FlameVault {
    vault_path: PathBuf,
    honeypot_keys: HashMap<String, String>,
    machine_key: Vec<u8>,
}

impl FlameVault {
    /// Create a new FlameVault instance
    pub fn new() -> FlameVaultResult<Self> {
        let vault_path = Self::get_vault_path()?;
        let machine_key = encryption::derive_machine_key()?;
        let honeypot_keys = honeypot::get_default_honeypot_keys();
        
        // Ensure vault directory exists
        std::fs::create_dir_all(&vault_path)?;
        
        Ok(Self {
            vault_path,
            honeypot_keys,
            machine_key,
        })
    }
    
    /// Get the vault directory path
    fn get_vault_path() -> FlameVaultResult<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| FlameVaultError::Storage("Could not determine home directory".into()))?;
        Ok(home.join(".flamevault"))
    }
    
    /// Set a secret (encrypt and store)
    pub fn set_secret(&self, name: &str, value: &str) -> FlameVaultResult<()> {
        storage::store_secret(&self.vault_path, &self.machine_key, name, value)
    }
    
    /// Get a secret (retrieve and decrypt)
    pub fn get_secret(&self, name: &str) -> FlameVaultResult<String> {
        storage::retrieve_secret(&self.vault_path, &self.machine_key, name)
    }
    
    /// Deploy honeypot keys to environment
    pub fn deploy_honeypot(&self) -> FlameVaultResult<()> {
        honeypot::deploy_honeypot(&self.vault_path, &self.honeypot_keys)
    }
    
    /// Get secret with smart routing (checks honeypot first)
    pub fn get_real_secret(&self, name: &str) -> FlameVaultResult<String> {
        // Check if this is a honeypot key
        if self.honeypot_keys.contains_key(name) {
            honeypot::log_honeypot_access(&self.vault_path, name)?;
            return Ok(self.honeypot_keys[name].clone());
        }
        
        // Return real encrypted secret with REAL_ prefix
        let real_name = format!("REAL_{}", name);
        self.get_secret(&real_name)
    }
    
    /// Get recent honeypot alerts
    pub fn get_alerts(&self, count: usize) -> FlameVaultResult<Vec<honeypot::Alert>> {
        honeypot::get_alerts(&self.vault_path, count)
    }
    
    /// List all stored secrets
    pub fn list_secrets(&self) -> FlameVaultResult<Vec<String>> {
        storage::list_secrets(&self.vault_path)
    }
    
    /// Remove a secret
    pub fn remove_secret(&self, name: &str) -> FlameVaultResult<()> {
        storage::remove_secret(&self.vault_path, name)
    }
}

impl Default for FlameVault {
    fn default() -> Self {
        Self::new().expect("Failed to initialize FlameVault")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_flamevault_creation() {
        let vault = FlameVault::new();
        assert!(vault.is_ok());
    }
    
    #[test]
    fn test_secret_roundtrip() {
        let vault = FlameVault::new().unwrap();
        let test_name = "TEST_SECRET";
        let test_value = "test_value_12345";
        
        // Store secret
        let store_result = vault.set_secret(test_name, test_value);
        assert!(store_result.is_ok());
        
        // Retrieve secret
        let retrieved = vault.get_secret(test_name);
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap(), test_value);
        
        // Cleanup
        let _ = vault.remove_secret(test_name);
    }
    
    #[test]
    fn test_honeypot_detection() {
        let vault = FlameVault::new().unwrap();
        
        // Access a honeypot key should return the honeypot value
        let result = vault.get_real_secret("OPENAI_API_KEY");
        assert!(result.is_ok());
    }
}
