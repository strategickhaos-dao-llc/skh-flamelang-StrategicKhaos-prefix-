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
}
