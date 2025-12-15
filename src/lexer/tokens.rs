//! Token definitions

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Glyphs
    Glyph(char),
    HebrewRoot([char; 3]),
    
    // Keywords
    Let, Fn, If, Else, Return,
    
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    
    // Operators
    Plus, Minus, Star, Slash,
    Eq, EqEq, Bang, BangEq,
    
    // Delimiters
    LParen, RParen, LBrace, RBrace,
    Comma, Colon, Semicolon,
    
    // End
    Eof,
}
