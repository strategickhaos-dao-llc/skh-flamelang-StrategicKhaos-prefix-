//! VesselMirror integration for secret injection into captured pages

use std::collections::HashMap;
use crate::FlameResult;

/// VesselMirror integration
pub struct VesselIntegration {
    vault_dir: std::path::PathBuf,
    secret_mappings: HashMap<String, String>,
}

impl VesselIntegration {
    /// Create a new VesselMirror integration
    pub fn new() -> FlameResult<Self> {
        let vault_dir = super::FlameVault::get_vault_dir()?;
        Ok(Self {
            vault_dir,
            secret_mappings: HashMap::new(),
        })
    }
    
    /// Map a honeypot name to a real secret name
    /// When processing HTML, honeypot references will be replaced with real secret values
    pub fn map_secret(&mut self, honeypot_name: &str, real_secret_name: &str) {
        self.secret_mappings.insert(
            honeypot_name.to_string(),
            real_secret_name.to_string(),
        );
    }
    
    /// Process a VesselMirror capture, replacing honeypot references with real secrets
    pub fn process_vessel_capture(&self, html: &str) -> FlameResult<String> {
        let mut processed = html.to_string();
        
        // Load the vault to access secrets
        let vault = super::FlameVault::new()?;
        
        // Replace each mapped secret
        for (honeypot_name, real_secret_name) in &self.secret_mappings {
            // Get the honeypot value (what's in the HTML)
            if let Ok(honeypot_value) = super::honeypot::get_honeypot_value(&self.vault_dir, honeypot_name) {
                // Get the real secret value
                let real_value = vault.get_secret(real_secret_name)?;
                
                // Replace all occurrences
                processed = processed.replace(&honeypot_value, &real_value);
            }
        }
        
        Ok(processed)
    }
    
    /// Process environment variable references in captured content
    pub fn process_env_references(&self, content: &str) -> FlameResult<String> {
        let mut processed = content.to_string();
        let vault = super::FlameVault::new()?;
        
        // Find patterns like ${VARNAME} or $VARNAME
        // Replace with real secret if mapped
        for (honeypot_name, real_secret_name) in &self.secret_mappings {
            let patterns = vec![
                format!("${{{}}}", honeypot_name),
                format!("${}", honeypot_name),
                format!("process.env.{}", honeypot_name),
                format!("os.environ['{}']", honeypot_name),
                format!("os.environ[\"{}\"]", honeypot_name),
            ];
            
            let real_value = vault.get_secret(real_secret_name)?;
            
            for pattern in patterns {
                processed = processed.replace(&pattern, &real_value);
            }
        }
        
        Ok(processed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vessel_integration() {
        let mut vessel = VesselIntegration::new().unwrap();
        vessel.map_secret("OPENAI_API_KEY", "REAL_OPENAI_KEY");
        
        let html = "<html><body>API Key: sk-fake-key-123</body></html>";
        
        // Note: This test would require actual vault setup to work properly
        // In production, the honeypot value would be replaced with the real secret
    }
    
    #[test]
    fn test_env_reference_processing() {
        let mut vessel = VesselIntegration::new().unwrap();
        vessel.map_secret("API_KEY", "REAL_API_KEY");
        
        let content = "const key = process.env.API_KEY;";
        
        // Note: This test would require actual vault setup to work properly
    }
}
