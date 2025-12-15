//! # AetherViz - Self-Referential Visualization & Sonification Core (INV-078)
//!
//! Visualizes and sonifies the repository's own source code tree,
//! turning the repository structure into:
//! - Interactive graph SVG (Obsidian-style local graph)
//! - Sonic representation (directory depth = pitch, file type = timbre)
//! - DNA-encoded hash of the tree
//! - On-chain commitment of the "brain state"

pub mod aetherviz;
pub mod aetherlingua;

pub use aetherviz::{AetherViz, Visualization, RepoTree};
pub use aetherlingua::AetherLingua;
