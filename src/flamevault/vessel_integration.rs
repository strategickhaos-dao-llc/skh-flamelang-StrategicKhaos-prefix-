//! FlameVault Vessel Integration
//! 
//! Integrates FlameVault secrets management with FlameLang's execution environment

use super::{FlameVault, SecretResult};
use std::collections::HashMap;

/// Integration layer between FlameVault and FlameLang execution
pub struct VesselIntegration {
    vault: FlameVault,
    cached_secrets: HashMap<String, String>,
}

impl VesselIntegration {
    /// Create a new vessel integration
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            vault: FlameVault::new()?,
            cached_secrets: HashMap::new(),
        })
    }
    
    /// Load a secret into the execution environment
    pub fn load_secret(&mut self, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Check cache first
        if let Some(value) = self.cached_secrets.get(name) {
            return Ok(value.clone());
        }
        
        // Retrieve from vault
        match self.vault.get_secret(name) {
            SecretResult::Real(value) => {
                self.cached_secrets.insert(name.to_string(), value.clone());
                Ok(value)
            }
            SecretResult::Honeypot(value) => {
                // Return honeypot value but don't cache it
                Ok(value)
            }
            SecretResult::NotFound => {
                Err(format!("Secret '{}' not found", name).into())
            }
            SecretResult::DecryptionFailed => {
                Err(format!("Failed to decrypt secret '{}' - wrong machine?", name).into())
            }
        }
    }
    
    /// Store a secret
    pub fn store_secret(&mut self, name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.vault.set_secret(name, value)?;
        self.cached_secrets.insert(name.to_string(), value.to_string());
        Ok(())
    }
    
    /// Deploy a honeypot
    pub fn deploy_honeypot(&mut self, name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.vault.set_honeypot(name, value)
    }
    
    /// Clear the secret cache
    pub fn clear_cache(&mut self) {
        self.cached_secrets.clear();
    }
    
    /// Get vault status
    pub fn status(&self) -> String {
        self.vault.status()
    }
}

impl Default for VesselIntegration {
    fn default() -> Self {
        Self::new().expect("Failed to initialize VesselIntegration")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vessel_integration_creation() {
        let integration = VesselIntegration::new();
        assert!(integration.is_ok());
    }
    
    #[test]
    fn test_store_and_load_secret() {
        let mut integration = VesselIntegration::new().unwrap();
        let secret_name = "test_vessel_secret";
        let secret_value = "vessel-secret-value";
        
        // Store secret
        let store_result = integration.store_secret(secret_name, secret_value);
        assert!(store_result.is_ok());
        
        // Load secret
        let load_result = integration.load_secret(secret_name);
        assert!(load_result.is_ok());
        assert_eq!(load_result.unwrap(), secret_value);
        
        // Clean up
        let vault = FlameVault::new().unwrap();
        let secret_file = vault.vault_path.join(format!("{}.enc.json", secret_name));
        let _ = std::fs::remove_file(secret_file);
    }
    
    #[test]
    fn test_secret_caching() {
        let mut integration = VesselIntegration::new().unwrap();
        let secret_name = "cached_secret";
        let secret_value = "cached-value";
        
        // Store secret
        integration.store_secret(secret_name, secret_value).unwrap();
        
        // Load secret twice - second should come from cache
        let first_load = integration.load_secret(secret_name);
        let second_load = integration.load_secret(secret_name);
        
        assert!(first_load.is_ok());
        assert!(second_load.is_ok());
        assert_eq!(first_load.unwrap(), second_load.unwrap());
        
        // Clear cache
        integration.clear_cache();
        
        // Clean up
        let vault = FlameVault::new().unwrap();
        let secret_file = vault.vault_path.join(format!("{}.enc.json", secret_name));
        let _ = std::fs::remove_file(secret_file);
    }
    
    #[test]
    fn test_deploy_honeypot() {
        let mut integration = VesselIntegration::new().unwrap();
        let result = integration.deploy_honeypot("TEST_TRAP", "trap-value");
        assert!(result.is_ok());
        
        // Clean up
        let vault = FlameVault::new().unwrap();
        let honeypot_file = vault.vault_path.join("honeypots.json");
        let _ = std::fs::remove_file(honeypot_file);
    }
    
    #[test]
    fn test_status() {
        let integration = VesselIntegration::new().unwrap();
        let status = integration.status();
        assert!(status.contains("FLAMEVAULT STATUS"));
    }
}
