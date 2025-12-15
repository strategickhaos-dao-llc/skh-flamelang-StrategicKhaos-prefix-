//! Layer 5: DNA Encoding
//! 
//! Encode data into biological DNA sequences for proof of integrity

/// DNA codon mapping (simplified)
/// Maps byte values to DNA base pairs
const DNA_BASES: [char; 4] = ['A', 'T', 'G', 'C'];

/// Encode data into DNA sequence
/// Each byte is encoded as 4 DNA bases (2 bits per base)
pub fn encode_to_dna(data: &[u8]) -> Vec<u8> {
    let mut dna_sequence = Vec::new();
    
    for &byte in data {
        // Encode each byte as 4 DNA bases
        let bases = byte_to_dna_bases(byte);
        dna_sequence.extend_from_slice(bases.as_bytes());
    }
    
    dna_sequence
}

/// Convert a byte to 4 DNA bases
fn byte_to_dna_bases(byte: u8) -> String {
    let mut result = String::with_capacity(4);
    
    // Extract 2-bit chunks and map to bases
    for i in 0..4 {
        let shift = 6 - (i * 2);
        let bits = (byte >> shift) & 0b11;
        result.push(DNA_BASES[bits as usize]);
    }
    
    result
}

/// Decode DNA sequence back to bytes
pub fn decode_from_dna(dna: &[u8]) -> Option<Vec<u8>> {
    let dna_str = String::from_utf8(dna.to_vec()).ok()?;
    
    if dna_str.len() % 4 != 0 {
        return None;
    }
    
    let mut bytes = Vec::new();
    
    for chunk in dna_str.as_bytes().chunks(4) {
        let byte = dna_bases_to_byte(chunk)?;
        bytes.push(byte);
    }
    
    Some(bytes)
}

/// Convert 4 DNA bases back to a byte
fn dna_bases_to_byte(bases: &[u8]) -> Option<u8> {
    if bases.len() != 4 {
        return None;
    }
    
    let mut byte = 0u8;
    
    for (i, &base) in bases.iter().enumerate() {
        let bits = match base as char {
            'A' => 0b00,
            'T' => 0b01,
            'G' => 0b10,
            'C' => 0b11,
            _ => return None,
        };
        
        let shift = 6 - (i * 2);
        byte |= bits << shift;
    }
    
    Some(byte)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_to_dna() {
        // 0b11001001 = 201
        // 11 00 10 01 = C A G T
        let dna = byte_to_dna_bases(0b11001001);
        assert_eq!(dna, "CAGT");
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let data = b"Hello, DNA!";
        let encoded = encode_to_dna(data);
        let decoded = decode_from_dna(&encoded).unwrap();
        assert_eq!(data, decoded.as_slice());
    }

    #[test]
    fn test_encode_to_dna() {
        let data = &[0xFF, 0x00];
        let dna = encode_to_dna(data);
        let dna_str = String::from_utf8(dna).unwrap();
        
        // 0xFF = 11 11 11 11 = CCCC
        // 0x00 = 00 00 00 00 = AAAA
        assert_eq!(dna_str, "CCCCAAAA");
    }
}
