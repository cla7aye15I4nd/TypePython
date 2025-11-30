/// Generator code generation using LLVM coroutines
///
/// This module implements the transformation of Python generator functions
/// into LLVM coroutines with a state machine pattern.
use super::super::CodeGen;
use crate::ast::visitor::Visitor;
use crate::ast::*;
use crate::codegen::types::iter_names;
use crate::types::{PyType, PyValue};
use inkwell::basic_block::BasicBlock;
use inkwell::values::{AnyValue, FunctionValue, PointerValue};
use std::collections::HashMap;

/// Tracks local variable info for generator frame storage
#[derive(Clone)]
#[allow(dead_code)]
pub struct LocalVarInfo {
    pub name: String,
    pub py_type: PyType,
    pub offset: i64,
}

/// Context for tracking generator state during code generation
#[allow(dead_code)]
pub struct GeneratorContext<'ctx> {
    /// The coroutine handle
    pub coro_handle: PointerValue<'ctx>,
    /// The promise pointer (where yielded values are stored)
    pub promise_ptr: PointerValue<'ctx>,
    /// Current yield point index (for state machine)
    pub yield_index: usize,
    /// Block to jump to for cleanup/final suspend
    pub cleanup_block: BasicBlock<'ctx>,
    /// Block for final suspend
    pub final_suspend_block: BasicBlock<'ctx>,
    /// Resume blocks for each yield point
    pub resume_blocks: Vec<BasicBlock<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    /// Generate a generator function using LLVM coroutines
    ///
    /// The generator is transformed into:
    /// 1. A ramp function that creates and returns the generator object
    /// 2. The coroutine body with suspend points at each yield
    /// 3. Runtime support functions for iteration
    pub fn generate_generator_function(&mut self, func: &Function) -> Result<(), String> {
        // For now, use a simpler state-machine based approach that doesn't require
        // full LLVM coroutine support (which needs specific pass pipeline)
        self.generate_generator_state_machine(func)
    }

    /// Collect all local variables declared in a function (including nested scopes)
    fn collect_local_variables(&self, func: &Function) -> Vec<(String, PyType)> {
        let mut locals = Vec::new();

        // Add parameters first
        for param in &func.params {
            if let Ok(py_type) = PyType::from_ast_type(&param.param_type) {
                locals.push((param.name.clone(), py_type));
            }
        }

        // Collect from body
        self.collect_locals_from_statements(&func.body, &mut locals);

        locals
    }

    fn collect_locals_from_statements(
        &self,
        stmts: &[Statement],
        locals: &mut Vec<(String, PyType)>,
    ) {
        for stmt in stmts {
            self.collect_locals_from_statement(stmt, locals);
        }
    }

    /// Check if a block of statements contains a yield expression
    fn body_has_yield(&self, stmts: &[Statement]) -> bool {
        for stmt in stmts {
            if self.stmt_has_yield(stmt) {
                return true;
            }
        }
        false
    }

    fn stmt_has_yield(&self, stmt: &Statement) -> bool {
        match stmt {
            Statement::Expr(expr) => matches!(expr, Expression::Yield { .. }),
            Statement::VarDecl { value, .. } => self.expr_contains_yield(value),
            Statement::Assignment { value, .. } => self.expr_contains_yield(value),
            Statement::If {
                then_block,
                elif_clauses,
                else_block,
                ..
            } => {
                self.body_has_yield(then_block)
                    || elif_clauses
                        .iter()
                        .any(|(_, stmts)| self.body_has_yield(stmts))
                    || else_block.as_ref().is_some_and(|b| self.body_has_yield(b))
            }
            Statement::While { body, .. } => self.body_has_yield(body),
            Statement::For { body, .. } => self.body_has_yield(body),
            Statement::Try {
                body,
                handlers,
                else_block,
                finally_block,
                ..
            } => {
                self.body_has_yield(body)
                    || handlers.iter().any(|h| self.body_has_yield(&h.body))
                    || else_block.as_ref().is_some_and(|b| self.body_has_yield(b))
                    || finally_block
                        .as_ref()
                        .is_some_and(|b| self.body_has_yield(b))
            }
            _ => false,
        }
    }

    fn expr_contains_yield(&self, expr: &Expression) -> bool {
        matches!(expr, Expression::Yield { .. })
    }

    fn collect_locals_from_statement(&self, stmt: &Statement, locals: &mut Vec<(String, PyType)>) {
        match stmt {
            Statement::VarDecl { name, var_type, .. } => {
                if let Ok(py_type) = PyType::from_ast_type(var_type) {
                    // Don't add duplicates
                    if !locals.iter().any(|(n, _)| n == name) {
                        locals.push((name.clone(), py_type));
                    }
                }
            }
            Statement::If {
                then_block,
                elif_clauses,
                else_block,
                ..
            } => {
                self.collect_locals_from_statements(then_block, locals);
                for (_, stmts) in elif_clauses {
                    self.collect_locals_from_statements(stmts, locals);
                }
                if let Some(else_stmts) = else_block {
                    self.collect_locals_from_statements(else_stmts, locals);
                }
            }
            Statement::While { body, .. } => {
                self.collect_locals_from_statements(body, locals);
            }
            Statement::For { targets, body, .. } => {
                // For loop variable - assume Int for range iteration
                // For now, only handle single target (no tuple unpacking in generators)
                if let Some(target) = targets.first() {
                    if !locals.iter().any(|(n, _)| n == target) {
                        locals.push((target.clone(), PyType::Int));
                    }
                }

                // If the loop body contains yield, we need synthetic variables for iterator state
                if self.body_has_yield(body) {
                    // Add synthetic variables with fixed names (matching generate_range_for_with_yield and generate_list_for_with_yield)
                    // For range: __range_iter, __range_out
                    // For list: __list_index, __list_len, __list_ptr
                    // We add all of them since we don't know the iterator type at this point

                    // Range iteration state
                    if !locals.iter().any(|(n, _)| n == "__range_iter") {
                        locals.push(("__range_iter".to_string(), PyType::Int)); // Pointer stored as Int
                    }
                    if !locals.iter().any(|(n, _)| n == "__range_out") {
                        locals.push(("__range_out".to_string(), PyType::Int));
                    }

                    // List iteration state
                    if !locals.iter().any(|(n, _)| n == "__list_index") {
                        locals.push(("__list_index".to_string(), PyType::Int));
                    }
                    if !locals.iter().any(|(n, _)| n == "__list_len") {
                        locals.push(("__list_len".to_string(), PyType::Int));
                    }
                    if !locals.iter().any(|(n, _)| n == "__list_ptr") {
                        locals.push(("__list_ptr".to_string(), PyType::Int)); // Pointer stored as Int
                    }
                }

                self.collect_locals_from_statements(body, locals);
            }
            Statement::Try {
                body,
                handlers,
                else_block,
                finally_block,
                ..
            } => {
                self.collect_locals_from_statements(body, locals);
                for handler in handlers {
                    self.collect_locals_from_statements(&handler.body, locals);
                }
                if let Some(else_stmts) = else_block {
                    self.collect_locals_from_statements(else_stmts, locals);
                }
                if let Some(finally_stmts) = finally_block {
                    self.collect_locals_from_statements(finally_stmts, locals);
                }
            }
            _ => {}
        }
    }

    /// Generate a generator using a manual state machine approach
    ///
    /// This creates:
    /// 1. A generator struct type with state, yield_point, and local variables
    /// 2. A creation function that returns the generator
    /// 3. A resume function that implements the state machine
    fn generate_generator_state_machine(&mut self, func: &Function) -> Result<(), String> {
        let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());
        let i64_type = self.cg.ctx.i64_type();

        // Collect all local variables
        let locals = self.collect_local_variables(func);

        // Build variable info map with offsets
        let mut var_infos: HashMap<String, LocalVarInfo> = HashMap::new();
        let mut offset = 0i64;
        for (name, py_type) in &locals {
            var_infos.insert(
                name.clone(),
                LocalVarInfo {
                    name: name.clone(),
                    py_type: py_type.clone(),
                    offset,
                },
            );
            offset += 8; // 8 bytes per variable
        }

        // Calculate total frame size
        let frame_size = offset;

        // Get the mangled name
        let mangled_name = self.mangle_function_name(&self.module_name, &func.name);

        // The declared function becomes the "create generator" function
        // It returns a pointer to a generator object
        let create_fn = self
            .cg
            .module
            .get_function(&mangled_name)
            .ok_or_else(|| format!("Function {} not declared", func.name))?;

        // Create the resume function that implements the state machine
        let resume_fn_name = format!("{}_resume", mangled_name);
        let resume_fn_type = i64_type.fn_type(&[ptr_type.into()], false);
        let resume_fn = self
            .cg
            .module
            .add_function(&resume_fn_name, resume_fn_type, None);

        // Generate the creation function body
        self.generate_generator_create_fn(func, create_fn, &resume_fn_name, frame_size)?;

        // Generate the resume function body
        self.generate_generator_resume_fn(func, resume_fn, &var_infos)?;

        Ok(())
    }

    /// Generate the generator creation function
    /// This allocates the generator state and returns a pointer to it
    fn generate_generator_create_fn(
        &mut self,
        func: &Function,
        create_fn: FunctionValue<'ctx>,
        resume_fn_name: &str,
        frame_size: i64,
    ) -> Result<(), String> {
        let _ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());
        let i64_type = self.cg.ctx.i64_type();

        self.current_function = Some(create_fn);
        let entry_bb = self.cg.ctx.append_basic_block(create_fn, "entry");
        self.cg.builder.position_at_end(entry_bb);

        // Clear variables
        self.variables.clear();

        // Store parameters for later use in resume function
        for (i, param) in func.params.iter().enumerate() {
            let param_value = create_fn.get_nth_param(i as u32).unwrap();
            let py_type = PyType::from_ast_type(&param.param_type)?;
            let alloca = self.create_entry_block_alloca(&func.name, &param.name, &py_type);
            self.cg.builder.build_store(alloca, param_value).unwrap();
            let var = PyValue::new(param_value, py_type, Some(alloca));
            self.variables.insert(param.name.clone(), var);
        }

        // Call generator_new to create the generator object
        let generator_new_fn = self.get_or_declare_generator_fn("generator_new");

        // Get the resume function pointer
        let resume_fn = self
            .cg
            .module
            .get_function(resume_fn_name)
            .ok_or_else(|| format!("Resume function {} not found", resume_fn_name))?;
        let resume_fn_ptr = resume_fn.as_global_value().as_pointer_value();

        // Use calculated frame size (at least 64 bytes for safety)
        let frame_size_val = i64_type.const_int(frame_size.max(64) as u64, false);

        let call = self
            .cg
            .builder
            .build_call(
                generator_new_fn,
                &[resume_fn_ptr.into(), frame_size_val.into()],
                "gen",
            )
            .unwrap();
        let gen_ptr = call.as_any_value_enum().into_pointer_value();

        // Store initial parameter values in the generator frame
        // Get the frame pointer from the generator
        let generator_frame_fn = self.get_or_declare_generator_fn("generator_frame");
        let frame_call = self
            .cg
            .builder
            .build_call(generator_frame_fn, &[gen_ptr.into()], "frame")
            .unwrap();
        let frame_ptr = frame_call.as_any_value_enum().into_pointer_value();

        // Store each parameter in the frame at known offsets
        let mut offset = 0i64;
        for (i, param) in func.params.iter().enumerate() {
            let param_value = create_fn.get_nth_param(i as u32).unwrap();
            let _py_type = PyType::from_ast_type(&param.param_type)?;

            // Calculate offset pointer
            let offset_val = i64_type.const_int(offset as u64, false);
            let param_ptr = unsafe {
                self.cg
                    .builder
                    .build_gep(
                        self.cg.ctx.i8_type(),
                        frame_ptr,
                        &[offset_val],
                        &format!("param_{}_ptr", param.name),
                    )
                    .unwrap()
            };

            // Store the value
            self.cg.builder.build_store(param_ptr, param_value).unwrap();

            // Advance offset (assume 8 bytes per value for simplicity)
            offset += 8;
        }

        // Return the generator pointer
        self.cg.builder.build_return(Some(&gen_ptr)).unwrap();

        Ok(())
    }

    /// Generate the generator resume function (state machine)
    fn generate_generator_resume_fn(
        &mut self,
        func: &Function,
        resume_fn: FunctionValue<'ctx>,
        var_infos: &HashMap<String, LocalVarInfo>,
    ) -> Result<(), String> {
        let i64_type = self.cg.ctx.i64_type();

        self.current_function = Some(resume_fn);
        let entry_bb = self.cg.ctx.append_basic_block(resume_fn, "entry");
        self.cg.builder.position_at_end(entry_bb);

        // Clear variables
        self.variables.clear();

        // Get generator pointer (first argument)
        let gen_ptr = resume_fn.get_nth_param(0).unwrap().into_pointer_value();

        // Get yield point from generator state
        let get_yield_point_fn = self.get_or_declare_generator_fn("generator_yield_point");
        let yield_point_call = self
            .cg
            .builder
            .build_call(get_yield_point_fn, &[gen_ptr.into()], "yield_point")
            .unwrap();
        let yield_point = yield_point_call.as_any_value_enum().into_int_value();

        // Get frame pointer to restore local variables
        let generator_frame_fn = self.get_or_declare_generator_fn("generator_frame");
        let frame_call = self
            .cg
            .builder
            .build_call(generator_frame_fn, &[gen_ptr.into()], "frame")
            .unwrap();
        let frame_ptr = frame_call.as_any_value_enum().into_pointer_value();

        // Create allocas for ALL local variables (not just parameters)
        // and restore their values from the frame
        for (name, info) in var_infos {
            let llvm_type = info.py_type.to_llvm(self.cg.ctx);

            // Create alloca
            let alloca = self.create_entry_block_alloca(&func.name, name, &info.py_type);

            // Load from frame
            let offset_val = i64_type.const_int(info.offset as u64, false);
            let var_ptr = unsafe {
                self.cg
                    .builder
                    .build_gep(
                        self.cg.ctx.i8_type(),
                        frame_ptr,
                        &[offset_val],
                        &format!("{}_frame_ptr", name),
                    )
                    .unwrap()
            };

            let loaded_value = self
                .cg
                .builder
                .build_load(llvm_type, var_ptr, name)
                .unwrap();
            self.cg.builder.build_store(alloca, loaded_value).unwrap();

            let var = PyValue::new(loaded_value, info.py_type.clone(), Some(alloca));
            self.variables.insert(name.clone(), var);
        }

        // Count yield points to create switch targets
        let yield_count = crate::codegen::generator::count_yield_points(func);

        // Create basic blocks for each yield resume point
        let mut resume_blocks = Vec::new();
        for i in 0..=yield_count {
            let bb = self
                .cg
                .ctx
                .append_basic_block(resume_fn, &format!("resume_{}", i));
            resume_blocks.push(bb);
        }

        // Create done block
        let done_bb = self.cg.ctx.append_basic_block(resume_fn, "done");

        // Build switch on yield_point with all cases
        let default_bb = if resume_blocks.is_empty() {
            done_bb
        } else {
            resume_blocks[0]
        };
        let cases: Vec<_> = resume_blocks
            .iter()
            .enumerate()
            .map(|(i, bb)| (i64_type.const_int(i as u64, false), *bb))
            .collect();
        self.cg
            .builder
            .build_switch(yield_point, default_bb, &cases)
            .unwrap();

        // Generate code for resume point 0 (initial entry)
        if !resume_blocks.is_empty() {
            self.cg.builder.position_at_end(resume_blocks[0]);

            // Generate the function body with yield handling
            let mut yield_index = 0usize;
            for stmt in &func.body {
                self.generate_statement_with_yield(
                    stmt,
                    gen_ptr,
                    &mut yield_index,
                    &resume_blocks,
                    done_bb,
                    frame_ptr,
                    var_infos,
                )?;

                if self.is_block_terminated() {
                    break;
                }
            }

            // After all statements, generator is done
            if !self.is_block_terminated() {
                self.cg.builder.build_unconditional_branch(done_bb).unwrap();
            }
        }

        // Generate done block - mark generator as closed and return 0
        self.cg.builder.position_at_end(done_bb);
        let set_closed_fn = self.get_or_declare_generator_fn("generator_set_closed");
        self.cg
            .builder
            .build_call(set_closed_fn, &[gen_ptr.into()], "")
            .unwrap();
        let zero = i64_type.const_zero();
        self.cg.builder.build_return(Some(&zero)).unwrap();

        Ok(())
    }

    /// Generate a statement, handling yield expressions specially
    #[allow(clippy::too_many_arguments)]
    fn generate_statement_with_yield(
        &mut self,
        stmt: &Statement,
        gen_ptr: PointerValue<'ctx>,
        yield_index: &mut usize,
        resume_blocks: &[BasicBlock<'ctx>],
        done_bb: BasicBlock<'ctx>,
        frame_ptr: PointerValue<'ctx>,
        var_infos: &HashMap<String, LocalVarInfo>,
    ) -> Result<(), String> {
        match stmt {
            Statement::Expr(expr) => {
                if let Expression::Yield { value, is_from } = expr {
                    self.generate_yield(
                        value.as_deref(),
                        *is_from,
                        gen_ptr,
                        yield_index,
                        resume_blocks,
                        frame_ptr,
                        var_infos,
                    )?;
                } else {
                    self.evaluate_expression(expr)?;
                }
            }
            Statement::Return(_) => {
                // Return in generator = done
                self.cg.builder.build_unconditional_branch(done_bb).unwrap();
            }
            Statement::VarDecl {
                name,
                var_type,
                value,
            } => {
                // Check if value contains yield
                if self.expr_has_yield(value) {
                    // Need to handle yield in value
                    let val = self.evaluate_expression_with_yield(
                        value,
                        gen_ptr,
                        yield_index,
                        resume_blocks,
                        frame_ptr,
                        var_infos,
                    )?;
                    let py_type = PyType::from_ast_type(var_type)?;
                    let alloca = self.create_entry_block_alloca("gen", name, &py_type);
                    self.cg.builder.build_store(alloca, val.value()).unwrap();
                    let var = PyValue::new(val.value(), py_type, Some(alloca));
                    self.variables.insert(name.clone(), var);
                } else {
                    self.visit_var_decl_impl(name, var_type, value)?;
                }
            }
            Statement::Assignment { target, value } => {
                if self.expr_has_yield(value) {
                    let val = self.evaluate_expression_with_yield(
                        value,
                        gen_ptr,
                        yield_index,
                        resume_blocks,
                        frame_ptr,
                        var_infos,
                    )?;
                    if let Expression::Var(name) = target {
                        if let Some(var) = self.variables.get(name).cloned() {
                            var.store_value(&self.cg.builder, &val)?;
                        }
                    }
                } else {
                    self.visit_assignment_impl(target, value)?;
                }
            }
            Statement::If {
                condition,
                then_block,
                elif_clauses,
                else_block,
            } => {
                // Generate if with yield support
                self.generate_if_with_yield(
                    condition,
                    then_block,
                    elif_clauses,
                    else_block,
                    gen_ptr,
                    yield_index,
                    resume_blocks,
                    done_bb,
                    frame_ptr,
                    var_infos,
                )?;
            }
            Statement::While { condition, body } => {
                self.generate_while_with_yield(
                    condition,
                    body,
                    gen_ptr,
                    yield_index,
                    resume_blocks,
                    done_bb,
                    frame_ptr,
                    var_infos,
                )?;
            }
            Statement::For {
                targets,
                iter,
                body,
                else_block: _,
            } => {
                // For now, only handle single target (no tuple unpacking in generators)
                // else_block is ignored in generators for now
                let target = targets.first().map(|s| s.as_str()).unwrap_or("_");
                self.generate_for_with_yield(
                    target,
                    iter,
                    body,
                    gen_ptr,
                    yield_index,
                    resume_blocks,
                    done_bb,
                    frame_ptr,
                    var_infos,
                )?;
            }
            _ => {
                // For other statements, use normal visitor
                self.visit_statement(stmt)?;
            }
        }
        Ok(())
    }

    /// Check if expression contains yield
    fn expr_has_yield(&self, expr: &Expression) -> bool {
        crate::codegen::generator::count_yield_points(&Function {
            name: String::new(),
            params: vec![],
            return_type: Type::None,
            body: vec![Statement::Expr(expr.clone())],
        }) > 0
    }

    /// Evaluate expression that may contain yield
    fn evaluate_expression_with_yield(
        &mut self,
        expr: &Expression,
        gen_ptr: PointerValue<'ctx>,
        yield_index: &mut usize,
        resume_blocks: &[BasicBlock<'ctx>],
        frame_ptr: PointerValue<'ctx>,
        var_infos: &HashMap<String, LocalVarInfo>,
    ) -> Result<PyValue<'ctx>, String> {
        if let Expression::Yield { value, is_from } = expr {
            self.generate_yield(
                value.as_deref(),
                *is_from,
                gen_ptr,
                yield_index,
                resume_blocks,
                frame_ptr,
                var_infos,
            )?;
            // After yield, the sent value is available
            let get_sent_fn = self.get_or_declare_generator_fn("generator_get_sent_value");
            let call = self
                .cg
                .builder
                .build_call(get_sent_fn, &[gen_ptr.into()], "sent")
                .unwrap();
            let sent_val = call.as_any_value_enum().into_int_value();
            Ok(PyValue::int(sent_val))
        } else {
            self.evaluate_expression(expr)
        }
    }

    /// Generate a yield point
    #[allow(clippy::too_many_arguments)]
    fn generate_yield(
        &mut self,
        value: Option<&Expression>,
        is_from: bool,
        gen_ptr: PointerValue<'ctx>,
        yield_index: &mut usize,
        resume_blocks: &[BasicBlock<'ctx>],
        frame_ptr: PointerValue<'ctx>,
        var_infos: &HashMap<String, LocalVarInfo>,
    ) -> Result<(), String> {
        let i64_type = self.cg.ctx.i64_type();

        if is_from {
            // yield from - delegate to sub-iterator
            // For now, treat as regular iteration
            if let Some(iter_expr) = value {
                let iter_val = self.evaluate_expression(iter_expr)?;
                // TODO: Implement yield from properly by iterating over iter_val
                // For now, just yield the value directly
                let yield_value_fn = self.get_or_declare_generator_fn("generator_yield_value");
                self.cg
                    .builder
                    .build_call(
                        yield_value_fn,
                        &[gen_ptr.into(), iter_val.value().into()],
                        "",
                    )
                    .unwrap();
            }
        } else {
            // Regular yield
            let yield_val = if let Some(val_expr) = value {
                let val = self.evaluate_expression(val_expr)?;
                val.value().into_int_value()
            } else {
                i64_type.const_zero()
            };

            // Store the yielded value
            let yield_value_fn = self.get_or_declare_generator_fn("generator_yield_value");
            self.cg
                .builder
                .build_call(yield_value_fn, &[gen_ptr.into(), yield_val.into()], "")
                .unwrap();
        }

        // CRITICAL: Save ALL local variables to the frame before returning
        for (name, info) in var_infos {
            if let Some(var) = self.variables.get(name) {
                if let Some(alloca) = var.ptr() {
                    // Load current value from local alloca
                    let llvm_type = info.py_type.to_llvm(self.cg.ctx);
                    let current_val = self
                        .cg
                        .builder
                        .build_load(llvm_type, alloca, &format!("{}_save", name))
                        .unwrap();

                    // Calculate frame pointer offset
                    let offset_val = i64_type.const_int(info.offset as u64, false);
                    let var_ptr = unsafe {
                        self.cg
                            .builder
                            .build_gep(
                                self.cg.ctx.i8_type(),
                                frame_ptr,
                                &[offset_val],
                                &format!("{}_frame_ptr", name),
                            )
                            .unwrap()
                    };

                    // Store to frame
                    self.cg.builder.build_store(var_ptr, current_val).unwrap();
                }
            }
        }

        // Increment yield index
        *yield_index += 1;

        // Set yield point for next resume
        let set_yield_point_fn = self.get_or_declare_generator_fn("generator_set_yield_point");
        let next_point = i64_type.const_int(*yield_index as u64, false);
        self.cg
            .builder
            .build_call(set_yield_point_fn, &[gen_ptr.into(), next_point.into()], "")
            .unwrap();

        // Return 1 (has value)
        let one = i64_type.const_int(1, false);
        self.cg.builder.build_return(Some(&one)).unwrap();

        // Continue in next resume block
        if *yield_index < resume_blocks.len() {
            self.cg.builder.position_at_end(resume_blocks[*yield_index]);
        }

        Ok(())
    }

    /// Generate if statement with yield support
    #[allow(clippy::too_many_arguments)]
    fn generate_if_with_yield(
        &mut self,
        condition: &Expression,
        then_block: &[Statement],
        elif_clauses: &[(Expression, Vec<Statement>)],
        else_block: &Option<Vec<Statement>>,
        gen_ptr: PointerValue<'ctx>,
        yield_index: &mut usize,
        resume_blocks: &[BasicBlock<'ctx>],
        done_bb: BasicBlock<'ctx>,
        frame_ptr: PointerValue<'ctx>,
        var_infos: &HashMap<String, LocalVarInfo>,
    ) -> Result<(), String> {
        // For simplicity, use normal if generation
        // A full implementation would handle yields in any branch
        let cond_val = self.evaluate_expression(condition)?;
        let cond_bool = self.cg.value_to_bool(&cond_val);

        let function = self.current_function.unwrap();
        let then_bb = self.cg.ctx.append_basic_block(function, "then");
        let merge_bb = self.cg.ctx.append_basic_block(function, "ifcont");

        let else_bb = if !elif_clauses.is_empty() || else_block.is_some() {
            self.cg.ctx.append_basic_block(function, "else")
        } else {
            merge_bb
        };

        self.cg
            .builder
            .build_conditional_branch(cond_bool, then_bb, else_bb)
            .unwrap();

        // Then block
        self.cg.builder.position_at_end(then_bb);
        for stmt in then_block {
            self.generate_statement_with_yield(
                stmt,
                gen_ptr,
                yield_index,
                resume_blocks,
                done_bb,
                frame_ptr,
                var_infos,
            )?;
            if self.is_block_terminated() {
                break;
            }
        }
        if !self.is_block_terminated() {
            self.cg
                .builder
                .build_unconditional_branch(merge_bb)
                .unwrap();
        }

        // Else block
        if !elif_clauses.is_empty() || else_block.is_some() {
            self.cg.builder.position_at_end(else_bb);

            if let Some(else_stmts) = else_block {
                for stmt in else_stmts {
                    self.generate_statement_with_yield(
                        stmt,
                        gen_ptr,
                        yield_index,
                        resume_blocks,
                        done_bb,
                        frame_ptr,
                        var_infos,
                    )?;
                    if self.is_block_terminated() {
                        break;
                    }
                }
            }

            if !self.is_block_terminated() {
                self.cg
                    .builder
                    .build_unconditional_branch(merge_bb)
                    .unwrap();
            }
        }

        self.cg.builder.position_at_end(merge_bb);
        Ok(())
    }

    /// Generate while statement with yield support
    ///
    /// For generators, we need to handle the case where a yield is inside the loop.
    /// When we resume after a yield, we need to:
    /// 1. Execute any statements after the yield in the loop body
    /// 2. Then jump back to the while condition check.
    #[allow(clippy::too_many_arguments)]
    fn generate_while_with_yield(
        &mut self,
        condition: &Expression,
        body: &[Statement],
        gen_ptr: PointerValue<'ctx>,
        yield_index: &mut usize,
        resume_blocks: &[BasicBlock<'ctx>],
        done_bb: BasicBlock<'ctx>,
        frame_ptr: PointerValue<'ctx>,
        var_infos: &HashMap<String, LocalVarInfo>,
    ) -> Result<(), String> {
        let function = self.current_function.unwrap();
        let cond_bb = self.cg.ctx.append_basic_block(function, "while_cond");
        let body_bb = self.cg.ctx.append_basic_block(function, "while_body");
        let after_bb = self.cg.ctx.append_basic_block(function, "while_after");

        self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

        // Condition
        self.cg.builder.position_at_end(cond_bb);
        let cond_val = self.evaluate_expression(condition)?;
        let cond_bool = self.cg.value_to_bool(&cond_val);
        self.cg
            .builder
            .build_conditional_branch(cond_bool, body_bb, after_bb)
            .unwrap();

        // Body - generate statements, handling yield specially
        self.cg.builder.position_at_end(body_bb);

        // Find the index of yield in the body (if any)
        let mut yield_stmt_idx = None;
        for (i, stmt) in body.iter().enumerate() {
            if matches!(stmt, Statement::Expr(Expression::Yield { .. })) {
                yield_stmt_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = yield_stmt_idx {
            // Generate statements before yield
            for stmt in &body[..idx] {
                self.generate_statement_with_yield(
                    stmt,
                    gen_ptr,
                    yield_index,
                    resume_blocks,
                    done_bb,
                    frame_ptr,
                    var_infos,
                )?;
                if self.is_block_terminated() {
                    break;
                }
            }

            // Generate the yield itself
            if !self.is_block_terminated() {
                if let Statement::Expr(Expression::Yield { value, is_from }) = &body[idx] {
                    self.generate_yield_for_while(
                        value.as_deref(),
                        *is_from,
                        gen_ptr,
                        yield_index,
                        resume_blocks,
                        frame_ptr,
                        var_infos,
                        &body[idx + 1..], // Statements after yield
                        cond_bb,          // Jump to condition after executing post-yield statements
                        done_bb,
                    )?;
                }
            }
        } else {
            // No yield in body, generate normally
            for stmt in body {
                self.generate_statement_with_yield(
                    stmt,
                    gen_ptr,
                    yield_index,
                    resume_blocks,
                    done_bb,
                    frame_ptr,
                    var_infos,
                )?;
                if self.is_block_terminated() {
                    break;
                }
            }
            if !self.is_block_terminated() {
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();
            }
        }

        self.cg.builder.position_at_end(after_bb);
        Ok(())
    }

    /// Generate a yield inside a while loop, with proper continuation handling
    #[allow(clippy::too_many_arguments)]
    fn generate_yield_for_while(
        &mut self,
        value: Option<&Expression>,
        is_from: bool,
        gen_ptr: PointerValue<'ctx>,
        yield_index: &mut usize,
        resume_blocks: &[BasicBlock<'ctx>],
        frame_ptr: PointerValue<'ctx>,
        var_infos: &HashMap<String, LocalVarInfo>,
        post_yield_stmts: &[Statement],
        cond_bb: BasicBlock<'ctx>,
        done_bb: BasicBlock<'ctx>,
    ) -> Result<(), String> {
        let i64_type = self.cg.ctx.i64_type();

        // Generate the yield value
        if is_from {
            if let Some(iter_expr) = value {
                let iter_val = self.evaluate_expression(iter_expr)?;
                let yield_value_fn = self.get_or_declare_generator_fn("generator_yield_value");
                self.cg
                    .builder
                    .build_call(
                        yield_value_fn,
                        &[gen_ptr.into(), iter_val.value().into()],
                        "",
                    )
                    .unwrap();
            }
        } else {
            let yield_val = if let Some(val_expr) = value {
                let val = self.evaluate_expression(val_expr)?;
                val.value().into_int_value()
            } else {
                i64_type.const_zero()
            };

            let yield_value_fn = self.get_or_declare_generator_fn("generator_yield_value");
            self.cg
                .builder
                .build_call(yield_value_fn, &[gen_ptr.into(), yield_val.into()], "")
                .unwrap();
        }

        // Save ALL local variables to the frame before returning
        for (name, info) in var_infos {
            if let Some(var) = self.variables.get(name) {
                if let Some(alloca) = var.ptr() {
                    let llvm_type = info.py_type.to_llvm(self.cg.ctx);
                    let current_val = self
                        .cg
                        .builder
                        .build_load(llvm_type, alloca, &format!("{}_save", name))
                        .unwrap();

                    let offset_val = i64_type.const_int(info.offset as u64, false);
                    let var_ptr = unsafe {
                        self.cg
                            .builder
                            .build_gep(
                                self.cg.ctx.i8_type(),
                                frame_ptr,
                                &[offset_val],
                                &format!("{}_frame_ptr", name),
                            )
                            .unwrap()
                    };

                    self.cg.builder.build_store(var_ptr, current_val).unwrap();
                }
            }
        }

        // Increment yield index
        *yield_index += 1;

        // Set yield point for next resume
        let set_yield_point_fn = self.get_or_declare_generator_fn("generator_set_yield_point");
        let next_point = i64_type.const_int(*yield_index as u64, false);
        self.cg
            .builder
            .build_call(set_yield_point_fn, &[gen_ptr.into(), next_point.into()], "")
            .unwrap();

        // Return 1 (has value)
        let one = i64_type.const_int(1, false);
        self.cg.builder.build_return(Some(&one)).unwrap();

        // Now generate the resume block - this is where we continue after yield
        if *yield_index < resume_blocks.len() {
            self.cg.builder.position_at_end(resume_blocks[*yield_index]);

            // First, reload all variables from the frame into the allocas
            // This is needed because we jumped here directly via switch, bypassing
            // the code that loaded variables at the start of resume_blocks[0]
            for (name, info) in var_infos {
                if let Some(var) = self.variables.get(name) {
                    if let Some(alloca) = var.ptr() {
                        let llvm_type = info.py_type.to_llvm(self.cg.ctx);
                        let offset_val = i64_type.const_int(info.offset as u64, false);
                        let var_ptr = unsafe {
                            self.cg
                                .builder
                                .build_gep(
                                    self.cg.ctx.i8_type(),
                                    frame_ptr,
                                    &[offset_val],
                                    &format!("{}_reload_ptr", name),
                                )
                                .unwrap()
                        };
                        let loaded_value = self
                            .cg
                            .builder
                            .build_load(llvm_type, var_ptr, &format!("{}_reload", name))
                            .unwrap();
                        self.cg.builder.build_store(alloca, loaded_value).unwrap();
                    }
                }
            }

            // Generate the statements that come after the yield in the loop body
            for stmt in post_yield_stmts {
                self.generate_statement_with_yield(
                    stmt,
                    gen_ptr,
                    yield_index,
                    resume_blocks,
                    done_bb,
                    frame_ptr,
                    var_infos,
                )?;
                if self.is_block_terminated() {
                    break;
                }
            }

            // Then jump back to while condition
            if !self.is_block_terminated() {
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();
            }
        }

        Ok(())
    }

    /// Generate for statement with yield support
    #[allow(clippy::too_many_arguments)]
    fn generate_for_with_yield(
        &mut self,
        target: &str,
        iter: &Expression,
        body: &[Statement],
        gen_ptr: PointerValue<'ctx>,
        yield_index: &mut usize,
        resume_blocks: &[BasicBlock<'ctx>],
        done_bb: BasicBlock<'ctx>,
        frame_ptr: PointerValue<'ctx>,
        var_infos: &HashMap<String, LocalVarInfo>,
    ) -> Result<(), String> {
        // Use normal for loop generation but with yield-aware body
        let iter_val = self.evaluate_expression(iter)?;

        match iter_val.ty() {
            PyType::Instance(inst) if inst.class_name == iter_names::RANGE => self
                .generate_range_for_with_yield(
                    target,
                    iter_val,
                    body,
                    gen_ptr,
                    yield_index,
                    resume_blocks,
                    done_bb,
                    frame_ptr,
                    var_infos,
                ),
            PyType::List(elem_type) => self.generate_list_for_with_yield(
                target,
                iter_val,
                &elem_type,
                body,
                gen_ptr,
                yield_index,
                resume_blocks,
                done_bb,
                frame_ptr,
                var_infos,
            ),
            _ => Err(format!(
                "Unsupported iterable type in generator: {:?}",
                iter_val.ty()
            )),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn generate_range_for_with_yield(
        &mut self,
        target: &str,
        range_val: PyValue<'ctx>,
        body: &[Statement],
        gen_ptr: PointerValue<'ctx>,
        yield_index: &mut usize,
        resume_blocks: &[BasicBlock<'ctx>],
        done_bb: BasicBlock<'ctx>,
        frame_ptr: PointerValue<'ctx>,
        var_infos: &HashMap<String, LocalVarInfo>,
    ) -> Result<(), String> {
        let function = self.current_function.unwrap();
        let i64_type = self.cg.ctx.i64_type();
        let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());

        // Check if there's a yield in the body
        let has_yield = self.body_has_yield(body);

        if has_yield {
            // For generators with yield in the for loop, we need to:
            // 1. Store iterator pointer in the frame
            // 2. On resume, recreate the iterator state from the frame

            // Get or create allocas for for-loop state
            // These may already exist if created by generate_generator_resume_fn from var_infos
            let iter_alloca = if let Some(var) = self.variables.get("__range_iter") {
                var.ptr().unwrap()
            } else {
                self.create_entry_block_alloca_with_type("__range_iter", ptr_type.into())
            };
            let out_value_alloca = if let Some(var) = self.variables.get("__range_out") {
                var.ptr().unwrap()
            } else {
                self.create_entry_block_alloca_with_type("__range_out", i64_type.into())
            };
            let target_alloca = self.create_entry_block_alloca_with_type(target, i64_type.into());

            // Create iterator
            let range_iter_fn = self.get_or_declare_c_builtin("range_iter");
            let iter_call = self
                .cg
                .builder
                .build_call(range_iter_fn, &[range_val.value().into()], "range_iter")
                .unwrap();
            let iter_ptr = iter_call.as_any_value_enum().into_pointer_value();

            // Store iterator pointer
            self.cg.builder.build_store(iter_alloca, iter_ptr).unwrap();

            // Update variables map with current allocas
            let iter_ptr_as_int = self
                .cg
                .builder
                .build_ptr_to_int(iter_ptr, i64_type, "iter_as_int")
                .unwrap();
            self.variables.insert(
                "__range_iter".to_string(),
                PyValue::new(iter_ptr_as_int.into(), PyType::Int, Some(iter_alloca)),
            );
            self.variables.insert(
                "__range_out".to_string(),
                PyValue::new(
                    i64_type.const_zero().into(),
                    PyType::Int,
                    Some(out_value_alloca),
                ),
            );

            let cond_bb = self.cg.ctx.append_basic_block(function, "for_cond");
            let body_bb = self.cg.ctx.append_basic_block(function, "for_body");
            let after_bb = self.cg.ctx.append_basic_block(function, "for_after");

            self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

            // Condition - load iter_ptr from alloca
            self.cg.builder.position_at_end(cond_bb);
            let iter_ptr_loaded = self
                .cg
                .builder
                .build_load(ptr_type, iter_alloca, "iter_ptr")
                .unwrap()
                .into_pointer_value();

            let range_iter_next_fn = self.get_or_declare_c_builtin("range_iter_next");
            let has_next_call = self
                .cg
                .builder
                .build_call(
                    range_iter_next_fn,
                    &[iter_ptr_loaded.into(), out_value_alloca.into()],
                    "has_next",
                )
                .unwrap();
            let has_next = has_next_call.as_any_value_enum().into_int_value();
            let zero = i64_type.const_zero();
            let cond_bool = self
                .cg
                .builder
                .build_int_compare(inkwell::IntPredicate::NE, has_next, zero, "cond")
                .unwrap();
            self.cg
                .builder
                .build_conditional_branch(cond_bool, body_bb, after_bb)
                .unwrap();

            // Body
            self.cg.builder.position_at_end(body_bb);
            let current_val = self
                .cg
                .builder
                .build_load(i64_type, out_value_alloca, "current")
                .unwrap();
            self.cg
                .builder
                .build_store(target_alloca, current_val)
                .unwrap();

            let loop_var = PyValue::new(current_val, PyType::Int, Some(target_alloca));
            self.variables.insert(target.to_string(), loop_var);

            // Find yield statement index
            let mut yield_stmt_idx = None;
            for (i, stmt) in body.iter().enumerate() {
                if matches!(stmt, Statement::Expr(Expression::Yield { .. })) {
                    yield_stmt_idx = Some(i);
                    break;
                }
            }

            if let Some(idx) = yield_stmt_idx {
                // Generate statements before yield
                for stmt in &body[..idx] {
                    self.generate_statement_with_yield(
                        stmt,
                        gen_ptr,
                        yield_index,
                        resume_blocks,
                        done_bb,
                        frame_ptr,
                        var_infos,
                    )?;
                    if self.is_block_terminated() {
                        break;
                    }
                }

                // Generate the yield with for-loop continuation
                if !self.is_block_terminated() {
                    if let Statement::Expr(Expression::Yield { value, is_from }) = &body[idx] {
                        self.generate_yield_for_range(
                            value.as_deref(),
                            *is_from,
                            gen_ptr,
                            yield_index,
                            resume_blocks,
                            frame_ptr,
                            var_infos,
                            &body[idx + 1..],
                            cond_bb,
                            done_bb,
                            iter_alloca,
                            out_value_alloca,
                            target_alloca,
                            target,
                        )?;
                    }
                }
            } else {
                // No yield in body directly - recursively generate
                for stmt in body {
                    self.generate_statement_with_yield(
                        stmt,
                        gen_ptr,
                        yield_index,
                        resume_blocks,
                        done_bb,
                        frame_ptr,
                        var_infos,
                    )?;
                    if self.is_block_terminated() {
                        break;
                    }
                }
                if !self.is_block_terminated() {
                    self.cg.builder.build_unconditional_branch(cond_bb).unwrap();
                }
            }

            // After
            self.cg.builder.position_at_end(after_bb);
            let iter_ptr_final = self
                .cg
                .builder
                .build_load(ptr_type, iter_alloca, "iter_ptr_final")
                .unwrap()
                .into_pointer_value();
            let range_iter_free_fn = self.get_or_declare_c_builtin("range_iter_free");
            self.cg
                .builder
                .build_call(range_iter_free_fn, &[iter_ptr_final.into()], "")
                .unwrap();

            Ok(())
        } else {
            // No yield in body - use simple implementation
            // Create iterator
            let range_iter_fn = self.get_or_declare_c_builtin("range_iter");
            let iter_call = self
                .cg
                .builder
                .build_call(range_iter_fn, &[range_val.value().into()], "range_iter")
                .unwrap();
            let iter_ptr = iter_call.as_any_value_enum().into_pointer_value();

            // Allocate loop variable
            let target_alloca = self.create_entry_block_alloca_with_type(target, i64_type.into());
            let out_value_alloca = self.cg.builder.build_alloca(i64_type, "iter_out").unwrap();

            let cond_bb = self.cg.ctx.append_basic_block(function, "for_cond");
            let body_bb = self.cg.ctx.append_basic_block(function, "for_body");
            let after_bb = self.cg.ctx.append_basic_block(function, "for_after");

            self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

            // Condition
            self.cg.builder.position_at_end(cond_bb);
            let range_iter_next_fn = self.get_or_declare_c_builtin("range_iter_next");
            let has_next_call = self
                .cg
                .builder
                .build_call(
                    range_iter_next_fn,
                    &[iter_ptr.into(), out_value_alloca.into()],
                    "has_next",
                )
                .unwrap();
            let has_next = has_next_call.as_any_value_enum().into_int_value();
            let zero = i64_type.const_zero();
            let cond_bool = self
                .cg
                .builder
                .build_int_compare(inkwell::IntPredicate::NE, has_next, zero, "cond")
                .unwrap();
            self.cg
                .builder
                .build_conditional_branch(cond_bool, body_bb, after_bb)
                .unwrap();

            // Body
            self.cg.builder.position_at_end(body_bb);
            let current_val = self
                .cg
                .builder
                .build_load(i64_type, out_value_alloca, "current")
                .unwrap();
            self.cg
                .builder
                .build_store(target_alloca, current_val)
                .unwrap();

            let loop_var = PyValue::new(current_val, PyType::Int, Some(target_alloca));
            self.variables.insert(target.to_string(), loop_var);

            for stmt in body {
                self.generate_statement_with_yield(
                    stmt,
                    gen_ptr,
                    yield_index,
                    resume_blocks,
                    done_bb,
                    frame_ptr,
                    var_infos,
                )?;
                if self.is_block_terminated() {
                    break;
                }
            }
            if !self.is_block_terminated() {
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();
            }

            // After
            self.cg.builder.position_at_end(after_bb);
            let range_iter_free_fn = self.get_or_declare_c_builtin("range_iter_free");
            self.cg
                .builder
                .build_call(range_iter_free_fn, &[iter_ptr.into()], "")
                .unwrap();

            Ok(())
        }
    }

    /// Generate yield inside a range for loop with proper continuation
    #[allow(clippy::too_many_arguments)]
    fn generate_yield_for_range(
        &mut self,
        value: Option<&Expression>,
        is_from: bool,
        gen_ptr: PointerValue<'ctx>,
        yield_index: &mut usize,
        resume_blocks: &[BasicBlock<'ctx>],
        frame_ptr: PointerValue<'ctx>,
        var_infos: &HashMap<String, LocalVarInfo>,
        post_yield_stmts: &[Statement],
        cond_bb: BasicBlock<'ctx>,
        done_bb: BasicBlock<'ctx>,
        iter_alloca: PointerValue<'ctx>,
        out_value_alloca: PointerValue<'ctx>,
        target_alloca: PointerValue<'ctx>,
        target_name: &str,
    ) -> Result<(), String> {
        let i64_type = self.cg.ctx.i64_type();
        let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());

        // Generate the yield value
        if is_from {
            if let Some(iter_expr) = value {
                let iter_val = self.evaluate_expression(iter_expr)?;
                let yield_value_fn = self.get_or_declare_generator_fn("generator_yield_value");
                self.cg
                    .builder
                    .build_call(
                        yield_value_fn,
                        &[gen_ptr.into(), iter_val.value().into()],
                        "",
                    )
                    .unwrap();
            }
        } else {
            let yield_val = if let Some(val_expr) = value {
                let val = self.evaluate_expression(val_expr)?;
                val.value().into_int_value()
            } else {
                i64_type.const_zero()
            };

            let yield_value_fn = self.get_or_declare_generator_fn("generator_yield_value");
            self.cg
                .builder
                .build_call(yield_value_fn, &[gen_ptr.into(), yield_val.into()], "")
                .unwrap();
        }

        // Save ALL local variables to the frame
        for (name, info) in var_infos {
            if let Some(var) = self.variables.get(name) {
                if let Some(alloca) = var.ptr() {
                    let llvm_type = info.py_type.to_llvm(self.cg.ctx);
                    let current_val = self
                        .cg
                        .builder
                        .build_load(llvm_type, alloca, &format!("{}_save", name))
                        .unwrap();

                    let offset_val = i64_type.const_int(info.offset as u64, false);
                    let var_ptr = unsafe {
                        self.cg
                            .builder
                            .build_gep(
                                self.cg.ctx.i8_type(),
                                frame_ptr,
                                &[offset_val],
                                &format!("{}_frame_ptr", name),
                            )
                            .unwrap()
                    };

                    self.cg.builder.build_store(var_ptr, current_val).unwrap();
                }
            }
        }

        // Also save the iterator pointer to the frame (using a synthetic variable offset)
        // Find the offset for __for_iter_0 (or similar)
        // For now, save it at a fixed offset after all other variables
        let iter_frame_offset = (var_infos.len() as i64) * 8;
        let out_frame_offset = iter_frame_offset + 8;

        // Save iter_alloca value
        let iter_ptr_val = self
            .cg
            .builder
            .build_load(ptr_type, iter_alloca, "iter_save")
            .unwrap();
        let iter_offset_val = i64_type.const_int(iter_frame_offset as u64, false);
        let iter_frame_ptr = unsafe {
            self.cg
                .builder
                .build_gep(
                    self.cg.ctx.i8_type(),
                    frame_ptr,
                    &[iter_offset_val],
                    "iter_frame_ptr",
                )
                .unwrap()
        };
        self.cg
            .builder
            .build_store(iter_frame_ptr, iter_ptr_val)
            .unwrap();

        // Save out_value_alloca value
        let out_val = self
            .cg
            .builder
            .build_load(i64_type, out_value_alloca, "out_save")
            .unwrap();
        let out_offset_val = i64_type.const_int(out_frame_offset as u64, false);
        let out_frame_ptr = unsafe {
            self.cg
                .builder
                .build_gep(
                    self.cg.ctx.i8_type(),
                    frame_ptr,
                    &[out_offset_val],
                    "out_frame_ptr",
                )
                .unwrap()
        };
        self.cg.builder.build_store(out_frame_ptr, out_val).unwrap();

        // Increment yield index
        *yield_index += 1;

        // Set yield point for next resume
        let set_yield_point_fn = self.get_or_declare_generator_fn("generator_set_yield_point");
        let next_point = i64_type.const_int(*yield_index as u64, false);
        self.cg
            .builder
            .build_call(set_yield_point_fn, &[gen_ptr.into(), next_point.into()], "")
            .unwrap();

        // Return 1 (has value)
        let one = i64_type.const_int(1, false);
        self.cg.builder.build_return(Some(&one)).unwrap();

        // Generate the resume block
        if *yield_index < resume_blocks.len() {
            self.cg.builder.position_at_end(resume_blocks[*yield_index]);

            // Reload all variables from the frame
            for (name, info) in var_infos {
                if let Some(var) = self.variables.get(name) {
                    if let Some(alloca) = var.ptr() {
                        let llvm_type = info.py_type.to_llvm(self.cg.ctx);
                        let offset_val = i64_type.const_int(info.offset as u64, false);
                        let var_ptr = unsafe {
                            self.cg
                                .builder
                                .build_gep(
                                    self.cg.ctx.i8_type(),
                                    frame_ptr,
                                    &[offset_val],
                                    &format!("{}_reload_ptr", name),
                                )
                                .unwrap()
                        };
                        let loaded_value = self
                            .cg
                            .builder
                            .build_load(llvm_type, var_ptr, &format!("{}_reload", name))
                            .unwrap();
                        self.cg.builder.build_store(alloca, loaded_value).unwrap();
                    }
                }
            }

            // Restore iterator pointer from frame
            let iter_offset_val = i64_type.const_int(iter_frame_offset as u64, false);
            let iter_frame_ptr = unsafe {
                self.cg
                    .builder
                    .build_gep(
                        self.cg.ctx.i8_type(),
                        frame_ptr,
                        &[iter_offset_val],
                        "iter_reload_ptr",
                    )
                    .unwrap()
            };
            let iter_restored = self
                .cg
                .builder
                .build_load(ptr_type, iter_frame_ptr, "iter_restored")
                .unwrap();
            self.cg
                .builder
                .build_store(iter_alloca, iter_restored)
                .unwrap();

            // Restore out value from frame
            let out_offset_val = i64_type.const_int(out_frame_offset as u64, false);
            let out_frame_ptr = unsafe {
                self.cg
                    .builder
                    .build_gep(
                        self.cg.ctx.i8_type(),
                        frame_ptr,
                        &[out_offset_val],
                        "out_reload_ptr",
                    )
                    .unwrap()
            };
            let out_restored = self
                .cg
                .builder
                .build_load(i64_type, out_frame_ptr, "out_restored")
                .unwrap();
            self.cg
                .builder
                .build_store(out_value_alloca, out_restored)
                .unwrap();

            // Restore target variable
            let target_val = self
                .cg
                .builder
                .build_load(i64_type, out_value_alloca, "target_reload")
                .unwrap();
            self.cg
                .builder
                .build_store(target_alloca, target_val)
                .unwrap();

            // Update loop variable in variables map
            let loop_var = PyValue::new(target_val, PyType::Int, Some(target_alloca));
            self.variables.insert(target_name.to_string(), loop_var);

            // Generate post-yield statements
            for stmt in post_yield_stmts {
                self.generate_statement_with_yield(
                    stmt,
                    gen_ptr,
                    yield_index,
                    resume_blocks,
                    done_bb,
                    frame_ptr,
                    var_infos,
                )?;
                if self.is_block_terminated() {
                    break;
                }
            }

            // Jump back to condition
            if !self.is_block_terminated() {
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn generate_list_for_with_yield(
        &mut self,
        target: &str,
        list_val: PyValue<'ctx>,
        elem_type: &PyType,
        body: &[Statement],
        gen_ptr: PointerValue<'ctx>,
        yield_index: &mut usize,
        resume_blocks: &[BasicBlock<'ctx>],
        done_bb: BasicBlock<'ctx>,
        frame_ptr: PointerValue<'ctx>,
        var_infos: &HashMap<String, LocalVarInfo>,
    ) -> Result<(), String> {
        let function = self.current_function.unwrap();
        let i64_type = self.cg.ctx.i64_type();
        let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());

        // Check if there's a yield in the body
        let has_yield = self.body_has_yield(body);

        if has_yield {
            // Get or create allocas for for-loop state
            // These may already exist if created by generate_generator_resume_fn from var_infos
            let index_alloca = if let Some(var) = self.variables.get("__list_index") {
                var.ptr().unwrap()
            } else {
                self.create_entry_block_alloca_with_type("__list_index", i64_type.into())
            };
            let len_alloca = if let Some(var) = self.variables.get("__list_len") {
                var.ptr().unwrap()
            } else {
                self.create_entry_block_alloca_with_type("__list_len", i64_type.into())
            };
            let list_ptr_alloca = if let Some(var) = self.variables.get("__list_ptr") {
                var.ptr().unwrap()
            } else {
                self.create_entry_block_alloca_with_type("__list_ptr", ptr_type.into())
            };
            let elem_llvm_type = elem_type.to_llvm(self.cg.ctx);
            let target_alloca = self.create_entry_block_alloca_with_type(target, elem_llvm_type);

            // Get list length and store it
            let list_len_fn = self.get_or_declare_c_builtin("list_len");
            let len_call = self
                .cg
                .builder
                .build_call(list_len_fn, &[list_val.value().into()], "list_len")
                .unwrap();
            let len = len_call.as_any_value_enum().into_int_value();

            // Store list pointer and length
            self.cg
                .builder
                .build_store(list_ptr_alloca, list_val.value())
                .unwrap();
            self.cg.builder.build_store(len_alloca, len).unwrap();
            self.cg
                .builder
                .build_store(index_alloca, i64_type.const_zero())
                .unwrap();

            // Update variables map with current allocas (they may have been created by resume_fn)
            self.variables.insert(
                "__list_index".to_string(),
                PyValue::new(
                    i64_type.const_zero().into(),
                    PyType::Int,
                    Some(index_alloca),
                ),
            );
            self.variables.insert(
                "__list_len".to_string(),
                PyValue::new(len.into(), PyType::Int, Some(len_alloca)),
            );
            // For pointer, we need to cast it to i64 for storage, but we still use Int type
            let list_ptr_as_int = self
                .cg
                .builder
                .build_ptr_to_int(
                    list_val.value().into_pointer_value(),
                    i64_type,
                    "list_as_int",
                )
                .unwrap();
            self.variables.insert(
                "__list_ptr".to_string(),
                PyValue::new(list_ptr_as_int.into(), PyType::Int, Some(list_ptr_alloca)),
            );

            let cond_bb = self.cg.ctx.append_basic_block(function, "for_cond");
            let body_bb = self.cg.ctx.append_basic_block(function, "for_body");
            let incr_bb = self.cg.ctx.append_basic_block(function, "for_incr");
            let after_bb = self.cg.ctx.append_basic_block(function, "for_after");

            self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

            // Condition - load index and len from allocas
            self.cg.builder.position_at_end(cond_bb);
            let current_index = self
                .cg
                .builder
                .build_load(i64_type, index_alloca, "idx")
                .unwrap()
                .into_int_value();
            let len_loaded = self
                .cg
                .builder
                .build_load(i64_type, len_alloca, "len")
                .unwrap()
                .into_int_value();
            let cond_bool = self
                .cg
                .builder
                .build_int_compare(
                    inkwell::IntPredicate::SLT,
                    current_index,
                    len_loaded,
                    "cond",
                )
                .unwrap();
            self.cg
                .builder
                .build_conditional_branch(cond_bool, body_bb, after_bb)
                .unwrap();

            // Body
            self.cg.builder.position_at_end(body_bb);
            let list_ptr_loaded = self
                .cg
                .builder
                .build_load(ptr_type, list_ptr_alloca, "list")
                .unwrap();
            let current_index = self
                .cg
                .builder
                .build_load(i64_type, index_alloca, "idx")
                .unwrap()
                .into_int_value();
            let elem_val = self.list_getitem(list_ptr_loaded, current_index.into(), elem_type)?;
            self.cg
                .builder
                .build_store(target_alloca, elem_val.value())
                .unwrap();

            let loop_var = PyValue::new(elem_val.value(), elem_type.clone(), Some(target_alloca));
            self.variables.insert(target.to_string(), loop_var);

            // Find yield statement index
            let mut yield_stmt_idx = None;
            for (i, stmt) in body.iter().enumerate() {
                if matches!(stmt, Statement::Expr(Expression::Yield { .. })) {
                    yield_stmt_idx = Some(i);
                    break;
                }
            }

            if let Some(idx) = yield_stmt_idx {
                // Generate statements before yield
                for stmt in &body[..idx] {
                    self.generate_statement_with_yield(
                        stmt,
                        gen_ptr,
                        yield_index,
                        resume_blocks,
                        done_bb,
                        frame_ptr,
                        var_infos,
                    )?;
                    if self.is_block_terminated() {
                        break;
                    }
                }

                // Generate the yield with for-loop continuation
                if !self.is_block_terminated() {
                    if let Statement::Expr(Expression::Yield { value, is_from }) = &body[idx] {
                        self.generate_yield_for_list(
                            value.as_deref(),
                            *is_from,
                            gen_ptr,
                            yield_index,
                            resume_blocks,
                            frame_ptr,
                            var_infos,
                            &body[idx + 1..],
                            cond_bb,
                            incr_bb,
                            done_bb,
                            index_alloca,
                            len_alloca,
                            list_ptr_alloca,
                            target_alloca,
                            target,
                            elem_type,
                        )?;
                    }
                }
            } else {
                // No yield in body directly
                for stmt in body {
                    self.generate_statement_with_yield(
                        stmt,
                        gen_ptr,
                        yield_index,
                        resume_blocks,
                        done_bb,
                        frame_ptr,
                        var_infos,
                    )?;
                    if self.is_block_terminated() {
                        break;
                    }
                }
                if !self.is_block_terminated() {
                    self.cg.builder.build_unconditional_branch(incr_bb).unwrap();
                }
            }

            // Increment
            self.cg.builder.position_at_end(incr_bb);
            let current_index = self
                .cg
                .builder
                .build_load(i64_type, index_alloca, "idx")
                .unwrap()
                .into_int_value();
            let next_index = self
                .cg
                .builder
                .build_int_add(current_index, i64_type.const_int(1, false), "next_idx")
                .unwrap();
            self.cg
                .builder
                .build_store(index_alloca, next_index)
                .unwrap();
            self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

            // After
            self.cg.builder.position_at_end(after_bb);

            Ok(())
        } else {
            // No yield in body - use simple implementation
            // Get list length
            let list_len_fn = self.get_or_declare_c_builtin("list_len");
            let len_call = self
                .cg
                .builder
                .build_call(list_len_fn, &[list_val.value().into()], "list_len")
                .unwrap();
            let len = len_call.as_any_value_enum().into_int_value();

            // Allocate index and loop variable
            let index_alloca = self.cg.builder.build_alloca(i64_type, "for_index").unwrap();
            self.cg
                .builder
                .build_store(index_alloca, i64_type.const_zero())
                .unwrap();

            let elem_llvm_type = elem_type.to_llvm(self.cg.ctx);
            let target_alloca = self.create_entry_block_alloca_with_type(target, elem_llvm_type);

            let cond_bb = self.cg.ctx.append_basic_block(function, "for_cond");
            let body_bb = self.cg.ctx.append_basic_block(function, "for_body");
            let incr_bb = self.cg.ctx.append_basic_block(function, "for_incr");
            let after_bb = self.cg.ctx.append_basic_block(function, "for_after");

            self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

            // Condition
            self.cg.builder.position_at_end(cond_bb);
            let current_index = self
                .cg
                .builder
                .build_load(i64_type, index_alloca, "idx")
                .unwrap()
                .into_int_value();
            let cond_bool = self
                .cg
                .builder
                .build_int_compare(inkwell::IntPredicate::SLT, current_index, len, "cond")
                .unwrap();
            self.cg
                .builder
                .build_conditional_branch(cond_bool, body_bb, after_bb)
                .unwrap();

            // Body
            self.cg.builder.position_at_end(body_bb);
            let elem_val = self.list_getitem(list_val.value(), current_index.into(), elem_type)?;
            self.cg
                .builder
                .build_store(target_alloca, elem_val.value())
                .unwrap();

            let loop_var = PyValue::new(elem_val.value(), elem_type.clone(), Some(target_alloca));
            self.variables.insert(target.to_string(), loop_var);

            for stmt in body {
                self.generate_statement_with_yield(
                    stmt,
                    gen_ptr,
                    yield_index,
                    resume_blocks,
                    done_bb,
                    frame_ptr,
                    var_infos,
                )?;
                if self.is_block_terminated() {
                    break;
                }
            }
            if !self.is_block_terminated() {
                self.cg.builder.build_unconditional_branch(incr_bb).unwrap();
            }

            // Increment
            self.cg.builder.position_at_end(incr_bb);
            let current_index = self
                .cg
                .builder
                .build_load(i64_type, index_alloca, "idx")
                .unwrap()
                .into_int_value();
            let next_index = self
                .cg
                .builder
                .build_int_add(current_index, i64_type.const_int(1, false), "next_idx")
                .unwrap();
            self.cg
                .builder
                .build_store(index_alloca, next_index)
                .unwrap();
            self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

            // After
            self.cg.builder.position_at_end(after_bb);

            Ok(())
        }
    }

    /// Generate yield inside a list for loop with proper continuation
    #[allow(clippy::too_many_arguments)]
    fn generate_yield_for_list(
        &mut self,
        value: Option<&Expression>,
        is_from: bool,
        gen_ptr: PointerValue<'ctx>,
        yield_index: &mut usize,
        resume_blocks: &[BasicBlock<'ctx>],
        frame_ptr: PointerValue<'ctx>,
        var_infos: &HashMap<String, LocalVarInfo>,
        post_yield_stmts: &[Statement],
        _cond_bb: BasicBlock<'ctx>,
        incr_bb: BasicBlock<'ctx>,
        done_bb: BasicBlock<'ctx>,
        index_alloca: PointerValue<'ctx>,
        len_alloca: PointerValue<'ctx>,
        list_ptr_alloca: PointerValue<'ctx>,
        target_alloca: PointerValue<'ctx>,
        target_name: &str,
        elem_type: &PyType,
    ) -> Result<(), String> {
        let i64_type = self.cg.ctx.i64_type();
        let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());

        // Generate the yield value
        if is_from {
            if let Some(iter_expr) = value {
                let iter_val = self.evaluate_expression(iter_expr)?;
                let yield_value_fn = self.get_or_declare_generator_fn("generator_yield_value");
                self.cg
                    .builder
                    .build_call(
                        yield_value_fn,
                        &[gen_ptr.into(), iter_val.value().into()],
                        "",
                    )
                    .unwrap();
            }
        } else {
            let yield_val = if let Some(val_expr) = value {
                let val = self.evaluate_expression(val_expr)?;
                val.value().into_int_value()
            } else {
                i64_type.const_zero()
            };

            let yield_value_fn = self.get_or_declare_generator_fn("generator_yield_value");
            self.cg
                .builder
                .build_call(yield_value_fn, &[gen_ptr.into(), yield_val.into()], "")
                .unwrap();
        }

        // Save ALL local variables to the frame
        for (name, info) in var_infos {
            if let Some(var) = self.variables.get(name) {
                if let Some(alloca) = var.ptr() {
                    let llvm_type = info.py_type.to_llvm(self.cg.ctx);
                    let current_val = self
                        .cg
                        .builder
                        .build_load(llvm_type, alloca, &format!("{}_save", name))
                        .unwrap();

                    let offset_val = i64_type.const_int(info.offset as u64, false);
                    let var_ptr = unsafe {
                        self.cg
                            .builder
                            .build_gep(
                                self.cg.ctx.i8_type(),
                                frame_ptr,
                                &[offset_val],
                                &format!("{}_frame_ptr", name),
                            )
                            .unwrap()
                    };

                    self.cg.builder.build_store(var_ptr, current_val).unwrap();
                }
            }
        }

        // Save list iteration state to the frame
        let index_frame_offset = (var_infos.len() as i64) * 8;
        let len_frame_offset = index_frame_offset + 8;
        let list_ptr_frame_offset = len_frame_offset + 8;

        // Save index
        let index_val = self
            .cg
            .builder
            .build_load(i64_type, index_alloca, "idx_save")
            .unwrap();
        let index_offset_val = i64_type.const_int(index_frame_offset as u64, false);
        let index_frame_ptr = unsafe {
            self.cg
                .builder
                .build_gep(
                    self.cg.ctx.i8_type(),
                    frame_ptr,
                    &[index_offset_val],
                    "idx_frame_ptr",
                )
                .unwrap()
        };
        self.cg
            .builder
            .build_store(index_frame_ptr, index_val)
            .unwrap();

        // Save len
        let len_val = self
            .cg
            .builder
            .build_load(i64_type, len_alloca, "len_save")
            .unwrap();
        let len_offset_val = i64_type.const_int(len_frame_offset as u64, false);
        let len_frame_ptr = unsafe {
            self.cg
                .builder
                .build_gep(
                    self.cg.ctx.i8_type(),
                    frame_ptr,
                    &[len_offset_val],
                    "len_frame_ptr",
                )
                .unwrap()
        };
        self.cg.builder.build_store(len_frame_ptr, len_val).unwrap();

        // Save list_ptr
        let list_ptr_val = self
            .cg
            .builder
            .build_load(ptr_type, list_ptr_alloca, "list_save")
            .unwrap();
        let list_ptr_offset_val = i64_type.const_int(list_ptr_frame_offset as u64, false);
        let list_ptr_frame_ptr = unsafe {
            self.cg
                .builder
                .build_gep(
                    self.cg.ctx.i8_type(),
                    frame_ptr,
                    &[list_ptr_offset_val],
                    "list_frame_ptr",
                )
                .unwrap()
        };
        self.cg
            .builder
            .build_store(list_ptr_frame_ptr, list_ptr_val)
            .unwrap();

        // Increment yield index
        *yield_index += 1;

        // Set yield point for next resume
        let set_yield_point_fn = self.get_or_declare_generator_fn("generator_set_yield_point");
        let next_point = i64_type.const_int(*yield_index as u64, false);
        self.cg
            .builder
            .build_call(set_yield_point_fn, &[gen_ptr.into(), next_point.into()], "")
            .unwrap();

        // Return 1 (has value)
        let one = i64_type.const_int(1, false);
        self.cg.builder.build_return(Some(&one)).unwrap();

        // Generate the resume block
        if *yield_index < resume_blocks.len() {
            self.cg.builder.position_at_end(resume_blocks[*yield_index]);

            // Reload all variables from the frame
            for (name, info) in var_infos {
                if let Some(var) = self.variables.get(name) {
                    if let Some(alloca) = var.ptr() {
                        let llvm_type = info.py_type.to_llvm(self.cg.ctx);
                        let offset_val = i64_type.const_int(info.offset as u64, false);
                        let var_ptr = unsafe {
                            self.cg
                                .builder
                                .build_gep(
                                    self.cg.ctx.i8_type(),
                                    frame_ptr,
                                    &[offset_val],
                                    &format!("{}_reload_ptr", name),
                                )
                                .unwrap()
                        };
                        let loaded_value = self
                            .cg
                            .builder
                            .build_load(llvm_type, var_ptr, &format!("{}_reload", name))
                            .unwrap();
                        self.cg.builder.build_store(alloca, loaded_value).unwrap();
                    }
                }
            }

            // Restore index from frame
            let index_offset_val = i64_type.const_int(index_frame_offset as u64, false);
            let index_frame_ptr = unsafe {
                self.cg
                    .builder
                    .build_gep(
                        self.cg.ctx.i8_type(),
                        frame_ptr,
                        &[index_offset_val],
                        "idx_reload_ptr",
                    )
                    .unwrap()
            };
            let index_restored = self
                .cg
                .builder
                .build_load(i64_type, index_frame_ptr, "idx_restored")
                .unwrap();
            self.cg
                .builder
                .build_store(index_alloca, index_restored)
                .unwrap();

            // Restore len from frame
            let len_offset_val = i64_type.const_int(len_frame_offset as u64, false);
            let len_frame_ptr = unsafe {
                self.cg
                    .builder
                    .build_gep(
                        self.cg.ctx.i8_type(),
                        frame_ptr,
                        &[len_offset_val],
                        "len_reload_ptr",
                    )
                    .unwrap()
            };
            let len_restored = self
                .cg
                .builder
                .build_load(i64_type, len_frame_ptr, "len_restored")
                .unwrap();
            self.cg
                .builder
                .build_store(len_alloca, len_restored)
                .unwrap();

            // Restore list_ptr from frame
            let list_ptr_offset_val = i64_type.const_int(list_ptr_frame_offset as u64, false);
            let list_ptr_frame_ptr = unsafe {
                self.cg
                    .builder
                    .build_gep(
                        self.cg.ctx.i8_type(),
                        frame_ptr,
                        &[list_ptr_offset_val],
                        "list_reload_ptr",
                    )
                    .unwrap()
            };
            let list_ptr_restored = self
                .cg
                .builder
                .build_load(ptr_type, list_ptr_frame_ptr, "list_restored")
                .unwrap();
            self.cg
                .builder
                .build_store(list_ptr_alloca, list_ptr_restored)
                .unwrap();

            // Restore target variable from list
            let current_index = self
                .cg
                .builder
                .build_load(i64_type, index_alloca, "idx")
                .unwrap()
                .into_int_value();
            let list_ptr_loaded = self
                .cg
                .builder
                .build_load(ptr_type, list_ptr_alloca, "list")
                .unwrap();
            let elem_val = self.list_getitem(list_ptr_loaded, current_index.into(), elem_type)?;
            self.cg
                .builder
                .build_store(target_alloca, elem_val.value())
                .unwrap();

            // Update loop variable in variables map
            let loop_var = PyValue::new(elem_val.value(), elem_type.clone(), Some(target_alloca));
            self.variables.insert(target_name.to_string(), loop_var);

            // Generate post-yield statements
            for stmt in post_yield_stmts {
                self.generate_statement_with_yield(
                    stmt,
                    gen_ptr,
                    yield_index,
                    resume_blocks,
                    done_bb,
                    frame_ptr,
                    var_infos,
                )?;
                if self.is_block_terminated() {
                    break;
                }
            }

            // Jump to increment block
            if !self.is_block_terminated() {
                self.cg.builder.build_unconditional_branch(incr_bb).unwrap();
            }
        }

        Ok(())
    }

    /// Get or declare a generator runtime function
    pub(crate) fn get_or_declare_generator_fn(&mut self, name: &str) -> FunctionValue<'ctx> {
        let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());
        let i64_type = self.cg.ctx.i64_type();
        let void_type = self.cg.ctx.void_type();

        // Use the mangled name with __builtin_tpy_ prefix
        let full_name = format!("__builtin_tpy_{}", name);
        if let Some(f) = self.cg.module.get_function(&full_name) {
            return f;
        }

        // Mark generator module as used
        self.used_builtin_modules.insert("generator".to_string());

        let fn_type = match name {
            "generator_new" => ptr_type.fn_type(&[ptr_type.into(), i64_type.into()], false),
            "generator_free" => void_type.fn_type(&[ptr_type.into()], false),
            "generator_state" => i64_type.fn_type(&[ptr_type.into()], false),
            "generator_frame" => ptr_type.fn_type(&[ptr_type.into()], false),
            "generator_yield_point" => i64_type.fn_type(&[ptr_type.into()], false),
            "generator_set_yield_point" => {
                void_type.fn_type(&[ptr_type.into(), i64_type.into()], false)
            }
            "generator_set_closed" => void_type.fn_type(&[ptr_type.into()], false),
            "generator_next" => i64_type.fn_type(&[ptr_type.into(), ptr_type.into()], false),
            "generator_send" => {
                i64_type.fn_type(&[ptr_type.into(), i64_type.into(), ptr_type.into()], false)
            }
            "generator_yield_value" => {
                void_type.fn_type(&[ptr_type.into(), i64_type.into()], false)
            }
            "generator_get_sent_value" => i64_type.fn_type(&[ptr_type.into()], false),
            "generator_close" => void_type.fn_type(&[ptr_type.into()], false),
            _ => panic!("Unknown generator function: {}", name),
        };

        self.cg.module.add_function(&full_name, fn_type, None)
    }
}
