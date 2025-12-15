//! FlameLang 5-Layer Transformation Pipeline
//!
//! - Layer 1: Linguistic (English → Hebrew)
//! - Layer 2: Numeric (Unicode → Decimal)
//! - Layer 3: Wave (c=2πr → Hz)
//! - Layer 4: DNA (Freq → Codon)
//! - Layer 5: LLVM IR generation

pub mod layer1_linguistic;
pub mod layer2_numeric;
pub mod layer3_wave;
pub mod layer4_dna;
pub mod layer5_llvm;
