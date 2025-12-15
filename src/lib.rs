//! FlameLang v2.0.0 Compiler Toolchain
//!
//! A sovereign compiler toolchain implementing a 5-layer transformation pipeline:
//! English → Hebrew → Unicode → Wave → DNA → LLVM

pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod stdlib;
pub mod transform;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
