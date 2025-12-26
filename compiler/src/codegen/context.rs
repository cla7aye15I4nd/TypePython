//! Core code generation context

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target, TargetTriple};
use inkwell::types::StructType;
use inkwell::values::{FunctionValue, PointerValue};
use std::collections::HashMap;

use crate::driver::Target as CompilerTarget;

/// Code generation context
pub struct CodegenContext<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,

    /// Current function being generated
    pub(crate) current_function: Option<FunctionValue<'ctx>>,

    /// Current class being generated (for methods)
    pub(crate) current_class: Option<String>,

    /// Global module-level variables (name -> global variable pointer)
    pub(crate) global_variables: HashMap<String, PointerValue<'ctx>>,

    /// Function name -> function value
    pub(crate) functions: HashMap<String, FunctionValue<'ctx>>,

    /// Class name -> LLVM struct type
    pub(crate) class_types: HashMap<String, StructType<'ctx>>,
}

impl<'ctx> CodegenContext<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str, target: CompilerTarget) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        // Initialize LLVM targets based on the compilation target
        match target {
            CompilerTarget::X86_64 => {
                Target::initialize_x86(&InitializationConfig::default());
            }
            CompilerTarget::RiscV64 => {
                Target::initialize_riscv(&InitializationConfig::default());
            }
        }

        // Set the target triple for the specified target architecture
        let target_triple = TargetTriple::create(target.triple());
        module.set_triple(&target_triple);

        CodegenContext {
            context,
            module,
            builder,
            current_function: None,
            current_class: None,
            global_variables: HashMap::new(),
            functions: HashMap::new(),
            class_types: HashMap::new(),
        }
    }

    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }
}
