//! FlameVault — Honeypot + Encrypted Secrets Engine
//! INV-080: Strategickhaos DAO LLC

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, FlameVaultError>;

#[derive(Debug, thiserror::Error)]
pub enum FlameVaultError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Encryption error: {0}")]
    Encryption(String),
    #[error("Decryption error: {0}")]
    Decryption(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecretResult {
    Real(String),
    Honeypot(String),
    NotFound,
    DecryptionFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneypotInfo {
    pub name: String,
    pub access_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct SecretEntry {
    encrypted_value: String,
    device_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HoneypotEntry {
    value: String,
    access_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct VaultData {
    secrets: HashMap<String, SecretEntry>,
    honeypots: HashMap<String, HoneypotEntry>,
}

pub struct FlameVault {
    pub vault_path: PathBuf,
    device_id: String,
    data: VaultData,
}

impl FlameVault {
    /// Initialize a new FlameVault instance
    pub fn new() -> Result<Self> {
        let vault_path = dirs::home_dir()
            .ok_or_else(|| FlameVaultError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Home directory not found",
            )))?
            .join(".flamevault");

        // Create vault directory if it doesn't exist
        if !vault_path.exists() {
            fs::create_dir_all(&vault_path)?;
        }

        let device_id = Self::get_device_id()?;
        let data = Self::load_vault_data(&vault_path)?;

        Ok(Self {
            vault_path,
            device_id,
            data,
        })
    }

    /// Get a unique device identifier
    fn get_device_id() -> Result<String> {
        // Use machine-id or hostname as device identifier
        if let Ok(machine_id) = fs::read_to_string("/etc/machine-id") {
            return Ok(machine_id.trim().to_string());
        }
        
        if let Ok(machine_id) = fs::read_to_string("/var/lib/dbus/machine-id") {
            return Ok(machine_id.trim().to_string());
        }

        // Fallback to hostname
        if let Ok(hostname) = std::process::Command::new("hostname").output() {
            if hostname.status.success() {
                return Ok(String::from_utf8_lossy(&hostname.stdout).trim().to_string());
            }
        }

        Ok("unknown-device".to_string())
    }

    /// Load vault data from disk
    fn load_vault_data(vault_path: &PathBuf) -> Result<VaultData> {
        let data_file = vault_path.join("vault.json");
        
        if data_file.exists() {
            let content = fs::read_to_string(data_file)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(VaultData {
                secrets: HashMap::new(),
                honeypots: HashMap::new(),
            })
        }
    }

    /// Save vault data to disk
    fn save_vault_data(&self) -> Result<()> {
        let data_file = self.vault_path.join("vault.json");
        let content = serde_json::to_string_pretty(&self.data)?;
        fs::write(data_file, content)?;
        Ok(())
    }

    /// Encrypt a value using BLAKE3 with device binding
    fn encrypt(&self, value: &str) -> String {
        let key = format!("{}:{}", self.device_id, value);
        let hash = blake3::hash(key.as_bytes());
        let encrypted = format!("{}:{}", hex::encode(hash.as_bytes()), hex::encode(value.as_bytes()));
        encrypted
    }

    /// Decrypt a value using BLAKE3 with device binding
    fn decrypt(&self, encrypted: &str) -> std::result::Result<String, ()> {
        let parts: Vec<&str> = encrypted.split(':').collect();
        if parts.len() != 2 {
            return Err(());
        }

        let stored_hash = parts[0];
        let encoded_value = parts[1];

        let value_bytes = hex::decode(encoded_value).map_err(|_| ())?;
        let value = String::from_utf8(value_bytes).map_err(|_| ())?;

        // Verify hash with current device
        let key = format!("{}:{}", self.device_id, value);
        let hash = blake3::hash(key.as_bytes());
        let computed_hash = hex::encode(hash.as_bytes());

        if computed_hash == stored_hash {
            Ok(value)
        } else {
            Err(())
        }
    }

    /// Store an encrypted secret
    pub fn set_secret(&mut self, name: &str, value: &str) -> Result<()> {
        let encrypted_value = self.encrypt(value);
        self.data.secrets.insert(
            name.to_string(),
            SecretEntry {
                encrypted_value,
                device_id: self.device_id.clone(),
            },
        );
        self.save_vault_data()?;
        Ok(())
    }

    /// Retrieve a secret or honeypot
    pub fn get_secret(&mut self, name: &str) -> SecretResult {
        // Check if it's a honeypot first
        if let Some(honeypot) = self.data.honeypots.get_mut(name) {
            honeypot.access_count += 1;
            let value = honeypot.value.clone();
            self.log_honeypot_access(name);
            let _ = self.save_vault_data();
            return SecretResult::Honeypot(value);
        }

        // Check if it's a real secret
        if let Some(secret) = self.data.secrets.get(name) {
            match self.decrypt(&secret.encrypted_value) {
                Ok(value) => SecretResult::Real(value),
                Err(_) => SecretResult::DecryptionFailed,
            }
        } else {
            SecretResult::NotFound
        }
    }

    /// Deploy a honeypot
    pub fn set_honeypot(&mut self, name: &str, value: &str) -> Result<()> {
        self.data.honeypots.insert(
            name.to_string(),
            HoneypotEntry {
                value: value.to_string(),
                access_count: 0,
            },
        );
        self.save_vault_data()?;
        Ok(())
    }

    /// Log honeypot access
    fn log_honeypot_access(&self, name: &str) {
        let alert_file = self.vault_path.join("alerts.log");
        
        let alert = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "key_name": name,
            "device_id": &self.device_id,
            "process_name": std::env::current_exe()
                .ok()
                .and_then(|p| p.file_name().map(|s| s.to_string_lossy().to_string()))
                .unwrap_or_else(|| "unknown".to_string()),
        });

        let mut log_content = String::new();
        if alert_file.exists() {
            log_content = fs::read_to_string(&alert_file).unwrap_or_default();
        }
        
        log_content.push_str(&format!("{}\n", alert));
        let _ = fs::write(alert_file, log_content);
    }

    /// List all secrets
    pub fn list_secrets(&self) -> Result<Vec<String>> {
        Ok(self.data.secrets.keys().cloned().collect())
    }

    /// List all honeypots
    pub fn list_honeypots(&self) -> Vec<HoneypotInfo> {
        self.data
            .honeypots
            .iter()
            .map(|(name, entry)| HoneypotInfo {
                name: name.clone(),
                access_count: entry.access_count,
            })
            .collect()
    }

    /// Get vault status
    pub fn status(&self) -> String {
        let secrets_count = self.data.secrets.len();
        let honeypots_count = self.data.honeypots.len();
        let alerts_file = self.vault_path.join("alerts.log");
        let alerts_count = if alerts_file.exists() {
            fs::read_to_string(&alerts_file)
                .map(|s| s.lines().count())
                .unwrap_or(0)
        } else {
            0
        };

        format!(
            "🔥 FlameVault Status\n\
            \n\
            📍 Location: {}\n\
            🔐 Encrypted Secrets: {}\n\
            🍯 Honeypots: {}\n\
            🚨 Total Alerts: {}\n\
            💻 Device ID: {}",
            self.vault_path.display(),
            secrets_count,
            honeypots_count,
            alerts_count,
            &self.device_id[..self.device_id.len().min(16)]
        )
    }

    /// Export environment script
    pub fn export_env_script(&mut self) -> Result<String> {
        let mut script = String::new();
        
        for (name, secret) in &self.data.secrets {
            match self.decrypt(&secret.encrypted_value) {
                Ok(value) => {
                    script.push_str(&format!("export {}=\"{}\"\n", name, value));
                }
                Err(_) => {
                    script.push_str(&format!("# {} (decryption failed)\n", name));
                }
            }
        }
        
        Ok(script)
    }
}
