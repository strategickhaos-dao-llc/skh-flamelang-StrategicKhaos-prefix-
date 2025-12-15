//! FlameLang Lexer - Tokenizes source input

pub mod tokens;

pub use tokens::Token;

pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, position: 0 }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        vec![Token::Eof]
    }
}
