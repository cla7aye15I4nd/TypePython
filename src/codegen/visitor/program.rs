/// Program and function visitor implementation for code generation
use super::super::CodeGen;
use crate::ast::visitor::Visitor;
use crate::ast::*;
use crate::types::{PyType, PyValue};

use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn visit_program_impl(
        &mut self,
        program: &Program,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // Call visitor method for all imports for coverage
        for import in &program.imports {
            self.visit_import(import)?;
        }

        // External functions are now lazily declared when called

        // Process all class definitions
        for class in &program.classes {
            self.visit_class(class)?;
        }

        // Pre-pass: Create LLVM globals for module-level VarDecl statements
        // This must happen before functions are compiled so they can use global keyword
        for stmt in &program.statements {
            if let Statement::VarDecl { name, var_type, .. } = stmt {
                let py_type = PyType::from_ast_type(var_type)?;
                let llvm_type = py_type.to_llvm(self.cg.ctx);

                // Create a global variable with zero initializer
                let global = self.cg.module.add_global(llvm_type, None, name);
                global.set_initializer(&llvm_type.const_zero());

                // Store in module_vars for lookup by global keyword
                self.module_vars
                    .insert(name.clone(), (global.as_pointer_value(), py_type));
            }
        }

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

        Ok(self.cg.ctx.i32_type().const_zero().into())
    }

    pub(crate) fn visit_class_impl(&mut self, class: &Class) -> Result<(), String> {
        // Register the class in our class registry
        self.register_class(class)?;
        Ok(())
    }

    pub(crate) fn enter_function_impl(&mut self, func: &Function) -> Result<(), String> {
        // Get the already-declared function
        let mangled_name = self.mangle_function_name(&self.module_name, &func.name);
        let function = self.cg.module.get_function(&mangled_name).ok_or_else(|| {
            format!(
                "Function {} not found (should have been declared in first pass)",
                func.name
            )
        })?;
        self.current_function = Some(function);

        let entry_bb = self.cg.ctx.append_basic_block(function, "entry");
        self.cg.builder.position_at_end(entry_bb);

        // Clear variables for new function scope
        self.variables.clear();
        // Clear global/nonlocal declarations for new function scope
        self.global_vars.clear();
        self.nonlocal_vars.clear();

        // Allocate space for parameters and store them
        for (i, param) in func.params.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).unwrap();
            let py_type = PyType::from_ast_type(&param.param_type)?;
            let alloca = self.create_entry_block_alloca(&func.name, &param.name, &py_type);
            self.cg.builder.build_store(alloca, param_value).unwrap();
            let var = PyValue::new(param_value, py_type, Some(alloca));
            self.variables.insert(param.name.clone(), var);
        }

        Ok(())
    }

    pub(crate) fn exit_function_impl(&mut self, func: &Function) -> Result<(), String> {
        // Add default return if needed
        if !self.is_block_terminated() {
            match func.return_type {
                Type::None => {
                    self.cg.builder.build_return(None).unwrap();
                }
                Type::Int => {
                    let zero = self.cg.ctx.i64_type().const_zero();
                    self.cg.builder.build_return(Some(&zero)).unwrap();
                }
                Type::Float => {
                    let zero = self.cg.ctx.f64_type().const_zero();
                    self.cg.builder.build_return(Some(&zero)).unwrap();
                }
                Type::Bool => {
                    let zero = self.cg.ctx.bool_type().const_zero();
                    self.cg.builder.build_return(Some(&zero)).unwrap();
                }
                Type::Str
                | Type::Bytes
                | Type::List(_)
                | Type::Dict(_, _)
                | Type::Set(_)
                | Type::Tuple(_)
                | Type::Custom(_)
                | Type::Range => {
                    // Pointer types return null by default
                    let null = self
                        .cg
                        .ctx
                        .ptr_type(inkwell::AddressSpace::default())
                        .const_null();
                    self.cg.builder.build_return(Some(&null)).unwrap();
                }
            }
        }

        Ok(())
    }
}
