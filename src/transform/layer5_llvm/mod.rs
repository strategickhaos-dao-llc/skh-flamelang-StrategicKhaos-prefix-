//! Layer 5: LLVM IR Generation
//! Codons -> LLVM IR bytecode

const HIGH_BITS_SHIFT: u8 = 4; // Shift for high-order codon bits
const MID_BITS_SHIFT: u8 = 2; // Shift for middle-order codon bits

pub fn transform(codons: &[[u8; 3]]) -> Vec<u8> {
    codons.iter().flat_map(codon_to_opcode).collect()
}

fn codon_to_opcode(codon: &[u8; 3]) -> Vec<u8> {
    // Map codon to LLVM-compatible opcode
    vec![codon[0] << HIGH_BITS_SHIFT | codon[1] << MID_BITS_SHIFT | codon[2]]
}
