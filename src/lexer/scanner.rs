//! Lexer implementation

use super::Token;

pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, position: 0 }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        // TODO: Implement tokenization
        vec![Token::Eof]
    }
}
