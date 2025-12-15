//! Honeypot layer for FlameVault
//! 
//! Manages fake keys that trigger alerts when accessed

use crate::flamevault::{FlameVaultError, FlameVaultResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

/// Alert structure for honeypot access attempts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub timestamp: String,
    pub alert: String,
    pub key: String,
    pub process: String,
    pub user: String,
    pub computer: String,
}

/// Get the default honeypot keys (fake keys that trigger alerts)
pub fn get_default_honeypot_keys() -> HashMap<String, String> {
    let mut keys = HashMap::new();
    
    keys.insert(
        "OPENAI_API_KEY".to_string(),
        "sk-svcacct-sbfXUZ_PeKwvpwplir8qGOw1w3WHzoWL2kostsd4YBYkijcqPnkKlgxSF9sSfZlt-galia9E1U4CT3BlbkFJbq0w-cz73G4U1MQrmWtRTFfD7LNGHJhmujyPzf7".to_string(),
    );
    
    keys.insert(
        "XAI_API_KEY".to_string(),
        "xai-A8DdRfGLfXnAT9Mwzg9wxvpQhEDLRA4BKPoKAlU7AgM1gvbjUuHkE8XeLTONTGUE1CMX5pc0dAlR2ddy".to_string(),
    );
    
    keys.insert(
        "EMAIL_PASS".to_string(),
        "imcnaicqemiiuzqr".to_string(),
    );
    
    keys
}

/// Deploy honeypot keys as environment variables
pub fn deploy_honeypot(
    vault_path: &PathBuf,
    honeypot_keys: &HashMap<String, String>,
) -> FlameVaultResult<()> {
    // Note: Setting environment variables at runtime only affects the current process
    // For system-wide deployment, this would need OS-specific implementation
    for (key, value) in honeypot_keys {
        std::env::set_var(key, value);
        eprintln!("🍯 Honeypot set: {}", key);
    }
    
    // Log honeypot deployment
    let log_entry = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "action": "honeypot_deployed",
        "keys": honeypot_keys.keys().collect::<Vec<_>>(),
        "node": std::env::var("STRAT_DEVICE_ID").unwrap_or_else(|_| "UNKNOWN".to_string()),
        "computer": hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "UNKNOWN".to_string()),
    });
    
    let log_file = vault_path.join("honeypot.log");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .map_err(|e| FlameVaultError::Storage(format!("Failed to open log file: {}", e)))?;
    
    writeln!(file, "{}", log_entry)
        .map_err(|e| FlameVaultError::Storage(format!("Failed to write log: {}", e)))?;
    
    eprintln!("📝 Honeypot deployment logged to {:?}", log_file);
    
    Ok(())
}

/// Log a honeypot access attempt
pub fn log_honeypot_access(vault_path: &PathBuf, key: &str) -> FlameVaultResult<()> {
    let alert = Alert {
        timestamp: chrono::Utc::now().to_rfc3339(),
        alert: "HONEYPOT_ACCESS_ATTEMPT".to_string(),
        key: key.to_string(),
        process: std::env::current_exe()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "UNKNOWN".to_string()),
        user: whoami::username(),
        computer: hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "UNKNOWN".to_string()),
    };
    
    let alert_json = serde_json::to_string(&alert)
        .map_err(|e| FlameVaultError::Storage(format!("Failed to serialize alert: {}", e)))?;
    
    let alert_file = vault_path.join("alerts.log");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&alert_file)
        .map_err(|e| FlameVaultError::Storage(format!("Failed to open alert file: {}", e)))?;
    
    writeln!(file, "{}", alert_json)
        .map_err(|e| FlameVaultError::Storage(format!("Failed to write alert: {}", e)))?;
    
    eprintln!("🚨 HONEYPOT TRIGGERED: {}", key);
    
    Ok(())
}

/// Get recent honeypot alerts
pub fn get_alerts(vault_path: &PathBuf, count: usize) -> FlameVaultResult<Vec<Alert>> {
    let alert_file = vault_path.join("alerts.log");
    
    if !alert_file.exists() {
        return Ok(Vec::new());
    }
    
    let file = std::fs::File::open(&alert_file)?;
    let reader = BufReader::new(file);
    
    let mut alerts = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if let Ok(alert) = serde_json::from_str::<Alert>(&line) {
            alerts.push(alert);
        }
    }
    
    // Return last `count` alerts
    let start = if alerts.len() > count {
        alerts.len() - count
    } else {
        0
    };
    
    Ok(alerts[start..].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_default_honeypot_keys() {
        let keys = get_default_honeypot_keys();
        assert!(keys.contains_key("OPENAI_API_KEY"));
        assert!(keys.contains_key("XAI_API_KEY"));
        assert!(keys.contains_key("EMAIL_PASS"));
    }
    
    #[test]
    fn test_honeypot_logging() {
        let temp_dir = env::temp_dir().join("flamevault_test");
        std::fs::create_dir_all(&temp_dir).unwrap();
        
        let result = log_honeypot_access(&temp_dir, "TEST_KEY");
        assert!(result.is_ok());
        
        let alerts = get_alerts(&temp_dir, 10).unwrap();
        assert!(!alerts.is_empty());
        
        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
