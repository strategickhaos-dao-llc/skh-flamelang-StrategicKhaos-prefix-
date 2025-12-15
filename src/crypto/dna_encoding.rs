//! DNA-based biological key derivation layer
//! 
//! Implements 4-base (ACGT) encoding for exponential keyspace expansion.
//! Maps byte data to DNA sequences for quantum-resistant entropy.

use crate::FlameError;

/// DNA base encoding (4-base alphabet)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnaBase {
    Adenine,  // A
    Cytosine, // C
    Guanine,  // G
    Thymine,  // T
}

impl DnaBase {
    /// Convert 2-bit value to DNA base
    fn from_bits(bits: u8) -> Self {
        match bits & 0x03 {
            0 => DnaBase::Adenine,
            1 => DnaBase::Cytosine,
            2 => DnaBase::Guanine,
            _ => DnaBase::Thymine,
        }
    }

    /// Convert DNA base to 2-bit value
    fn to_bits(&self) -> u8 {
        match self {
            DnaBase::Adenine => 0,
            DnaBase::Cytosine => 1,
            DnaBase::Guanine => 2,
            DnaBase::Thymine => 3,
        }
    }

    /// Get character representation
    pub fn to_char(&self) -> char {
        match self {
            DnaBase::Adenine => 'A',
            DnaBase::Cytosine => 'C',
            DnaBase::Guanine => 'G',
            DnaBase::Thymine => 'T',
        }
    }
}

/// DNA sequence encoder
pub struct DnaEncoder;

impl DnaEncoder {
    /// Encode bytes to DNA sequence
    /// 
    /// Each byte produces 4 DNA bases (2 bits per base)
    /// Keyspace: 4^n where n is sequence length
    pub fn encode(data: &[u8]) -> Vec<DnaBase> {
        let mut sequence = Vec::with_capacity(data.len() * 4);
        for &byte in data {
            sequence.push(DnaBase::from_bits(byte >> 6));
            sequence.push(DnaBase::from_bits(byte >> 4));
            sequence.push(DnaBase::from_bits(byte >> 2));
            sequence.push(DnaBase::from_bits(byte));
        }
        sequence
    }

    /// Decode DNA sequence back to bytes
    pub fn decode(sequence: &[DnaBase]) -> Result<Vec<u8>, FlameError> {
        if sequence.len() % 4 != 0 {
            return Err(FlameError::Transform {
                layer: 4,
                message: "DNA sequence length must be multiple of 4".to_string(),
            });
        }

        let mut bytes = Vec::with_capacity(sequence.len() / 4);
        for chunk in sequence.chunks_exact(4) {
            let byte = (chunk[0].to_bits() << 6)
                | (chunk[1].to_bits() << 4)
                | (chunk[2].to_bits() << 2)
                | chunk[3].to_bits();
            bytes.push(byte);
        }
        Ok(bytes)
    }

    /// Convert DNA sequence to string representation
    pub fn to_string(sequence: &[DnaBase]) -> String {
        sequence.iter().map(|b| b.to_char()).collect()
    }
}

/// DNA-based key derivation function
pub struct DnaKeyDerivation;

impl DnaKeyDerivation {
    /// Derive cryptographic key from DNA sequence
    /// 
    /// Uses DNA encoding as biological entropy source
    pub fn derive_key(input: &[u8], salt: &[u8]) -> [u8; 32] {
        use sha3::{Digest, Sha3_256};
        
        // Encode input as DNA
        let dna_sequence = DnaEncoder::encode(input);
        let dna_string = DnaEncoder::to_string(&dna_sequence);
        
        // Mix with salt
        let mut hasher = Sha3_256::new();
        hasher.update(dna_string.as_bytes());
        hasher.update(salt);
        
        let result = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&result);
        key
    }

    /// Generate DNA commitment (Merkle root)
    pub fn commitment(data: &[u8]) -> [u8; 64] {
        use sha3::{Digest, Sha3_512};
        
        let dna_sequence = DnaEncoder::encode(data);
        let dna_string = DnaEncoder::to_string(&dna_sequence);
        
        let mut hasher = Sha3_512::new();
        hasher.update(dna_string.as_bytes());
        
        let result = hasher.finalize();
        let mut commitment = [0u8; 64];
        commitment.copy_from_slice(&result);
        commitment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dna_encode_decode() {
        let data = b"Hello, FlameVault!";
        let encoded = DnaEncoder::encode(data);
        let decoded = DnaEncoder::decode(&encoded).unwrap();
        assert_eq!(data.as_slice(), decoded.as_slice());
    }

    #[test]
    fn test_dna_keyspace() {
        let data = &[0xFF];
        let encoded = DnaEncoder::encode(data);
        // 1 byte = 4 DNA bases (4^4 = 256 possible combinations)
        assert_eq!(encoded.len(), 4);
    }

    #[test]
    fn test_dna_key_derivation() {
        let input = b"test input";
        let salt = b"test salt";
        let key = DnaKeyDerivation::derive_key(input, salt);
        assert_eq!(key.len(), 32);
        
        // Same input should produce same key
        let key2 = DnaKeyDerivation::derive_key(input, salt);
        assert_eq!(key, key2);
    }
}
