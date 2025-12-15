//! # FlameViz v1.0 - Categorical Visualization Engine
//!
//! Transforms categorical data through the 5-layer FlameLang pipeline into
//! provable, multi-modal visualizations with quantum-resistant provenance.
//!
//! ## Pipeline
//! - Layer 1-2: Parse English/Hebrew intent → structured data
//! - Layer 3: Unicode glyphs for labels
//! - Layer 4: Wave frequencies → audible chart (AetherLingua integration)
//! - Layer 5: Generate SVG + natural language explanation
//! - Output: SVG chart, WAV audio, SHA-256 hash, on-chain JSON payload

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use chrono::Utc;
use std::collections::HashMap;

mod chart;
mod parser;
mod audio;

pub use chart::ChartGenerator;
pub use parser::TextParser;
pub use audio::AudioGenerator;

/// Categorical data entry (label, value)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEntry {
    pub label: String,
    pub value: f64,
}

/// Categorical dataset with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoricalData {
    pub title: String,
    pub entries: Vec<DataEntry>,
    pub unit: Option<String>,
}

/// Complete visualization output with multi-modal representations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visualization {
    /// SVG chart representation
    pub svg: String,
    /// Audio WAV file path (sonic fingerprint)
    pub audio_wav: Option<String>,
    /// SHA-256 hash for provenance
    pub hash: String,
    /// Natural language explanation
    pub explanation: String,
    /// On-chain commitment payload
    pub on_chain_payload: String,
}

/// FlameViz engine - main interface for visualization generation
pub struct FlameViz {
    parser: TextParser,
    chart_gen: ChartGenerator,
    audio_gen: AudioGenerator,
}

impl FlameViz {
    /// Create a new FlameViz instance
    pub fn new() -> Self {
        Self {
            parser: TextParser::new(),
            chart_gen: ChartGenerator::new(),
            audio_gen: AudioGenerator::new(),
        }
    }

    /// Generate visualization from text description
    /// 
    /// # Example
    /// ```
    /// let flameviz = FlameViz::new();
    /// let viz = flameviz.from_text("Walmart Stores: 2300000, Amazon: 566000, Yum! Brands: 450000, Kroger: 435000")?;
    /// ```
    pub fn from_text(&self, text: &str) -> crate::FlameResult<Visualization> {
        // Layer 1-2: Parse categorical data from text
        let data = self.parser.parse_categorical(text)
            .map_err(|e| crate::FlameError::Transform { 
                layer: 1, 
                message: format!("Failed to parse categorical data: {}", e) 
            })?;

        // Layer 3-5: Generate multi-modal output
        self.generate_visualization(&data)
    }

    /// Generate visualization from structured categorical data
    pub fn from_data(&self, data: &CategoricalData) -> crate::FlameResult<Visualization> {
        self.generate_visualization(data)
    }

    /// Core visualization generation pipeline
    fn generate_visualization(&self, data: &CategoricalData) -> crate::FlameResult<Visualization> {
        // Generate SVG bar chart (relative frequency)
        let svg = self.chart_gen.generate_bar_chart(data)
            .map_err(|e| crate::FlameError::Transform { 
                layer: 5, 
                message: format!("SVG generation failed: {}", e) 
            })?;

        // Generate sonic fingerprint (Layer 3-4: Wave frequencies)
        let audio_path = self.audio_gen.render_audio(data)
            .map_err(|e| crate::FlameError::Transform { 
                layer: 4, 
                message: format!("Audio generation failed: {}", e) 
            })?;

        // Compute SHA-256 hash for provenance
        // Note: For complete integrity, we hash the SVG and audio signature
        // In production, this would hash the actual audio bytes
        let mut hasher = Sha256::new();
        hasher.update(svg.as_bytes());
        if let Some(ref path) = audio_path {
            // Hash the path as a proxy for audio content
            // TODO: Hash actual audio bytes when real WAV generation is implemented
            hasher.update(path.as_bytes());
        }
        let hash_bytes = hasher.finalize();
        let hash = format!("{:x}", hash_bytes);

        // Generate natural language explanation
        let explanation = self.generate_explanation(data);

        // Prepare on-chain payload
        let on_chain_payload = self.prepare_payload(&hash, data)?;

        Ok(Visualization {
            svg,
            audio_wav: audio_path,
            hash,
            explanation,
            on_chain_payload,
        })
    }

    /// Generate natural language explanation of the chart
    fn generate_explanation(&self, data: &CategoricalData) -> String {
        if data.entries.is_empty() {
            return "Empty dataset - no data to visualize.".to_string();
        }

        let total: f64 = data.entries.iter().map(|e| e.value).sum();
        
        let mut explanation = format!("The bar chart shows {}. ", data.title);
        
        // Find the leading entry
        if let Some(max_entry) = data.entries.iter().max_by(|a, b| {
            a.value.partial_cmp(&b.value).unwrap_or(std::cmp::Ordering::Equal)
        }) {
            let percentage = (max_entry.value / total) * 100.0;
            explanation.push_str(&format!(
                "{} leads with ~{:.0}% of total", 
                max_entry.label, 
                percentage
            ));

            // Add other entries
            if data.entries.len() > 1 {
                explanation.push_str(", followed by ");
                let others: Vec<String> = data.entries.iter()
                    .filter(|e| e.label != max_entry.label)
                    .map(|e| {
                        let pct = (e.value / total) * 100.0;
                        format!("{} ({:.0}%)", e.label, pct)
                    })
                    .collect();
                explanation.push_str(&others.join(", "));
            }

            explanation.push_str(". Relative frequency highlights proportional contribution — ideal for comparing shares without raw numbers dominating perception.");
        }

        explanation
    }

    /// Prepare JSON payload for on-chain commitment
    fn prepare_payload(&self, hash: &str, data: &CategoricalData) -> crate::FlameResult<String> {
        let mut payload = HashMap::new();
        payload.insert("type", "flameviz_bar_relative".to_string());
        payload.insert("sonic_hash", hash.to_string());
        payload.insert("timestamp", Utc::now().to_rfc3339());
        payload.insert("data_entries", data.entries.len().to_string());
        
        // Add trigger detection based on data patterns
        // Check for 7% threshold (charity gliss trigger)
        let total: f64 = data.entries.iter().map(|e| e.value).sum();
        for entry in &data.entries {
            let percentage = (entry.value / total) * 100.0;
            if (percentage - 7.0).abs() < 0.5 {
                payload.insert("triggers", "charity_gliss".to_string());
                break;
            }
        }

        serde_json::to_string_pretty(&payload)
            .map_err(|e| crate::FlameError::Transform { 
                layer: 5, 
                message: format!("Failed to serialize payload: {}", e) 
            })
    }
}

impl Default for FlameViz {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flameviz_creation() {
        let _flameviz = FlameViz::new();
        assert!(true, "FlameViz instance created successfully");
    }

    #[test]
    fn test_categorical_data_creation() {
        let data = CategoricalData {
            title: "Test Data".to_string(),
            entries: vec![
                DataEntry { label: "A".to_string(), value: 100.0 },
                DataEntry { label: "B".to_string(), value: 200.0 },
            ],
            unit: Some("units".to_string()),
        };
        assert_eq!(data.entries.len(), 2);
    }

    #[test]
    fn test_explanation_generation() {
        let flameviz = FlameViz::new();
        let data = CategoricalData {
            title: "Employer Data".to_string(),
            entries: vec![
                DataEntry { label: "Walmart".to_string(), value: 2300000.0 },
                DataEntry { label: "Amazon".to_string(), value: 566000.0 },
            ],
            unit: Some("employees".to_string()),
        };
        
        let explanation = flameviz.generate_explanation(&data);
        assert!(explanation.contains("Walmart"));
        assert!(explanation.contains("leads"));
    }
}
