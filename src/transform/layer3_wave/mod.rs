//! Layer 3: Wave Transformation
//! Gematria -> Frequency (c = 2*pi*r)

use std::f64::consts::PI;

const BASE_FREQ: f64 = 440.0; // A4
const MODULO_RANGE: f64 = 1000.0; // Modulo range for frequency normalization

pub fn transform(values: &[u64]) -> Vec<f64> {
    values.iter().map(|v| to_frequency(*v)).collect()
}

fn to_frequency(value: u64) -> f64 {
    let r = value as f64;
    let circumference = 2.0 * PI * r;
    BASE_FREQ * (1.0 + (circumference % MODULO_RANGE) / MODULO_RANGE)
}
