/// Abstract Syntax Tree for FlameLang v2.0.0
/// Supports multi-dimensional representation (Hebrew→Unicode normalization ready)
use flamelang_lexer::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    Const(ConstDecl),
    Module(Module),
    Use(UseDecl),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub name: Identifier,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub is_public: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: Identifier,
    pub ty: Type,
    pub is_mut: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Struct {
    pub name: Identifier,
    pub fields: Vec<Field>,
    pub is_public: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: Identifier,
    pub ty: Type,
    pub is_public: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enum {
    pub name: Identifier,
    pub variants: Vec<Variant>,
    pub is_public: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    pub name: Identifier,
    pub fields: Vec<Type>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstDecl {
    pub name: Identifier,
    pub ty: Type,
    pub value: Expression,
    pub is_public: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub name: Identifier,
    pub items: Vec<Item>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseDecl {
    pub path: Vec<Identifier>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Let(LetStatement),
    Expression(Expression),
    Return(ReturnStatement),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Break(Span),
    Continue(Span),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetStatement {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub value: Option<Expression>,
    pub is_mut: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Block,
    pub else_block: Option<Block>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForStatement {
    pub var: Identifier,
    pub iter: Expression,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Integer(i64, Span),
    Float(f64, Span),
    String(String, Span),
    Boolean(bool, Span),
    Identifier(Identifier),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    Quantum(QuantumExpr),
    Wave(WaveExpr),
    DNA(DNAExpr),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod, Pow,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpr {
    pub func: Box<Expression>,
    pub args: Vec<Expression>,
    pub span: Span,
}

// Quantum primitives expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuantumExpr {
    Superpose {
        states: Vec<Expression>,
        span: Span,
    },
    Entangle {
        left: Box<Expression>,
        right: Box<Expression>,
        span: Span,
    },
    Measure {
        expr: Box<Expression>,
        span: Span,
    },
}

// Wave primitives expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WaveExpr {
    Wave {
        frequency: Box<Expression>,
        amplitude: Box<Expression>,
        phase: Box<Expression>,
        span: Span,
    },
    Frequency {
        expr: Box<Expression>,
        span: Span,
    },
    Amplitude {
        expr: Box<Expression>,
        span: Span,
    },
    Phase {
        expr: Box<Expression>,
        span: Span,
    },
}

// DNA primitives expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DNAExpr {
    Encode {
        data: Box<Expression>,
        span: Span,
    },
    Decode {
        sequence: Box<Expression>,
        span: Span,
    },
    Sequence {
        bases: Vec<Base>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Base {
    A, T, C, G,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Quantum,
    Wave,
    DNA,
    Custom(Identifier),
    Array(Box<Type>),
    Function(Vec<Type>, Box<Type>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
}
