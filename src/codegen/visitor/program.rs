/// Program and function visitor implementation for code generation
use super::super::CodeGen;
use crate::ast::visitor::Visitor;
use crate::ast::*;
use crate::types::PyType;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn visit_program_impl(
        &mut self,
        program: &Program,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // External functions are now lazily declared when called

        // First pass: Declare all functions (for mutual recursion support)
        for function in &program.functions {
            self.declare_function(function)?;
        }

        // Second pass: Visit all function declarations to generate bodies
        for function in &program.functions {
            self.visit_function(function)?;
        }

        // Generate a main function that contains all top-level statements
        if !program.statements.is_empty() {
            self.generate_main_function(&program.statements)?;
        }

        Ok(self.context.i32_type().const_zero().into())
    }

    pub(crate) fn finish_program_impl(
        &mut self,
        _program: &Program,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // This is called by the default implementation but we handle everything in visit_program
        Ok(self.context.i32_type().const_zero().into())
    }

    pub(crate) fn enter_function_impl(&mut self, func: &Function) -> Result<(), String> {
        // Get the already-declared function
        let mangled_name = self.mangle_function_name(&self.module_name, &func.name);
        let function = self.module.get_function(&mangled_name).ok_or_else(|| {
            format!(
                "Function {} not found (should have been declared in first pass)",
                func.name
            )
        })?;
        self.current_function = Some(function);

        let entry_bb = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_bb);

        // Clear variables for new function scope
        self.variables.clear();

        // Allocate space for parameters and store them
        for (i, param) in func.params.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).unwrap();
            let alloca = self.create_entry_block_alloca(&func.name, &param.name, &param.param_type);
            self.builder.build_store(alloca, param_value).unwrap();
            let llvm_type = self.type_to_llvm(&param.param_type);
            let py_type = PyType::from_ast_type(&param.param_type)?;
            self.variables
                .insert(param.name.clone(), (alloca, llvm_type, py_type));
        }

        Ok(())
    }

    pub(crate) fn exit_function_impl(&mut self, func: &Function) -> Result<(), String> {
        // Add default return if needed
        if !self.is_block_terminated() {
            match func.return_type {
                Type::None => {
                    self.builder.build_return(None).unwrap();
                }
                Type::Int => {
                    let zero = self.context.i64_type().const_zero();
                    self.builder.build_return(Some(&zero)).unwrap();
                }
                Type::Float => {
                    let zero = self.context.f64_type().const_zero();
                    self.builder.build_return(Some(&zero)).unwrap();
                }
                Type::Bool => {
                    let zero = self.context.bool_type().const_zero();
                    self.builder.build_return(Some(&zero)).unwrap();
                }
                _ => {
                    return Err("Unsupported return type".to_string());
                }
            }
        }

        Ok(())
    }
}
