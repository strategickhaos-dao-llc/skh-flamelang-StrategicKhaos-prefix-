//! Vessel Integration Module for FlameVault
//! Provides integration with Strategickhaos Vessel system

use super::FlameVault;

/// Integration layer for Vessel system
pub struct VesselIntegration;

impl VesselIntegration {
    /// Create a new Vessel integration
    pub fn new() -> Self {
        Self
    }
    
    /// Initialize FlameVault for Vessel use
    pub fn init_vault() -> Result<FlameVault, Box<dyn std::error::Error>> {
        FlameVault::new()
    }
    
    /// Get vault configuration for Vessel
    pub fn get_vault_config(vault: &FlameVault) -> String {
        vault.status()
    }
}

impl Default for VesselIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vessel_integration() {
        let _integration = VesselIntegration::new();
        assert!(VesselIntegration::init_vault().is_ok());
    }
}
