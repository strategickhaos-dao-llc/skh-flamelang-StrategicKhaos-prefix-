//! 11-Limit Intervals (Partch's Full Diamond)
//! 
//! 11-limit adds prime 11 → exotic, shimmering intervals.
//! Used by Harry Partch for "cloud chamber" sonorities
//! Extended by James Tenney with stochastic microtonality

use std::f64::consts::PI;

/// 11-limit just intonation ratios (Partch's full diamond)
/// Includes all intervals within the 11-limit harmonic space
pub const RATIOS_11_LIMIT: [f64; 18] = [
    1.0,        // 1/1 - Unison
    16.0/15.0,  // 16/15 - Minor semitone
    11.0/10.0,  // 11/10 - Narrow major second (165 cents)
    8.0/7.0,    // 8/7 - Septimal whole tone
    7.0/6.0,    // 7/6 - Septimal minor third
    6.0/5.0,    // 6/5 - Minor third
    11.0/9.0,   // 11/9 - Undecimal neutral third
    5.0/4.0,    // 5/4 - Major third
    4.0/3.0,    // 4/3 - Perfect fourth
    11.0/8.0,   // 11/8 - Undecimal neutral fourth (551 cents) - floating, otherworldly
    7.0/5.0,    // 7/5 - Septimal tritone
    3.0/2.0,    // 3/2 - Perfect fifth
    8.0/5.0,    // 8/5 - Minor sixth
    5.0/3.0,    // 5/3 - Major sixth
    7.0/4.0,    // 7/4 - Harmonic seventh
    11.0/6.0,   // 11/6 - Undecimal neutral seventh
    15.0/8.0,   // 15/8 - Major seventh
    2.0,        // 2/1 - Octave
];

/// Generate carrier frequencies for 11-limit synthesis
/// Base frequency (A) = 110 Hz
pub fn generate_carriers(base_freq: f64, count: usize) -> Vec<f64> {
    RATIOS_11_LIMIT
        .iter()
        .take(count.min(RATIOS_11_LIMIT.len()))
        .map(|&ratio| base_freq * ratio)
        .collect()
}

/// Tenney stochastic micro-detune
/// Adds living "shimmer" to carriers with slight frequency modulation
/// 
/// # Arguments
/// * `carrier` - Base carrier frequency
/// * `t` - Time in seconds
/// * `depth` - Detune depth (default 0.005 = ±0.5%)
/// 
/// # Returns
/// Detuned frequency value
pub fn tenney_detune(carrier: f64, t: f64, depth: f64) -> f64 {
    // Stochastic shimmer using slow LFO
    let detune = (2.0 * PI * 0.3 * t).sin() * depth;
    carrier * (1.0 + detune)
}

/// Generate 11-limit harmonic series chord
/// Returns frequencies for a complete 11-limit chord
pub fn harmonic_series_chord(fundamental: f64, num_harmonics: usize) -> Vec<f64> {
    (1..=num_harmonics)
        .map(|n| fundamental * n as f64)
        .collect()
}

/// Calculate cents from ratio
/// Used for tuning analysis and comparison
pub fn ratio_to_cents(ratio: f64) -> f64 {
    1200.0 * ratio.log2()
}

/// Partch's Tonality Diamond for 11-limit
/// Returns both otonality (over-series) and utonality (under-series)
#[derive(Debug, Clone)]
pub struct ToналityDiamond {
    pub otonality: Vec<f64>,  // Harmonic series (overtones)
    pub utonality: Vec<f64>,  // Subharmonic series (undertones)
}

impl ToналityDiamond {
    /// Create an 11-limit tonality diamond
    pub fn new_11_limit(fundamental: f64) -> Self {
        let primes = vec![1.0, 3.0, 5.0, 7.0, 9.0, 11.0];
        
        // Otonality: fundamental * prime ratios
        let otonality: Vec<f64> = primes
            .iter()
            .map(|&p| fundamental * p)
            .collect();
        
        // Utonality: fundamental / prime ratios
        let utonality: Vec<f64> = primes
            .iter()
            .map(|&p| fundamental / p)
            .collect();
        
        Self { otonality, utonality }
    }
}

/// Tenney's Harmonic Distance
/// Measures perceptual dissonance based on harmonic complexity
pub fn harmonic_distance(ratio_a: f64, ratio_b: f64) -> f64 {
    // Simplified Tenney distance: log of product of numerator and denominator
    let combined = ratio_a * ratio_b;
    combined.log2().abs()
}

/// Spectral CANON generator (inspired by James Tenney)
/// Generates a sequence of 11-limit intervals with rhythmic spacing
#[derive(Debug)]
pub struct SpectralCanon {
    pub intervals: Vec<f64>,
    pub durations: Vec<f64>,
}

impl SpectralCanon {
    /// Create new spectral canon with 11-limit intervals
    pub fn new(base_duration: f64) -> Self {
        let intervals = RATIOS_11_LIMIT.to_vec();
        
        // Generate durations based on harmonic complexity
        let durations: Vec<f64> = intervals
            .iter()
            .map(|&ratio| {
                // More complex ratios get longer durations
                let complexity = ratio_to_cents(ratio) / 1200.0;
                base_duration * (1.0 + complexity * 0.5)
            })
            .collect();
        
        Self { intervals, durations }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_11_limit_ratios() {
        // Test that all ratios are valid and in ascending order
        for i in 1..RATIOS_11_LIMIT.len() {
            assert!(RATIOS_11_LIMIT[i] >= RATIOS_11_LIMIT[i-1]);
        }
        
        // Test that octave is exactly 2.0
        assert_eq!(RATIOS_11_LIMIT[RATIOS_11_LIMIT.len() - 1], 2.0);
    }
    
    #[test]
    fn test_carrier_generation() {
        let carriers = generate_carriers(110.0, 8);
        assert_eq!(carriers.len(), 8);
        assert_eq!(carriers[0], 110.0); // Base frequency
    }
    
    #[test]
    fn test_tenney_detune() {
        let carrier = 440.0;
        let detuned = tenney_detune(carrier, 0.0, 0.005);
        // At t=0, sin(0) = 0, so detune should be 0
        assert!((detuned - carrier).abs() < 0.01);
    }
    
    #[test]
    fn test_ratio_to_cents() {
        // Test octave = 1200 cents
        let cents = ratio_to_cents(2.0);
        assert!((cents - 1200.0).abs() < 0.01);
        
        // Test perfect fifth = ~702 cents
        let fifth_cents = ratio_to_cents(3.0/2.0);
        assert!((fifth_cents - 701.955).abs() < 0.01);
    }
    
    #[test]
    fn test_tonality_diamond() {
        let diamond = ToналityDiamond::new_11_limit(110.0);
        assert_eq!(diamond.otonality.len(), 6);
        assert_eq!(diamond.utonality.len(), 6);
    }
}
