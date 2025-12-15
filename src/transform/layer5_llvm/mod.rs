//! Layer 5: LLVM IR Generation
//! Codons -> LLVM IR bytecode

pub fn transform(codons: &[[u8; 3]]) -> Vec<u8> {
    codons.iter().flat_map(codon_to_opcode).collect()
}

fn codon_to_opcode(codon: &[u8; 3]) -> Vec<u8> {
    // Map codon to LLVM-compatible opcode
    vec![codon[0] << 4 | codon[1] << 2 | codon[2]]
}
