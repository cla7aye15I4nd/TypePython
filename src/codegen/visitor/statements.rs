/// Statement visitor implementation for code generation
use super::super::CodeGen;
use crate::ast::visitor::Visitor;
use crate::ast::*;
use crate::codegen::types::iter_names;
use crate::types::{InstanceType, PyType, PyValue};
use inkwell::values::AnyValue;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn visit_var_decl_impl(
        &mut self,
        name: &str,
        var_type: &Type,
        value: &Expression,
    ) -> Result<(), String> {
        let val = self.evaluate_expression(value)?;

        // For Instance types, use the type from the evaluated value since it has field info
        // For other types, use the declared type
        let py_type = if matches!(val.ty(), PyType::Instance(_)) {
            val.ty()
        } else {
            PyType::from_ast_type(var_type)?
        };

        // Coerce the value to match the declared type if needed
        let coerced_val = self.coerce_value_to_type(val.value(), var_type)?;

        // Check if this is a module-level variable (has LLVM global)
        if let Some((global_ptr, _)) = self.module_vars.get(name).cloned() {
            // Store to the global variable
            self.cg
                .builder
                .build_store(global_ptr, coerced_val)
                .unwrap();
            // Also create local reference for easier access
            let var = PyValue::new(coerced_val, py_type, Some(global_ptr));
            self.variables.insert(name.to_string(), var);
        } else {
            // Regular local variable
            // Check if an alloca already exists (e.g., from generator resume function setup)
            let alloca = if let Some(existing) = self.variables.get(name) {
                if let Some(ptr) = existing.ptr() {
                    ptr
                } else {
                    let fn_name = self
                        .current_function
                        .unwrap()
                        .get_name()
                        .to_string_lossy()
                        .to_string();
                    self.create_entry_block_alloca(&fn_name, name, &py_type)
                }
            } else {
                let fn_name = self
                    .current_function
                    .unwrap()
                    .get_name()
                    .to_string_lossy()
                    .to_string();
                self.create_entry_block_alloca(&fn_name, name, &py_type)
            };
            self.cg.builder.build_store(alloca, coerced_val).unwrap();
            let var = PyValue::new(coerced_val, py_type, Some(alloca));
            self.variables.insert(name.to_string(), var);
        }
        Ok(())
    }

    pub(crate) fn visit_assignment_impl(
        &mut self,
        target: &Expression,
        value: &Expression,
    ) -> Result<(), String> {
        match target {
            Expression::Var(name) => {
                // First evaluate the value to get its type
                let val = self.evaluate_expression(value)?;

                // If this variable is declared as global, store to module_vars
                if self.global_vars.contains(name) {
                    if let Some((ptr, _py_type)) = self.module_vars.get(name).cloned() {
                        self.cg.builder.build_store(ptr, val.value()).unwrap();
                        return Ok(());
                    }
                    // If global var doesn't exist yet, we need to create it
                    // This happens when global is declared before the module-level var
                }

                // Check if variable exists
                if let Some(var) = self.variables.get(name).cloned() {
                    // Variable exists, store to it
                    var.store_value(&self.cg.builder, &val)?;
                } else {
                    // Variable doesn't exist, create it with inferred type
                    // Functions/Modules are stored directly without alloca
                    match val.ty() {
                        PyType::Function(_) | PyType::Module => {
                            self.variables.insert(name.to_string(), val);
                        }
                        _ => {
                            let llvm_type = val.ty().to_llvm(self.cg.ctx);
                            let alloca = self.create_entry_block_alloca_with_type(name, llvm_type);
                            self.cg.builder.build_store(alloca, val.value()).unwrap();
                            let var = PyValue::new(val.value(), val.ty().clone(), Some(alloca));
                            self.variables.insert(name.to_string(), var);
                        }
                    }
                }
                Ok(())
            }
            Expression::Subscript { object, index } => {
                let obj = self.evaluate_expression(object)?;
                let idx = self.evaluate_expression(index)?;
                let val = self.evaluate_expression(value)?;

                match &obj.ty() {
                    PyType::List(_) => {
                        self.list_setitem(obj.value(), idx.value(), val.value())?;
                        Ok(())
                    }
                    PyType::Dict(key_type, _) => {
                        // Convert value to i64 for storage
                        let val_as_i64 = self.convert_value_to_i64(&val)?;

                        // Select the appropriate setitem function based on key type
                        let setitem_fn = if matches!(key_type.as_ref(), PyType::Str) {
                            self.get_or_declare_c_builtin("str_dict_setitem")
                        } else {
                            self.get_or_declare_c_builtin("dict_setitem")
                        };
                        self.cg
                            .builder
                            .build_call(
                                setitem_fn,
                                &[obj.value().into(), idx.value().into(), val_as_i64.into()],
                                "dict_setitem",
                            )
                            .unwrap();
                        Ok(())
                    }
                    _ => panic!("Subscript assignment not supported for type {:?}", obj.ty()),
                }
            }
            Expression::Attribute { object, attr } => {
                // Attribute assignment to class instances (e.g., self.x = value)
                let obj = self.evaluate_expression(object)?;
                let val = self.evaluate_expression(value)?;

                match obj.ty() {
                    PyType::Instance(ref inst) => {
                        // Find the field index and type
                        let field_idx = inst
                            .fields
                            .iter()
                            .position(|(name, _)| name == attr)
                            .ok_or_else(|| format!("Instance has no field '{}'", attr))?;

                        // Get the instance pointer
                        let instance_ptr = obj.value().into_pointer_value();

                        // Create struct type for GEP
                        let num_fields = inst.fields.len();
                        let struct_type = self
                            .cg
                            .ctx
                            .struct_type(&vec![self.cg.ctx.i64_type().into(); num_fields], false);

                        // Get pointer to the field
                        let field_ptr = self
                            .cg
                            .builder
                            .build_struct_gep(
                                struct_type,
                                instance_ptr,
                                field_idx as u32,
                                &format!("field_{}_ptr", attr),
                            )
                            .unwrap();

                        // Convert value to i64 for storage
                        let val_as_i64 = self.convert_value_to_i64(&val)?;

                        // Store the value
                        self.cg.builder.build_store(field_ptr, val_as_i64).unwrap();
                        Ok(())
                    }
                    _ => Err(format!(
                        "Cannot assign to attribute '{}' on type {:?}",
                        attr,
                        obj.ty()
                    )),
                }
            }
            _ => panic!("Invalid assignment target: {:?}", target),
        }
    }

    pub(crate) fn visit_tuple_unpack_assignment_impl(
        &mut self,
        targets: &[Expression],
        value: &Expression,
    ) -> Result<(), String> {
        // Evaluate the RHS which should be a tuple
        let val = self.evaluate_expression(value)?;

        match val.ty() {
            PyType::Tuple(element_types) => {
                if targets.len() != element_types.len() {
                    return Err(format!(
                        "Tuple unpacking: {} targets but tuple has {} elements",
                        targets.len(),
                        element_types.len()
                    ));
                }

                // Get tuple_getitem function
                let tuple_getitem_fn = self.get_or_declare_c_builtin("tuple_getitem");
                let tuple_ptr = val.value().into_pointer_value();

                // First, extract ALL values from the tuple BEFORE any assignment
                // This is critical for swap patterns like: a, b = b, a
                let mut extracted_values: Vec<PyValue<'ctx>> = Vec::new();

                for (i, _target) in targets.iter().enumerate() {
                    let elem_ty = &element_types[i];
                    let index = self.cg.ctx.i64_type().const_int(i as u64, false);

                    // Call tuple_getitem to get the raw i64 value
                    use inkwell::values::AnyValue;
                    let call_site = self
                        .cg
                        .builder
                        .build_call(
                            tuple_getitem_fn,
                            &[tuple_ptr.into(), index.into()],
                            &format!("tuple_elem_{}", i),
                        )
                        .unwrap();
                    let raw_i64 = call_site.as_any_value_enum().into_int_value();

                    // Convert the raw i64 value back to the appropriate type
                    let elem_val: inkwell::values::BasicValueEnum = match elem_ty {
                        PyType::Float => {
                            // Bitcast i64 back to f64
                            self.cg
                                .builder
                                .build_bit_cast(raw_i64, self.cg.ctx.f64_type(), "i64_as_float")
                                .unwrap()
                        }
                        PyType::Bool => {
                            // Truncate i64 to i1 for bool
                            self.cg
                                .builder
                                .build_int_truncate(raw_i64, self.cg.ctx.bool_type(), "i64_as_bool")
                                .unwrap()
                                .into()
                        }
                        PyType::Str
                        | PyType::Bytes
                        | PyType::List(_)
                        | PyType::Dict(_, _)
                        | PyType::Set(_)
                        | PyType::Tuple(_)
                        | PyType::Instance(_) => {
                            // Int to pointer for reference types
                            self.cg
                                .builder
                                .build_int_to_ptr(
                                    raw_i64,
                                    self.cg.ctx.ptr_type(inkwell::AddressSpace::default()),
                                    "i64_as_ptr",
                                )
                                .unwrap()
                                .into()
                        }
                        _ => {
                            // Int - use directly
                            raw_i64.into()
                        }
                    };

                    extracted_values.push(PyValue::new(elem_val, elem_ty.clone(), None));
                }

                // Now assign all values to their targets
                for (i, target) in targets.iter().enumerate() {
                    self.assign_to_target(target, extracted_values[i].clone())?;
                }

                Ok(())
            }
            _ => Err(format!(
                "Tuple unpacking requires a tuple, got {:?}",
                val.ty()
            )),
        }
    }

    /// Helper function to assign a value to a target expression
    fn assign_to_target(&mut self, target: &Expression, val: PyValue<'ctx>) -> Result<(), String> {
        match target {
            Expression::Var(name) => {
                // Check if variable exists
                if let Some(var) = self.variables.get(name).cloned() {
                    var.store_value(&self.cg.builder, &val)?;
                } else {
                    // Create new variable
                    let llvm_type = val.ty().to_llvm(self.cg.ctx);
                    let alloca = self.create_entry_block_alloca_with_type(name, llvm_type);
                    self.cg.builder.build_store(alloca, val.value()).unwrap();
                    let var = PyValue::new(val.value(), val.ty().clone(), Some(alloca));
                    self.variables.insert(name.to_string(), var);
                }
                Ok(())
            }
            Expression::Attribute { object, attr } => {
                let obj = self.evaluate_expression(object)?;
                match obj.ty() {
                    PyType::Instance(ref inst) => {
                        // Find the field index
                        let field_idx = inst
                            .fields
                            .iter()
                            .position(|(name, _)| name == attr)
                            .ok_or_else(|| format!("Instance has no field '{}'", attr))?;

                        let instance_ptr = obj.value().into_pointer_value();
                        let num_fields = inst.fields.len();
                        let struct_type = self
                            .cg
                            .ctx
                            .struct_type(&vec![self.cg.ctx.i64_type().into(); num_fields], false);

                        let field_ptr = self
                            .cg
                            .builder
                            .build_struct_gep(
                                struct_type,
                                instance_ptr,
                                field_idx as u32,
                                &format!("field_{}_ptr", attr),
                            )
                            .unwrap();

                        let val_as_i64 = self.convert_value_to_i64(&val)?;
                        self.cg.builder.build_store(field_ptr, val_as_i64).unwrap();
                        Ok(())
                    }
                    _ => Err(format!(
                        "Cannot assign to attribute '{}' on type {:?}",
                        attr,
                        obj.ty()
                    )),
                }
            }
            Expression::Subscript { object, index } => {
                let obj = self.evaluate_expression(object)?;
                let idx = self.evaluate_expression(index)?;

                match &obj.ty() {
                    PyType::List(_) => {
                        self.list_setitem(obj.value(), idx.value(), val.value())?;
                        Ok(())
                    }
                    PyType::Dict(key_type, _) => {
                        let val_as_i64 = self.convert_value_to_i64(&val)?;
                        let setitem_fn = if matches!(key_type.as_ref(), PyType::Str) {
                            self.get_or_declare_c_builtin("str_dict_setitem")
                        } else {
                            self.get_or_declare_c_builtin("dict_setitem")
                        };
                        self.cg
                            .builder
                            .build_call(
                                setitem_fn,
                                &[obj.value().into(), idx.value().into(), val_as_i64.into()],
                                "dict_setitem",
                            )
                            .unwrap();
                        Ok(())
                    }
                    _ => Err(format!(
                        "Subscript assignment not supported for type {:?}",
                        obj.ty()
                    )),
                }
            }
            _ => Err(format!("Invalid assignment target: {:?}", target)),
        }
    }

    pub(crate) fn visit_aug_assignment_impl(
        &mut self,
        target: &Expression,
        op: &AugAssignOp,
        value: &Expression,
    ) -> Result<(), String> {
        match target {
            Expression::Var(name) => {
                let var = self
                    .variables
                    .get(name)
                    .ok_or_else(|| format!("Variable {} not found", name))?
                    .clone();

                // Load current value from the addressable PyValue
                let current = var.load(&self.cg.builder, self.cg.ctx, name);

                // Evaluate RHS
                let rhs = self.evaluate_expression(value)?;

                // Convert augmented op to binary op
                let bin_op = match op {
                    AugAssignOp::Add => BinaryOp::Add,
                    AugAssignOp::Sub => BinaryOp::Sub,
                    AugAssignOp::Mul => BinaryOp::Mul,
                    AugAssignOp::Div => BinaryOp::Div,
                    AugAssignOp::FloorDiv => BinaryOp::FloorDiv,
                    AugAssignOp::Mod => BinaryOp::Mod,
                    AugAssignOp::Pow => BinaryOp::Pow,
                    AugAssignOp::BitOr => BinaryOp::BitOr,
                    AugAssignOp::BitXor => BinaryOp::BitXor,
                    AugAssignOp::BitAnd => BinaryOp::BitAnd,
                    AugAssignOp::LShift => BinaryOp::LShift,
                    AugAssignOp::RShift => BinaryOp::RShift,
                };

                // Delegate to the left type's implementation
                let result = current.binary_op(&self.cg, &bin_op, &rhs)?;

                // Store result to the addressable variable
                var.store_value(&self.cg.builder, &result)?;
                Ok(())
            }
            _ => panic!("Augmented assignment only supported for variables"),
        }
    }

    pub(crate) fn visit_break_impl(&mut self) -> Result<(), String> {
        if let Some(loop_ctx) = self.loop_stack.last() {
            self.cg
                .builder
                .build_unconditional_branch(loop_ctx.break_block)
                .unwrap();
            Ok(())
        } else {
            Err("Break statement outside of loop".to_string())
        }
    }

    pub(crate) fn visit_continue_impl(&mut self) -> Result<(), String> {
        if let Some(loop_ctx) = self.loop_stack.last() {
            self.cg
                .builder
                .build_unconditional_branch(loop_ctx.continue_block)
                .unwrap();
            Ok(())
        } else {
            Err("Continue statement outside of loop".to_string())
        }
    }

    pub(crate) fn visit_if_impl(
        &mut self,
        condition: &Expression,
        then_block: &[Statement],
        elif_clauses: &[(Expression, Vec<Statement>)],
        else_block: &Option<Vec<Statement>>,
    ) -> Result<(), String> {
        self.generate_if_statement(condition, then_block, elif_clauses, else_block)
    }

    pub(crate) fn visit_while_impl(
        &mut self,
        condition: &Expression,
        body: &[Statement],
    ) -> Result<(), String> {
        self.generate_while_statement(condition, body)
    }

    pub(crate) fn visit_for_impl(
        &mut self,
        targets: &[String],
        iter: &Expression,
        body: &[Statement],
        else_block: &Option<Vec<Statement>>,
    ) -> Result<(), String> {
        self.generate_for_statement(targets, iter, body, else_block)
    }

    pub(crate) fn visit_return_impl(&mut self, expr: Option<&Expression>) -> Result<(), String> {
        if let Some(expr) = expr {
            let val = self.evaluate_expression(expr)?;
            self.cg.builder.build_return(Some(&val.value())).unwrap();
        } else {
            self.cg.builder.build_return(None).unwrap();
        }
        Ok(())
    }

    pub(crate) fn visit_pass_impl(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub(crate) fn visit_expr_statement_impl(&mut self, expr: &Expression) -> Result<(), String> {
        self.evaluate_expression(expr)?;
        Ok(())
    }

    pub(crate) fn visit_delete_impl(&mut self, target: &Expression) -> Result<(), String> {
        match target {
            Expression::Var(_name) => {
                // del variable - not commonly used, could unset the variable
                // For now, we don't support deleting simple variables
                Err("del on variables is not supported".to_string())
            }
            Expression::Subscript { object, index } => {
                let obj = self.evaluate_expression(object)?;
                let idx = self.evaluate_expression(index)?;

                match &obj.ty() {
                    PyType::List(_) => {
                        self.list_delitem(obj.value(), idx.value())?;
                        Ok(())
                    }
                    PyType::Dict(_, _) => {
                        self.dict_delitem(obj.value(), idx.value())?;
                        Ok(())
                    }
                    _ => panic!("del not supported for type {:?}", obj.ty()),
                }
            }
            _ => panic!("del not supported for target: {:?}", target),
        }
    }

    pub(crate) fn visit_try_impl(
        &mut self,
        body: &[Statement],
        handlers: &[ExceptHandler],
        else_block: &Option<Vec<Statement>>,
        finally_block: &Option<Vec<Statement>>,
    ) -> Result<(), String> {
        let function = self.current_function.unwrap();
        let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());

        // Create basic blocks
        let try_bb = self.cg.ctx.append_basic_block(function, "try_body");
        let handler_dispatch_bb = self.cg.ctx.append_basic_block(function, "handler_dispatch");
        let else_bb = if else_block.is_some() {
            Some(self.cg.ctx.append_basic_block(function, "try_else"))
        } else {
            None
        };
        let finally_bb = if finally_block.is_some() {
            Some(self.cg.ctx.append_basic_block(function, "try_finally"))
        } else {
            None
        };
        let merge_bb = self.cg.ctx.append_basic_block(function, "try_merge");

        // Create handler blocks
        let handler_blocks: Vec<_> = handlers
            .iter()
            .map(|_| self.cg.ctx.append_basic_block(function, "except_handler"))
            .collect();

        // Push exception handler - get jump buffer
        let push_handler_fn = self.get_or_declare_exception_fn("exception_push_handler");
        let call = self
            .cg
            .builder
            .build_call(push_handler_fn, &[], "jmp_buf")
            .unwrap();
        let jmp_buf = extract_ptr_from_call(call, "exception_push_handler");

        // Call setjmp - returns 0 on first call, non-zero when longjmp is called
        let setjmp_fn = self.get_or_declare_setjmp();
        let call = self
            .cg
            .builder
            .build_call(setjmp_fn, &[jmp_buf.into()], "setjmp_result")
            .unwrap();
        let setjmp_result = extract_int_from_call(call, "setjmp");

        // Branch: if setjmp returns 0, execute try body; else handle exception
        let zero = self.cg.ctx.i32_type().const_zero();
        let is_try = self
            .cg
            .builder
            .build_int_compare(inkwell::IntPredicate::EQ, setjmp_result, zero, "is_try")
            .unwrap();
        self.cg
            .builder
            .build_conditional_branch(is_try, try_bb, handler_dispatch_bb)
            .unwrap();

        // Try body block
        self.cg.builder.position_at_end(try_bb);
        for stmt in body {
            if self.is_block_terminated() {
                break;
            }
            self.visit_statement(stmt)?;
        }

        // Pop handler after successful try
        if !self.is_block_terminated() {
            let pop_handler_fn = self.get_or_declare_exception_fn("exception_pop_handler");
            self.cg.builder.build_call(pop_handler_fn, &[], "").unwrap();

            // If no exception, execute else block or go to finally/merge
            let next_bb = else_bb.unwrap_or(finally_bb.unwrap_or(merge_bb));
            self.cg.builder.build_unconditional_branch(next_bb).unwrap();
        }

        // Handler dispatch block - match exception type
        self.cg.builder.position_at_end(handler_dispatch_bb);

        // Get current exception
        let get_current_fn = self.get_or_declare_exception_fn("exception_current");
        let call = self
            .cg
            .builder
            .build_call(get_current_fn, &[], "current_exc")
            .unwrap();
        let current_exc = extract_ptr_from_call(call, "exception_current");

        // Get exception type ID
        let get_type_id_fn = self.get_or_declare_exception_fn("exception_type_id");
        let call = self
            .cg
            .builder
            .build_call(get_type_id_fn, &[current_exc.into()], "exc_type_id")
            .unwrap();
        let exc_type_id = extract_int_from_call(call, "exception_type_id");

        // Pop the handler since we're handling the exception
        let pop_handler_fn = self.get_or_declare_exception_fn("exception_pop_handler");
        self.cg.builder.build_call(pop_handler_fn, &[], "").unwrap();

        // Generate type checks and branches to handlers
        if handlers.is_empty() {
            // No handlers - should not happen in valid Python, but go to finally/merge
            let next_bb = finally_bb.unwrap_or(merge_bb);
            self.cg.builder.build_unconditional_branch(next_bb).unwrap();
        } else {
            // Build switch on exception type
            // For now, use simple linear search with isinstance checks
            let isinstance_fn = self.get_or_declare_exception_fn("exception_isinstance");

            let mut current_bb = handler_dispatch_bb;

            for (i, handler) in handlers.iter().enumerate() {
                if i > 0 {
                    self.cg.builder.position_at_end(current_bb);
                }

                let handler_bb = handler_blocks[i];
                let next_check_bb = if i + 1 < handlers.len() {
                    self.cg
                        .ctx
                        .append_basic_block(function, &format!("check_handler_{}", i + 1))
                } else {
                    // Last handler - if bare except or no match, go to finally/merge
                    finally_bb.unwrap_or(merge_bb)
                };

                if handler.exception_types.is_empty() {
                    // Bare except - catches everything
                    self.cg
                        .builder
                        .build_unconditional_branch(handler_bb)
                        .unwrap();
                } else {
                    // Check if exception matches any of the types
                    let mut matches = self.cg.ctx.bool_type().const_zero();

                    for type_name in &handler.exception_types {
                        let type_id = self.exception_type_id_from_name(type_name);
                        let type_id_val = self.cg.ctx.i64_type().const_int(type_id as u64, false);

                        let call = self
                            .cg
                            .builder
                            .build_call(
                                isinstance_fn,
                                &[current_exc.into(), type_id_val.into()],
                                "is_match",
                            )
                            .unwrap();
                        let is_match = extract_int_from_call(call, "exception_isinstance");

                        // Truncate i64 to i1 for boolean
                        let is_match_bool = self
                            .cg
                            .builder
                            .build_int_truncate(is_match, self.cg.ctx.bool_type(), "is_match_bool")
                            .unwrap();

                        matches = self
                            .cg
                            .builder
                            .build_or(matches, is_match_bool, "matches")
                            .unwrap();
                    }

                    self.cg
                        .builder
                        .build_conditional_branch(matches, handler_bb, next_check_bb)
                        .unwrap();
                }

                current_bb = next_check_bb;
            }
        }

        // Generate handler blocks
        for (i, handler) in handlers.iter().enumerate() {
            self.cg.builder.position_at_end(handler_blocks[i]);

            // Bind exception to name if specified
            if let Some(name) = &handler.name {
                // Store exception pointer as the bound variable
                let exc_type =
                    PyType::Instance(InstanceType::new(iter_names::EXCEPTION.to_string(), vec![]));
                let alloca = self.create_entry_block_alloca(
                    &function.get_name().to_string_lossy(),
                    name,
                    &exc_type,
                );
                self.cg.builder.build_store(alloca, current_exc).unwrap();
                let exc_val = PyValue::new(current_exc.into(), exc_type, Some(alloca));
                self.variables.insert(name.clone(), exc_val);
            }

            // Execute handler body
            for stmt in &handler.body {
                if self.is_block_terminated() {
                    break;
                }
                self.visit_statement(stmt)?;
            }

            // Clear exception after handling
            if !self.is_block_terminated() {
                let clear_fn = self.get_or_declare_exception_fn("exception_clear");
                self.cg.builder.build_call(clear_fn, &[], "").unwrap();

                // Go to finally or merge
                let next_bb = finally_bb.unwrap_or(merge_bb);
                self.cg.builder.build_unconditional_branch(next_bb).unwrap();
            }
        }

        // Else block (only executed if no exception)
        if let Some(else_bb_val) = else_bb {
            self.cg.builder.position_at_end(else_bb_val);
            if let Some(else_stmts) = else_block {
                for stmt in else_stmts {
                    if self.is_block_terminated() {
                        break;
                    }
                    self.visit_statement(stmt)?;
                }
            }
            if !self.is_block_terminated() {
                let next_bb = finally_bb.unwrap_or(merge_bb);
                self.cg.builder.build_unconditional_branch(next_bb).unwrap();
            }
        }

        // Finally block (always executed)
        if let Some(finally_bb_val) = finally_bb {
            self.cg.builder.position_at_end(finally_bb_val);
            if let Some(finally_stmts) = finally_block {
                for stmt in finally_stmts {
                    if self.is_block_terminated() {
                        break;
                    }
                    self.visit_statement(stmt)?;
                }
            }
            if !self.is_block_terminated() {
                self.cg
                    .builder
                    .build_unconditional_branch(merge_bb)
                    .unwrap();
            }
        }

        // Merge block
        self.cg.builder.position_at_end(merge_bb);

        // Suppress warnings
        let _ = ptr_type;
        let _ = exc_type_id;

        Ok(())
    }

    /// Get or declare an exception runtime function
    fn get_or_declare_exception_fn(&self, name: &str) -> inkwell::values::FunctionValue<'ctx> {
        let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());

        if let Some(f) = self
            .cg
            .module
            .get_function(&format!("__builtin_tpy_{}", name))
        {
            return f;
        }

        let fn_type = match name {
            "exception_push_handler" => ptr_type.fn_type(&[], false),
            "exception_pop_handler" => self.cg.ctx.void_type().fn_type(&[], false),
            "exception_current" => ptr_type.fn_type(&[], false),
            "exception_clear" => self.cg.ctx.void_type().fn_type(&[], false),
            "exception_type_id" => self.cg.ctx.i64_type().fn_type(&[ptr_type.into()], false),
            "exception_isinstance" => self
                .cg
                .ctx
                .i64_type()
                .fn_type(&[ptr_type.into(), self.cg.ctx.i64_type().into()], false),
            "exception_raise" => self.cg.ctx.void_type().fn_type(&[ptr_type.into()], false),
            "exception_reraise" => self.cg.ctx.void_type().fn_type(&[], false),
            "exception_new" => ptr_type.fn_type(
                &[
                    self.cg.ctx.i64_type().into(),
                    ptr_type.into(),
                    ptr_type.into(),
                ],
                false,
            ),
            "raise_assertion_error" => self.cg.ctx.void_type().fn_type(&[ptr_type.into()], false),
            _ => panic!("Unknown exception function: {}", name),
        };

        self.cg
            .module
            .add_function(&format!("__builtin_tpy_{}", name), fn_type, None)
    }

    /// Get or declare setjmp
    fn get_or_declare_setjmp(&self) -> inkwell::values::FunctionValue<'ctx> {
        let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());

        if let Some(f) = self.cg.module.get_function("setjmp") {
            return f;
        }

        let fn_type = self.cg.ctx.i32_type().fn_type(&[ptr_type.into()], false);
        self.cg.module.add_function("setjmp", fn_type, None)
    }

    /// Get exception type ID from name
    fn exception_type_id_from_name(&self, name: &str) -> i64 {
        match name {
            "BaseException" => 0,
            "Exception" => 1,
            "ValueError" => 2,
            "TypeError" => 3,
            "KeyError" => 4,
            "IndexError" => 5,
            "ZeroDivisionError" => 6,
            "RuntimeError" => 7,
            "StopIteration" => 8,
            "AssertionError" => 9,
            "GeneratorExit" => 10,
            "AttributeError" => 11,
            "NameError" => 12,
            "OverflowError" => 13,
            "MemoryError" => 14,
            _ => 1, // Default to generic Exception
        }
    }

    pub(crate) fn visit_raise_impl(
        &mut self,
        exception: &Option<Expression>,
        cause: &Option<Expression>,
    ) -> Result<(), String> {
        let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());

        let exc_ptr = if let Some(exc_expr) = exception {
            // Evaluate exception expression
            // Could be: raise ValueError, raise ValueError("msg"), raise exc_instance
            match exc_expr {
                Expression::Var(name) => {
                    // Special handling for StopIteration - just set flag and return
                    if name == "StopIteration" {
                        self.set_stop_iteration_flag();
                        // Return from the current function with a default value
                        if let Some(current_fn) = self.current_function {
                            let ret_type = current_fn.get_type().get_return_type();
                            if let Some(ret_ty) = ret_type {
                                if ret_ty.is_int_type() {
                                    self.cg
                                        .builder
                                        .build_return(Some(&self.cg.ctx.i64_type().const_zero()))
                                        .unwrap();
                                } else if ret_ty.is_pointer_type() {
                                    self.cg
                                        .builder
                                        .build_return(Some(&ptr_type.const_null()))
                                        .unwrap();
                                } else {
                                    self.cg.builder.build_return(None).unwrap();
                                }
                            } else {
                                self.cg.builder.build_return(None).unwrap();
                            }
                        } else {
                            self.cg.builder.build_return(None).unwrap();
                        }
                        return Ok(());
                    }

                    // Could be an exception type name or an exception instance
                    let type_id = self.exception_type_id_from_name(name);

                    if type_id != 1 {
                        // Known exception type - create new instance
                        let exception_new_fn = self.get_or_declare_exception_fn("exception_new");
                        let type_id_val = self.cg.ctx.i64_type().const_int(type_id as u64, false);
                        let type_name = self
                            .cg
                            .builder
                            .build_global_string_ptr(name, "exc_type_name")
                            .unwrap();
                        let null_msg = ptr_type.const_null();

                        let call = self
                            .cg
                            .builder
                            .build_call(
                                exception_new_fn,
                                &[
                                    type_id_val.into(),
                                    type_name.as_pointer_value().into(),
                                    null_msg.into(),
                                ],
                                "exc",
                            )
                            .unwrap();
                        extract_ptr_from_call(call, "exception_new")
                    } else {
                        // Try to look up as variable (exception instance)
                        if let Some(var) = self.variables.get(name) {
                            var.value().into_pointer_value()
                        } else {
                            // Unknown - treat as generic Exception
                            let exception_new_fn =
                                self.get_or_declare_exception_fn("exception_new");
                            let type_id_val = self.cg.ctx.i64_type().const_int(1, false);
                            let type_name = self
                                .cg
                                .builder
                                .build_global_string_ptr(name, "exc_type_name")
                                .unwrap();
                            let null_msg = ptr_type.const_null();

                            let call = self
                                .cg
                                .builder
                                .build_call(
                                    exception_new_fn,
                                    &[
                                        type_id_val.into(),
                                        type_name.as_pointer_value().into(),
                                        null_msg.into(),
                                    ],
                                    "exc",
                                )
                                .unwrap();
                            extract_ptr_from_call(call, "exception_new")
                        }
                    }
                }
                Expression::Call { func, args } => {
                    // raise ValueError("message")
                    if let Expression::Var(type_name) = func.as_ref() {
                        // Special handling for StopIteration - just set flag and return
                        if type_name == "StopIteration" {
                            self.set_stop_iteration_flag();
                            // Return from the current function with a default value
                            if let Some(current_fn) = self.current_function {
                                let ret_type = current_fn.get_type().get_return_type();
                                if let Some(ret_ty) = ret_type {
                                    if ret_ty.is_int_type() {
                                        self.cg
                                            .builder
                                            .build_return(Some(
                                                &self.cg.ctx.i64_type().const_zero(),
                                            ))
                                            .unwrap();
                                    } else if ret_ty.is_pointer_type() {
                                        self.cg
                                            .builder
                                            .build_return(Some(&ptr_type.const_null()))
                                            .unwrap();
                                    } else {
                                        self.cg.builder.build_return(None).unwrap();
                                    }
                                } else {
                                    self.cg.builder.build_return(None).unwrap();
                                }
                            } else {
                                self.cg.builder.build_return(None).unwrap();
                            }
                            return Ok(());
                        }

                        let type_id = self.exception_type_id_from_name(type_name);
                        let exception_new_fn = self.get_or_declare_exception_fn("exception_new");
                        let type_id_val = self.cg.ctx.i64_type().const_int(type_id as u64, false);
                        let type_name_ptr = self
                            .cg
                            .builder
                            .build_global_string_ptr(type_name, "exc_type_name")
                            .unwrap();

                        // Get message if provided
                        let msg_ptr = if !args.is_empty() {
                            let msg_val = self.evaluate_expression(&args[0])?;
                            match msg_val.ty() {
                                PyType::Str | PyType::Bytes => msg_val.value().into_pointer_value(),
                                _ => ptr_type.const_null(),
                            }
                        } else {
                            ptr_type.const_null()
                        };

                        let call = self
                            .cg
                            .builder
                            .build_call(
                                exception_new_fn,
                                &[
                                    type_id_val.into(),
                                    type_name_ptr.as_pointer_value().into(),
                                    msg_ptr.into(),
                                ],
                                "exc",
                            )
                            .unwrap();
                        extract_ptr_from_call(call, "exception_new")
                    } else {
                        return Err("Invalid raise expression".to_string());
                    }
                }
                _ => {
                    // Try to evaluate as expression (might be exception instance)
                    let exc_val = self.evaluate_expression(exc_expr)?;
                    exc_val.value().into_pointer_value()
                }
            }
        } else {
            // Bare raise - re-raise current exception
            let reraise_fn = self.get_or_declare_exception_fn("exception_reraise");
            self.cg.builder.build_call(reraise_fn, &[], "").unwrap();
            self.cg.builder.build_unreachable().unwrap();
            return Ok(());
        };

        // Suppress warning for cause (not yet implemented)
        let _ = cause;

        // Call exception_raise
        let raise_fn = self.get_or_declare_exception_fn("exception_raise");
        self.cg
            .builder
            .build_call(raise_fn, &[exc_ptr.into()], "")
            .unwrap();
        self.cg.builder.build_unreachable().unwrap();

        Ok(())
    }

    pub(crate) fn visit_assert_impl(
        &mut self,
        test: &Expression,
        msg: &Option<Expression>,
    ) -> Result<(), String> {
        let function = self.current_function.unwrap();

        // Evaluate test condition
        let test_val = self.evaluate_expression(test)?;
        let test_bool = self.cg.value_to_bool(&test_val);

        // Create blocks for assertion pass/fail
        let fail_bb = self.cg.ctx.append_basic_block(function, "assert_fail");
        let cont_bb = self.cg.ctx.append_basic_block(function, "assert_cont");

        self.cg
            .builder
            .build_conditional_branch(test_bool, cont_bb, fail_bb)
            .unwrap();

        // Fail block - print message and abort
        self.cg.builder.position_at_end(fail_bb);

        if let Some(msg_expr) = msg {
            let _msg_val = self.evaluate_expression(msg_expr)?;
            // TODO: Print the message
        }

        // Call abort()
        let abort_fn = self.cg.module.get_function("abort").unwrap_or_else(|| {
            let fn_type = self.cg.ctx.void_type().fn_type(&[], false);
            self.cg.module.add_function("abort", fn_type, None)
        });
        self.cg.builder.build_call(abort_fn, &[], "").unwrap();
        self.cg.builder.build_unreachable().unwrap();

        // Continue block
        self.cg.builder.position_at_end(cont_bb);

        Ok(())
    }

    /// Implement global statement - marks variables as referring to module-level globals
    pub(crate) fn visit_global_impl(&mut self, names: &[String]) -> Result<(), String> {
        // Track which names should be treated as global
        for name in names {
            self.global_vars.insert(name.clone());
        }
        Ok(())
    }

    /// Implement nonlocal statement - marks variables as referring to enclosing scope
    pub(crate) fn visit_nonlocal_impl(&mut self, names: &[String]) -> Result<(), String> {
        // Track which names should be treated as nonlocal
        for name in names {
            self.nonlocal_vars.insert(name.clone());
        }
        Ok(())
    }

    /// Set the global __stop_iteration_flag to true
    /// This is used by the iterator protocol when StopIteration is raised
    pub(crate) fn set_stop_iteration_flag(&mut self) {
        // Get or create the global stop iteration flag
        let stop_flag_global = self
            .cg
            .module
            .get_global("__stop_iteration_flag")
            .unwrap_or_else(|| {
                let g = self.cg.module.add_global(
                    self.cg.ctx.bool_type(),
                    None,
                    "__stop_iteration_flag",
                );
                g.set_initializer(&self.cg.ctx.bool_type().const_zero());
                g
            });

        // Set the flag to true
        self.cg
            .builder
            .build_store(
                stop_flag_global.as_pointer_value(),
                self.cg.ctx.bool_type().const_int(1, false),
            )
            .unwrap();
    }
}

/// Extract pointer value from a call site result
fn extract_ptr_from_call<'ctx>(
    call_site: inkwell::values::CallSiteValue<'ctx>,
    fn_name: &str,
) -> inkwell::values::PointerValue<'ctx> {
    let any_val = call_site.as_any_value_enum();
    if let inkwell::values::AnyValueEnum::PointerValue(pv) = any_val {
        pv
    } else {
        panic!("{} did not return a pointer value", fn_name)
    }
}

/// Extract int value from a call site result
fn extract_int_from_call<'ctx>(
    call_site: inkwell::values::CallSiteValue<'ctx>,
    fn_name: &str,
) -> inkwell::values::IntValue<'ctx> {
    let any_val = call_site.as_any_value_enum();
    if let inkwell::values::AnyValueEnum::IntValue(iv) = any_val {
        iv
    } else {
        panic!("{} did not return an int value", fn_name)
    }
}
