//! Alert logging system for honeypot access

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use crate::FlameResult;

/// Alert entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub timestamp: DateTime<Utc>,
    pub honeypot_name: String,
    pub process_id: u32,
    pub process_name: String,
    pub user: String,
}

/// Get alerts log path
fn get_alerts_path(vault_dir: &Path) -> std::path::PathBuf {
    vault_dir.join("alerts.log")
}

/// Log a honeypot access attempt
pub fn log_honeypot_access(vault_dir: &Path, honeypot_name: &str) -> FlameResult<()> {
    let alert = Alert {
        timestamp: Utc::now(),
        honeypot_name: honeypot_name.to_string(),
        process_id: std::process::id(),
        process_name: get_process_name(),
        user: get_current_user(),
    };
    
    let path = get_alerts_path(vault_dir);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    
    let alert_json = serde_json::to_string(&alert)?;
    writeln!(file, "{}", alert_json)?;
    
    Ok(())
}

/// Get all alerts
pub fn get_alerts(vault_dir: &Path) -> FlameResult<Vec<Alert>> {
    let path = get_alerts_path(vault_dir);
    
    if !path.exists() {
        return Ok(Vec::new());
    }
    
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut alerts = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        
        match serde_json::from_str::<Alert>(&line) {
            Ok(alert) => alerts.push(alert),
            Err(e) => {
                eprintln!("Warning: Failed to parse alert line: {}", e);
            }
        }
    }
    
    Ok(alerts)
}

/// Get process name
fn get_process_name() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "unknown".to_string())
}

/// Get current user
fn get_current_user() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
}

/// Clear all alerts
pub fn clear_alerts(vault_dir: &Path) -> FlameResult<()> {
    let path = get_alerts_path(vault_dir);
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_alert_logging() {
        let temp_dir = std::env::temp_dir().join("flamevault-alerts-test");
        let _ = fs::create_dir_all(&temp_dir);
        
        // Log an alert
        log_honeypot_access(&temp_dir, "TEST_HONEYPOT").unwrap();
        
        // Get alerts
        let alerts = get_alerts(&temp_dir).unwrap();
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].honeypot_name, "TEST_HONEYPOT");
        
        // Log another alert
        log_honeypot_access(&temp_dir, "ANOTHER_TRAP").unwrap();
        
        let alerts = get_alerts(&temp_dir).unwrap();
        assert_eq!(alerts.len(), 2);
        
        // Clear alerts
        clear_alerts(&temp_dir).unwrap();
        let alerts = get_alerts(&temp_dir).unwrap();
        assert_eq!(alerts.len(), 0);
        
        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
