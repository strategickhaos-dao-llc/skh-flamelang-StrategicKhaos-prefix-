//! FlameLang High-level Intermediate Representation (HIR)
//!
//! HIR is the first IR stage after AST, with type information and semantic analysis

use flamelang_ast as ast;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export commonly used AST types
pub use ast::{BinaryOp, UnaryOp};

/// HIR Program with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub functions: HashMap<String, Function>,
    pub structs: HashMap<String, Struct>,
    pub type_info: TypeInfo,
}

/// Function with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<Statement>,
    pub is_pub: bool,
}

/// Statement in HIR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Let {
        name: String,
        ty: Type,
        init: Option<Expression>,
    },
    Assign {
        target: String,
        value: Expression,
    },
    Expression(Expression),
    Return(Option<Expression>),
}

/// Expression with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub ty: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionKind {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Variable(String),
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expression>,
    },
    Call {
        func: String,
        args: Vec<Expression>,
    },
}

/// Type system for HIR
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Unit,
    Named(String),
    Array(Box<Type>),
    Function(Vec<Type>, Box<Type>),
}

/// Struct definition in HIR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<(String, Type)>,
    pub is_pub: bool,
}

/// Type information collected during HIR lowering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    pub var_types: HashMap<String, Type>,
}

/// Lower AST to HIR
pub struct HirLowering {
    type_info: TypeInfo,
}

impl HirLowering {
    pub fn new() -> Self {
        Self {
            type_info: TypeInfo {
                var_types: HashMap::new(),
            },
        }
    }

    /// Lower an AST program to HIR
    pub fn lower_program(&mut self, program: &ast::Program) -> Result<Program, LoweringError> {
        let mut functions = HashMap::new();
        let mut structs = HashMap::new();

        for item in &program.items {
            match item {
                ast::Item::Function(func) => {
                    let hir_func = self.lower_function(func)?;
                    functions.insert(func.name.clone(), hir_func);
                }
                ast::Item::Struct(s) => {
                    let hir_struct = self.lower_struct(s)?;
                    structs.insert(s.name.clone(), hir_struct);
                }
                _ => {}
            }
        }

        Ok(Program {
            functions,
            structs,
            type_info: self.type_info.clone(),
        })
    }

    fn lower_function(&mut self, func: &ast::Function) -> Result<Function, LoweringError> {
        let params: Vec<_> = func
            .params
            .iter()
            .map(|p| (p.name.clone(), self.lower_type(&p.ty)))
            .collect();

        let return_type = func
            .return_type
            .as_ref()
            .map(|t| self.lower_type(t))
            .unwrap_or(Type::Unit);

        let body = func
            .body
            .statements
            .iter()
            .map(|s| self.lower_statement(s))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Function {
            name: func.name.clone(),
            params,
            return_type,
            body,
            is_pub: func.is_pub,
        })
    }

    fn lower_struct(&self, s: &ast::Struct) -> Result<Struct, LoweringError> {
        let fields: Vec<_> = s
            .fields
            .iter()
            .map(|f| (f.name.clone(), self.lower_type(&f.ty)))
            .collect();

        Ok(Struct {
            name: s.name.clone(),
            fields,
            is_pub: s.is_pub,
        })
    }

    fn lower_statement(&mut self, stmt: &ast::Statement) -> Result<Statement, LoweringError> {
        match stmt {
            ast::Statement::Let {
                name,
                ty,
                init,
                mutable: _,
                span: _,
            } => {
                let ty = ty.as_ref().map(|t| self.lower_type(t)).unwrap_or(Type::Int);
                let init = init
                    .as_ref()
                    .map(|e| self.lower_expression(e))
                    .transpose()?;

                self.type_info.var_types.insert(name.clone(), ty.clone());

                Ok(Statement::Let {
                    name: name.clone(),
                    ty,
                    init,
                })
            }
            ast::Statement::Expression(expr) => {
                Ok(Statement::Expression(self.lower_expression(expr)?))
            }
            ast::Statement::Return(expr, _) => Ok(Statement::Return(
                expr.as_ref()
                    .map(|e| self.lower_expression(e))
                    .transpose()?,
            )),
            _ => Err(LoweringError::UnsupportedConstruct(
                "Unsupported statement".to_string(),
            )),
        }
    }

    fn lower_expression(&self, expr: &ast::Expression) -> Result<Expression, LoweringError> {
        let (kind, ty) = match expr {
            ast::Expression::IntLiteral(n, _) => (ExpressionKind::IntLiteral(*n), Type::Int),
            ast::Expression::FloatLiteral(f, _) => (ExpressionKind::FloatLiteral(*f), Type::Float),
            ast::Expression::StringLiteral(s, _) => {
                (ExpressionKind::StringLiteral(s.clone()), Type::String)
            }
            ast::Expression::BoolLiteral(b, _) => (ExpressionKind::BoolLiteral(*b), Type::Bool),
            ast::Expression::Variable(name, _) => {
                let ty = self
                    .type_info
                    .var_types
                    .get(name)
                    .cloned()
                    .unwrap_or(Type::Int);
                (ExpressionKind::Variable(name.clone()), ty)
            }
            ast::Expression::Binary {
                op,
                left,
                right,
                span: _,
            } => {
                let left_expr = self.lower_expression(left)?;
                let right_expr = self.lower_expression(right)?;
                let ty = left_expr.ty.clone();

                (
                    ExpressionKind::Binary {
                        op: *op,
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                    },
                    ty,
                )
            }
            ast::Expression::Call { func, args, span: _ } => {
                if let ast::Expression::Variable(name, _) = &**func {
                    let args_exprs = args
                        .iter()
                        .map(|a| self.lower_expression(a))
                        .collect::<Result<Vec<_>, _>>()?;

                    (
                        ExpressionKind::Call {
                            func: name.clone(),
                            args: args_exprs,
                        },
                        Type::Unit,
                    )
                } else {
                    return Err(LoweringError::UnsupportedConstruct(
                        "Complex function expressions not yet supported".to_string(),
                    ));
                }
            }
            _ => {
                return Err(LoweringError::UnsupportedConstruct(
                    "Unsupported expression".to_string(),
                ))
            }
        };

        Ok(Expression { kind, ty })
    }

    fn lower_type(&self, ty: &ast::Type) -> Type {
        match ty {
            ast::Type::Int => Type::Int,
            ast::Type::Float => Type::Float,
            ast::Type::Bool => Type::Bool,
            ast::Type::String => Type::String,
            ast::Type::Unit => Type::Unit,
            ast::Type::Named(name) => Type::Named(name.clone()),
            ast::Type::Array(inner, _) => Type::Array(Box::new(self.lower_type(inner))),
            _ => Type::Unit,
        }
    }
}

impl Default for HirLowering {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoweringError {
    #[error("Unsupported construct: {0}")]
    UnsupportedConstruct(String),

    #[error("Type error: {0}")]
    TypeError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hir_type_system() {
        let ty1 = Type::Int;
        let ty2 = Type::Function(vec![Type::Int, Type::Int], Box::new(Type::Int));

        assert_eq!(ty1, Type::Int);
        assert_ne!(ty1, Type::Float);
        assert!(matches!(ty2, Type::Function(_, _)));
    }
}
