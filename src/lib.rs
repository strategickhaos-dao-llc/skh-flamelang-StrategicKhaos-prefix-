//! # FlameLang v2.0.0
//! 
//! 5-Layer Transformation Pipeline:
//! English → Hebrew → Unicode → Wave → DNA → LLVM IR
//!
//! © 2025 Strategickhaos DAO LLC

pub mod lexer;
pub mod parser;
pub mod transform;
pub mod codegen;
pub mod stdlib;

pub use lexer::{Lexer, Token};
pub use parser::{Parser, AstNode};

/// FlameLang error type
#[derive(Debug, thiserror::Error)]
pub enum FlameError {
    #[error("Lexer error: {0}")]
    Lexer(String),
    #[error("Parser error: {0}")]
    Parser(String),
    #[error("Transform error at layer {layer}: {message}")]
    Transform { layer: u8, message: String },
    #[error("Codegen error: {0}")]
    Codegen(String),
}

pub type FlameResult<T> = Result<T, FlameError>;
