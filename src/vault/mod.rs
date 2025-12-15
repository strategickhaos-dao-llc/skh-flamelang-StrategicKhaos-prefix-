//! # FlameVault — Honeypot + Encrypted Secrets Engine
//! 
//! Machine-bound encryption and honeypot system for secrets management.
//! 
//! ## Layers:
//! - **VISIBLE (Environment)**: Honeypot keys (bait)
//! - **HIDDEN (Vault)**: Real encrypted secrets
//! - **ALERTS (Log)**: Access attempts logged

pub mod encryption;
pub mod honeypot;
pub mod alerts;
pub mod vessel;

use std::path::PathBuf;
use std::fs;
use crate::FlameResult;

/// FlameVault main structure
pub struct FlameVault {
    vault_dir: PathBuf,
    machine_key: Vec<u8>,
}

impl FlameVault {
    /// Initialize a new FlameVault instance
    pub fn new() -> FlameResult<Self> {
        let vault_dir = Self::get_vault_dir()?;
        
        // Create vault directory if it doesn't exist
        if !vault_dir.exists() {
            fs::create_dir_all(&vault_dir)?;
        }
        
        // Derive machine-bound encryption key
        let machine_key = encryption::derive_machine_key()?;
        
        Ok(Self {
            vault_dir,
            machine_key,
        })
    }
    
    /// Get the vault directory path
    pub fn get_vault_dir() -> FlameResult<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| crate::FlameError::Vault("Could not determine home directory".to_string()))?;
        Ok(home.join(".flamevault"))
    }
    
    /// Set (encrypt and store) a secret
    pub fn set_secret(&self, name: &str, value: &str) -> FlameResult<()> {
        let encrypted = encryption::encrypt_secret(&self.machine_key, value)?;
        let secret_path = self.vault_dir.join(format!("{}.enc.json", name));
        fs::write(secret_path, encrypted)?;
        Ok(())
    }
    
    /// Get (decrypt and retrieve) a secret
    pub fn get_secret(&self, name: &str) -> FlameResult<String> {
        // Check if it's a honeypot first
        if honeypot::is_honeypot(&self.vault_dir, name)? {
            // Log the honeypot access
            alerts::log_honeypot_access(&self.vault_dir, name)?;
            // Return the honeypot value
            return honeypot::get_honeypot_value(&self.vault_dir, name);
        }
        
        // Otherwise, decrypt the real secret
        let secret_path = self.vault_dir.join(format!("{}.enc.json", name));
        if !secret_path.exists() {
            return Err(crate::FlameError::Vault(format!("Secret '{}' not found", name)));
        }
        
        let encrypted = fs::read_to_string(secret_path)?;
        encryption::decrypt_secret(&self.machine_key, &encrypted)
    }
    
    /// Set a honeypot trap
    pub fn set_honeypot(&self, name: &str, value: &str) -> FlameResult<()> {
        honeypot::set_honeypot(&self.vault_dir, name, value)
    }
    
    /// Deploy standard honeypot traps from environment variables
    pub fn deploy_traps(&self) -> FlameResult<Vec<String>> {
        honeypot::deploy_standard_traps(&self.vault_dir)
    }
    
    /// Get all alerts
    pub fn get_alerts(&self) -> FlameResult<Vec<alerts::Alert>> {
        alerts::get_alerts(&self.vault_dir)
    }
    
    /// List all secrets and honeypots
    pub fn list_secrets(&self) -> FlameResult<Vec<String>> {
        let mut secrets = Vec::new();
        
        // List encrypted secrets
        for entry in fs::read_dir(&self.vault_dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(filename) = path.file_name() {
                let filename_str = filename.to_string_lossy();
                if filename_str.ends_with(".enc.json") {
                    let name = filename_str.trim_end_matches(".enc.json");
                    secrets.push(format!("{} (encrypted)", name));
                }
            }
        }
        
        // List honeypots
        let honeypots = honeypot::list_honeypots(&self.vault_dir)?;
        for hp in honeypots {
            secrets.push(format!("{} (honeypot)", hp));
        }
        
        Ok(secrets)
    }
    
    /// Get vault status
    pub fn status(&self) -> FlameResult<VaultStatus> {
        let secret_count = fs::read_dir(&self.vault_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
            .filter(|e| e.path().file_name().unwrap().to_string_lossy().ends_with(".enc.json"))
            .count();
        
        let honeypot_count = honeypot::list_honeypots(&self.vault_dir)?.len();
        let alert_count = alerts::get_alerts(&self.vault_dir)?.len();
        
        Ok(VaultStatus {
            vault_dir: self.vault_dir.clone(),
            secret_count,
            honeypot_count,
            alert_count,
            machine_id: encryption::get_machine_id(),
        })
    }
}

/// Vault status information
#[derive(Debug)]
pub struct VaultStatus {
    pub vault_dir: PathBuf,
    pub secret_count: usize,
    pub honeypot_count: usize,
    pub alert_count: usize,
    pub machine_id: String,
}
