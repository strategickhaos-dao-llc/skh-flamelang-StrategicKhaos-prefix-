//! Transform pipeline modules

pub mod layer1_linguistic;
pub mod layer2_numeric;
pub mod layer3_wave;
pub mod layer4_dna;
pub mod layer5_llvm;

#[derive(Debug, Clone)]
pub struct LinguisticOutput;

#[derive(Debug, Clone)]
pub struct NumericOutput;

#[derive(Debug, Clone)]
pub struct WaveOutput;

#[derive(Debug, Clone)]
pub struct DnaOutput;

#[derive(Debug, Clone)]
pub struct LlvmOutput;
