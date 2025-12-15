//! Abstract Syntax Tree

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Binary { left: Box<Expr>, op: BinOp, right: Box<Expr> },
    Glyph(char),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add, Sub, Mul, Div,
}
