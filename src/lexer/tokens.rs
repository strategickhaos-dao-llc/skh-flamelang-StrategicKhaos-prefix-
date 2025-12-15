//! Token definitions for FlameLang

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Glyphs (Layer 1)
    Glyph(char),
    HebrewRoot([char; 3]),
    
    // Keywords
    Let, Fn, If, Else, Loop, Return,
    
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    
    // Operators
    Plus, Minus, Star, Slash, Percent,
    Eq, EqEq, Bang, BangEq,
    Lt, LtEq, Gt, GtEq,
    
    // Delimiters
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Comma, Colon, Semicolon, Arrow,
    
    // Special
    Eof,
    Error(String),
}
