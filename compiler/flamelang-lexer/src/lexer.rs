/// Lexer for FlameLang v2.0.0
/// Implements English→Hebrew linguistic transformation as first layer
use crate::token::{Token, TokenKind, Span};
use logos::Logos;
use std::collections::HashMap;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
enum LogosToken {
    // Keywords (English)
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("const")]
    Const,
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
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("impl")]
    Impl,
    #[token("trait")]
    Trait,
    #[token("type")]
    Type,
    #[token("module")]
    Module,
    #[token("use")]
    Use,
    #[token("pub")]
    Pub,
    
    // Quantum primitives
    #[token("quantum")]
    Quantum,
    #[token("superpose")]
    Superpose,
    #[token("entangle")]
    Entangle,
    #[token("measure")]
    Measure,
    
    // Wave primitives
    #[token("wave")]
    Wave,
    #[token("frequency")]
    Frequency,
    #[token("amplitude")]
    Amplitude,
    #[token("phase")]
    Phase,
    
    // DNA primitives
    #[token("dna")]
    DNA,
    #[token("encode")]
    Encode,
    #[token("decode")]
    Decode,
    #[token("sequence")]
    Sequence,
    
    // Boolean literals
    #[token("true")]
    True,
    #[token("false")]
    False,
    
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
    #[token("^")]
    Caret,
    
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
    
    #[token("=")]
    Assign,
    #[token("+=")]
    PlusAssign,
    #[token("-=")]
    MinusAssign,
    
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
    #[token(".")]
    Dot,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,
    
    // Literals and identifiers
    #[regex(r"[0-9]+", priority = 3)]
    Integer,
    
    #[regex(r"[0-9]+\.[0-9]+")]
    Float,
    
    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#)]
    String,
    
    #[regex(r"'([^'\\]|\\['\\bnfrt])'")]
    Char,
    
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 2)]
    Identifier,
    
    // Comments
    #[regex(r"//[^\n]*")]
    LineComment,
    
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    english_to_hebrew: HashMap<String, String>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut english_to_hebrew = HashMap::new();
        
        // Build English→Hebrew mapping for linguistic transformation
        english_to_hebrew.insert("fn".to_string(), "פונקציה".to_string());
        english_to_hebrew.insert("let".to_string(), "תן".to_string());
        english_to_hebrew.insert("const".to_string(), "קבוע".to_string());
        english_to_hebrew.insert("mut".to_string(), "משתנה".to_string());
        english_to_hebrew.insert("if".to_string(), "אם".to_string());
        english_to_hebrew.insert("else".to_string(), "אחרת".to_string());
        english_to_hebrew.insert("while".to_string(), "בזמן".to_string());
        english_to_hebrew.insert("for".to_string(), "עבור".to_string());
        english_to_hebrew.insert("return".to_string(), "החזר".to_string());
        english_to_hebrew.insert("break".to_string(), "שבור".to_string());
        english_to_hebrew.insert("continue".to_string(), "המשך".to_string());
        english_to_hebrew.insert("struct".to_string(), "מבנה".to_string());
        english_to_hebrew.insert("enum".to_string(), "מנייה".to_string());
        english_to_hebrew.insert("impl".to_string(), "יישום".to_string());
        english_to_hebrew.insert("trait".to_string(), "תכונה".to_string());
        english_to_hebrew.insert("type".to_string(), "סוג".to_string());
        english_to_hebrew.insert("module".to_string(), "מודול".to_string());
        english_to_hebrew.insert("use".to_string(), "השתמש".to_string());
        english_to_hebrew.insert("pub".to_string(), "ציבורי".to_string());
        english_to_hebrew.insert("quantum".to_string(), "קוונטי".to_string());
        english_to_hebrew.insert("superpose".to_string(), "על־מצב".to_string());
        english_to_hebrew.insert("entangle".to_string(), "שזר".to_string());
        english_to_hebrew.insert("measure".to_string(), "מדוד".to_string());
        english_to_hebrew.insert("wave".to_string(), "גל".to_string());
        english_to_hebrew.insert("frequency".to_string(), "תדר".to_string());
        english_to_hebrew.insert("amplitude".to_string(), "משרעת".to_string());
        english_to_hebrew.insert("phase".to_string(), "פאזה".to_string());
        english_to_hebrew.insert("dna".to_string(), "דנ״א".to_string());
        english_to_hebrew.insert("encode".to_string(), "קודד".to_string());
        english_to_hebrew.insert("decode".to_string(), "פענח".to_string());
        english_to_hebrew.insert("sequence".to_string(), "רצף".to_string());
        
        Self {
            source,
            tokens: Vec::new(),
            english_to_hebrew,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut lex = LogosToken::lexer(&self.source);
        let mut line = 1;
        let mut column = 1;
        
        while let Some(token_result) = lex.next() {
            let span_range = lex.span();
            let lexeme = lex.slice().to_string();
            
            let span = Span::new(span_range.start, span_range.end, line, column);
            
            match token_result {
                Ok(logos_token) => {
                    let token_kind = self.logos_to_token_kind(logos_token, &lexeme)?;
                    let mut token = Token::new(token_kind, lexeme.clone(), span);
                    
                    // Apply English→Hebrew transformation (Layer 1)
                    if let Some(hebrew) = self.english_to_hebrew.get(&lexeme) {
                        token = token.with_hebrew(hebrew.clone());
                    }
                    
                    self.tokens.push(token);
                }
                Err(_) => {
                    return Err(LexerError::UnknownToken {
                        lexeme: lexeme.clone(),
                        span,
                    });
                }
            }
            
            // Update line and column tracking
            for ch in lexeme.chars() {
                if ch == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
            }
        }
        
        // Add EOF token
        let span = Span::new(self.source.len(), self.source.len(), line, column);
        self.tokens.push(Token::new(TokenKind::Eof, "".to_string(), span));
        
        Ok(self.tokens.clone())
    }
    
    fn logos_to_token_kind(&self, logos_token: LogosToken, lexeme: &str) -> Result<TokenKind, LexerError> {
        Ok(match logos_token {
            LogosToken::Fn => TokenKind::Fn,
            LogosToken::Let => TokenKind::Let,
            LogosToken::Const => TokenKind::Const,
            LogosToken::Mut => TokenKind::Mut,
            LogosToken::If => TokenKind::If,
            LogosToken::Else => TokenKind::Else,
            LogosToken::While => TokenKind::While,
            LogosToken::For => TokenKind::For,
            LogosToken::Return => TokenKind::Return,
            LogosToken::Break => TokenKind::Break,
            LogosToken::Continue => TokenKind::Continue,
            LogosToken::Struct => TokenKind::Struct,
            LogosToken::Enum => TokenKind::Enum,
            LogosToken::Impl => TokenKind::Impl,
            LogosToken::Trait => TokenKind::Trait,
            LogosToken::Type => TokenKind::Type,
            LogosToken::Module => TokenKind::Module,
            LogosToken::Use => TokenKind::Use,
            LogosToken::Pub => TokenKind::Pub,
            LogosToken::Quantum => TokenKind::Quantum,
            LogosToken::Superpose => TokenKind::Superpose,
            LogosToken::Entangle => TokenKind::Entangle,
            LogosToken::Measure => TokenKind::Measure,
            LogosToken::Wave => TokenKind::Wave,
            LogosToken::Frequency => TokenKind::Frequency,
            LogosToken::Amplitude => TokenKind::Amplitude,
            LogosToken::Phase => TokenKind::Phase,
            LogosToken::DNA => TokenKind::DNA,
            LogosToken::Encode => TokenKind::Encode,
            LogosToken::Decode => TokenKind::Decode,
            LogosToken::Sequence => TokenKind::Sequence,
            LogosToken::True => TokenKind::Boolean(true),
            LogosToken::False => TokenKind::Boolean(false),
            LogosToken::Plus => TokenKind::Plus,
            LogosToken::Minus => TokenKind::Minus,
            LogosToken::Star => TokenKind::Star,
            LogosToken::Slash => TokenKind::Slash,
            LogosToken::Percent => TokenKind::Percent,
            LogosToken::Caret => TokenKind::Caret,
            LogosToken::Eq => TokenKind::Eq,
            LogosToken::Ne => TokenKind::Ne,
            LogosToken::Lt => TokenKind::Lt,
            LogosToken::Le => TokenKind::Le,
            LogosToken::Gt => TokenKind::Gt,
            LogosToken::Ge => TokenKind::Ge,
            LogosToken::And => TokenKind::And,
            LogosToken::Or => TokenKind::Or,
            LogosToken::Not => TokenKind::Not,
            LogosToken::Assign => TokenKind::Assign,
            LogosToken::PlusAssign => TokenKind::PlusAssign,
            LogosToken::MinusAssign => TokenKind::MinusAssign,
            LogosToken::LParen => TokenKind::LParen,
            LogosToken::RParen => TokenKind::RParen,
            LogosToken::LBrace => TokenKind::LBrace,
            LogosToken::RBrace => TokenKind::RBrace,
            LogosToken::LBracket => TokenKind::LBracket,
            LogosToken::RBracket => TokenKind::RBracket,
            LogosToken::Comma => TokenKind::Comma,
            LogosToken::Dot => TokenKind::Dot,
            LogosToken::Semicolon => TokenKind::Semicolon,
            LogosToken::Colon => TokenKind::Colon,
            LogosToken::Arrow => TokenKind::Arrow,
            LogosToken::FatArrow => TokenKind::FatArrow,
            LogosToken::Integer => {
                let n = lexeme.parse().map_err(|_| LexerError::InvalidInteger {
                    lexeme: lexeme.to_string(),
                })?;
                TokenKind::Integer(n)
            }
            LogosToken::Float => {
                let n = lexeme.parse().map_err(|_| LexerError::InvalidFloat {
                    lexeme: lexeme.to_string(),
                })?;
                TokenKind::Float(n)
            }
            LogosToken::String => {
                // Remove quotes and unescape
                let s = lexeme[1..lexeme.len()-1].to_string();
                TokenKind::String(s)
            }
            LogosToken::Char => {
                let c = lexeme.chars().nth(1).unwrap_or('\0');
                TokenKind::Char(c)
            }
            LogosToken::Identifier => TokenKind::Identifier(lexeme.to_string()),
            LogosToken::LineComment | LogosToken::BlockComment => TokenKind::Comment,
        })
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum LexerError {
    #[error("Unknown token: {lexeme} at {span:?}")]
    UnknownToken { lexeme: String, span: Span },
    
    #[error("Invalid integer literal: {lexeme}")]
    InvalidInteger { lexeme: String },
    
    #[error("Invalid float literal: {lexeme}")]
    InvalidFloat { lexeme: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokenization() {
        let source = "fn main() { let x = 42; }".to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::Fn);
        assert_eq!(tokens[0].hebrew_form, Some("פונקציה".to_string()));
        assert!(matches!(tokens[1].kind, TokenKind::Identifier(_)));
    }
    
    #[test]
    fn test_quantum_primitives() {
        let source = "quantum superpose entangle measure".to_string();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::Quantum);
        assert_eq!(tokens[1].kind, TokenKind::Superpose);
        assert_eq!(tokens[2].kind, TokenKind::Entangle);
        assert_eq!(tokens[3].kind, TokenKind::Measure);
    }
}
