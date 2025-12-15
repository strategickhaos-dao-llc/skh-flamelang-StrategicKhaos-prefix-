//! Core data types for FlameViz

/// Represents a single data point in categorical data
#[derive(Debug, Clone)]
pub struct DataPoint {
    pub label: String,
    pub value: f64,
    pub glyph: Option<char>,
}

impl DataPoint {
    pub fn new(label: String, value: f64) -> Self {
        Self {
            label,
            value,
            glyph: None,
        }
    }

    pub fn with_glyph(mut self, glyph: char) -> Self {
        self.glyph = Some(glyph);
        self
    }
}

/// Collection of categorical data points
#[derive(Debug, Clone)]
pub struct CategoricalData {
    pub title: String,
    pub points: Vec<DataPoint>,
    pub is_relative: bool, // True if values are percentages/relative frequencies
}

impl CategoricalData {
    pub fn new(title: String, points: Vec<DataPoint>) -> Self {
        Self {
            title,
            points,
            is_relative: false,
        }
    }

    pub fn with_relative(mut self, is_relative: bool) -> Self {
        self.is_relative = is_relative;
        self
    }

    /// Get the maximum value in the dataset
    pub fn max_value(&self) -> f64 {
        self.points
            .iter()
            .map(|p| p.value)
            .fold(0.0, f64::max)
    }

    /// Get the total sum of all values
    pub fn sum(&self) -> f64 {
        self.points.iter().map(|p| p.value).sum()
    }
}

/// Visualization mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualizationMode {
    Vertical,       // Traditional vertical bar chart
    Horizontal,     // Horizontal bar chart (for long labels)
    Grouped,        // Grouped bars (for nested categories)
    Stacked,        // Stacked bars
}

/// Complete visualization output
#[derive(Debug, Clone)]
pub struct Visualization {
    pub svg: String,
    pub audio: Vec<u8>,
    pub hash: String,
    pub dna_commitment: Vec<u8>,
    pub on_chain_payload: Vec<u8>,
}

impl Visualization {
    /// Verify the visualization integrity by recomputing hash
    pub fn verify(&self) -> bool {
        let computed = crate::viz::hash::compute_hash(&self.svg);
        computed == self.hash
    }
}

/// Sonic representation from AetherLingua
#[derive(Debug, Clone)]
pub struct SonicRepresentation {
    pub wav_data: Vec<u8>,
    pub hash: Vec<u8>,
    pub frequencies: Vec<f64>,
}

impl SonicRepresentation {
    pub fn new(wav_data: Vec<u8>, hash: Vec<u8>, frequencies: Vec<f64>) -> Self {
        Self {
            wav_data,
            hash,
            frequencies,
        }
    }
}
