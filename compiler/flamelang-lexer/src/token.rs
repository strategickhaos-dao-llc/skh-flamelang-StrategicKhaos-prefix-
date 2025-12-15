/// Token types for FlameLang v2.0.0
/// Supports multi-lingual representation (English, Hebrew, Unicode)
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
    pub hebrew_form: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    // Keywords
    Fn,          // function / פונקציה
    Let,         // let / תן
    Const,       // const / קבוע
    Mut,         // mut / משתנה
    If,          // if / אם
    Else,        // else / אחרת
    While,       // while / בזמן
    For,         // for / עבור
    Return,      // return / החזר
    Break,       // break / שבור
    Continue,    // continue / המשך
    Struct,      // struct / מבנה
    Enum,        // enum / מנייה
    Impl,        // impl / יישום
    Trait,       // trait / תכונה
    Type,        // type / סוג
    Module,      // module / מודול
    Use,         // use / השתמש
    Pub,         // pub / ציבורי
    
    // Quantum primitives
    Quantum,     // quantum / קוונטי
    Superpose,   // superpose / על־מצב
    Entangle,    // entangle / שזר
    Measure,     // measure / מדוד
    
    // Wave primitives
    Wave,        // wave / גל
    Frequency,   // frequency / תדר
    Amplitude,   // amplitude / משרעת
    Phase,       // phase / פאזה
    
    // DNA primitives
    DNA,         // dna / דנ״א
    Encode,      // encode / קודד
    Decode,      // decode / פענח
    Sequence,    // sequence / רצף
    
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Boolean(bool),
    
    // Identifiers
    Identifier(String),
    
    // Operators
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Percent,     // %
    Caret,       // ^
    
    Eq,          // ==
    Ne,          // !=
    Lt,          // <
    Le,          // <=
    Gt,          // >
    Ge,          // >=
    
    And,         // &&
    Or,          // ||
    Not,         // !
    
    Assign,      // =
    PlusAssign,  // +=
    MinusAssign, // -=
    
    // Delimiters
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]
    
    Comma,       // ,
    Dot,         // .
    Semicolon,   // ;
    Colon,       // :
    Arrow,       // ->
    FatArrow,    // =>
    
    // Special
    Eof,
    Whitespace,
    Comment,
    Unknown,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Integer(n) => write!(f, "{}", n),
            TokenKind::Float(n) => write!(f, "{}", n),
            TokenKind::String(s) => write!(f, "\"{}\"", s),
            TokenKind::Char(c) => write!(f, "'{}'", c),
            TokenKind::Boolean(b) => write!(f, "{}", b),
            TokenKind::Identifier(id) => write!(f, "{}", id),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, span: Span) -> Self {
        Self {
            kind,
            lexeme,
            span,
            hebrew_form: None,
        }
    }
    
    pub fn with_hebrew(mut self, hebrew: String) -> Self {
        self.hebrew_form = Some(hebrew);
        self
    }
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
}
