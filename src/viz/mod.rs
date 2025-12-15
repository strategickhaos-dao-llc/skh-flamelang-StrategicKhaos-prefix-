//! FlameViz — Categorical Data Visualization Engine
//! 
//! 5-Layer Pipeline for Sovereign Visualizations:
//! English → Hebrew → Unicode → Wave → DNA → LLVM IR (SVG/Audio)
//!
//! © 2025 Strategickhaos DAO LLC

pub mod types;
pub mod parser;
pub mod gematria;
pub mod unicode_labels;
pub mod aether_lingua;
pub mod dna_encode;
pub mod svg_render;
pub mod hash;

pub use types::{CategoricalData, DataPoint, Visualization, VisualizationMode};
pub use parser::parse_categorical;
pub use svg_render::render_svg;
pub use hash::compute_hash;

use crate::FlameResult;

/// Main FlameViz struct for the visualization engine
#[derive(Debug)]
pub struct FlameViz {
    mode: VisualizationMode,
}

impl FlameViz {
    /// Create a new FlameViz engine with default settings
    pub fn new() -> Self {
        Self {
            mode: VisualizationMode::Vertical,
        }
    }

    /// Create FlameViz with custom mode
    pub fn with_mode(mode: VisualizationMode) -> Self {
        Self { mode }
    }

    /// Parse categorical data from Zybooks-style text
    pub fn from_zybooks(&self, text: &str) -> FlameResult<Visualization> {
        // Layer 1: Parse categorical data from text
        let data = parser::parse_categorical(text)?;
        
        // Layer 2: Apply gematria ranking (optional, for sacred ordering)
        let ordered = gematria::rank_data(data);
        
        // Layer 3: Add Unicode glyphs for labels
        let labeled = unicode_labels::add_glyphs(ordered);
        
        // Layer 4: Generate sonic representation (AetherLingua)
        let sonic = aether_lingua::render_sonic(&labeled)?;
        
        // Layer 5: DNA encoding and commitment
        let dna_proof = dna_encode::encode_to_dna(&sonic.hash);
        
        // Render final visualization
        let svg = svg_render::render_svg(&labeled, self.mode)?;
        let hash = hash::compute_hash(&svg);
        
        let on_chain_payload = self.prepare_payload(&dna_proof);
        
        Ok(Visualization {
            svg,
            audio: sonic.wav_data,
            hash,
            dna_commitment: dna_proof,
            on_chain_payload,
        })
    }

    /// Prepare on-chain commitment payload
    fn prepare_payload(&self, dna_proof: &[u8]) -> Vec<u8> {
        // Format: version(1) + timestamp(8) + dna_proof(variable)
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut payload = Vec::new();
        payload.push(1u8); // Version 1
        payload.extend_from_slice(&timestamp.to_le_bytes());
        payload.extend_from_slice(dna_proof);
        payload
    }
}

impl Default for FlameViz {
    fn default() -> Self {
        Self::new()
    }
}
