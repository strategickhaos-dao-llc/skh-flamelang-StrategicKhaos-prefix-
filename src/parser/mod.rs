// src/parser/mod.rs
// Parser for FlameLang: Builds AST from tokens, mapping to register memory.
// Phase 2: Register Memory Mapping - Manages symbolic states and quantum branching.

use crate::lexer::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum AstNode {
    Identifier(String),
    Literal(Token), // Wraps numeric/DNA literals
    BinaryOp(Box<AstNode>, Token, Box<AstNode>), // e.g., x + y
    QuantumEntangle(Box<AstNode>, Box<AstNode>), // x ~> y
    QuantumMeasure(Box<AstNode>), // x |->
    WaveCore(String, Box<AstNode>), // sin~ expr
    DnaSeq(String),
    SwarmInvoke(String, Vec<AstNode>), // swarmbot:func(args)
    NeuralTick(Box<AstNode>), // @tick { expr }
    QubitDecl(String), // qubit x;
    GateApply(String, Box<AstNode>), // H x
    SuperposState(String), // |psi>
    BellEntangle(String, Vec<AstNode>), // bell_phi+ x y
    ReasonHook(String), // #reason{query}
    Block(Vec<AstNode>),
    Eof,
}

pub struct Parser {
    lexer: Lexer,
    current: Token,
    peek: Token,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token();
        let peek = lexer.next_token();
        Parser { lexer, current, peek }
    }

    pub fn parse_program(&mut self) -> AstNode {
        let mut statements = Vec::new();
        while !matches!(self.current, Token::Eof) {
            statements.push(self.parse_statement());
            self.advance();
        }
        AstNode::Block(statements)
    }

    fn advance(&mut self) {
        self.current = std::mem::replace(&mut self.peek, self.lexer.next_token());
    }

    fn parse_statement(&mut self) -> AstNode {
        match &self.current {
            Token::Keyword(k) if k == "entangle" => self.parse_entangle(),
            Token::QubitDecl => self.parse_qubit_decl(),
            Token::GateOp(g) => self.parse_gate_apply(g.clone()),
            Token::BellState(b) => self.parse_bell_entangle(b.clone()),
            Token::Identifier(_) => self.parse_expr(),
            Token::NeuralTick => self.parse_neural_tick(),
            Token::ReasonStub(q) => AstNode::ReasonHook(q.clone()),
            _ => self.parse_expr(),
        }
    }

    fn parse_entangle(&mut self) -> AstNode {
        self.advance(); // consume 'entangle'
        let left = self.parse_expr();
        if matches!(self.current, Token::QuantumEntangle) {
            self.advance();
            let right = self.parse_expr();
            AstNode::QuantumEntangle(Box::new(left), Box::new(right))
        } else {
            left
        }
    }

    fn parse_qubit_decl(&mut self) -> AstNode {
        self.advance(); // consume 'qubit'
        if let Token::Identifier(id) = &self.current {
            AstNode::QubitDecl(id.clone())
        } else {
            AstNode::Eof // Error handling stub
        }
    }

    fn parse_gate_apply(&mut self, gate: String) -> AstNode {
        self.advance();
        let target = self.parse_expr();
        AstNode::GateApply(gate, Box::new(target))
    }

    fn parse_bell_entangle(&mut self, bell: String) -> AstNode {
        self.advance();
        let mut args = Vec::new();
        while !matches!(self.current, Token::Eof | Token::Semicolon) { // Simple arg parsing
            args.push(self.parse_expr());
            if matches!(self.current, Token::Semicolon | Token::Eof) {
                break;
            }
            self.advance();
        }
        AstNode::BellEntangle(bell, args)
    }

    fn parse_neural_tick(&mut self) -> AstNode {
        self.advance(); // consume '@tick'
        if matches!(self.current, Token::LBrace) {
            self.advance();
            let expr = self.parse_expr();
            if matches!(self.current, Token::RBrace) {
                self.advance();
            }
            AstNode::NeuralTick(Box::new(expr))
        } else {
            AstNode::NeuralTick(Box::new(AstNode::Eof))
        }
    }

    fn parse_expr(&mut self) -> AstNode {
        // Simplified: Handle identifiers, literals, binary ops, etc.
        let mut left = match &self.current {
            Token::Identifier(id) => AstNode::Identifier(id.clone()),
            Token::Integer(i) => AstNode::Literal(Token::Integer(*i)),
            Token::Float(f) => AstNode::Literal(Token::Float(*f)),
            Token::Complex(r, i) => AstNode::Literal(Token::Complex(*r, *i)),
            Token::DnaSequence(d) => AstNode::DnaSeq(d.clone()),
            Token::Superpos(s) => AstNode::SuperposState(s.clone()),
            Token::WaveOp(w) => {
                let wave_op = w.clone();
                self.advance();
                let arg = self.parse_expr();
                return AstNode::WaveCore(wave_op, Box::new(arg));
            }
            Token::SwarmBot(s) => {
                let bot = s.clone();
                self.advance();
                let mut args = Vec::new();
                while !matches!(self.current, Token::Eof | Token::Semicolon) {
                    args.push(self.parse_expr());
                    if matches!(self.current, Token::Semicolon | Token::Eof) {
                        break;
                    }
                    self.advance();
                }
                return AstNode::SwarmInvoke(bot, args);
            }
            Token::QuantumMeasure => {
                self.advance();
                let expr = self.parse_expr();
                return AstNode::QuantumMeasure(Box::new(expr));
            }
            _ => AstNode::Eof,
        };

        // Handle binary ops (priority stub)
        if matches!(self.peek, Token::Plus | Token::Minus | Token::Mul | Token::Div) {
            self.advance();
            let op = self.current.clone();
            self.advance();
            let right = self.parse_expr();
            left = AstNode::BinaryOp(Box::new(left), op, Box::new(right));
        }
        left
    }
}

// Export for use in transform phases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_quantum() {
        let mut parser = Parser::new("qubit x; entangle x ~> y; H |psi>; bell_phi+ a b; @tick { sin~ 3+4i }; #reason{phase3}");
        let ast = parser.parse_program();
        if let AstNode::Block(stmts) = ast {
            assert!(stmts.len() >= 5, "Expected at least 5 statements, got {}", stmts.len());
            // Check first: qubit x;
            if let AstNode::QubitDecl(id) = &stmts[0] { 
                assert_eq!(id, "x"); 
            } else {
                panic!("Expected QubitDecl, got {:?}", stmts[0]);
            }
        } else {
            panic!("Expected Block, got {:?}", ast);
        }
    }

    #[test]
    fn test_parse_entangle() {
        let mut parser = Parser::new("entangle x ~> y");
        let ast = parser.parse_program();
        if let AstNode::Block(stmts) = ast {
            assert_eq!(stmts.len(), 1);
            if let AstNode::QuantumEntangle(left, right) = &stmts[0] {
                if let AstNode::Identifier(l) = &**left { 
                    assert_eq!(l, "x"); 
                }
                if let AstNode::Identifier(r) = &**right { 
                    assert_eq!(r, "y"); 
                }
            } else {
                panic!("Expected QuantumEntangle");
            }
        }
    }
}
