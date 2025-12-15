//! FlameVault — Honeypot + Encrypted Secrets Engine
//! INV-080: Strategickhaos DAO LLC
//!
//! Machine-bound encryption with honeypot trap for exposed credentials.
//! Attackers get bait keys that trigger alerts. Real keys stay encrypted.

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Utc};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// FlameVault configuration
#[derive(Debug, Clone)]
pub struct FlameVault {
    pub vault_path: PathBuf,
    pub machine_key: Vec<u8>,
    pub device_id: String,
    pub hemisphere: String,
    pub region: String,
    honeypots: HashMap<String, HoneypotKey>,
}

/// A honeypot (bait) key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneypotKey {
    pub name: String,
    pub value: String,
    pub deployed_at: DateTime<Utc>,
    pub access_count: u64,
}

/// Alert when honeypot is accessed
#[derive(Debug, Serialize, Deserialize)]
pub struct HoneypotAlert {
    pub timestamp: DateTime<Utc>,
    pub alert_type: String,
    pub key_name: String,
    pub process_name: String,
    pub username: String,
    pub device_id: String,
    pub hemisphere: String,
}

/// Encrypted secret storage format
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedSecret {
    pub name: String,
    pub nonce: String,      // Base64 encoded
    pub ciphertext: String, // Base64 encoded
    pub created_at: DateTime<Utc>,
    pub machine_hash: String, // For verification
}

/// Result of secret retrieval
#[derive(Debug)]
pub enum SecretResult {
    /// Real decrypted secret
    Real(String),
    /// Honeypot value (triggers alert)
    Honeypot(String),
    /// Secret not found
    NotFound,
    /// Decryption failed (wrong machine?)
    DecryptionFailed,
}

impl FlameVault {
    /// Initialize FlameVault with machine-bound encryption
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Get machine identifiers
        let device_id = std::env::var("STRAT_DEVICE_ID")
            .or_else(|_| std::env::var("COMPUTERNAME"))
            .unwrap_or_else(|_| whoami::devicename());
        
        let hemisphere = std::env::var("STRAT_HEMISPHERE")
            .unwrap_or_else(|_| "Unknown".to_string());
        
        let region = std::env::var("STRAT_REGION")
            .unwrap_or_else(|_| "US".to_string());
        
        // Create vault directory
        let vault_path = dirs::home_dir()
            .ok_or("Could not find home directory")?
            .join(".flamevault");
        
        fs::create_dir_all(&vault_path)?;
        
        // Generate machine-bound key
        let machine_key = Self::derive_machine_key(&device_id, &hemisphere, &region)?;
        
        // Load existing honeypots
        let honeypots = Self::load_honeypots(&vault_path)?;
        
        Ok(Self {
            vault_path,
            machine_key,
            device_id,
            hemisphere,
            region,
            honeypots,
        })
    }
    
    /// Derive encryption key from machine identifiers
    fn derive_machine_key(
        device_id: &str,
        hemisphere: &str,
        region: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Get machine UID if available
        let machine_uid = machine_uid::get().unwrap_or_else(|_| "fallback-uid".to_string());
        
        // Combine identifiers
        let combined = format!(
            "{}|{}|{}|{}|STRATEGICKHAOS-FLAMEVAULT-7%",
            device_id, hemisphere, region, machine_uid
        );
        
        // Hash to get salt material
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        let hash = hasher.finalize();
        
        // Use Argon2 to derive a strong key
        let salt = SaltString::encode_b64(&hash[..16])
            .map_err(|e| format!("Salt error: {}", e))?;
        
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(combined.as_bytes(), &salt)
            .map_err(|e| format!("Argon2 error: {}", e))?;
        
        // Extract the hash bytes (32 bytes for AES-256)
        let hash_str = password_hash.hash.ok_or("No hash output")?;
        let key_bytes: Vec<u8> = hash_str.as_bytes().iter().take(32).copied().collect();
        
        // Pad to 32 bytes if needed
        let mut key = vec![0u8; 32];
        for (i, &b) in key_bytes.iter().enumerate() {
            key[i] = b;
        }
        
        Ok(key)
    }
    
    /// Load honeypot configuration
    fn load_honeypots(vault_path: &PathBuf) -> Result<HashMap<String, HoneypotKey>, Box<dyn std::error::Error>> {
        let honeypot_file = vault_path.join("honeypots.json");
        
        if honeypot_file.exists() {
            let content = fs::read_to_string(&honeypot_file)?;
            let honeypots: HashMap<String, HoneypotKey> = serde_json::from_str(&content)?;
            Ok(honeypots)
        } else {
            Ok(HashMap::new())
        }
    }
    
    /// Save honeypot configuration
    fn save_honeypots(&self) -> Result<(), Box<dyn std::error::Error>> {
        let honeypot_file = self.vault_path.join("honeypots.json");
        let content = serde_json::to_string_pretty(&self.honeypots)?;
        fs::write(honeypot_file, content)?;
        Ok(())
    }
    
    /// Register a honeypot (bait) key
    pub fn set_honeypot(&mut self, name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let honeypot = HoneypotKey {
            name: name.to_string(),
            value: value.to_string(),
            deployed_at: Utc::now(),
            access_count: 0,
        };
        
        self.honeypots.insert(name.to_string(), honeypot);
        self.save_honeypots()?;
        
        println!("🍯 Honeypot deployed: {}", name);
        Ok(())
    }
    
    /// Encrypt and store a real secret
    pub fn set_secret(&self, name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Create cipher
        let cipher = Aes256Gcm::new_from_slice(&self.machine_key)
            .map_err(|e| format!("Cipher error: {}", e))?;
        
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, value.as_bytes())
            .map_err(|e| format!("Encryption error: {}", e))?;
        
        // Create machine hash for verification
        let mut hasher = Sha256::new();
        hasher.update(&self.machine_key);
        let machine_hash = hex::encode(hasher.finalize());
        
        // Build encrypted secret struct
        let secret = EncryptedSecret {
            name: name.to_string(),
            nonce: BASE64.encode(nonce_bytes),
            ciphertext: BASE64.encode(&ciphertext),
            created_at: Utc::now(),
            machine_hash: machine_hash[..16].to_string(),
        };
        
        // Save to file
        let secret_file = self.vault_path.join(format!("{}.enc.json", name));
        let content = serde_json::to_string_pretty(&secret)?;
        fs::write(&secret_file, content)?;
        
        println!("🔥 Encrypted: {} → {:?}", name, secret_file);
        Ok(())
    }
    
    /// Retrieve a secret (checks honeypot first, then real secrets)
    pub fn get_secret(&mut self, name: &str) -> SecretResult {
        // Check if it's a honeypot and clone the value before doing mutations
        if let Some(honeypot) = self.honeypots.get(name) {
            let honeypot_value = honeypot.value.clone();
            
            // Now do mutations
            if let Some(honeypot) = self.honeypots.get_mut(name) {
                honeypot.access_count += 1;
            }
            let _ = self.save_honeypots();
            
            // Log alert
            let _ = self.log_honeypot_alert(name);
            
            println!("🚨 HONEYPOT TRIGGERED: {}", name);
            return SecretResult::Honeypot(honeypot_value);
        }
        
        // Try to load real secret
        let secret_file = self.vault_path.join(format!("{}.enc.json", name));
        
        if !secret_file.exists() {
            return SecretResult::NotFound;
        }
        
        // Load and decrypt
        match self.decrypt_secret(&secret_file) {
            Ok(value) => SecretResult::Real(value),
            Err(_) => SecretResult::DecryptionFailed,
        }
    }
    
    /// Decrypt a secret file
    fn decrypt_secret(&self, path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let secret: EncryptedSecret = serde_json::from_str(&content)?;
        
        // Verify machine hash
        let mut hasher = Sha256::new();
        hasher.update(&self.machine_key);
        let current_hash = hex::encode(hasher.finalize());
        
        if !current_hash.starts_with(&secret.machine_hash) {
            return Err("Machine hash mismatch - wrong device?".into());
        }
        
        // Decrypt
        let cipher = Aes256Gcm::new_from_slice(&self.machine_key)
            .map_err(|e| format!("Cipher error: {}", e))?;
        
        let nonce_bytes = BASE64.decode(&secret.nonce)?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = BASE64.decode(&secret.ciphertext)?;
        
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| format!("Decryption error: {}", e))?;
        
        String::from_utf8(plaintext).map_err(|e| e.into())
    }
    
    /// Log a honeypot access alert
    fn log_honeypot_alert(&self, key_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let alert = HoneypotAlert {
            timestamp: Utc::now(),
            alert_type: "HONEYPOT_ACCESS".to_string(),
            key_name: key_name.to_string(),
            process_name: std::env::current_exe()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "unknown".to_string()),
            username: whoami::username(),
            device_id: self.device_id.clone(),
            hemisphere: self.hemisphere.clone(),
        };
        
        let alert_file = self.vault_path.join("alerts.log");
        let alert_json = serde_json::to_string(&alert)?;
        
        use std::io::Write;
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(alert_file)?;
        
        writeln!(file, "{}", alert_json)?;
        
        Ok(())
    }
    
    /// List all stored secrets (names only, not values)
    pub fn list_secrets(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut secrets = Vec::new();
        
        for entry in fs::read_dir(&self.vault_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str.ends_with(".enc.json") {
                    secrets.push(name_str.replace(".enc.json", ""));
                }
            }
        }
        
        Ok(secrets)
    }
    
    /// List all honeypots
    pub fn list_honeypots(&self) -> Vec<&HoneypotKey> {
        self.honeypots.values().collect()
    }
    
    /// Export environment setup script
    pub fn export_env_script(&self) -> Result<String, Box<dyn std::error::Error>> {
        let secrets = self.list_secrets()?;
        
        let mut script = String::from(r#"# ═══════════════════════════════════════════════════════════════════════════════
# FLAMEVAULT ENVIRONMENT LOADER
# Generated by FlameVault - Strategickhaos DAO LLC
# ═══════════════════════════════════════════════════════════════════════════════

"#);
        
        // Add honeypot keys (visible bait)
        script.push_str("# === HONEYPOT LAYER (BAIT) ===\n");
        for honeypot in self.honeypots.values() {
            script.push_str(&format!(
                "export {}=\"{}\"  # 🍯 HONEYPOT\n",
                honeypot.name, honeypot.value
            ));
        }
        
        script.push_str("\n# === REAL SECRETS (load via flamevault) ===\n");
        for secret in secrets {
            script.push_str(&format!(
                "# export {}=$(flamevault get {})\n",
                secret.to_uppercase(),
                secret
            ));
        }
        
        Ok(script)
    }
    
    /// Get vault status
    pub fn status(&self) -> String {
        format!(
            r#"
🔥 FLAMEVAULT STATUS
═══════════════════════════════════════
Device ID:    {}
Hemisphere:   {}
Region:       {}
Vault Path:   {:?}
Secrets:      {} encrypted
Honeypots:    {} deployed
Machine Key:  {}...
═══════════════════════════════════════
"#,
            self.device_id,
            self.hemisphere,
            self.region,
            self.vault_path,
            self.list_secrets().unwrap_or_default().len(),
            self.honeypots.len(),
            hex::encode(&self.machine_key[..8])
        )
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
    fn test_vault_creation() {
        let vault = FlameVault::new();
        assert!(vault.is_ok());
    }
}
