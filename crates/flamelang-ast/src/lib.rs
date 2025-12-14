//! FlameLang Abstract Syntax Tree
//!
//! AST representation with visitor pattern for FlameLang v2.0.0

use serde::{Deserialize, Serialize};

/// Source location information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// Top-level program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub items: Vec<Item>,
}

/// Top-level item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    Trait(Trait),
    Impl(Impl),
    Mod(Mod),
    Use(Use),
}

/// Function declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub is_pub: bool,
    pub span: Span,
}

/// Function parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// Type annotation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Unit,
    Named(String),
    Array(Box<Type>, Option<usize>),
    Tuple(Vec<Type>),
}

/// Block of statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

/// Statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Let {
        name: String,
        ty: Option<Type>,
        init: Option<Expression>,
        mutable: bool,
        span: Span,
    },
    Expression(Expression),
    Return(Option<Expression>, Span),
    While {
        condition: Expression,
        body: Block,
        span: Span,
    },
    For {
        var: String,
        iter: Expression,
        body: Block,
        span: Span,
    },
}

/// Expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    IntLiteral(i64, Span),
    FloatLiteral(f64, Span),
    StringLiteral(String, Span),
    BoolLiteral(bool, Span),
    Variable(String, Span),
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expression>,
        span: Span,
    },
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
        span: Span,
    },
    If {
        condition: Box<Expression>,
        then_block: Block,
        else_block: Option<Block>,
        span: Span,
    },
    Block(Block),
    Assign {
        target: Box<Expression>,
        value: Box<Expression>,
        span: Span,
    },
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Neg,
    Not,
}

/// Struct declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub is_pub: bool,
    pub span: Span,
}

/// Struct field
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub is_pub: bool,
    pub span: Span,
}

/// Enum declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<Variant>,
    pub is_pub: bool,
    pub span: Span,
}

/// Enum variant
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    pub data: Option<Vec<Type>>,
    pub span: Span,
}

/// Trait declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trait {
    pub name: String,
    pub methods: Vec<Function>,
    pub is_pub: bool,
    pub span: Span,
}

/// Implementation block
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Impl {
    pub ty: Type,
    pub trait_name: Option<String>,
    pub methods: Vec<Function>,
    pub span: Span,
}

/// Module declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mod {
    pub name: String,
    pub items: Vec<Item>,
    pub is_pub: bool,
    pub span: Span,
}

/// Use declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Use {
    pub path: Vec<String>,
    pub is_pub: bool,
    pub span: Span,
}

/// Visitor trait for traversing the AST
pub trait Visitor<T = ()> {
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_item(&mut self, item: &Item) -> T;
    fn visit_function(&mut self, func: &Function) -> T;
    fn visit_statement(&mut self, stmt: &Statement) -> T;
    fn visit_expression(&mut self, expr: &Expression) -> T;
    fn visit_type(&mut self, ty: &Type) -> T;
}

/// Mutable visitor trait
pub trait VisitorMut<T = ()> {
    fn visit_program_mut(&mut self, program: &mut Program) -> T;
    fn visit_item_mut(&mut self, item: &mut Item) -> T;
    fn visit_function_mut(&mut self, func: &mut Function) -> T;
    fn visit_statement_mut(&mut self, stmt: &mut Statement) -> T;
    fn visit_expression_mut(&mut self, expr: &mut Expression) -> T;
    fn visit_type_mut(&mut self, ty: &mut Type) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_construction() {
        let program = Program {
            items: vec![Item::Function(Function {
                name: "main".to_string(),
                params: vec![],
                return_type: None,
                body: Block {
                    statements: vec![Statement::Return(None, Span { start: 0, end: 6 })],
                    span: Span { start: 0, end: 7 },
                },
                is_pub: false,
                span: Span { start: 0, end: 20 },
            })],
        };

        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_expression_types() {
        let expr = Expression::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expression::IntLiteral(1, Span { start: 0, end: 1 })),
            right: Box::new(Expression::IntLiteral(2, Span { start: 4, end: 5 })),
            span: Span { start: 0, end: 5 },
        };

        match expr {
            Expression::Binary { op, .. } => assert_eq!(op, BinaryOp::Add),
            _ => panic!("Expected binary expression"),
        }
    }
}
