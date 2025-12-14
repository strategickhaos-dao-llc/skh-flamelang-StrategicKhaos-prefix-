//! FlameLang Code Generation
//!
//! LLVM IR generation using inkwell

use flamelang_hir as hir;
use flamelang_mir as mir;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::AddressSpace;
use std::collections::HashMap;

/// Code generator
pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<mir::Local, PointerValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    /// Create a new code generator
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        Self {
            context,
            module,
            builder,
            variables: HashMap::new(),
        }
    }

    /// Generate LLVM IR for a MIR program
    pub fn generate(&mut self, program: &mir::Program) -> Result<(), CodeGenError> {
        for (_name, func) in &program.functions {
            self.generate_function(func)?;
        }

        Ok(())
    }

    /// Generate LLVM IR for a function
    fn generate_function(&mut self, func: &mir::Function) -> Result<(), CodeGenError> {
        let return_type = self.hir_type_to_llvm(&func.return_type);

        let param_types: Vec<BasicMetadataTypeEnum> = func
            .params
            .iter()
            .map(|_| self.context.i64_type().into())
            .collect();

        let fn_type = match return_type {
            Some(ty) => ty.fn_type(&param_types, false),
            None => self.context.void_type().fn_type(&param_types, false),
        };

        let function = self.module.add_function(&func.name, fn_type, None);

        if func.blocks.is_empty() {
            return Ok(());
        }

        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        self.variables.clear();

        // Generate code for basic blocks
        for block in &func.blocks {
            self.generate_basic_block(block, function)?;
        }

        Ok(())
    }

    /// Generate LLVM IR for a basic block
    fn generate_basic_block(
        &mut self,
        block: &mir::BasicBlock,
        _function: FunctionValue<'ctx>,
    ) -> Result<(), CodeGenError> {
        for stmt in &block.statements {
            self.generate_statement(stmt)?;
        }

        self.generate_terminator(&block.terminator)?;

        Ok(())
    }

    /// Generate LLVM IR for a statement
    fn generate_statement(&mut self, stmt: &mir::Statement) -> Result<(), CodeGenError> {
        match stmt {
            mir::Statement::Assign { place, rvalue } => {
                let value = self.generate_rvalue(rvalue)?;
                
                if !self.variables.contains_key(&place.local) {
                    let ptr = self.builder.build_alloca(
                        self.context.i64_type(),
                        &format!("local_{}", place.local.0),
                    ).map_err(|e| CodeGenError::LLVMError(e.to_string()))?;
                    
                    self.variables.insert(place.local, ptr);
                }

                let ptr = self.variables[&place.local];
                self.builder
                    .build_store(ptr, value)
                    .map_err(|e| CodeGenError::LLVMError(e.to_string()))?;
            }
            mir::Statement::StorageLive(local) => {
                let ptr = self.builder.build_alloca(
                    self.context.i64_type(),
                    &format!("local_{}", local.0),
                ).map_err(|e| CodeGenError::LLVMError(e.to_string()))?;
                
                self.variables.insert(*local, ptr);
            }
            mir::Statement::StorageDead(_) => {
                // No-op in LLVM IR
            }
        }

        Ok(())
    }

    /// Generate LLVM IR for an rvalue
    fn generate_rvalue(&self, rvalue: &mir::Rvalue) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        match rvalue {
            mir::Rvalue::Use(operand) => self.generate_operand(operand),
            mir::Rvalue::BinaryOp(op, left, right) => {
                let left_val = self.generate_operand(left)?;
                let right_val = self.generate_operand(right)?;

                let left_int = left_val.into_int_value();
                let right_int = right_val.into_int_value();

                let result = match op {
                    mir::BinOp::Add => self.builder.build_int_add(left_int, right_int, "add"),
                    mir::BinOp::Sub => self.builder.build_int_sub(left_int, right_int, "sub"),
                    mir::BinOp::Mul => self.builder.build_int_mul(left_int, right_int, "mul"),
                    mir::BinOp::Div => self.builder.build_int_signed_div(left_int, right_int, "div"),
                    _ => {
                        return Err(CodeGenError::UnsupportedOperation(
                            "Binary operation not yet implemented".to_string(),
                        ))
                    }
                }.map_err(|e| CodeGenError::LLVMError(e.to_string()))?;

                Ok(result.into())
            }
            _ => Err(CodeGenError::UnsupportedOperation(
                "Rvalue type not yet implemented".to_string(),
            )),
        }
    }

    /// Generate LLVM IR for an operand
    fn generate_operand(&self, operand: &mir::Operand) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        match operand {
            mir::Operand::Constant(constant) => self.generate_constant(constant),
            mir::Operand::Copy(place) | mir::Operand::Move(place) => {
                let ptr = self.variables.get(&place.local).ok_or_else(|| {
                    CodeGenError::UndefinedVariable(format!("local_{}", place.local.0))
                })?;

                let value = self.builder
                    .build_load(self.context.i64_type(), *ptr, "load")
                    .map_err(|e| CodeGenError::LLVMError(e.to_string()))?;
                
                Ok(value)
            }
        }
    }

    /// Generate LLVM IR for a constant
    fn generate_constant(&self, constant: &mir::Constant) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        match constant {
            mir::Constant::Int(n) => {
                Ok(self.context.i64_type().const_int(*n as u64, true).into())
            }
            mir::Constant::Float(f) => Ok(self.context.f64_type().const_float(*f).into()),
            mir::Constant::Bool(b) => {
                Ok(self.context.bool_type().const_int(*b as u64, false).into())
            }
            mir::Constant::String(_s) => {
                // String constants require more complex handling
                Err(CodeGenError::UnsupportedOperation(
                    "String constants not yet implemented".to_string(),
                ))
            }
        }
    }

    /// Generate LLVM IR for a terminator
    fn generate_terminator(&self, terminator: &mir::Terminator) -> Result<(), CodeGenError> {
        match terminator {
            mir::Terminator::Return(operand) => {
                if let Some(op) = operand {
                    let value = self.generate_operand(op)?;
                    self.builder
                        .build_return(Some(&value))
                        .map_err(|e| CodeGenError::LLVMError(e.to_string()))?;
                } else {
                    self.builder
                        .build_return(None)
                        .map_err(|e| CodeGenError::LLVMError(e.to_string()))?;
                }
            }
            _ => {
                return Err(CodeGenError::UnsupportedOperation(
                    "Terminator type not yet implemented".to_string(),
                ))
            }
        }

        Ok(())
    }

    /// Convert HIR type to LLVM type
    fn hir_type_to_llvm(&self, ty: &hir::Type) -> Option<BasicTypeEnum<'ctx>> {
        match ty {
            hir::Type::Int => Some(self.context.i64_type().into()),
            hir::Type::Float => Some(self.context.f64_type().into()),
            hir::Type::Bool => Some(self.context.bool_type().into()),
            hir::Type::String => Some(
                self.context
                    .ptr_type(AddressSpace::default())
                    .into(),
            ),
            hir::Type::Unit => None,
            _ => Some(self.context.i64_type().into()),
        }
    }

    /// Get the LLVM module
    pub fn module(&self) -> &Module<'ctx> {
        &self.module
    }

    /// Print the LLVM IR to a string
    pub fn to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CodeGenError {
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),

    #[error("LLVM error: {0}")]
    LLVMError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codegen_creation() {
        let context = Context::create();
        let codegen = CodeGen::new(&context, "test");
        assert_eq!(codegen.module().get_name().to_str(), Ok("test"));
    }
}
