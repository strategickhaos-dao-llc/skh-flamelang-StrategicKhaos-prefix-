//! FlameLang Lexer
//!
//! Tokenization for FlameLang v2.0.0 using logos.

use logos::Logos;
use std::fmt;

/// Token types for FlameLang
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"//[^\n]*")]
pub enum Token {
    // Keywords
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("impl")]
    Impl,
    #[token("trait")]
    Trait,
    #[token("mod")]
    Mod,
    #[token("use")]
    Use,
    #[token("pub")]
    Pub,

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("!=")]
    Ne,
    #[token("<")]
    Lt,
    #[token("<=")]
    Le,
    #[token(">")]
    Gt,
    #[token(">=")]
    Ge,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,

    // Delimiters
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token("->")]
    Arrow,
    #[token(".")]
    Dot,

    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),
    
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().ok())]
    Float(f64),
    
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    String(String),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Fn => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::Mut => write!(f, "mut"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::While => write!(f, "while"),
            Token::For => write!(f, "for"),
            Token::Return => write!(f, "return"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Struct => write!(f, "struct"),
            Token::Enum => write!(f, "enum"),
            Token::Impl => write!(f, "impl"),
            Token::Trait => write!(f, "trait"),
            Token::Mod => write!(f, "mod"),
            Token::Use => write!(f, "use"),
            Token::Pub => write!(f, "pub"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Assign => write!(f, "="),
            Token::Eq => write!(f, "=="),
            Token::Ne => write!(f, "!="),
            Token::Lt => write!(f, "<"),
            Token::Le => write!(f, "<="),
            Token::Gt => write!(f, ">"),
            Token::Ge => write!(f, ">="),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::Not => write!(f, "!"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::DoubleColon => write!(f, "::"),
            Token::Arrow => write!(f, "->"),
            Token::Dot => write!(f, "."),
            Token::Integer(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, r#""{}""#, s),
            Token::Identifier(s) => write!(f, "{}", s),
        }
    }
}

/// Lexer for FlameLang
pub struct Lexer<'source> {
    inner: logos::Lexer<'source, Token>,
}

impl<'source> Lexer<'source> {
    /// Create a new lexer for the given source code
    pub fn new(source: &'source str) -> Self {
        Self {
            inner: Token::lexer(source),
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|result| {
            result.map_err(|_| LexError {
                span: self.inner.span(),
            })
        })
    }
}

/// Lexer error
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("Lexical error at {span:?}")]
pub struct LexError {
    pub span: std::ops::Range<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let source = "fn let mut if else";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next(), Some(Ok(Token::Fn)));
        assert_eq!(lexer.next(), Some(Ok(Token::Let)));
        assert_eq!(lexer.next(), Some(Ok(Token::Mut)));
        assert_eq!(lexer.next(), Some(Ok(Token::If)));
        assert_eq!(lexer.next(), Some(Ok(Token::Else)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_identifiers() {
        let source = "foo bar_baz my_var123";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("foo".to_string()))));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("bar_baz".to_string()))));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("my_var123".to_string()))));
    }

    #[test]
    fn test_numbers() {
        let source = "42 3.14";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next(), Some(Ok(Token::Integer(42))));
        assert_eq!(lexer.next(), Some(Ok(Token::Float(3.14))));
    }

    #[test]
    fn test_string_literal() {
        let source = r#""hello world""#;
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next(), Some(Ok(Token::String("hello world".to_string()))));
    }

    #[test]
    fn test_operators() {
        let source = "+ - * / == !=";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next(), Some(Ok(Token::Plus)));
        assert_eq!(lexer.next(), Some(Ok(Token::Minus)));
        assert_eq!(lexer.next(), Some(Ok(Token::Star)));
        assert_eq!(lexer.next(), Some(Ok(Token::Slash)));
        assert_eq!(lexer.next(), Some(Ok(Token::Eq)));
        assert_eq!(lexer.next(), Some(Ok(Token::Ne)));
    }
}
