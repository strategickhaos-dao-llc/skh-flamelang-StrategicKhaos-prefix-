//! FlameLang Lexer - Tokenizes source input

pub mod tokens;

pub use tokens::Token;

pub struct Lexer<'a> {
    #[allow(dead_code)]
    source: &'a str,
    #[allow(dead_code)]
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        vec![Token::Eof]
    }
}
