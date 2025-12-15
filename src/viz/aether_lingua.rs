//! Layer 4: AetherLingua Sonic Rendering
//! 
//! Transform data into wave frequencies for audible bar charts
//! Taller bar = higher pitch (tamper-evident sonic proof)

use crate::{FlameError, FlameResult};
use super::types::{CategoricalData, SonicRepresentation};

/// Base frequency for sonic rendering (A4 = 440 Hz)
const BASE_FREQUENCY: f64 = 440.0;

/// Sample rate for WAV generation (44.1 kHz standard)
const SAMPLE_RATE: u32 = 44100;

/// Duration of each tone in seconds
const TONE_DURATION: f64 = 0.5;

/// Render sonic representation of categorical data
pub fn render_sonic(data: &CategoricalData) -> FlameResult<SonicRepresentation> {
    if data.points.is_empty() {
        return Err(FlameError::Transform {
            layer: 4,
            message: "Cannot render sonic representation of empty data".to_string(),
        });
    }

    // Calculate frequencies for each data point
    // Map values to frequency range: higher value = higher pitch
    let max_value = data.max_value();
    let frequencies: Vec<f64> = data.points
        .iter()
        .map(|point| value_to_frequency(point.value, max_value))
        .collect();

    // Generate WAV data
    let wav_data = generate_wav(&frequencies)?;

    // Compute hash of sonic data
    let hash = compute_sonic_hash(&wav_data);

    Ok(SonicRepresentation::new(wav_data, hash, frequencies))
}

/// Convert a data value to a frequency
/// Maps linearly from BASE_FREQUENCY to 2*BASE_FREQUENCY (one octave)
fn value_to_frequency(value: f64, max_value: f64) -> f64 {
    if max_value == 0.0 {
        return BASE_FREQUENCY;
    }
    
    // Map value to [0, 1] range
    let normalized = value / max_value;
    
    // Map to frequency range [BASE_FREQUENCY, 2*BASE_FREQUENCY]
    BASE_FREQUENCY * (1.0 + normalized)
}

/// Generate simple WAV audio data from frequencies
fn generate_wav(frequencies: &[f64]) -> FlameResult<Vec<u8>> {
    let mut wav_data = Vec::new();
    
    // Simple WAV header (44 bytes)
    wav_data.extend_from_slice(b"RIFF");
    
    let samples_per_tone = (SAMPLE_RATE as f64 * TONE_DURATION) as u32;
    let total_samples = samples_per_tone * frequencies.len() as u32;
    let data_size = total_samples * 2; // 16-bit samples
    let file_size = data_size + 36;
    
    wav_data.extend_from_slice(&file_size.to_le_bytes());
    wav_data.extend_from_slice(b"WAVE");
    wav_data.extend_from_slice(b"fmt ");
    wav_data.extend_from_slice(&16u32.to_le_bytes()); // fmt chunk size
    wav_data.extend_from_slice(&1u16.to_le_bytes());  // PCM format
    wav_data.extend_from_slice(&1u16.to_le_bytes());  // mono
    wav_data.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    wav_data.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes()); // byte rate
    wav_data.extend_from_slice(&2u16.to_le_bytes());  // block align
    wav_data.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    wav_data.extend_from_slice(b"data");
    wav_data.extend_from_slice(&data_size.to_le_bytes());
    
    // Generate audio samples for each frequency
    for frequency in frequencies {
        for i in 0..samples_per_tone {
            let t = i as f64 / SAMPLE_RATE as f64;
            let sample = (2.0 * std::f64::consts::PI * frequency * t).sin();
            let sample_i16 = (sample * 32767.0) as i16;
            wav_data.extend_from_slice(&sample_i16.to_le_bytes());
        }
    }
    
    Ok(wav_data)
}

/// Compute hash of sonic data (simple checksum for now)
fn compute_sonic_hash(wav_data: &[u8]) -> Vec<u8> {
    // Simple hash: take SHA-256 equivalent (we'll use a simple sum for now)
    // In production, this should use actual SHA-256
    let sum: u64 = wav_data.iter().map(|&b| b as u64).sum();
    sum.to_le_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::viz::types::DataPoint;

    #[test]
    fn test_value_to_frequency() {
        let freq1 = value_to_frequency(0.0, 100.0);
        let freq2 = value_to_frequency(100.0, 100.0);
        
        assert_eq!(freq1, BASE_FREQUENCY);
        assert_eq!(freq2, BASE_FREQUENCY * 2.0);
    }

    #[test]
    fn test_render_sonic() {
        let points = vec![
            DataPoint::new("Low".to_string(), 100.0),
            DataPoint::new("High".to_string(), 200.0),
        ];
        let data = CategoricalData::new("Test".to_string(), points);
        let sonic = render_sonic(&data).unwrap();
        
        assert!(!sonic.wav_data.is_empty());
        assert_eq!(sonic.frequencies.len(), 2);
        assert!(sonic.frequencies[1] > sonic.frequencies[0]); // Higher value = higher frequency
    }
}
