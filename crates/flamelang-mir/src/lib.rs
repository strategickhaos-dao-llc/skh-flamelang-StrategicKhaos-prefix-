//! FlameLang Mid-level Intermediate Representation (MIR)
//!
//! MIR is a lower-level IR with control flow graph representation,
//! suitable for optimization passes before LLVM IR generation

use flamelang_hir as hir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MIR Program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub functions: HashMap<String, Function>,
}

/// Function in MIR with basic blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub params: Vec<Local>,
    pub return_type: hir::Type,
    pub blocks: Vec<BasicBlock>,
    pub local_count: usize,
}

/// Local variable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Local(pub usize);

/// Basic block in the control flow graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlock {
    pub id: BlockId,
    pub statements: Vec<Statement>,
    pub terminator: Terminator,
}

/// Basic block identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockId(pub usize);

/// Statement in MIR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Assign {
        place: Place,
        rvalue: Rvalue,
    },
    StorageLive(Local),
    StorageDead(Local),
}

/// Place (lvalue)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Place {
    pub local: Local,
    pub projection: Vec<PlaceElem>,
}

/// Place projection element
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceElem {
    Deref,
    Field(usize),
    Index(Local),
}

/// Rvalue (computation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rvalue {
    Use(Operand),
    BinaryOp(BinOp, Operand, Operand),
    UnaryOp(UnOp, Operand),
    Ref(Place),
}

/// Operand
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operand {
    Copy(Place),
    Move(Place),
    Constant(Constant),
}

/// Constant value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinOp {
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
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnOp {
    Neg,
    Not,
}

/// Block terminator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Terminator {
    Return(Option<Operand>),
    Goto(BlockId),
    SwitchInt {
        discriminant: Operand,
        targets: Vec<(i64, BlockId)>,
        otherwise: BlockId,
    },
    Call {
        func: String,
        args: Vec<Operand>,
        destination: Place,
        target: BlockId,
    },
}

/// Lower HIR to MIR
pub struct MirLowering {
    local_count: usize,
    block_count: usize,
    locals: HashMap<String, Local>,
}

impl MirLowering {
    pub fn new() -> Self {
        Self {
            local_count: 0,
            block_count: 0,
            locals: HashMap::new(),
        }
    }

    /// Lower a HIR program to MIR
    pub fn lower_program(&mut self, program: &hir::Program) -> Result<Program, LoweringError> {
        let mut functions = HashMap::new();

        for (name, func) in &program.functions {
            let mir_func = self.lower_function(func)?;
            functions.insert(name.clone(), mir_func);
        }

        Ok(Program { functions })
    }

    fn lower_function(&mut self, func: &hir::Function) -> Result<Function, LoweringError> {
        self.local_count = 0;
        self.block_count = 0;
        self.locals.clear();

        let params: Vec<Local> = func
            .params
            .iter()
            .map(|(name, _ty)| {
                let local = self.alloc_local();
                self.locals.insert(name.clone(), local);
                local
            })
            .collect();

        let mut blocks = Vec::new();
        let mut statements = Vec::new();

        for stmt in &func.body {
            self.lower_statement(stmt, &mut statements)?;
        }

        let terminator = Terminator::Return(None);

        blocks.push(BasicBlock {
            id: BlockId(0),
            statements,
            terminator,
        });

        Ok(Function {
            name: func.name.clone(),
            params,
            return_type: func.return_type.clone(),
            blocks,
            local_count: self.local_count,
        })
    }

    fn lower_statement(
        &mut self,
        stmt: &hir::Statement,
        statements: &mut Vec<Statement>,
    ) -> Result<(), LoweringError> {
        match stmt {
            hir::Statement::Let { name, ty: _, init } => {
                let local = self.alloc_local();
                self.locals.insert(name.clone(), local);

                statements.push(Statement::StorageLive(local));

                if let Some(expr) = init {
                    let rvalue = self.lower_expression_to_rvalue(expr)?;
                    statements.push(Statement::Assign {
                        place: Place {
                            local,
                            projection: Vec::new(),
                        },
                        rvalue,
                    });
                }
            }
            hir::Statement::Assign { target, value } => {
                let local = *self
                    .locals
                    .get(target)
                    .ok_or_else(|| LoweringError::UndefinedVariable(target.clone()))?;

                let rvalue = self.lower_expression_to_rvalue(value)?;
                statements.push(Statement::Assign {
                    place: Place {
                        local,
                        projection: Vec::new(),
                    },
                    rvalue,
                });
            }
            hir::Statement::Expression(_expr) => {
                // Expression statements are typically ignored in MIR unless they have side effects
            }
            hir::Statement::Return(_expr) => {
                // Return is handled as a terminator
            }
        }

        Ok(())
    }

    fn lower_expression_to_rvalue(&self, expr: &hir::Expression) -> Result<Rvalue, LoweringError> {
        match &expr.kind {
            hir::ExpressionKind::IntLiteral(n) => Ok(Rvalue::Use(Operand::Constant(Constant::Int(*n)))),
            hir::ExpressionKind::FloatLiteral(f) => {
                Ok(Rvalue::Use(Operand::Constant(Constant::Float(*f))))
            }
            hir::ExpressionKind::StringLiteral(s) => {
                Ok(Rvalue::Use(Operand::Constant(Constant::String(s.clone()))))
            }
            hir::ExpressionKind::BoolLiteral(b) => {
                Ok(Rvalue::Use(Operand::Constant(Constant::Bool(*b))))
            }
            hir::ExpressionKind::Variable(name) => {
                let local = *self
                    .locals
                    .get(name)
                    .ok_or_else(|| LoweringError::UndefinedVariable(name.clone()))?;

                Ok(Rvalue::Use(Operand::Copy(Place {
                    local,
                    projection: Vec::new(),
                })))
            }
            hir::ExpressionKind::Binary { op, left, right } => {
                let left_op = self.lower_expression_to_operand(left)?;
                let right_op = self.lower_expression_to_operand(right)?;
                let mir_op = self.lower_binary_op(*op);

                Ok(Rvalue::BinaryOp(mir_op, left_op, right_op))
            }
            _ => Err(LoweringError::UnsupportedConstruct(
                "Unsupported expression in MIR".to_string(),
            )),
        }
    }

    fn lower_expression_to_operand(&self, expr: &hir::Expression) -> Result<Operand, LoweringError> {
        match &expr.kind {
            hir::ExpressionKind::IntLiteral(n) => Ok(Operand::Constant(Constant::Int(*n))),
            hir::ExpressionKind::FloatLiteral(f) => Ok(Operand::Constant(Constant::Float(*f))),
            hir::ExpressionKind::StringLiteral(s) => {
                Ok(Operand::Constant(Constant::String(s.clone())))
            }
            hir::ExpressionKind::BoolLiteral(b) => Ok(Operand::Constant(Constant::Bool(*b))),
            hir::ExpressionKind::Variable(name) => {
                let local = *self
                    .locals
                    .get(name)
                    .ok_or_else(|| LoweringError::UndefinedVariable(name.clone()))?;

                Ok(Operand::Copy(Place {
                    local,
                    projection: Vec::new(),
                }))
            }
            _ => Err(LoweringError::UnsupportedConstruct(
                "Complex expression as operand".to_string(),
            )),
        }
    }

    fn lower_binary_op(&self, op: hir::BinaryOp) -> BinOp {
        match op {
            hir::BinaryOp::Add => BinOp::Add,
            hir::BinaryOp::Sub => BinOp::Sub,
            hir::BinaryOp::Mul => BinOp::Mul,
            hir::BinaryOp::Div => BinOp::Div,
            hir::BinaryOp::Mod => BinOp::Mod,
            hir::BinaryOp::Eq => BinOp::Eq,
            hir::BinaryOp::Ne => BinOp::Ne,
            hir::BinaryOp::Lt => BinOp::Lt,
            hir::BinaryOp::Le => BinOp::Le,
            hir::BinaryOp::Gt => BinOp::Gt,
            hir::BinaryOp::Ge => BinOp::Ge,
            _ => BinOp::Add, // Default
        }
    }

    fn alloc_local(&mut self) -> Local {
        let local = Local(self.local_count);
        self.local_count += 1;
        local
    }
}

impl Default for MirLowering {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoweringError {
    #[error("Unsupported construct: {0}")]
    UnsupportedConstruct(String),

    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mir_basic_block() {
        let block = BasicBlock {
            id: BlockId(0),
            statements: vec![],
            terminator: Terminator::Return(None),
        };

        assert_eq!(block.id, BlockId(0));
    }

    #[test]
    fn test_mir_local() {
        let local1 = Local(0);
        let local2 = Local(1);

        assert_ne!(local1, local2);
    }
}
