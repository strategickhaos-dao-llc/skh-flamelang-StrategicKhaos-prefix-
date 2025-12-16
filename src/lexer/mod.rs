// src/lexer/mod.rs
// Lexer for FlameLang: Tokenizes quantum-inspired symbolic AI constructs.
// Phase 1: Control Unit Mapping - Handles input routing to symbolic modules.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Integer(i64),
    Float(f64),
    Complex(f64, f64), // e.g., 3+4i for quantum states
    DnaSequence(String), // e.g., [ATGC]
    WaveOp(String), // e.g., sin~, cos~ for trig-formula wave cores
    QuantumEntangle, // ~>
    QuantumMeasure, // |->
    NeuralTick, // @tick for neural tick clocks
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    Plus, Minus, Mul, Div, Assign, Semicolon,
    SwarmBot(String), // Stub for Strategickhaos swarm bots integration
    QubitDecl, // qubit
    Superpos(String), // e.g., |psi> for superposition states
    GateOp(String), // e.g., H, X, CNOT for quantum gates
    BellState(String), // e.g., bell_phi+ for Bell states
    PipefitGlyph, // 🛠️ pipefit glyph for cortex computation
    Eof,
    // AI Agent Scaffolding: Placeholder for GPT reasoning hooks
    ReasonStub(String), // e.g., #reason{query} for recursive evolution
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Token::Eof;
        }

        let ch = self.input[self.pos];
        self.pos += 1;

        match ch {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '+' => Token::Plus,
            '-' => if self.peek() == '>' { self.pos += 1; Token::QuantumMeasure } else { Token::Minus },
            '*' => Token::Mul,
            '/' => Token::Div,
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '~' => if self.peek() == '>' { self.pos += 1; Token::QuantumEntangle } else { Token::WaveOp("~".to_string()) },
            '@' => if self.match_str("tick") { Token::NeuralTick } else { Token::Identifier("@".to_string()) },
            '#' => if self.match_str("reason") { self.parse_reason_stub() } else { Token::Identifier("#".to_string()) },
            '|' => if self.peek() == '-' && self.peek_ahead(1) == '>' { 
                self.pos += 2; 
                Token::QuantumMeasure 
            } else { 
                self.parse_superpos(ch) 
            },
            'a'..='z' | 'A'..='Z' => self.parse_identifier(ch),
            '0'..='9' => self.parse_number(ch),
            '🛠' => if self.peek() == '️' { self.pos += 1; Token::PipefitGlyph } else { Token::Identifier(ch.to_string()) },
            _ => Token::Identifier(ch.to_string()), // Fallback
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn peek(&self) -> char {
        if self.pos < self.input.len() { self.input[self.pos] } else { '\0' }
    }

    fn peek_ahead(&self, n: usize) -> char {
        if self.pos + n < self.input.len() { self.input[self.pos + n] } else { '\0' }
    }

    fn match_str(&mut self, s: &str) -> bool {
        let start = self.pos;
        for c in s.chars() {
            if self.pos >= self.input.len() || self.input[self.pos] != c {
                self.pos = start;
                return false;
            }
            self.pos += 1;
        }
        true
    }

    fn parse_identifier(&mut self, first: char) -> Token {
        let mut id = first.to_string();
        while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_' || self.input[self.pos] == '~' || self.input[self.pos] == '+' || self.input[self.pos] == '-') {
            id.push(self.input[self.pos]);
            self.pos += 1;
        }
        match id.as_str() {
            "qubit" => Token::QubitDecl,
            "H" | "X" | "Y" | "Z" | "CNOT" | "SWAP" => Token::GateOp(id),
            "entangle" | "wavecore" | "swarmbot" => Token::Keyword(id),
            "sin~" | "cos~" | "tan~" => Token::WaveOp(id), // Trig-formula wave cores
            _ if id.starts_with("bell_phi") || id.starts_with("bell_psi") => Token::BellState(id),
            _ if id.starts_with("[") && id.ends_with("]") && id.chars().skip(1).take(id.len()-2).all(|c| "ATGC".contains(c)) => Token::DnaSequence(id),
            _ if id.starts_with("swarm:") => Token::SwarmBot(id), // Strategickhaos integration stub
            _ => Token::Identifier(id),
        }
    }

    fn parse_superpos(&mut self, first: char) -> Token {
        let mut sup = first.to_string();
        while self.pos < self.input.len() && self.input[self.pos] != '>' {
            sup.push(self.input[self.pos]);
            self.pos += 1;
        }
        if self.peek() == '>' {
            self.pos += 1;
            sup.push('>');
        }
        Token::Superpos(sup)
    }

    fn parse_number(&mut self, first: char) -> Token {
        let mut num = first.to_string();
        while self.pos < self.input.len() && (self.input[self.pos].is_digit(10) || self.input[self.pos] == '.') {
            num.push(self.input[self.pos]);
            self.pos += 1;
        }
        if self.peek() == '+' || self.peek() == '-' {
            num.push(self.input[self.pos]);
            self.pos += 1;
            while self.pos < self.input.len() && self.input[self.pos].is_digit(10) {
                num.push(self.input[self.pos]);
                self.pos += 1;
            }
            if self.peek() == 'i' {
                self.pos += 1;
                // Parse complex: real + imag i
                let parts: Vec<&str> = num.split(['+', '-']).collect();
                let real = parts[0].parse::<f64>().unwrap_or(0.0);
                let imag = parts[1].parse::<f64>().unwrap_or(0.0);
                return Token::Complex(real, imag);
            }
        }
        if num.contains('.') {
            Token::Float(num.parse().unwrap_or(0.0))
        } else {
            Token::Integer(num.parse().unwrap_or(0))
        }
    }

    fn parse_reason_stub(&mut self) -> Token {
        // AI agent scaffolding: Parse #reason{query} for GPT contribution hooks
        if self.peek() != '{' { return Token::Keyword("#reason".to_string()); }
        self.pos += 1;
        let mut query = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != '}' {
            query.push(self.input[self.pos]);
            self.pos += 1;
        }
        if self.pos < self.input.len() { self.pos += 1; }
        Token::ReasonStub(query)
    }
}

// Export for use in parser phase
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("entangle ~> 3+4i sin~ @tick #reason{phase2}");
        assert_eq!(lexer.next_token(), Token::Keyword("entangle".to_string()));
        assert_eq!(lexer.next_token(), Token::QuantumEntangle);
        assert_eq!(lexer.next_token(), Token::Complex(3.0, 4.0));
        assert_eq!(lexer.next_token(), Token::WaveOp("sin~".to_string()));
        assert_eq!(lexer.next_token(), Token::NeuralTick);
        assert_eq!(lexer.next_token(), Token::ReasonStub("phase2".to_string()));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_quantum_tokens() {
        let mut lexer = Lexer::new("qubit x; H ~> |psi> bell_phi+ X");
        assert_eq!(lexer.next_token(), Token::QubitDecl);
        assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::GateOp("H".to_string()));
        assert_eq!(lexer.next_token(), Token::QuantumEntangle);
        assert_eq!(lexer.next_token(), Token::Superpos("|psi>".to_string()));
        assert_eq!(lexer.next_token(), Token::BellState("bell_phi+".to_string()));
        assert_eq!(lexer.next_token(), Token::GateOp("X".to_string()));
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
