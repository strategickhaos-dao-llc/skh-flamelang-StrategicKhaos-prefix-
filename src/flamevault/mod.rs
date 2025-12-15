//! FlameVault — Honeypot + Encrypted Secrets Engine
//! INV-080: Strategickhaos DAO LLC
//!
//! Machine-bound encryption with honeypot trap for exposed credentials.
//! Attackers get bait keys that trigger alerts. Real keys stay encrypted.

mod lib;
pub mod vessel_integration;

pub use lib::{
    EncryptedSecret, FlameVault, HoneypotAlert, HoneypotKey, SecretResult,
};
pub use vessel_integration::VesselIntegration;
