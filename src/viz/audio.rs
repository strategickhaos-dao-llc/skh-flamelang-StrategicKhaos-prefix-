//! Audio Generation Module (AetherLingua Integration)
//!
//! Generates sonic fingerprints for categorical data
//! Maps data values to wave frequencies for audible verification

use super::CategoricalData;

pub struct AudioGenerator {
    sample_rate: u32,
    base_frequency: f64,
}

impl AudioGenerator {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100, // Standard audio sample rate
            base_frequency: 440.0, // A4 note
        }
    }

    /// Render audio representation of categorical data
    /// 
    /// Maps each data entry to a frequency and duration based on its value.
    /// Returns path to generated WAV file (currently simulated).
    pub fn render_audio(&self, data: &CategoricalData) -> Result<Option<String>, String> {
        if data.entries.is_empty() {
            return Ok(None);
        }

        // For now, we'll simulate audio generation by returning a conceptual path
        // In a full implementation, this would:
        // 1. Map each value to a frequency (higher value = higher pitch)
        // 2. Generate sine wave samples for each entry
        // 3. Write to WAV file format
        // 4. Apply ADSR envelope for smooth transitions
        // 5. Add harmonics based on data characteristics

        let total: f64 = data.entries.iter().map(|e| e.value).sum();
        let mut sonic_signature = String::new();

        for entry in &data.entries {
            let relative_freq = entry.value / total;
            let frequency = self.base_frequency * (1.0 + relative_freq * 2.0);
            let duration_ms = (relative_freq * 1000.0) as u32;
            
            sonic_signature.push_str(&format!(
                "{}Hz:{}ms|",
                frequency as u32,
                duration_ms
            ));
        }

        // Generate conceptual WAV path based on sonic signature
        let audio_path = format!("/tmp/flameviz_audio_{}.wav", self.hash_signature(&sonic_signature));
        
        Ok(Some(audio_path))
    }

    /// Generate a simple hash of the sonic signature
    fn hash_signature(&self, signature: &str) -> String {
        // Simple hash for demonstration
        let sum: u32 = signature.bytes().map(|b| b as u32).sum();
        format!("{:08x}", sum)
    }

    /// Map data value to frequency (Layer 3-4 integration)
    /// 
    /// Used by tests and future WAV generation implementation
    pub fn value_to_frequency(&self, value: f64, min_val: f64, max_val: f64) -> f64 {
        // Map value to frequency range (440 Hz to 880 Hz)
        let min_freq = 440.0;
        let max_freq = 880.0;
        
        if max_val <= min_val {
            return self.base_frequency;
        }
        
        let normalized = (value - min_val) / (max_val - min_val);
        min_freq + (normalized * (max_freq - min_freq))
    }

    /// Generate sine wave samples for a given frequency
    /// 
    /// Used by tests and future WAV generation implementation
    pub fn generate_sine_wave(&self, frequency: f64, duration_ms: u32) -> Vec<f32> {
        let num_samples = (self.sample_rate * duration_ms / 1000) as usize;
        let mut samples = Vec::with_capacity(num_samples);
        
        for i in 0..num_samples {
            let t = i as f64 / self.sample_rate as f64;
            let sample = (2.0 * std::f64::consts::PI * frequency * t).sin();
            samples.push(sample as f32);
        }
        
        samples
    }

    /// Apply ADSR envelope to samples
    /// 
    /// Used for future WAV generation implementation
    #[allow(dead_code)]
    pub fn apply_envelope(&self, samples: &mut [f32]) {
        let len = samples.len();
        if len == 0 {
            return;
        }
        
        let attack = len / 10;  // 10% attack
        let _decay = len / 10;   // 10% decay
        let release = len / 5;  // 20% release
        
        // Attack
        for i in 0..attack.min(len) {
            samples[i] *= i as f32 / attack as f32;
        }
        
        // Release
        for i in 0..release.min(len) {
            let idx = len - 1 - i;
            if idx < len {
                samples[idx] *= i as f32 / release as f32;
            }
        }
    }
}

impl Default for AudioGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::viz::DataEntry;

    #[test]
    fn test_audio_generation() {
        let gen = AudioGenerator::new();
        let data = CategoricalData {
            title: "Test".to_string(),
            entries: vec![
                DataEntry { label: "A".to_string(), value: 100.0 },
                DataEntry { label: "B".to_string(), value: 200.0 },
            ],
            unit: None,
        };

        let result = gen.render_audio(&data);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_empty_data_audio() {
        let gen = AudioGenerator::new();
        let data = CategoricalData {
            title: "Empty".to_string(),
            entries: vec![],
            unit: None,
        };

        let result = gen.render_audio(&data);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_value_to_frequency() {
        let gen = AudioGenerator::new();
        let freq = gen.value_to_frequency(50.0, 0.0, 100.0);
        assert!(freq >= 440.0 && freq <= 880.0);
    }

    #[test]
    fn test_sine_wave_generation() {
        let gen = AudioGenerator::new();
        let samples = gen.generate_sine_wave(440.0, 100);
        assert_eq!(samples.len(), (gen.sample_rate / 10) as usize);
    }
}
