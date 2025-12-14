//! FlameLang Parser
//!
//! Parser for FlameLang v2.0.0 using a recursive descent approach.
//! (lalrpop integration would require a .lalrpop grammar file and build script)

use flamelang_ast::*;
use flamelang_lexer::{Lexer, Token};
use std::iter::Peekable;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected}, got {got}")]
    UnexpectedToken { expected: String, got: String },
    
    #[error("Unexpected end of file")]
    UnexpectedEof,
    
    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),
    
    #[error("Lexer error: {0}")]
    LexerError(#[from] flamelang_lexer::LexError),
}

pub type ParseResult<T> = Result<T, ParseError>;

/// Parser for FlameLang
pub struct Parser<'source> {
    tokens: Peekable<Lexer<'source>>,
    current_pos: usize,
}

impl<'source> Parser<'source> {
    /// Create a new parser for the given source code
    pub fn new(source: &'source str) -> Self {
        Self {
            tokens: Lexer::new(source).peekable(),
            current_pos: 0,
        }
    }

    /// Parse a complete program
    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let mut items = Vec::new();

        while self.peek().is_some() {
            items.push(self.parse_item()?);
        }

        Ok(Program { items })
    }

    /// Parse a top-level item
    fn parse_item(&mut self) -> ParseResult<Item> {
        let start = self.current_pos;
        let is_pub = if self.peek_token() == Some(&Token::Pub) {
            self.consume()?;
            true
        } else {
            false
        };

        match self.peek_token() {
            Some(Token::Fn) => {
                let func = self.parse_function(is_pub, start)?;
                Ok(Item::Function(func))
            }
            Some(Token::Struct) => {
                let s = self.parse_struct(is_pub, start)?;
                Ok(Item::Struct(s))
            }
            _ => Err(ParseError::InvalidSyntax(
                "Expected item declaration".to_string(),
            )),
        }
    }

    /// Parse a function
    fn parse_function(&mut self, is_pub: bool, start: usize) -> ParseResult<Function> {
        self.expect(Token::Fn)?;
        
        let name = self.expect_identifier()?;
        
        self.expect(Token::LParen)?;
        let params = self.parse_params()?;
        self.expect(Token::RParen)?;

        let return_type = if self.peek_token() == Some(&Token::Arrow) {
            self.consume()?;
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_block()?;
        let end = self.current_pos;

        Ok(Function {
            name,
            params,
            return_type,
            body,
            is_pub,
            span: Span { start, end },
        })
    }

    /// Parse function parameters
    fn parse_params(&mut self) -> ParseResult<Vec<Param>> {
        let mut params = Vec::new();

        if self.peek_token() == Some(&Token::RParen) {
            return Ok(params);
        }

        loop {
            let start = self.current_pos;
            let name = self.expect_identifier()?;
            self.expect(Token::Colon)?;
            let ty = self.parse_type()?;
            let end = self.current_pos;

            params.push(Param {
                name,
                ty,
                span: Span { start, end },
            });

            if self.peek_token() != Some(&Token::Comma) {
                break;
            }
            self.consume()?;
        }

        Ok(params)
    }

    /// Parse a type
    fn parse_type(&mut self) -> ParseResult<Type> {
        match self.peek_token() {
            Some(Token::Identifier(name)) if name == "int" => {
                self.consume()?;
                Ok(Type::Int)
            }
            Some(Token::Identifier(name)) if name == "float" => {
                self.consume()?;
                Ok(Type::Float)
            }
            Some(Token::Identifier(name)) if name == "bool" => {
                self.consume()?;
                Ok(Type::Bool)
            }
            Some(Token::Identifier(name)) if name == "string" => {
                self.consume()?;
                Ok(Type::String)
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.consume()?;
                Ok(Type::Named(name))
            }
            _ => Err(ParseError::InvalidSyntax("Expected type".to_string())),
        }
    }

    /// Parse a struct
    fn parse_struct(&mut self, is_pub: bool, start: usize) -> ParseResult<Struct> {
        self.expect(Token::Struct)?;
        let name = self.expect_identifier()?;
        
        self.expect(Token::LBrace)?;
        let fields = self.parse_fields()?;
        self.expect(Token::RBrace)?;
        
        let end = self.current_pos;

        Ok(Struct {
            name,
            fields,
            is_pub,
            span: Span { start, end },
        })
    }

    /// Parse struct fields
    fn parse_fields(&mut self) -> ParseResult<Vec<Field>> {
        let mut fields = Vec::new();

        while self.peek_token() != Some(&Token::RBrace) {
            let start = self.current_pos;
            let is_pub = if self.peek_token() == Some(&Token::Pub) {
                self.consume()?;
                true
            } else {
                false
            };

            let name = self.expect_identifier()?;
            self.expect(Token::Colon)?;
            let ty = self.parse_type()?;
            let end = self.current_pos;

            fields.push(Field {
                name,
                ty,
                is_pub,
                span: Span { start, end },
            });

            if self.peek_token() == Some(&Token::Comma) {
                self.consume()?;
            }
        }

        Ok(fields)
    }

    /// Parse a block
    fn parse_block(&mut self) -> ParseResult<Block> {
        let start = self.current_pos;
        self.expect(Token::LBrace)?;

        let mut statements = Vec::new();
        while self.peek_token() != Some(&Token::RBrace) {
            statements.push(self.parse_statement()?);
        }

        self.expect(Token::RBrace)?;
        let end = self.current_pos;

        Ok(Block {
            statements,
            span: Span { start, end },
        })
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> ParseResult<Statement> {
        let start = self.current_pos;

        match self.peek_token() {
            Some(Token::Let) => {
                self.consume()?;
                let mutable = if self.peek_token() == Some(&Token::Mut) {
                    self.consume()?;
                    true
                } else {
                    false
                };

                let name = self.expect_identifier()?;

                let ty = if self.peek_token() == Some(&Token::Colon) {
                    self.consume()?;
                    Some(self.parse_type()?)
                } else {
                    None
                };

                let init = if self.peek_token() == Some(&Token::Assign) {
                    self.consume()?;
                    Some(self.parse_expression()?)
                } else {
                    None
                };

                self.expect(Token::Semicolon)?;
                let end = self.current_pos;

                Ok(Statement::Let {
                    name,
                    ty,
                    init,
                    mutable,
                    span: Span { start, end },
                })
            }
            Some(Token::Return) => {
                self.consume()?;
                let value = if self.peek_token() != Some(&Token::Semicolon) {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                self.expect(Token::Semicolon)?;
                let end = self.current_pos;
                Ok(Statement::Return(value, Span { start, end }))
            }
            _ => {
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    /// Parse an expression
    fn parse_expression(&mut self) -> ParseResult<Expression> {
        self.parse_binary_expr(0)
    }

    /// Parse binary expressions with precedence
    fn parse_binary_expr(&mut self, min_prec: u8) -> ParseResult<Expression> {
        let mut left = self.parse_primary_expr()?;

        while let Some(op) = self.peek_binary_op() {
            let prec = Self::binary_op_precedence(&op);
            if prec < min_prec {
                break;
            }

            self.consume()?;
            let right = self.parse_binary_expr(prec + 1)?;
            let start = match &left {
                Expression::IntLiteral(_, span) => span.start,
                Expression::FloatLiteral(_, span) => span.start,
                Expression::StringLiteral(_, span) => span.start,
                Expression::BoolLiteral(_, span) => span.start,
                Expression::Variable(_, span) => span.start,
                Expression::Binary { span, .. } => span.start,
                Expression::Unary { span, .. } => span.start,
                Expression::Call { span, .. } => span.start,
                Expression::If { span, .. } => span.start,
                Expression::Block(block) => block.span.start,
                Expression::Assign { span, .. } => span.start,
            };
            let end = self.current_pos;

            left = Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span: Span { start, end },
            };
        }

        Ok(left)
    }

    /// Parse primary expressions
    fn parse_primary_expr(&mut self) -> ParseResult<Expression> {
        let start = self.current_pos;

        match self.peek_token() {
            Some(Token::Integer(n)) => {
                let n = *n;
                self.consume()?;
                Ok(Expression::IntLiteral(n, Span { start, end: self.current_pos }))
            }
            Some(Token::Float(f)) => {
                let f = *f;
                self.consume()?;
                Ok(Expression::FloatLiteral(f, Span { start, end: self.current_pos }))
            }
            Some(Token::String(s)) => {
                let s = s.clone();
                self.consume()?;
                Ok(Expression::StringLiteral(s, Span { start, end: self.current_pos }))
            }
            Some(Token::True) => {
                self.consume()?;
                Ok(Expression::BoolLiteral(true, Span { start, end: self.current_pos }))
            }
            Some(Token::False) => {
                self.consume()?;
                Ok(Expression::BoolLiteral(false, Span { start, end: self.current_pos }))
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.consume()?;
                
                // Check for function call
                if self.peek_token() == Some(&Token::LParen) {
                    self.consume()?;
                    let args = self.parse_args()?;
                    self.expect(Token::RParen)?;
                    Ok(Expression::Call {
                        func: Box::new(Expression::Variable(name, Span { start, end: self.current_pos })),
                        args,
                        span: Span { start, end: self.current_pos },
                    })
                } else {
                    Ok(Expression::Variable(name, Span { start, end: self.current_pos }))
                }
            }
            Some(Token::LParen) => {
                self.consume()?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => Err(ParseError::InvalidSyntax("Expected expression".to_string())),
        }
    }

    /// Parse function call arguments
    fn parse_args(&mut self) -> ParseResult<Vec<Expression>> {
        let mut args = Vec::new();

        if self.peek_token() == Some(&Token::RParen) {
            return Ok(args);
        }

        loop {
            args.push(self.parse_expression()?);

            if self.peek_token() != Some(&Token::Comma) {
                break;
            }
            self.consume()?;
        }

        Ok(args)
    }

    /// Get binary operator from current token
    fn peek_binary_op(&mut self) -> Option<BinaryOp> {
        match self.peek_token() {
            Some(Token::Plus) => Some(BinaryOp::Add),
            Some(Token::Minus) => Some(BinaryOp::Sub),
            Some(Token::Star) => Some(BinaryOp::Mul),
            Some(Token::Slash) => Some(BinaryOp::Div),
            Some(Token::Percent) => Some(BinaryOp::Mod),
            Some(Token::Eq) => Some(BinaryOp::Eq),
            Some(Token::Ne) => Some(BinaryOp::Ne),
            Some(Token::Lt) => Some(BinaryOp::Lt),
            Some(Token::Le) => Some(BinaryOp::Le),
            Some(Token::Gt) => Some(BinaryOp::Gt),
            Some(Token::Ge) => Some(BinaryOp::Ge),
            Some(Token::And) => Some(BinaryOp::And),
            Some(Token::Or) => Some(BinaryOp::Or),
            _ => None,
        }
    }

    /// Get precedence for binary operators
    fn binary_op_precedence(op: &BinaryOp) -> u8 {
        match op {
            BinaryOp::Or => 1,
            BinaryOp::And => 2,
            BinaryOp::Eq | BinaryOp::Ne => 3,
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => 4,
            BinaryOp::Add | BinaryOp::Sub => 5,
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => 6,
        }
    }

    /// Peek at the next token
    fn peek(&mut self) -> Option<&Result<Token, flamelang_lexer::LexError>> {
        self.tokens.peek()
    }

    /// Peek at the next token value
    fn peek_token(&mut self) -> Option<&Token> {
        self.peek()?.as_ref().ok()
    }

    /// Consume the next token
    fn consume(&mut self) -> ParseResult<Token> {
        self.current_pos += 1;
        self.tokens.next()
            .ok_or(ParseError::UnexpectedEof)?
            .map_err(ParseError::from)
    }

    /// Expect a specific token
    fn expect(&mut self, expected: Token) -> ParseResult<()> {
        let token = self.consume()?;
        if std::mem::discriminant(&token) == std::mem::discriminant(&expected) {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                got: format!("{:?}", token),
            })
        }
    }

    /// Expect an identifier and return its name
    fn expect_identifier(&mut self) -> ParseResult<String> {
        match self.consume()? {
            Token::Identifier(name) => Ok(name),
            token => Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                got: format!("{:?}", token),
            }),
        }
    }
}

/// Parse a FlameLang program from source code
pub fn parse(source: &str) -> ParseResult<Program> {
    let mut parser = Parser::new(source);
    parser.parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let source = "fn main() { return; }";
        let result = parse(source);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_parse_function_with_params() {
        let source = "fn add(x: int, y: int) -> int { return x; }";
        let result = parse(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_expression() {
        let source = "fn test() { let x: int = 1 + 2; }";
        let result = parse(source);
        assert!(result.is_ok());
    }
}
