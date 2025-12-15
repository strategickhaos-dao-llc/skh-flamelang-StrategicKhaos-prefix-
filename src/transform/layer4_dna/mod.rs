//! Layer 4: DNA Transformation
//! Frequency -> Codon (64-codon ISA)

pub fn transform(frequencies: &[f64]) -> Vec<[u8; 3]> {
    frequencies.iter().map(|f| to_codon(*f)).collect()
}

fn to_codon(freq: f64) -> [u8; 3] {
    let index = ((freq * 100.0) as u64 % 64) as u8;
    [(index >> 4) & 0b11, (index >> 2) & 0b11, index & 0b11]
}
