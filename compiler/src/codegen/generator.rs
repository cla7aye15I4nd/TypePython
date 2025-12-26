//! TIR-based code generation
//!
//! This module generates LLVM IR from a TIR program.
//!
//! Unlike AST-based codegen, all operations are infallible since types
//! and symbols are pre-resolved in the TIR.

use inkwell::context::Context;
use inkwell::module::Module as LLVMModule;

use crate::driver::Target;
use crate::tir::TirProgram;

use super::context::CodegenContext;

/// Code generator
///
/// Generates LLVM IR from a TIR program.
pub struct Codegen<'ctx> {
    context: &'ctx Context,
    target: Target,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, target: Target) -> Self {
        Codegen { context, target }
    }

    /// Generate code from a TIR program
    ///
    /// Since TIR has all types and symbols resolved, this operation is infallible.
    pub fn codegen_tir(self, program: &TirProgram) -> LLVMModule<'ctx> {
        let mut codegen = CodegenContext::new(self.context, "main", self.target);

        // Declare runtime functions
        codegen.declare_runtime_functions();

        // Generate all code from TIR
        codegen.codegen_tir_program(program);

        codegen.get_module().clone()
    }
}

impl<'ctx> CodegenContext<'ctx> {
    /// Generate code from a TIR program using a 6-pass algorithm
    ///
    /// Pass 1: Declare all class struct types
    /// Pass 2: Declare all function signatures
    /// Pass 3: Declare all global variables
    /// Pass 4: Generate all function bodies
    /// Pass 5: Generate module initialization functions
    /// Pass 6: Generate main entry point
    pub fn codegen_tir_program(&mut self, program: &TirProgram) {
        // Pass 1: Declare all class struct types
        for class in &program.classes {
            self.declare_tir_class(class, program);
        }

        // Pass 2: Declare all functions
        for func in &program.functions {
            self.declare_tir_function(func, program);
        }

        // Pass 3: Declare all global variables (before function bodies)
        for module in &program.modules {
            self.declare_tir_module_globals(module, program);
        }

        // Pass 4: Generate all function bodies
        for func in &program.functions {
            self.codegen_tir_function(func, program);
        }

        // Pass 5: Generate module init functions
        for module in &program.modules {
            self.generate_tir_module_init(module, program);
        }

        // Pass 6: Generate main
        self.generate_tir_main(program);
    }
}
