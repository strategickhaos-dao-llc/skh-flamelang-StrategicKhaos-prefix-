//! Honeypot trap system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::{FlameResult, FlameError};

/// Honeypot configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct HoneypotConfig {
    pub honeypots: HashMap<String, String>,
}

/// Get honeypots file path
fn get_honeypots_path(vault_dir: &Path) -> std::path::PathBuf {
    vault_dir.join("honeypots.json")
}

/// Load honeypots configuration
fn load_honeypots(vault_dir: &Path) -> FlameResult<HoneypotConfig> {
    let path = get_honeypots_path(vault_dir);
    
    if !path.exists() {
        return Ok(HoneypotConfig {
            honeypots: HashMap::new(),
        });
    }
    
    let content = fs::read_to_string(path)?;
    let config: HoneypotConfig = serde_json::from_str(&content)?;
    Ok(config)
}

/// Save honeypots configuration
fn save_honeypots(vault_dir: &Path, config: &HoneypotConfig) -> FlameResult<()> {
    let path = get_honeypots_path(vault_dir);
    let content = serde_json::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}

/// Check if a name is a honeypot
pub fn is_honeypot(vault_dir: &Path, name: &str) -> FlameResult<bool> {
    let config = load_honeypots(vault_dir)?;
    Ok(config.honeypots.contains_key(name))
}

/// Get honeypot value
pub fn get_honeypot_value(vault_dir: &Path, name: &str) -> FlameResult<String> {
    let config = load_honeypots(vault_dir)?;
    config.honeypots.get(name)
        .cloned()
        .ok_or_else(|| FlameError::Vault(format!("Honeypot '{}' not found", name)))
}

/// Set a honeypot
pub fn set_honeypot(vault_dir: &Path, name: &str, value: &str) -> FlameResult<()> {
    let mut config = load_honeypots(vault_dir)?;
    config.honeypots.insert(name.to_string(), value.to_string());
    save_honeypots(vault_dir, &config)?;
    Ok(())
}

/// Deploy standard honeypot traps from environment variables
pub fn deploy_standard_traps(vault_dir: &Path) -> FlameResult<Vec<String>> {
    let mut deployed = Vec::new();
    
    // Standard environment variables that might be exposed
    let standard_vars = vec![
        "OPENAI_API_KEY",
        "XAI_API_KEY",
        "EMAIL_PASS",
        "GITHUB_TOKEN",
        "AWS_ACCESS_KEY_ID",
        "AWS_SECRET_ACCESS_KEY",
        "ANTHROPIC_API_KEY",
        "GOOGLE_API_KEY",
    ];
    
    let mut config = load_honeypots(vault_dir)?;
    
    for var_name in standard_vars {
        if let Ok(value) = std::env::var(var_name) {
            if !value.is_empty() {
                config.honeypots.insert(var_name.to_string(), value);
                deployed.push(var_name.to_string());
            }
        }
    }
    
    save_honeypots(vault_dir, &config)?;
    Ok(deployed)
}

/// List all honeypots
pub fn list_honeypots(vault_dir: &Path) -> FlameResult<Vec<String>> {
    let config = load_honeypots(vault_dir)?;
    Ok(config.honeypots.keys().cloned().collect())
}

/// Remove a honeypot
pub fn remove_honeypot(vault_dir: &Path, name: &str) -> FlameResult<()> {
    let mut config = load_honeypots(vault_dir)?;
    config.honeypots.remove(name);
    save_honeypots(vault_dir, &config)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_honeypot_operations() {
        let temp_dir = std::env::temp_dir().join("flamevault-test");
        let _ = fs::create_dir_all(&temp_dir);
        
        // Set honeypot
        set_honeypot(&temp_dir, "TEST_KEY", "test-value-123").unwrap();
        
        // Check if honeypot exists
        assert!(is_honeypot(&temp_dir, "TEST_KEY").unwrap());
        assert!(!is_honeypot(&temp_dir, "NONEXISTENT").unwrap());
        
        // Get honeypot value
        let value = get_honeypot_value(&temp_dir, "TEST_KEY").unwrap();
        assert_eq!(value, "test-value-123");
        
        // List honeypots
        let list = list_honeypots(&temp_dir).unwrap();
        assert!(list.contains(&"TEST_KEY".to_string()));
        
        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
