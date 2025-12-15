//! Layer 5: LLVM IR Generation
//! Codons -> LLVM IR bytecode

pub fn transform(codons: &[[u8; 3]]) -> Vec<u8> {
    codons.iter().flat_map(|c| codon_to_opcode(c)).collect()
}

fn codon_to_opcode(codon: &[u8; 3]) -> Vec<u8> {
    // Map codon to LLVM-compatible opcode
    vec![codon[0] << 4 | codon[1] << 2 | codon[2]]
}
