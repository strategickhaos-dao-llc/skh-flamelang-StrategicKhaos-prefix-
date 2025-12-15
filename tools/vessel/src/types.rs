//! Core types for VesselMirror

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Captured webpage data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedPage {
    pub url: String,
    pub html: String,
    pub css: Vec<String>,
    pub scripts: Vec<String>,
    pub assets: HashMap<String, Vec<u8>>,
    pub timestamp: i64,
}

/// Atomic component extracted from page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub component_type: ComponentType,
    pub html: String,
    pub css: String,
    pub js: Option<String>,
    pub metadata: ComponentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentType {
    Form,
    Button,
    Navigation,
    Layout,
    Content,
    Style,
    Script,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetadata {
    pub selectors: Vec<String>,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, String>,
}

/// Analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub structure: StructureAnalysis,
    pub styles: StyleAnalysis,
    pub patterns: Vec<Pattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureAnalysis {
    pub dom_depth: usize,
    pub element_count: usize,
    pub semantic_elements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleAnalysis {
    pub color_palette: Vec<String>,
    pub fonts: Vec<String>,
    pub dimensions: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub name: String,
    pub occurrences: usize,
    pub selector: String,
}

/// Synthesized document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesizedDocument {
    pub sources: Vec<String>,
    pub method: SynthesisMethod,
    pub html: String,
    pub css: String,
    pub js: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SynthesisMethod {
    DaVinci,
    Golden,
    Fibonacci,
}

/// Integrity proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityProof {
    pub hash: String,
    pub algorithm: String,
    pub file_path: String,
    pub timestamp: i64,
    pub chain_attestation: Option<ChainAttestation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainAttestation {
    pub chain: String,
    pub transaction_hash: String,
    pub block_number: u64,
}
