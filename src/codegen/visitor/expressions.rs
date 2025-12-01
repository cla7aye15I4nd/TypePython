/// Expression visitor implementation for code generation
use super::super::CodeGen;
use crate::ast::Expression;
use crate::types::{PyType, PyValue};

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn visit_int_lit_impl(&mut self, val: i64) -> Result<PyValue<'ctx>, String> {
        let ir_val = self.cg.ctx.i64_type().const_int(val as u64, false);
        Ok(PyValue::int(ir_val))
    }

    pub(crate) fn visit_float_lit_impl(&mut self, val: f64) -> Result<PyValue<'ctx>, String> {
        let ir_val = self.cg.ctx.f64_type().const_float(val);
        Ok(PyValue::float(ir_val))
    }

    pub(crate) fn visit_str_lit_impl(&mut self, val: &str) -> Result<PyValue<'ctx>, String> {
        // Str literals are the same as bytes but with Str type
        let str_name = if let Some(&id) = self.strings.get(val) {
            format!(".str_{}", id)
        } else {
            let id = self.strings.len() as u64;
            self.strings.insert(val.to_string(), id);
            format!(".str_{}", id)
        };
        let str_const = self
            .cg
            .builder
            .build_global_string_ptr(val, &str_name)
            .unwrap();
        Ok(PyValue::new_str(str_const.as_pointer_value()))
    }

    pub(crate) fn visit_bytes_lit_impl(&mut self, val: &str) -> Result<PyValue<'ctx>, String> {
        // Bytes literals are the same as string literals in C (char*)
        // They're both null-terminated byte sequences
        let str_name = if let Some(&id) = self.strings.get(val) {
            format!(".bytes_{}", id)
        } else {
            let id = self.strings.len() as u64;
            self.strings.insert(val.to_string(), id);
            format!(".bytes_{}", id)
        };
        let str_const = self
            .cg
            .builder
            .build_global_string_ptr(val, &str_name)
            .unwrap();
        Ok(PyValue::bytes(str_const.as_pointer_value()))
    }

    pub(crate) fn visit_bool_lit_impl(&mut self, val: bool) -> Result<PyValue<'ctx>, String> {
        let ir_val = self.cg.ctx.bool_type().const_int(val as u64, false);
        Ok(PyValue::bool(ir_val))
    }

    pub(crate) fn visit_none_lit_impl(&mut self) -> Result<PyValue<'ctx>, String> {
        // Represent None as i32(0) - consistent with PyType::None.to_llvm()
        let ir_val = self.cg.ctx.i32_type().const_zero();
        Ok(PyValue::none(ir_val))
    }

    pub(crate) fn visit_var_impl(&mut self, name: &str) -> Result<PyValue<'ctx>, String> {
        // If this variable is declared as global, look it up in module_vars first
        if self.global_vars.contains(name) {
            if let Some((ptr, py_type)) = self.module_vars.get(name).cloned() {
                let llvm_type = py_type.to_llvm(self.cg.ctx);
                let loaded = self.cg.builder.build_load(llvm_type, ptr, name).unwrap();
                // Return with None ptr so it uses immediate value, not pointer
                // The ptr is still stored in module_vars for assignments
                return Ok(PyValue::new(loaded, py_type.clone(), None));
            }
        }

        // First check local variables (stack allocations)
        if let Some(var) = self.variables.get(name) {
            return Ok(var.load(&self.cg.builder, self.cg.ctx, name));
        }

        // Check if it's a function in the current LLVM module (local functions)
        // This must come before global_variables because global_variables contains
        // placeholder functions from preprocessing
        let mangled_name = self.mangle_function_name(&self.module_name, name);
        if self.cg.module.get_function(&mangled_name).is_some() {
            // Check if this is a generator function
            let is_generator = self.generator_functions.contains(&mangled_name);

            // Get type info from global_variables if available (for correct return type)
            let (param_types, mut return_type) =
                if let Some(global) = self.global_variables.get(name) {
                    if matches!(global.ty(), crate::types::PyType::Function(_)) {
                        let info = global.get_function();
                        (info.fn_type.param_types.clone(), info.fn_type.return_type())
                    } else {
                        (vec![], crate::types::PyType::None)
                    }
                } else {
                    (vec![], crate::types::PyType::None)
                };

            // If it's a generator, wrap the return type as a generator Instance
            if is_generator {
                return_type = PyType::Instance(crate::types::InstanceType::new(
                    crate::codegen::types::iter_names::GENERATOR.to_string(),
                    vec![("yield_type".to_string(), return_type)],
                ));
            }

            return Ok(PyValue::function(crate::types::FunctionInfo {
                storage: crate::types::FunctionStorage {
                    mangled_name,
                    bound_args: vec![],
                    macro_kind: None,
                },
                fn_type: crate::types::FunctionType::new(param_types, return_type),
            }));
        }

        // Then check global variables (imported modules)
        if let Some(global) = self.global_variables.get(name) {
            return Ok(global.clone());
        }

        // Check if it's a builtin function
        if let Some(func_value) = self.get_builtin_function(name) {
            return Ok(func_value);
        }

        // Check if it's a class (class constructor)
        if let Some(class_info) = self.classes.get(name).cloned() {
            return self.get_class_constructor(&class_info);
        }

        Err(format!("Variable {} not found", name))
    }

    /// Get a class constructor as a callable
    fn get_class_constructor(
        &mut self,
        class_info: &crate::codegen::ClassInfo,
    ) -> Result<PyValue<'ctx>, String> {
        let class_name = &class_info.name;

        // Create instance type with class fields
        let instance_type = crate::types::PyType::Instance(crate::types::InstanceType::new(
            class_name.clone(),
            class_info.fields.clone(),
        ));

        // Return a function that constructs this class
        // The constructor function should be <classname>_init (or similar)
        let constructor_name = format!("__main___{}_init", class_name);

        Ok(PyValue::function(crate::types::FunctionInfo {
            storage: crate::types::FunctionStorage {
                mangled_name: constructor_name,
                bound_args: vec![],
                macro_kind: None,
            },
            fn_type: crate::types::FunctionType::new(
                // Use __init__ params (excluding self) as constructor params
                class_info.init_params.clone(),
                instance_type,
            ),
        }))
    }

    // ========================================================================
    // Container Literal Implementations
    // ========================================================================

    /// Create a list literal: [1, 2, 3]
    pub(crate) fn visit_list_lit_impl(
        &mut self,
        elements: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        // Infer element type from first element (or default to Int for empty list)
        let elem_type = if elements.is_empty() {
            PyType::Int
        } else {
            let first = self.evaluate_expression(&elements[0])?;
            first.ty().clone()
        };

        // Create a new list with capacity for all elements
        // Use type-specific list creation based on element type
        let capacity = self
            .cg
            .ctx
            .i64_type()
            .const_int(elements.len() as u64, false);

        let (list_new_fn_name, list_append_fn_name) = match &elem_type {
            PyType::Str => ("str_list_with_capacity", "str_list_append"),
            PyType::Float => ("float_list_with_capacity", "float_list_append"),
            PyType::Bool => ("bool_list_with_capacity", "bool_list_append"),
            _ => ("list_with_capacity", "list_append"),
        };

        let list_new_fn = self.get_or_declare_c_builtin(list_new_fn_name);
        let call_site = self
            .cg
            .builder
            .build_call(list_new_fn, &[capacity.into()], "list_new")
            .unwrap();
        let list_ptr = self.extract_ptr_call_result(call_site).value();

        // Append each element (list_append mutates in place, returns void)
        let list_append_fn = self.get_or_declare_c_builtin(list_append_fn_name);
        for elem in elements {
            let elem_val = self.evaluate_expression(elem)?;
            // TODO: Type check that elem_val.ty() matches elem_type

            // Convert the element value to the appropriate storage type
            let arg_val = match &elem_type {
                PyType::Bool => {
                    // For bools, extend i1 to i8 for C ABI compatibility
                    self.cg
                        .builder
                        .build_int_z_extend(
                            elem_val.value().into_int_value(),
                            self.cg.ctx.i8_type(),
                            "bool_to_i8",
                        )
                        .unwrap()
                        .into()
                }
                PyType::Tuple(_)
                | PyType::List(_)
                | PyType::Dict(_, _)
                | PyType::Set(_)
                | PyType::Bytes => {
                    // Pointer types need to be converted to i64 for storage
                    self.cg
                        .builder
                        .build_ptr_to_int(
                            elem_val.value().into_pointer_value(),
                            self.cg.ctx.i64_type(),
                            "ptr_to_i64",
                        )
                        .unwrap()
                        .into()
                }
                _ => elem_val.value().into(),
            };

            self.cg
                .builder
                .build_call(list_append_fn, &[list_ptr.into(), arg_val], "list_append")
                .unwrap();
        }

        Ok(PyValue::new(
            list_ptr,
            PyType::List(Box::new(elem_type)),
            None,
        ))
    }

    /// Create a list comprehension: [expr for var in iterable if condition]
    /// Supports nested for clauses: [expr for x in xs for y in ys if cond]
    pub(crate) fn visit_list_comprehension_impl(
        &mut self,
        element: &Expression,
        clauses: &[crate::ast::ComprehensionClause],
    ) -> Result<PyValue<'ctx>, String> {
        if clauses.is_empty() {
            return Err("List comprehension must have at least one clause".to_string());
        }

        // Create result list once at the start
        let list_new_fn = self.get_or_declare_c_builtin("list_with_capacity");
        let initial_capacity = self.cg.ctx.i64_type().const_int(16, false);
        let call_site = self
            .cg
            .builder
            .build_call(list_new_fn, &[initial_capacity.into()], "listcomp_new")
            .unwrap();
        let result_list_ptr = self.extract_ptr_call_result(call_site).value();

        // Get current function
        let function = self.current_function.unwrap();

        // Create the final "after" block that we'll jump to when done
        let final_after_bb = self.cg.ctx.append_basic_block(function, "listcomp_done");

        // Generate nested loops for all clauses
        let result_type = self.generate_listcomp_clause(
            result_list_ptr,
            element,
            clauses,
            0, // start with first clause
            final_after_bb,
            function,
        )?;

        // Position at final after block
        self.cg.builder.position_at_end(final_after_bb);

        Ok(PyValue::new(
            result_list_ptr,
            PyType::List(Box::new(result_type)),
            None,
        ))
    }

    /// Recursively generate code for a single comprehension clause
    /// Returns the element type of the comprehension
    fn generate_listcomp_clause(
        &mut self,
        result_list_ptr: inkwell::values::BasicValueEnum<'ctx>,
        element: &Expression,
        clauses: &[crate::ast::ComprehensionClause],
        clause_idx: usize,
        final_after_bb: inkwell::basic_block::BasicBlock<'ctx>,
        function: inkwell::values::FunctionValue<'ctx>,
    ) -> Result<PyType, String> {
        let clause = &clauses[clause_idx];
        let is_last_clause = clause_idx == clauses.len() - 1;

        // Only support single variable binding for now
        if clause.target.len() != 1 {
            return Err("Multi-variable list comprehension not yet supported".to_string());
        }
        let var_name = &clause.target[0];

        // Evaluate the iterable
        let iter_val = self.evaluate_expression(&clause.iterable)?;

        // Create basic blocks for this loop level
        let suffix = if clause_idx == 0 {
            String::new()
        } else {
            format!("_{}", clause_idx)
        };
        let cond_bb = self
            .cg
            .ctx
            .append_basic_block(function, &format!("listcomp_cond{}", suffix));
        let body_bb = self
            .cg
            .ctx
            .append_basic_block(function, &format!("listcomp_body{}", suffix));
        let after_bb = self
            .cg
            .ctx
            .append_basic_block(function, &format!("listcomp_after{}", suffix));

        // Generate the for-loop iteration based on the iterable type
        // Check for Range first using the helper function
        if crate::codegen::types::is_range(&iter_val.ty()) {
            return self.generate_listcomp_clause_range(
                result_list_ptr,
                var_name,
                &iter_val,
                element,
                clauses,
                clause_idx,
                cond_bb,
                body_bb,
                after_bb,
                final_after_bb,
                function,
            );
        }

        match iter_val.ty() {
            PyType::List(elem_type) => {
                // Iterate over list
                let list_ptr = iter_val.value().into_pointer_value();
                let list_len_fn = self.get_or_declare_c_builtin("list_len");
                let len_call = self
                    .cg
                    .builder
                    .build_call(list_len_fn, &[list_ptr.into()], "list_len")
                    .unwrap();
                use inkwell::values::AnyValue;
                let len = len_call.as_any_value_enum().into_int_value();

                // Index variable
                let i64_type = self.cg.ctx.i64_type();
                let idx_name = format!("__listcomp_idx{}", clause_idx);
                let index_alloca =
                    self.create_entry_block_alloca_with_type(&idx_name, i64_type.into());
                self.cg
                    .builder
                    .build_store(index_alloca, i64_type.const_zero())
                    .unwrap();

                // Loop variable alloca
                let elem_llvm_ty = elem_type.to_llvm(self.cg.ctx);
                let var_alloca = self.create_entry_block_alloca_with_type(var_name, elem_llvm_ty);

                // Jump to condition
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                // Condition block: check index < len
                self.cg.builder.position_at_end(cond_bb);
                let current_idx = self
                    .cg
                    .builder
                    .build_load(i64_type, index_alloca, "idx")
                    .unwrap()
                    .into_int_value();
                let cond = self
                    .cg
                    .builder
                    .build_int_compare(
                        inkwell::IntPredicate::SLT,
                        current_idx,
                        len,
                        "listcomp_cond",
                    )
                    .unwrap();
                self.cg
                    .builder
                    .build_conditional_branch(cond, body_bb, after_bb)
                    .unwrap();

                // Body block: get element and evaluate expression
                self.cg.builder.position_at_end(body_bb);

                // Get element at index
                let list_getitem_fn = self.get_or_declare_c_builtin("list_getitem");
                let elem_call = self
                    .cg
                    .builder
                    .build_call(
                        list_getitem_fn,
                        &[list_ptr.into(), current_idx.into()],
                        "getitem",
                    )
                    .unwrap();
                let elem_i64 = elem_call.as_any_value_enum().into_int_value();

                // Convert to proper type and store in loop variable
                let elem_val = self.convert_i64_to_value(elem_i64, elem_type.as_ref())?;
                self.cg
                    .builder
                    .build_store(var_alloca, elem_val.value())
                    .unwrap();

                // Register the loop variable in scope
                let old_var = self.variables.get(var_name).cloned();
                self.variables.insert(
                    var_name.to_string(),
                    PyValue::new(
                        elem_val.value(),
                        elem_type.as_ref().clone(),
                        Some(var_alloca),
                    ),
                );

                // Create increment block for the loop
                let inc_suffix = if clause_idx == 0 {
                    String::new()
                } else {
                    format!("_{}", clause_idx)
                };
                let inc_bb = self
                    .cg
                    .ctx
                    .append_basic_block(function, &format!("listcomp_inc{}", inc_suffix));

                // Evaluate all conditions if present - chain them with short-circuit AND
                if !clause.conditions.is_empty() {
                    let inner_bb = self
                        .cg
                        .ctx
                        .append_basic_block(function, &format!("listcomp_inner{}", inc_suffix));

                    for (i, cond_expr) in clause.conditions.iter().enumerate() {
                        let cond_val = self.evaluate_expression(cond_expr)?;
                        let cond_bool = self.cg.value_to_bool(&cond_val);

                        if i < clause.conditions.len() - 1 {
                            // More conditions to check - create next condition block
                            let next_cond_bb = self.cg.ctx.append_basic_block(
                                function,
                                &format!("listcomp_cond{}_{}", clause_idx, i + 1),
                            );
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, next_cond_bb, inc_bb)
                                .unwrap();
                            self.cg.builder.position_at_end(next_cond_bb);
                        } else {
                            // Last condition - branch to inner or inc
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, inner_bb, inc_bb)
                                .unwrap();
                        }
                    }

                    self.cg.builder.position_at_end(inner_bb);
                }

                // Now either recurse into inner clause or evaluate element and append
                let result_type = if is_last_clause {
                    // Evaluate the element expression
                    let result_val = self.evaluate_expression(element)?;
                    let result_type = result_val.ty().clone();

                    // Append to result list - need to convert pointer types to i64
                    let append_fn_name = match &result_type {
                        PyType::Str => "str_list_append",
                        PyType::Float => "float_list_append",
                        PyType::Bool => "bool_list_append",
                        _ => "list_append",
                    };
                    let list_append_fn = self.get_or_declare_c_builtin(append_fn_name);
                    // Convert values to the right type for append functions
                    let append_val = match &result_type {
                        PyType::List(_)
                        | PyType::Dict(_, _)
                        | PyType::Set(_)
                        | PyType::Tuple(_)
                        | PyType::Bytes => self
                            .cg
                            .builder
                            .build_ptr_to_int(
                                result_val.value().into_pointer_value(),
                                self.cg.ctx.i64_type(),
                                "ptr_to_i64",
                            )
                            .unwrap()
                            .into(),
                        PyType::Bool => {
                            // bool_list_append expects i8, but bool values are i1
                            self.cg
                                .builder
                                .build_int_z_extend(
                                    result_val.value().into_int_value(),
                                    self.cg.ctx.i8_type(),
                                    "bool_to_i8",
                                )
                                .unwrap()
                                .into()
                        }
                        _ => result_val.value().into(),
                    };
                    self.cg
                        .builder
                        .build_call(
                            list_append_fn,
                            &[result_list_ptr.into(), append_val],
                            "append",
                        )
                        .unwrap();

                    result_type
                } else {
                    // Recursively process next clause
                    self.generate_listcomp_clause(
                        result_list_ptr,
                        element,
                        clauses,
                        clause_idx + 1,
                        final_after_bb,
                        function,
                    )?
                };

                // Jump to increment block
                self.cg.builder.build_unconditional_branch(inc_bb).unwrap();

                // Position at increment block and increment index
                self.cg.builder.position_at_end(inc_bb);
                let next_idx = self
                    .cg
                    .builder
                    .build_int_add(current_idx, i64_type.const_int(1, false), "next_idx")
                    .unwrap();
                self.cg.builder.build_store(index_alloca, next_idx).unwrap();
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                // After block - when this loop finishes, jump to parent's increment block
                // or to final_after_bb if this is the outermost clause
                self.cg.builder.position_at_end(after_bb);

                // Restore old variable
                if let Some(old) = old_var {
                    self.variables.insert(var_name.to_string(), old);
                } else {
                    self.variables.remove(var_name);
                }

                // If this is the outermost clause, jump to final after block
                if clause_idx == 0 {
                    self.cg
                        .builder
                        .build_unconditional_branch(final_after_bb)
                        .unwrap();
                }

                Ok(result_type)
            }
            PyType::Bytes => {
                // Iterate over bytes - similar to list but uses bytes_len and bytes_getitem
                let bytes_ptr = iter_val.value().into_pointer_value();
                let bytes_len_fn = self.get_or_declare_c_builtin("bytes_len");
                let len_call = self
                    .cg
                    .builder
                    .build_call(bytes_len_fn, &[bytes_ptr.into()], "bytes_len")
                    .unwrap();
                use inkwell::values::AnyValue;
                let len = len_call.as_any_value_enum().into_int_value();

                // Index variable
                let i64_type = self.cg.ctx.i64_type();
                let idx_name = format!("__bytescomp_idx{}", clause_idx);
                let index_alloca =
                    self.create_entry_block_alloca_with_type(&idx_name, i64_type.into());
                self.cg
                    .builder
                    .build_store(index_alloca, i64_type.const_zero())
                    .unwrap();

                // Loop variable alloca (bytes iteration yields int)
                let var_alloca =
                    self.create_entry_block_alloca_with_type(var_name, i64_type.into());

                // Jump to condition
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                // Condition block: check index < len
                self.cg.builder.position_at_end(cond_bb);
                let current_idx = self
                    .cg
                    .builder
                    .build_load(i64_type, index_alloca, "idx")
                    .unwrap()
                    .into_int_value();
                let cond = self
                    .cg
                    .builder
                    .build_int_compare(
                        inkwell::IntPredicate::SLT,
                        current_idx,
                        len,
                        "bytescomp_cond",
                    )
                    .unwrap();
                self.cg
                    .builder
                    .build_conditional_branch(cond, body_bb, after_bb)
                    .unwrap();

                // Body block: get byte and evaluate expression
                self.cg.builder.position_at_end(body_bb);

                // Get byte at index
                let bytes_getitem_fn = self.get_or_declare_c_builtin("bytes_getitem");
                let byte_call = self
                    .cg
                    .builder
                    .build_call(
                        bytes_getitem_fn,
                        &[bytes_ptr.into(), current_idx.into()],
                        "getitem",
                    )
                    .unwrap();
                let byte_val = byte_call.as_any_value_enum().into_int_value();

                // Store in loop variable
                self.cg.builder.build_store(var_alloca, byte_val).unwrap();

                // Register the loop variable in scope
                let old_var = self.variables.get(var_name).cloned();
                self.variables.insert(
                    var_name.to_string(),
                    PyValue::new(byte_val.into(), PyType::Int, Some(var_alloca)),
                );

                // Create increment block for the loop
                let inc_suffix = if clause_idx == 0 {
                    String::new()
                } else {
                    format!("_{}", clause_idx)
                };
                let inc_bb = self
                    .cg
                    .ctx
                    .append_basic_block(function, &format!("bytescomp_inc{}", inc_suffix));

                // Evaluate all conditions if present - chain them with short-circuit AND
                if !clause.conditions.is_empty() {
                    let inner_bb = self
                        .cg
                        .ctx
                        .append_basic_block(function, &format!("bytescomp_inner{}", inc_suffix));

                    for (i, cond_expr) in clause.conditions.iter().enumerate() {
                        let cond_val = self.evaluate_expression(cond_expr)?;
                        let cond_bool = self.cg.value_to_bool(&cond_val);

                        if i < clause.conditions.len() - 1 {
                            let next_cond_bb = self.cg.ctx.append_basic_block(
                                function,
                                &format!("bytescomp_cond{}_{}", clause_idx, i + 1),
                            );
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, next_cond_bb, inc_bb)
                                .unwrap();
                            self.cg.builder.position_at_end(next_cond_bb);
                        } else {
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, inner_bb, inc_bb)
                                .unwrap();
                        }
                    }

                    self.cg.builder.position_at_end(inner_bb);
                }

                // Now either recurse into inner clause or evaluate element and append
                let result_type = if is_last_clause {
                    let result_val = self.evaluate_expression(element)?;
                    let result_type = result_val.ty().clone();

                    let append_fn_name = match &result_type {
                        PyType::Str => "str_list_append",
                        PyType::Float => "float_list_append",
                        PyType::Bool => "bool_list_append",
                        _ => "list_append",
                    };
                    let list_append_fn = self.get_or_declare_c_builtin(append_fn_name);
                    let append_val = match &result_type {
                        PyType::List(_)
                        | PyType::Dict(_, _)
                        | PyType::Set(_)
                        | PyType::Tuple(_)
                        | PyType::Bytes => self
                            .cg
                            .builder
                            .build_ptr_to_int(
                                result_val.value().into_pointer_value(),
                                self.cg.ctx.i64_type(),
                                "ptr_to_i64",
                            )
                            .unwrap()
                            .into(),
                        PyType::Bool => self
                            .cg
                            .builder
                            .build_int_z_extend(
                                result_val.value().into_int_value(),
                                self.cg.ctx.i8_type(),
                                "bool_to_i8",
                            )
                            .unwrap()
                            .into(),
                        _ => result_val.value().into(),
                    };
                    self.cg
                        .builder
                        .build_call(
                            list_append_fn,
                            &[result_list_ptr.into(), append_val],
                            "append",
                        )
                        .unwrap();

                    result_type
                } else {
                    self.generate_listcomp_clause(
                        result_list_ptr,
                        element,
                        clauses,
                        clause_idx + 1,
                        final_after_bb,
                        function,
                    )?
                };

                // Jump to increment block
                self.cg.builder.build_unconditional_branch(inc_bb).unwrap();

                // Position at increment block and increment index
                self.cg.builder.position_at_end(inc_bb);
                let next_idx = self
                    .cg
                    .builder
                    .build_int_add(current_idx, i64_type.const_int(1, false), "next_idx")
                    .unwrap();
                self.cg.builder.build_store(index_alloca, next_idx).unwrap();
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                // After block
                self.cg.builder.position_at_end(after_bb);

                // Restore old variable
                if let Some(old) = old_var {
                    self.variables.insert(var_name.to_string(), old);
                } else {
                    self.variables.remove(var_name);
                }

                if clause_idx == 0 {
                    self.cg
                        .builder
                        .build_unconditional_branch(final_after_bb)
                        .unwrap();
                }

                Ok(result_type)
            }
            PyType::Str => {
                // Iterate over string characters
                let str_ptr = iter_val.value().into_pointer_value();
                let str_len_fn = self.get_or_declare_c_builtin("str_len");
                let len_call = self
                    .cg
                    .builder
                    .build_call(str_len_fn, &[str_ptr.into()], "str_len")
                    .unwrap();
                use inkwell::values::AnyValue;
                let len = len_call.as_any_value_enum().into_int_value();

                // Index variable
                let i64_type = self.cg.ctx.i64_type();
                let idx_name = format!("__strcomp_idx{}", clause_idx);
                let index_alloca =
                    self.create_entry_block_alloca_with_type(&idx_name, i64_type.into());
                self.cg
                    .builder
                    .build_store(index_alloca, i64_type.const_zero())
                    .unwrap();

                // Loop variable alloca (string iteration yields single-char strings)
                let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());
                let var_alloca =
                    self.create_entry_block_alloca_with_type(var_name, ptr_type.into());

                // Jump to condition
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                // Condition block: check index < len
                self.cg.builder.position_at_end(cond_bb);
                let current_idx = self
                    .cg
                    .builder
                    .build_load(i64_type, index_alloca, "idx")
                    .unwrap()
                    .into_int_value();
                let cond = self
                    .cg
                    .builder
                    .build_int_compare(inkwell::IntPredicate::SLT, current_idx, len, "strcomp_cond")
                    .unwrap();
                self.cg
                    .builder
                    .build_conditional_branch(cond, body_bb, after_bb)
                    .unwrap();

                // Body block: get character and evaluate expression
                self.cg.builder.position_at_end(body_bb);

                // Get character at index using str_slice(s, index, index+1)
                let str_slice_fn = self.get_or_declare_c_builtin("str_slice");
                let next_idx_slice = self
                    .cg
                    .builder
                    .build_int_add(current_idx, i64_type.const_int(1, false), "next_idx_slice")
                    .unwrap();
                let char_call = self
                    .cg
                    .builder
                    .build_call(
                        str_slice_fn,
                        &[str_ptr.into(), current_idx.into(), next_idx_slice.into()],
                        "char_slice",
                    )
                    .unwrap();
                let char_ptr = char_call.as_any_value_enum().into_pointer_value();

                // Store in loop variable
                self.cg.builder.build_store(var_alloca, char_ptr).unwrap();

                // Register the loop variable in scope
                let old_var = self.variables.get(var_name).cloned();
                self.variables.insert(
                    var_name.to_string(),
                    PyValue::new(char_ptr.into(), PyType::Str, Some(var_alloca)),
                );

                // Create increment block for the loop
                let inc_suffix = if clause_idx == 0 {
                    String::new()
                } else {
                    format!("_{}", clause_idx)
                };
                let inc_bb = self
                    .cg
                    .ctx
                    .append_basic_block(function, &format!("strcomp_inc{}", inc_suffix));

                // Evaluate all conditions if present - chain them with short-circuit AND
                if !clause.conditions.is_empty() {
                    let inner_bb = self
                        .cg
                        .ctx
                        .append_basic_block(function, &format!("strcomp_inner{}", inc_suffix));

                    for (i, cond_expr) in clause.conditions.iter().enumerate() {
                        let cond_val = self.evaluate_expression(cond_expr)?;
                        let cond_bool = self.cg.value_to_bool(&cond_val);

                        if i < clause.conditions.len() - 1 {
                            let next_cond_bb = self.cg.ctx.append_basic_block(
                                function,
                                &format!("strcomp_cond{}_{}", clause_idx, i + 1),
                            );
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, next_cond_bb, inc_bb)
                                .unwrap();
                            self.cg.builder.position_at_end(next_cond_bb);
                        } else {
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, inner_bb, inc_bb)
                                .unwrap();
                        }
                    }

                    self.cg.builder.position_at_end(inner_bb);
                }

                // Now either recurse into inner clause or evaluate element and append
                let result_type = if is_last_clause {
                    let result_val = self.evaluate_expression(element)?;
                    let result_type = result_val.ty().clone();

                    let append_fn_name = match &result_type {
                        PyType::Str => "str_list_append",
                        PyType::Float => "float_list_append",
                        PyType::Bool => "bool_list_append",
                        _ => "list_append",
                    };
                    let list_append_fn = self.get_or_declare_c_builtin(append_fn_name);
                    let append_val = match &result_type {
                        PyType::List(_)
                        | PyType::Dict(_, _)
                        | PyType::Set(_)
                        | PyType::Tuple(_)
                        | PyType::Bytes => self
                            .cg
                            .builder
                            .build_ptr_to_int(
                                result_val.value().into_pointer_value(),
                                self.cg.ctx.i64_type(),
                                "ptr_to_i64",
                            )
                            .unwrap()
                            .into(),
                        PyType::Bool => self
                            .cg
                            .builder
                            .build_int_z_extend(
                                result_val.value().into_int_value(),
                                self.cg.ctx.i8_type(),
                                "bool_to_i8",
                            )
                            .unwrap()
                            .into(),
                        _ => result_val.value().into(),
                    };
                    self.cg
                        .builder
                        .build_call(
                            list_append_fn,
                            &[result_list_ptr.into(), append_val],
                            "append",
                        )
                        .unwrap();

                    result_type
                } else {
                    self.generate_listcomp_clause(
                        result_list_ptr,
                        element,
                        clauses,
                        clause_idx + 1,
                        final_after_bb,
                        function,
                    )?
                };

                // Jump to increment block
                self.cg.builder.build_unconditional_branch(inc_bb).unwrap();

                // Position at increment block and increment index
                self.cg.builder.position_at_end(inc_bb);
                let next_idx = self
                    .cg
                    .builder
                    .build_int_add(current_idx, i64_type.const_int(1, false), "next_idx")
                    .unwrap();
                self.cg.builder.build_store(index_alloca, next_idx).unwrap();
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                // After block
                self.cg.builder.position_at_end(after_bb);

                // Restore old variable
                if let Some(old) = old_var {
                    self.variables.insert(var_name.to_string(), old);
                } else {
                    self.variables.remove(var_name);
                }

                if clause_idx == 0 {
                    self.cg
                        .builder
                        .build_unconditional_branch(final_after_bb)
                        .unwrap();
                }

                Ok(result_type)
            }
            _ => Err(format!(
                "List comprehension over {:?} not yet supported",
                iter_val.ty()
            )),
        }
    }

    /// Helper to generate list comprehension over range with nested clause support
    #[allow(clippy::too_many_arguments)]
    fn generate_listcomp_clause_range(
        &mut self,
        result_list_ptr: inkwell::values::BasicValueEnum<'ctx>,
        var_name: &str,
        iter_val: &PyValue<'ctx>,
        element: &Expression,
        clauses: &[crate::ast::ComprehensionClause],
        clause_idx: usize,
        cond_bb: inkwell::basic_block::BasicBlock<'ctx>,
        body_bb: inkwell::basic_block::BasicBlock<'ctx>,
        after_bb: inkwell::basic_block::BasicBlock<'ctx>,
        final_after_bb: inkwell::basic_block::BasicBlock<'ctx>,
        function: inkwell::values::FunctionValue<'ctx>,
    ) -> Result<PyType, String> {
        use inkwell::values::AnyValue;

        let clause = &clauses[clause_idx];
        let is_last_clause = clause_idx == clauses.len() - 1;

        // Iterate over range
        let range_ptr = iter_val.value().into_pointer_value();
        let range_iter_fn = self.get_or_declare_c_builtin("range_iter");
        let iter_call = self
            .cg
            .builder
            .build_call(range_iter_fn, &[range_ptr.into()], "range_iter")
            .unwrap();
        let iter_ptr = iter_call.as_any_value_enum().into_pointer_value();

        // Loop variable and output allocas
        let i64_type = self.cg.ctx.i64_type();
        let var_alloca = self.create_entry_block_alloca_with_type(var_name, i64_type.into());
        let out_name = format!("iter_out{}", clause_idx);
        let out_value_alloca = self.cg.builder.build_alloca(i64_type, &out_name).unwrap();

        // Jump to condition
        self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

        // Condition block
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

        // Body block
        self.cg.builder.position_at_end(body_bb);

        // Load the value
        let elem_val = self
            .cg
            .builder
            .build_load(i64_type, out_value_alloca, "elem")
            .unwrap();
        self.cg.builder.build_store(var_alloca, elem_val).unwrap();

        // Register the loop variable
        let old_var = self.variables.get(var_name).cloned();
        self.variables.insert(
            var_name.to_string(),
            PyValue::new(elem_val, PyType::Int, Some(var_alloca)),
        );

        // Evaluate all conditions if present - chain them with short-circuit AND
        if !clause.conditions.is_empty() {
            let inc_suffix = if clause_idx == 0 {
                String::new()
            } else {
                format!("_{}", clause_idx)
            };
            let inner_bb = self
                .cg
                .ctx
                .append_basic_block(function, &format!("rangecomp_inner{}", inc_suffix));

            for (i, cond_expr) in clause.conditions.iter().enumerate() {
                let cond_val = self.evaluate_expression(cond_expr)?;
                let cond_bool = self.cg.value_to_bool(&cond_val);

                if i < clause.conditions.len() - 1 {
                    let next_cond_bb = self.cg.ctx.append_basic_block(
                        function,
                        &format!("rangecomp_cond{}_{}", clause_idx, i + 1),
                    );
                    self.cg
                        .builder
                        .build_conditional_branch(cond_bool, next_cond_bb, cond_bb)
                        .unwrap();
                    self.cg.builder.position_at_end(next_cond_bb);
                } else {
                    self.cg
                        .builder
                        .build_conditional_branch(cond_bool, inner_bb, cond_bb)
                        .unwrap();
                }
            }

            self.cg.builder.position_at_end(inner_bb);
        }

        // Now either recurse into inner clause or evaluate element and append
        let result_type = if is_last_clause {
            let result_val = self.evaluate_expression(element)?;
            let result_type = result_val.ty().clone();

            let append_fn_name = match &result_type {
                PyType::Str => "str_list_append",
                PyType::Float => "float_list_append",
                PyType::Bool => "bool_list_append",
                _ => "list_append",
            };
            let list_append_fn = self.get_or_declare_c_builtin(append_fn_name);
            let append_val = match &result_type {
                PyType::List(_)
                | PyType::Dict(_, _)
                | PyType::Set(_)
                | PyType::Tuple(_)
                | PyType::Bytes => self
                    .cg
                    .builder
                    .build_ptr_to_int(
                        result_val.value().into_pointer_value(),
                        self.cg.ctx.i64_type(),
                        "ptr_to_i64",
                    )
                    .unwrap()
                    .into(),
                PyType::Bool => self
                    .cg
                    .builder
                    .build_int_z_extend(
                        result_val.value().into_int_value(),
                        self.cg.ctx.i8_type(),
                        "bool_to_i8",
                    )
                    .unwrap()
                    .into(),
                _ => result_val.value().into(),
            };
            self.cg
                .builder
                .build_call(
                    list_append_fn,
                    &[result_list_ptr.into(), append_val],
                    "append",
                )
                .unwrap();

            result_type
        } else {
            self.generate_listcomp_clause(
                result_list_ptr,
                element,
                clauses,
                clause_idx + 1,
                final_after_bb,
                function,
            )?
        };

        // Jump back to condition
        self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

        // After block
        self.cg.builder.position_at_end(after_bb);

        // Restore old variable
        if let Some(old) = old_var {
            self.variables.insert(var_name.to_string(), old);
        } else {
            self.variables.remove(var_name);
        }

        if clause_idx == 0 {
            self.cg
                .builder
                .build_unconditional_branch(final_after_bb)
                .unwrap();
        }

        Ok(result_type)
    }

    /// Create a dict literal: {1: 2, 3: 4}
    pub(crate) fn visit_dict_lit_impl(
        &mut self,
        pairs: &[(Expression, Expression)],
    ) -> Result<PyValue<'ctx>, String> {
        // Infer key/value types from first pair (or default to Int for empty dict)
        let (key_type, val_type) = if pairs.is_empty() {
            (PyType::Int, PyType::Int)
        } else {
            let first_key = self.evaluate_expression(&pairs[0].0)?;
            let first_val = self.evaluate_expression(&pairs[0].1)?;
            (first_key.ty().clone(), first_val.ty().clone())
        };

        // Select the appropriate dict type based on key type
        let is_str_keyed = matches!(key_type, PyType::Str);

        // Create a new empty dict
        let dict_new_fn = if is_str_keyed {
            self.get_or_declare_c_builtin("str_dict_new")
        } else {
            self.get_or_declare_c_builtin("dict_new")
        };
        let call_site = self
            .cg
            .builder
            .build_call(dict_new_fn, &[], "dict_new")
            .unwrap();
        let dict_ptr = self.extract_ptr_call_result(call_site).value();

        // Set each key-value pair
        let dict_setitem_fn = if is_str_keyed {
            self.get_or_declare_c_builtin("str_dict_setitem")
        } else {
            self.get_or_declare_c_builtin("dict_setitem")
        };

        for (key_expr, val_expr) in pairs {
            let key_val = self.evaluate_expression(key_expr)?;
            let val_val = self.evaluate_expression(val_expr)?;
            // TODO: Type check that key/val types match

            // Convert value to i64 for storage (dict stores all values as i64)
            let val_as_i64: inkwell::values::BasicValueEnum = match val_val.ty() {
                PyType::Float => {
                    // Bitcast f64 to i64 (preserves bit pattern)
                    self.cg
                        .builder
                        .build_bit_cast(
                            val_val.value().into_float_value(),
                            self.cg.ctx.i64_type(),
                            "val_to_i64",
                        )
                        .unwrap()
                }
                PyType::Bool => {
                    // Zero-extend i1 to i64
                    self.cg
                        .builder
                        .build_int_z_extend(
                            val_val.value().into_int_value(),
                            self.cg.ctx.i64_type(),
                            "val_to_i64",
                        )
                        .unwrap()
                        .into()
                }
                PyType::Str
                | PyType::Bytes
                | PyType::List(_)
                | PyType::Dict(_, _)
                | PyType::Set(_) => {
                    // Cast pointer to i64
                    self.cg
                        .builder
                        .build_ptr_to_int(
                            val_val.value().into_pointer_value(),
                            self.cg.ctx.i64_type(),
                            "val_to_i64",
                        )
                        .unwrap()
                        .into()
                }
                _ => val_val.value(), // Int and other types stay as-is
            };

            self.cg
                .builder
                .build_call(
                    dict_setitem_fn,
                    &[dict_ptr.into(), key_val.value().into(), val_as_i64.into()],
                    "dict_setitem",
                )
                .unwrap();
        }

        Ok(PyValue::new(
            dict_ptr,
            PyType::Dict(Box::new(key_type), Box::new(val_type)),
            None,
        ))
    }

    /// Create a set literal: {1, 2, 3}
    pub(crate) fn visit_set_lit_impl(
        &mut self,
        elements: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        // Infer element type from first element (or default to Int for empty set)
        let elem_type = if elements.is_empty() {
            PyType::Int
        } else {
            let first = self.evaluate_expression(&elements[0])?;
            first.ty().clone()
        };

        // Use type-specific set functions
        let (set_new_fn_name, set_add_fn_name) = match &elem_type {
            PyType::Str => ("str_set_new", "str_set_add"),
            PyType::Float => ("float_set_new", "float_set_add"),
            PyType::Bool => ("bool_set_new", "bool_set_add"),
            _ => ("set_new", "set_add"),
        };

        // Create a new empty set
        let set_new_fn = self.get_or_declare_c_builtin(set_new_fn_name);
        let call_site = self
            .cg
            .builder
            .build_call(set_new_fn, &[], "set_new")
            .unwrap();
        let set_ptr = self.extract_ptr_call_result(call_site).value();

        // Add each element
        let set_add_fn = self.get_or_declare_c_builtin(set_add_fn_name);
        for elem in elements {
            let elem_val = self.evaluate_expression(elem)?;

            // For bools, extend i1 to i8 for C ABI compatibility
            let arg_val = if matches!(elem_type, PyType::Bool) {
                self.cg
                    .builder
                    .build_int_z_extend(
                        elem_val.value().into_int_value(),
                        self.cg.ctx.i8_type(),
                        "bool_to_i8",
                    )
                    .unwrap()
                    .into()
            } else {
                elem_val.value().into()
            };

            self.cg
                .builder
                .build_call(set_add_fn, &[set_ptr.into(), arg_val], "set_add")
                .unwrap();
        }

        Ok(PyValue::new(
            set_ptr,
            PyType::Set(Box::new(elem_type)),
            None,
        ))
    }

    /// Create a dict comprehension: {k: v for k, v in items if cond}
    pub(crate) fn visit_dict_comprehension_impl(
        &mut self,
        key_expr: &Expression,
        value_expr: &Expression,
        clauses: &[crate::ast::ComprehensionClause],
    ) -> Result<PyValue<'ctx>, String> {
        if clauses.is_empty() {
            return Err("Dict comprehension must have at least one clause".to_string());
        }

        // Create empty dict
        let dict_new_fn = self.get_or_declare_c_builtin("dict_new");
        let call_site = self
            .cg
            .builder
            .build_call(dict_new_fn, &[], "dictcomp_new")
            .unwrap();
        let result_dict_ptr = self.extract_ptr_call_result(call_site).value();

        // Get current function
        let function = self.current_function.unwrap();

        // Create the final "after" block
        let final_after_bb = self.cg.ctx.append_basic_block(function, "dictcomp_done");

        // Generate nested loops for all clauses
        let (key_type, val_type) = self.generate_dictcomp_clause(
            result_dict_ptr,
            key_expr,
            value_expr,
            clauses,
            0,
            final_after_bb,
            function,
        )?;

        // Position at final after block
        self.cg.builder.position_at_end(final_after_bb);

        Ok(PyValue::new(
            result_dict_ptr,
            PyType::Dict(Box::new(key_type), Box::new(val_type)),
            None,
        ))
    }

    /// Recursively generate code for a single dict comprehension clause
    #[allow(clippy::too_many_arguments)]
    fn generate_dictcomp_clause(
        &mut self,
        result_dict_ptr: inkwell::values::BasicValueEnum<'ctx>,
        key_expr: &Expression,
        value_expr: &Expression,
        clauses: &[crate::ast::ComprehensionClause],
        clause_idx: usize,
        final_after_bb: inkwell::basic_block::BasicBlock<'ctx>,
        function: inkwell::values::FunctionValue<'ctx>,
    ) -> Result<(PyType, PyType), String> {
        let clause = &clauses[clause_idx];
        let is_last_clause = clause_idx == clauses.len() - 1;

        // Support multi-variable binding (e.g., for k, v in items)
        let targets = &clause.target;

        // Evaluate the iterable
        let iter_val = self.evaluate_expression(&clause.iterable)?;

        // Create basic blocks for this loop level
        let suffix = if clause_idx == 0 {
            String::new()
        } else {
            format!("_{}", clause_idx)
        };
        let cond_bb = self
            .cg
            .ctx
            .append_basic_block(function, &format!("dictcomp_cond{}", suffix));
        let body_bb = self
            .cg
            .ctx
            .append_basic_block(function, &format!("dictcomp_body{}", suffix));
        let after_bb = self
            .cg
            .ctx
            .append_basic_block(function, &format!("dictcomp_after{}", suffix));

        // Handle range iteration
        if crate::codegen::types::is_range(&iter_val.ty()) {
            if targets.len() != 1 {
                return Err("Multi-variable binding not supported for range iteration".to_string());
            }
            return self.generate_dictcomp_clause_range(
                result_dict_ptr,
                &targets[0],
                &iter_val,
                key_expr,
                value_expr,
                clauses,
                clause_idx,
                cond_bb,
                body_bb,
                after_bb,
                final_after_bb,
                function,
            );
        }

        match iter_val.ty() {
            PyType::List(elem_type) => {
                use inkwell::values::AnyValue;
                let list_ptr = iter_val.value().into_pointer_value();
                let list_len_fn = self.get_or_declare_c_builtin("list_len");
                let len_call = self
                    .cg
                    .builder
                    .build_call(list_len_fn, &[list_ptr.into()], "list_len")
                    .unwrap();
                let len = len_call.as_any_value_enum().into_int_value();

                let i64_type = self.cg.ctx.i64_type();
                let idx_name = format!("__dictcomp_idx{}", clause_idx);
                let index_alloca =
                    self.create_entry_block_alloca_with_type(&idx_name, i64_type.into());
                self.cg
                    .builder
                    .build_store(index_alloca, i64_type.const_zero())
                    .unwrap();

                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                self.cg.builder.position_at_end(cond_bb);
                let current_idx = self
                    .cg
                    .builder
                    .build_load(i64_type, index_alloca, "idx")
                    .unwrap()
                    .into_int_value();
                let cond = self
                    .cg
                    .builder
                    .build_int_compare(
                        inkwell::IntPredicate::SLT,
                        current_idx,
                        len,
                        "dictcomp_cond",
                    )
                    .unwrap();
                self.cg
                    .builder
                    .build_conditional_branch(cond, body_bb, after_bb)
                    .unwrap();

                self.cg.builder.position_at_end(body_bb);

                let list_getitem_fn = self.get_or_declare_c_builtin("list_getitem");
                let elem_call = self
                    .cg
                    .builder
                    .build_call(
                        list_getitem_fn,
                        &[list_ptr.into(), current_idx.into()],
                        "getitem",
                    )
                    .unwrap();
                let elem_i64 = elem_call.as_any_value_enum().into_int_value();

                // Save old variables
                let mut old_vars = Vec::new();

                // Handle tuple unpacking for multi-variable targets
                if targets.len() == 1 {
                    let var_name = &targets[0];
                    let elem_val = self.convert_i64_to_value(elem_i64, elem_type.as_ref())?;
                    let elem_llvm_ty = elem_type.to_llvm(self.cg.ctx);
                    let var_alloca =
                        self.create_entry_block_alloca_with_type(var_name, elem_llvm_ty);
                    self.cg
                        .builder
                        .build_store(var_alloca, elem_val.value())
                        .unwrap();
                    old_vars.push((var_name.clone(), self.variables.get(var_name).cloned()));
                    self.variables.insert(
                        var_name.clone(),
                        PyValue::new(
                            elem_val.value(),
                            elem_type.as_ref().clone(),
                            Some(var_alloca),
                        ),
                    );
                } else if targets.len() == 2 {
                    // Handle tuple unpacking (k, v in items)
                    if let PyType::Tuple(tuple_types) = elem_type.as_ref() {
                        if tuple_types.len() >= 2 {
                            let tuple_ptr = self
                                .cg
                                .builder
                                .build_int_to_ptr(
                                    elem_i64,
                                    self.cg.ctx.ptr_type(inkwell::AddressSpace::default()),
                                    "tuple_ptr",
                                )
                                .unwrap();

                            let tuple_getitem_fn = self.get_or_declare_c_builtin("tuple_getitem");
                            for (i, var_name) in targets.iter().enumerate() {
                                let idx = self.cg.ctx.i64_type().const_int(i as u64, false);
                                let item_call = self
                                    .cg
                                    .builder
                                    .build_call(
                                        tuple_getitem_fn,
                                        &[tuple_ptr.into(), idx.into()],
                                        "tuple_item",
                                    )
                                    .unwrap();
                                let item_i64 = item_call.as_any_value_enum().into_int_value();
                                let item_val =
                                    self.convert_i64_to_value(item_i64, &tuple_types[i])?;
                                let item_llvm_ty = tuple_types[i].to_llvm(self.cg.ctx);
                                let var_alloca = self
                                    .create_entry_block_alloca_with_type(var_name, item_llvm_ty);
                                self.cg
                                    .builder
                                    .build_store(var_alloca, item_val.value())
                                    .unwrap();
                                old_vars.push((
                                    var_name.clone(),
                                    self.variables.get(var_name).cloned(),
                                ));
                                self.variables.insert(
                                    var_name.clone(),
                                    PyValue::new(
                                        item_val.value(),
                                        tuple_types[i].clone(),
                                        Some(var_alloca),
                                    ),
                                );
                            }
                        } else {
                            return Err(
                                "Tuple unpacking requires tuple with enough elements".to_string()
                            );
                        }
                    } else {
                        return Err(format!(
                            "Multi-variable binding requires tuple type, got {:?}",
                            elem_type
                        ));
                    }
                } else {
                    return Err(
                        "Dict comprehension supports at most 2 target variables".to_string()
                    );
                }

                let inc_suffix = if clause_idx == 0 {
                    String::new()
                } else {
                    format!("_{}", clause_idx)
                };
                let inc_bb = self
                    .cg
                    .ctx
                    .append_basic_block(function, &format!("dictcomp_inc{}", inc_suffix));

                // Evaluate conditions
                if !clause.conditions.is_empty() {
                    let inner_bb = self
                        .cg
                        .ctx
                        .append_basic_block(function, &format!("dictcomp_inner{}", inc_suffix));
                    for (i, cond_expr) in clause.conditions.iter().enumerate() {
                        let cond_val = self.evaluate_expression(cond_expr)?;
                        let cond_bool = self.cg.value_to_bool(&cond_val);
                        if i < clause.conditions.len() - 1 {
                            let next_cond_bb = self.cg.ctx.append_basic_block(
                                function,
                                &format!("dictcomp_cond{}_{}", clause_idx, i + 1),
                            );
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, next_cond_bb, inc_bb)
                                .unwrap();
                            self.cg.builder.position_at_end(next_cond_bb);
                        } else {
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, inner_bb, inc_bb)
                                .unwrap();
                        }
                    }
                    self.cg.builder.position_at_end(inner_bb);
                }

                // Either recurse or insert into dict
                let (key_type, val_type) = if is_last_clause {
                    let key_val = self.evaluate_expression(key_expr)?;
                    let val_val = self.evaluate_expression(value_expr)?;
                    let key_type = key_val.ty().clone();
                    let val_type = val_val.ty().clone();

                    // Convert value to i64 for storage
                    let val_as_i64 = self.value_to_i64(&val_val);

                    let dict_setitem_fn = self.get_or_declare_c_builtin("dict_setitem");
                    self.cg
                        .builder
                        .build_call(
                            dict_setitem_fn,
                            &[
                                result_dict_ptr.into(),
                                key_val.value().into(),
                                val_as_i64.into(),
                            ],
                            "dict_setitem",
                        )
                        .unwrap();

                    (key_type, val_type)
                } else {
                    self.generate_dictcomp_clause(
                        result_dict_ptr,
                        key_expr,
                        value_expr,
                        clauses,
                        clause_idx + 1,
                        final_after_bb,
                        function,
                    )?
                };

                self.cg.builder.build_unconditional_branch(inc_bb).unwrap();

                self.cg.builder.position_at_end(inc_bb);
                let next_idx = self
                    .cg
                    .builder
                    .build_int_add(current_idx, i64_type.const_int(1, false), "next_idx")
                    .unwrap();
                self.cg.builder.build_store(index_alloca, next_idx).unwrap();
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                self.cg.builder.position_at_end(after_bb);

                // Restore old variables
                for (var_name, old_val) in old_vars {
                    if let Some(old) = old_val {
                        self.variables.insert(var_name, old);
                    } else {
                        self.variables.remove(&var_name);
                    }
                }

                if clause_idx == 0 {
                    self.cg
                        .builder
                        .build_unconditional_branch(final_after_bb)
                        .unwrap();
                }

                Ok((key_type, val_type))
            }
            PyType::Str => {
                // Iterate over string characters
                use inkwell::values::AnyValue;
                let str_ptr = iter_val.value().into_pointer_value();
                let str_len_fn = self.get_or_declare_c_builtin("str_len");
                let len_call = self
                    .cg
                    .builder
                    .build_call(str_len_fn, &[str_ptr.into()], "str_len")
                    .unwrap();
                let len = len_call.as_any_value_enum().into_int_value();

                if targets.len() != 1 {
                    return Err(
                        "Multi-variable binding not supported for string iteration".to_string()
                    );
                }
                let var_name = &targets[0];

                let i64_type = self.cg.ctx.i64_type();
                let idx_name = format!("__dictcomp_str_idx{}", clause_idx);
                let index_alloca =
                    self.create_entry_block_alloca_with_type(&idx_name, i64_type.into());
                self.cg
                    .builder
                    .build_store(index_alloca, i64_type.const_zero())
                    .unwrap();

                let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());
                let var_alloca =
                    self.create_entry_block_alloca_with_type(var_name, ptr_type.into());

                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                self.cg.builder.position_at_end(cond_bb);
                let current_idx = self
                    .cg
                    .builder
                    .build_load(i64_type, index_alloca, "idx")
                    .unwrap()
                    .into_int_value();
                let cond = self
                    .cg
                    .builder
                    .build_int_compare(
                        inkwell::IntPredicate::SLT,
                        current_idx,
                        len,
                        "dictcomp_str_cond",
                    )
                    .unwrap();
                self.cg
                    .builder
                    .build_conditional_branch(cond, body_bb, after_bb)
                    .unwrap();

                self.cg.builder.position_at_end(body_bb);

                // Get character at index using str_slice(s, index, index+1)
                let str_slice_fn = self.get_or_declare_c_builtin("str_slice");
                let next_idx_slice = self
                    .cg
                    .builder
                    .build_int_add(current_idx, i64_type.const_int(1, false), "next_idx_slice")
                    .unwrap();
                let char_call = self
                    .cg
                    .builder
                    .build_call(
                        str_slice_fn,
                        &[str_ptr.into(), current_idx.into(), next_idx_slice.into()],
                        "char_slice",
                    )
                    .unwrap();
                let char_ptr = char_call.as_any_value_enum().into_pointer_value();

                self.cg.builder.build_store(var_alloca, char_ptr).unwrap();

                let old_var = self.variables.get(var_name).cloned();
                self.variables.insert(
                    var_name.to_string(),
                    PyValue::new(char_ptr.into(), PyType::Str, Some(var_alloca)),
                );

                let inc_suffix = if clause_idx == 0 {
                    String::new()
                } else {
                    format!("_{}", clause_idx)
                };
                let inc_bb = self
                    .cg
                    .ctx
                    .append_basic_block(function, &format!("dictcomp_str_inc{}", inc_suffix));

                if !clause.conditions.is_empty() {
                    let inner_bb = self
                        .cg
                        .ctx
                        .append_basic_block(function, &format!("dictcomp_str_inner{}", inc_suffix));
                    for (i, cond_expr) in clause.conditions.iter().enumerate() {
                        let cond_val = self.evaluate_expression(cond_expr)?;
                        let cond_bool = self.cg.value_to_bool(&cond_val);
                        if i < clause.conditions.len() - 1 {
                            let next_cond_bb = self.cg.ctx.append_basic_block(
                                function,
                                &format!("dictcomp_str_cond{}_{}", clause_idx, i + 1),
                            );
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, next_cond_bb, inc_bb)
                                .unwrap();
                            self.cg.builder.position_at_end(next_cond_bb);
                        } else {
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, inner_bb, inc_bb)
                                .unwrap();
                        }
                    }
                    self.cg.builder.position_at_end(inner_bb);
                }

                let (key_type, val_type) = if is_last_clause {
                    let key_val = self.evaluate_expression(key_expr)?;
                    let val_val = self.evaluate_expression(value_expr)?;
                    let key_type = key_val.ty().clone();
                    let val_type = val_val.ty().clone();

                    let val_as_i64 = self.value_to_i64(&val_val);

                    // Use str_dict_setitem for string keys
                    let dict_setitem_fn = if matches!(key_type, PyType::Str) {
                        self.get_or_declare_c_builtin("str_dict_setitem")
                    } else {
                        self.get_or_declare_c_builtin("dict_setitem")
                    };
                    self.cg
                        .builder
                        .build_call(
                            dict_setitem_fn,
                            &[
                                result_dict_ptr.into(),
                                key_val.value().into(),
                                val_as_i64.into(),
                            ],
                            "dict_setitem",
                        )
                        .unwrap();

                    (key_type, val_type)
                } else {
                    self.generate_dictcomp_clause(
                        result_dict_ptr,
                        key_expr,
                        value_expr,
                        clauses,
                        clause_idx + 1,
                        final_after_bb,
                        function,
                    )?
                };

                self.cg.builder.build_unconditional_branch(inc_bb).unwrap();

                self.cg.builder.position_at_end(inc_bb);
                let next_idx = self
                    .cg
                    .builder
                    .build_int_add(current_idx, i64_type.const_int(1, false), "next_idx")
                    .unwrap();
                self.cg.builder.build_store(index_alloca, next_idx).unwrap();
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                self.cg.builder.position_at_end(after_bb);

                if let Some(old) = old_var {
                    self.variables.insert(var_name.to_string(), old);
                } else {
                    self.variables.remove(var_name);
                }

                if clause_idx == 0 {
                    self.cg
                        .builder
                        .build_unconditional_branch(final_after_bb)
                        .unwrap();
                }

                Ok((key_type, val_type))
            }
            _ => Err(format!(
                "Dict comprehension over {:?} not yet supported",
                iter_val.ty()
            )),
        }
    }

    /// Helper to generate dict comprehension over range
    #[allow(clippy::too_many_arguments)]
    fn generate_dictcomp_clause_range(
        &mut self,
        result_dict_ptr: inkwell::values::BasicValueEnum<'ctx>,
        var_name: &str,
        iter_val: &PyValue<'ctx>,
        key_expr: &Expression,
        value_expr: &Expression,
        clauses: &[crate::ast::ComprehensionClause],
        clause_idx: usize,
        cond_bb: inkwell::basic_block::BasicBlock<'ctx>,
        body_bb: inkwell::basic_block::BasicBlock<'ctx>,
        after_bb: inkwell::basic_block::BasicBlock<'ctx>,
        final_after_bb: inkwell::basic_block::BasicBlock<'ctx>,
        function: inkwell::values::FunctionValue<'ctx>,
    ) -> Result<(PyType, PyType), String> {
        use inkwell::values::AnyValue;

        let clause = &clauses[clause_idx];
        let is_last_clause = clause_idx == clauses.len() - 1;

        let range_ptr = iter_val.value().into_pointer_value();
        let range_iter_fn = self.get_or_declare_c_builtin("range_iter");
        let iter_call = self
            .cg
            .builder
            .build_call(range_iter_fn, &[range_ptr.into()], "range_iter")
            .unwrap();
        let iter_ptr = iter_call.as_any_value_enum().into_pointer_value();

        let i64_type = self.cg.ctx.i64_type();
        let var_alloca = self.create_entry_block_alloca_with_type(var_name, i64_type.into());
        let out_name = format!("dictcomp_iter_out{}", clause_idx);
        let out_value_alloca = self.cg.builder.build_alloca(i64_type, &out_name).unwrap();

        self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

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

        self.cg.builder.position_at_end(body_bb);

        let elem_val = self
            .cg
            .builder
            .build_load(i64_type, out_value_alloca, "elem")
            .unwrap();
        self.cg.builder.build_store(var_alloca, elem_val).unwrap();

        let old_var = self.variables.get(var_name).cloned();
        self.variables.insert(
            var_name.to_string(),
            PyValue::new(elem_val, PyType::Int, Some(var_alloca)),
        );

        // Evaluate conditions
        if !clause.conditions.is_empty() {
            let inc_suffix = if clause_idx == 0 {
                String::new()
            } else {
                format!("_{}", clause_idx)
            };
            let inner_bb = self
                .cg
                .ctx
                .append_basic_block(function, &format!("dictcomp_range_inner{}", inc_suffix));
            for (i, cond_expr) in clause.conditions.iter().enumerate() {
                let cond_val = self.evaluate_expression(cond_expr)?;
                let cond_bool = self.cg.value_to_bool(&cond_val);
                if i < clause.conditions.len() - 1 {
                    let next_cond_bb = self.cg.ctx.append_basic_block(
                        function,
                        &format!("dictcomp_range_cond{}_{}", clause_idx, i + 1),
                    );
                    self.cg
                        .builder
                        .build_conditional_branch(cond_bool, next_cond_bb, cond_bb)
                        .unwrap();
                    self.cg.builder.position_at_end(next_cond_bb);
                } else {
                    self.cg
                        .builder
                        .build_conditional_branch(cond_bool, inner_bb, cond_bb)
                        .unwrap();
                }
            }
            self.cg.builder.position_at_end(inner_bb);
        }

        let (key_type, val_type) = if is_last_clause {
            let key_val = self.evaluate_expression(key_expr)?;
            let val_val = self.evaluate_expression(value_expr)?;
            let key_type = key_val.ty().clone();
            let val_type = val_val.ty().clone();

            let val_as_i64 = self.value_to_i64(&val_val);

            let dict_setitem_fn = self.get_or_declare_c_builtin("dict_setitem");
            self.cg
                .builder
                .build_call(
                    dict_setitem_fn,
                    &[
                        result_dict_ptr.into(),
                        key_val.value().into(),
                        val_as_i64.into(),
                    ],
                    "dict_setitem",
                )
                .unwrap();

            (key_type, val_type)
        } else {
            self.generate_dictcomp_clause(
                result_dict_ptr,
                key_expr,
                value_expr,
                clauses,
                clause_idx + 1,
                final_after_bb,
                function,
            )?
        };

        self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

        self.cg.builder.position_at_end(after_bb);

        if let Some(old) = old_var {
            self.variables.insert(var_name.to_string(), old);
        } else {
            self.variables.remove(var_name);
        }

        if clause_idx == 0 {
            self.cg
                .builder
                .build_unconditional_branch(final_after_bb)
                .unwrap();
        }

        Ok((key_type, val_type))
    }

    /// Convert a PyValue to i64 for dict/set storage
    fn value_to_i64(&mut self, val: &PyValue<'ctx>) -> inkwell::values::BasicValueEnum<'ctx> {
        match val.ty() {
            PyType::Float => self
                .cg
                .builder
                .build_bit_cast(
                    val.value().into_float_value(),
                    self.cg.ctx.i64_type(),
                    "val_to_i64",
                )
                .unwrap(),
            PyType::Bool => self
                .cg
                .builder
                .build_int_z_extend(
                    val.value().into_int_value(),
                    self.cg.ctx.i64_type(),
                    "val_to_i64",
                )
                .unwrap()
                .into(),
            PyType::Str
            | PyType::Bytes
            | PyType::List(_)
            | PyType::Dict(_, _)
            | PyType::Set(_)
            | PyType::Tuple(_) => self
                .cg
                .builder
                .build_ptr_to_int(
                    val.value().into_pointer_value(),
                    self.cg.ctx.i64_type(),
                    "val_to_i64",
                )
                .unwrap()
                .into(),
            _ => val.value(),
        }
    }

    /// Create a set comprehension: {x for x in items if cond}
    pub(crate) fn visit_set_comprehension_impl(
        &mut self,
        element: &Expression,
        clauses: &[crate::ast::ComprehensionClause],
    ) -> Result<PyValue<'ctx>, String> {
        if clauses.is_empty() {
            return Err("Set comprehension must have at least one clause".to_string());
        }

        // Create empty set - we'll determine the type from first element
        let set_new_fn = self.get_or_declare_c_builtin("set_new");
        let call_site = self
            .cg
            .builder
            .build_call(set_new_fn, &[], "setcomp_new")
            .unwrap();
        let result_set_ptr = self.extract_ptr_call_result(call_site).value();

        let function = self.current_function.unwrap();
        let final_after_bb = self.cg.ctx.append_basic_block(function, "setcomp_done");

        let elem_type = self.generate_setcomp_clause(
            result_set_ptr,
            element,
            clauses,
            0,
            final_after_bb,
            function,
        )?;

        self.cg.builder.position_at_end(final_after_bb);

        Ok(PyValue::new(
            result_set_ptr,
            PyType::Set(Box::new(elem_type)),
            None,
        ))
    }

    /// Recursively generate code for a single set comprehension clause
    fn generate_setcomp_clause(
        &mut self,
        result_set_ptr: inkwell::values::BasicValueEnum<'ctx>,
        element: &Expression,
        clauses: &[crate::ast::ComprehensionClause],
        clause_idx: usize,
        final_after_bb: inkwell::basic_block::BasicBlock<'ctx>,
        function: inkwell::values::FunctionValue<'ctx>,
    ) -> Result<PyType, String> {
        let clause = &clauses[clause_idx];
        let is_last_clause = clause_idx == clauses.len() - 1;

        if clause.target.len() != 1 {
            return Err("Multi-variable set comprehension not yet supported".to_string());
        }
        let var_name = &clause.target[0];

        let iter_val = self.evaluate_expression(&clause.iterable)?;

        let suffix = if clause_idx == 0 {
            String::new()
        } else {
            format!("_{}", clause_idx)
        };
        let cond_bb = self
            .cg
            .ctx
            .append_basic_block(function, &format!("setcomp_cond{}", suffix));
        let body_bb = self
            .cg
            .ctx
            .append_basic_block(function, &format!("setcomp_body{}", suffix));
        let after_bb = self
            .cg
            .ctx
            .append_basic_block(function, &format!("setcomp_after{}", suffix));

        if crate::codegen::types::is_range(&iter_val.ty()) {
            return self.generate_setcomp_clause_range(
                result_set_ptr,
                var_name,
                &iter_val,
                element,
                clauses,
                clause_idx,
                cond_bb,
                body_bb,
                after_bb,
                final_after_bb,
                function,
            );
        }

        match iter_val.ty() {
            PyType::List(elem_type) => {
                use inkwell::values::AnyValue;
                let list_ptr = iter_val.value().into_pointer_value();
                let list_len_fn = self.get_or_declare_c_builtin("list_len");
                let len_call = self
                    .cg
                    .builder
                    .build_call(list_len_fn, &[list_ptr.into()], "list_len")
                    .unwrap();
                let len = len_call.as_any_value_enum().into_int_value();

                let i64_type = self.cg.ctx.i64_type();
                let idx_name = format!("__setcomp_idx{}", clause_idx);
                let index_alloca =
                    self.create_entry_block_alloca_with_type(&idx_name, i64_type.into());
                self.cg
                    .builder
                    .build_store(index_alloca, i64_type.const_zero())
                    .unwrap();

                let elem_llvm_ty = elem_type.to_llvm(self.cg.ctx);
                let var_alloca = self.create_entry_block_alloca_with_type(var_name, elem_llvm_ty);

                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                self.cg.builder.position_at_end(cond_bb);
                let current_idx = self
                    .cg
                    .builder
                    .build_load(i64_type, index_alloca, "idx")
                    .unwrap()
                    .into_int_value();
                let cond = self
                    .cg
                    .builder
                    .build_int_compare(inkwell::IntPredicate::SLT, current_idx, len, "setcomp_cond")
                    .unwrap();
                self.cg
                    .builder
                    .build_conditional_branch(cond, body_bb, after_bb)
                    .unwrap();

                self.cg.builder.position_at_end(body_bb);

                let list_getitem_fn = self.get_or_declare_c_builtin("list_getitem");
                let elem_call = self
                    .cg
                    .builder
                    .build_call(
                        list_getitem_fn,
                        &[list_ptr.into(), current_idx.into()],
                        "getitem",
                    )
                    .unwrap();
                let elem_i64 = elem_call.as_any_value_enum().into_int_value();

                let elem_val = self.convert_i64_to_value(elem_i64, elem_type.as_ref())?;
                self.cg
                    .builder
                    .build_store(var_alloca, elem_val.value())
                    .unwrap();

                let old_var = self.variables.get(var_name).cloned();
                self.variables.insert(
                    var_name.to_string(),
                    PyValue::new(
                        elem_val.value(),
                        elem_type.as_ref().clone(),
                        Some(var_alloca),
                    ),
                );

                let inc_suffix = if clause_idx == 0 {
                    String::new()
                } else {
                    format!("_{}", clause_idx)
                };
                let inc_bb = self
                    .cg
                    .ctx
                    .append_basic_block(function, &format!("setcomp_inc{}", inc_suffix));

                if !clause.conditions.is_empty() {
                    let inner_bb = self
                        .cg
                        .ctx
                        .append_basic_block(function, &format!("setcomp_inner{}", inc_suffix));
                    for (i, cond_expr) in clause.conditions.iter().enumerate() {
                        let cond_val = self.evaluate_expression(cond_expr)?;
                        let cond_bool = self.cg.value_to_bool(&cond_val);
                        if i < clause.conditions.len() - 1 {
                            let next_cond_bb = self.cg.ctx.append_basic_block(
                                function,
                                &format!("setcomp_cond{}_{}", clause_idx, i + 1),
                            );
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, next_cond_bb, inc_bb)
                                .unwrap();
                            self.cg.builder.position_at_end(next_cond_bb);
                        } else {
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, inner_bb, inc_bb)
                                .unwrap();
                        }
                    }
                    self.cg.builder.position_at_end(inner_bb);
                }

                let result_type = if is_last_clause {
                    let result_val = self.evaluate_expression(element)?;
                    let result_type = result_val.ty().clone();

                    // Use str_set_add for string elements (pass ptr directly),
                    // otherwise convert pointer types to i64 for generic set_add
                    let (set_add_fn, elem_arg) = if matches!(result_type, PyType::Str) {
                        (
                            self.get_or_declare_c_builtin("str_set_add"),
                            result_val.value().into(),
                        )
                    } else {
                        (
                            self.get_or_declare_c_builtin("set_add"),
                            self.value_to_i64(&result_val).into(),
                        )
                    };
                    self.cg
                        .builder
                        .build_call(set_add_fn, &[result_set_ptr.into(), elem_arg], "set_add")
                        .unwrap();

                    result_type
                } else {
                    self.generate_setcomp_clause(
                        result_set_ptr,
                        element,
                        clauses,
                        clause_idx + 1,
                        final_after_bb,
                        function,
                    )?
                };

                self.cg.builder.build_unconditional_branch(inc_bb).unwrap();

                self.cg.builder.position_at_end(inc_bb);
                let next_idx = self
                    .cg
                    .builder
                    .build_int_add(current_idx, i64_type.const_int(1, false), "next_idx")
                    .unwrap();
                self.cg.builder.build_store(index_alloca, next_idx).unwrap();
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                self.cg.builder.position_at_end(after_bb);

                if let Some(old) = old_var {
                    self.variables.insert(var_name.to_string(), old);
                } else {
                    self.variables.remove(var_name);
                }

                if clause_idx == 0 {
                    self.cg
                        .builder
                        .build_unconditional_branch(final_after_bb)
                        .unwrap();
                }

                Ok(result_type)
            }
            PyType::Str => {
                // Iterate over string characters for set comprehension
                use inkwell::values::AnyValue;
                let str_ptr = iter_val.value().into_pointer_value();
                let str_len_fn = self.get_or_declare_c_builtin("str_len");
                let len_call = self
                    .cg
                    .builder
                    .build_call(str_len_fn, &[str_ptr.into()], "str_len")
                    .unwrap();
                let len = len_call.as_any_value_enum().into_int_value();

                let i64_type = self.cg.ctx.i64_type();
                let idx_name = format!("__setcomp_str_idx{}", clause_idx);
                let index_alloca =
                    self.create_entry_block_alloca_with_type(&idx_name, i64_type.into());
                self.cg
                    .builder
                    .build_store(index_alloca, i64_type.const_zero())
                    .unwrap();

                let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());
                let var_alloca =
                    self.create_entry_block_alloca_with_type(var_name, ptr_type.into());

                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                self.cg.builder.position_at_end(cond_bb);
                let current_idx = self
                    .cg
                    .builder
                    .build_load(i64_type, index_alloca, "idx")
                    .unwrap()
                    .into_int_value();
                let cond = self
                    .cg
                    .builder
                    .build_int_compare(
                        inkwell::IntPredicate::SLT,
                        current_idx,
                        len,
                        "setcomp_str_cond",
                    )
                    .unwrap();
                self.cg
                    .builder
                    .build_conditional_branch(cond, body_bb, after_bb)
                    .unwrap();

                self.cg.builder.position_at_end(body_bb);

                // Get character at index using str_slice(s, index, index+1)
                let str_slice_fn = self.get_or_declare_c_builtin("str_slice");
                let next_idx_slice = self
                    .cg
                    .builder
                    .build_int_add(current_idx, i64_type.const_int(1, false), "next_idx_slice")
                    .unwrap();
                let char_call = self
                    .cg
                    .builder
                    .build_call(
                        str_slice_fn,
                        &[str_ptr.into(), current_idx.into(), next_idx_slice.into()],
                        "char_slice",
                    )
                    .unwrap();
                let char_ptr = char_call.as_any_value_enum().into_pointer_value();

                self.cg.builder.build_store(var_alloca, char_ptr).unwrap();

                let old_var = self.variables.get(var_name).cloned();
                self.variables.insert(
                    var_name.to_string(),
                    PyValue::new(char_ptr.into(), PyType::Str, Some(var_alloca)),
                );

                let inc_suffix = if clause_idx == 0 {
                    String::new()
                } else {
                    format!("_{}", clause_idx)
                };
                let inc_bb = self
                    .cg
                    .ctx
                    .append_basic_block(function, &format!("setcomp_str_inc{}", inc_suffix));

                if !clause.conditions.is_empty() {
                    let inner_bb = self
                        .cg
                        .ctx
                        .append_basic_block(function, &format!("setcomp_str_inner{}", inc_suffix));
                    for (i, cond_expr) in clause.conditions.iter().enumerate() {
                        let cond_val = self.evaluate_expression(cond_expr)?;
                        let cond_bool = self.cg.value_to_bool(&cond_val);
                        if i < clause.conditions.len() - 1 {
                            let next_cond_bb = self.cg.ctx.append_basic_block(
                                function,
                                &format!("setcomp_str_cond{}_{}", clause_idx, i + 1),
                            );
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, next_cond_bb, inc_bb)
                                .unwrap();
                            self.cg.builder.position_at_end(next_cond_bb);
                        } else {
                            self.cg
                                .builder
                                .build_conditional_branch(cond_bool, inner_bb, inc_bb)
                                .unwrap();
                        }
                    }
                    self.cg.builder.position_at_end(inner_bb);
                }

                let result_type = if is_last_clause {
                    let result_val = self.evaluate_expression(element)?;
                    let result_type = result_val.ty().clone();

                    // Use str_set_add for string elements (pass ptr directly),
                    // otherwise convert pointer types to i64 for generic set_add
                    let (set_add_fn, elem_arg) = if matches!(result_type, PyType::Str) {
                        (
                            self.get_or_declare_c_builtin("str_set_add"),
                            result_val.value().into(),
                        )
                    } else {
                        (
                            self.get_or_declare_c_builtin("set_add"),
                            self.value_to_i64(&result_val).into(),
                        )
                    };
                    self.cg
                        .builder
                        .build_call(set_add_fn, &[result_set_ptr.into(), elem_arg], "set_add")
                        .unwrap();

                    result_type
                } else {
                    self.generate_setcomp_clause(
                        result_set_ptr,
                        element,
                        clauses,
                        clause_idx + 1,
                        final_after_bb,
                        function,
                    )?
                };

                self.cg.builder.build_unconditional_branch(inc_bb).unwrap();

                self.cg.builder.position_at_end(inc_bb);
                let next_idx = self
                    .cg
                    .builder
                    .build_int_add(current_idx, i64_type.const_int(1, false), "next_idx")
                    .unwrap();
                self.cg.builder.build_store(index_alloca, next_idx).unwrap();
                self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

                self.cg.builder.position_at_end(after_bb);

                if let Some(old) = old_var {
                    self.variables.insert(var_name.to_string(), old);
                } else {
                    self.variables.remove(var_name);
                }

                if clause_idx == 0 {
                    self.cg
                        .builder
                        .build_unconditional_branch(final_after_bb)
                        .unwrap();
                }

                Ok(result_type)
            }
            _ => Err(format!(
                "Set comprehension over {:?} not yet supported",
                iter_val.ty()
            )),
        }
    }

    /// Helper to generate set comprehension over range
    #[allow(clippy::too_many_arguments)]
    fn generate_setcomp_clause_range(
        &mut self,
        result_set_ptr: inkwell::values::BasicValueEnum<'ctx>,
        var_name: &str,
        iter_val: &PyValue<'ctx>,
        element: &Expression,
        clauses: &[crate::ast::ComprehensionClause],
        clause_idx: usize,
        cond_bb: inkwell::basic_block::BasicBlock<'ctx>,
        body_bb: inkwell::basic_block::BasicBlock<'ctx>,
        after_bb: inkwell::basic_block::BasicBlock<'ctx>,
        final_after_bb: inkwell::basic_block::BasicBlock<'ctx>,
        function: inkwell::values::FunctionValue<'ctx>,
    ) -> Result<PyType, String> {
        use inkwell::values::AnyValue;

        let clause = &clauses[clause_idx];
        let is_last_clause = clause_idx == clauses.len() - 1;

        let range_ptr = iter_val.value().into_pointer_value();
        let range_iter_fn = self.get_or_declare_c_builtin("range_iter");
        let iter_call = self
            .cg
            .builder
            .build_call(range_iter_fn, &[range_ptr.into()], "range_iter")
            .unwrap();
        let iter_ptr = iter_call.as_any_value_enum().into_pointer_value();

        let i64_type = self.cg.ctx.i64_type();
        let var_alloca = self.create_entry_block_alloca_with_type(var_name, i64_type.into());
        let out_name = format!("setcomp_iter_out{}", clause_idx);
        let out_value_alloca = self.cg.builder.build_alloca(i64_type, &out_name).unwrap();

        self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

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

        self.cg.builder.position_at_end(body_bb);

        let elem_val = self
            .cg
            .builder
            .build_load(i64_type, out_value_alloca, "elem")
            .unwrap();
        self.cg.builder.build_store(var_alloca, elem_val).unwrap();

        let old_var = self.variables.get(var_name).cloned();
        self.variables.insert(
            var_name.to_string(),
            PyValue::new(elem_val, PyType::Int, Some(var_alloca)),
        );

        if !clause.conditions.is_empty() {
            let inc_suffix = if clause_idx == 0 {
                String::new()
            } else {
                format!("_{}", clause_idx)
            };
            let inner_bb = self
                .cg
                .ctx
                .append_basic_block(function, &format!("setcomp_range_inner{}", inc_suffix));
            for (i, cond_expr) in clause.conditions.iter().enumerate() {
                let cond_val = self.evaluate_expression(cond_expr)?;
                let cond_bool = self.cg.value_to_bool(&cond_val);
                if i < clause.conditions.len() - 1 {
                    let next_cond_bb = self.cg.ctx.append_basic_block(
                        function,
                        &format!("setcomp_range_cond{}_{}", clause_idx, i + 1),
                    );
                    self.cg
                        .builder
                        .build_conditional_branch(cond_bool, next_cond_bb, cond_bb)
                        .unwrap();
                    self.cg.builder.position_at_end(next_cond_bb);
                } else {
                    self.cg
                        .builder
                        .build_conditional_branch(cond_bool, inner_bb, cond_bb)
                        .unwrap();
                }
            }
            self.cg.builder.position_at_end(inner_bb);
        }

        let result_type = if is_last_clause {
            let result_val = self.evaluate_expression(element)?;
            let result_type = result_val.ty().clone();

            // Use str_set_add for string elements (pass ptr directly),
            // otherwise convert pointer types to i64 for generic set_add
            let (set_add_fn, elem_arg) = if matches!(result_type, PyType::Str) {
                (
                    self.get_or_declare_c_builtin("str_set_add"),
                    result_val.value().into(),
                )
            } else {
                (
                    self.get_or_declare_c_builtin("set_add"),
                    self.value_to_i64(&result_val).into(),
                )
            };
            self.cg
                .builder
                .build_call(set_add_fn, &[result_set_ptr.into(), elem_arg], "set_add")
                .unwrap();

            result_type
        } else {
            self.generate_setcomp_clause(
                result_set_ptr,
                element,
                clauses,
                clause_idx + 1,
                final_after_bb,
                function,
            )?
        };

        self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

        self.cg.builder.position_at_end(after_bb);

        if let Some(old) = old_var {
            self.variables.insert(var_name.to_string(), old);
        } else {
            self.variables.remove(var_name);
        }

        if clause_idx == 0 {
            self.cg
                .builder
                .build_unconditional_branch(final_after_bb)
                .unwrap();
        }

        Ok(result_type)
    }

    /// Create a generator expression: (x for x in items)
    /// For simplicity, we eagerly evaluate the generator to a list.
    /// This provides compatibility with for loops and functions like sum(), list(), etc.
    pub(crate) fn visit_generator_expression_impl(
        &mut self,
        element: &Expression,
        clauses: &[crate::ast::ComprehensionClause],
    ) -> Result<PyValue<'ctx>, String> {
        // Generator expressions are just lazy list comprehensions
        // We materialize them to a list for now, which works for:
        // - for loops: for x in (expr for x in items)
        // - built-in functions: sum(expr for x in items), list(expr for x in items)
        self.visit_list_comprehension_impl(element, clauses)
    }

    /// Create a ternary/conditional expression: true_value if condition else false_value
    pub(crate) fn visit_ternary_impl(
        &mut self,
        condition: &Expression,
        true_value: &Expression,
        false_value: &Expression,
    ) -> Result<PyValue<'ctx>, String> {
        let function = self
            .cg
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();

        // Create basic blocks
        let then_bb = self.cg.ctx.append_basic_block(function, "ternary_then");
        let else_bb = self.cg.ctx.append_basic_block(function, "ternary_else");
        let merge_bb = self.cg.ctx.append_basic_block(function, "ternary_merge");

        // Evaluate condition
        let cond_val = self.evaluate_expression(condition)?;
        let cond_bool = match cond_val.ty() {
            PyType::Bool => cond_val.value().into_int_value(),
            PyType::Int => {
                let zero = self.cg.ctx.i64_type().const_zero();
                self.cg
                    .builder
                    .build_int_compare(
                        inkwell::IntPredicate::NE,
                        cond_val.value().into_int_value(),
                        zero,
                        "int_to_bool",
                    )
                    .unwrap()
            }
            _ => {
                return Err(format!(
                    "Ternary condition must be bool or int, got {:?}",
                    cond_val.ty()
                ))
            }
        };

        self.cg
            .builder
            .build_conditional_branch(cond_bool, then_bb, else_bb)
            .unwrap();

        // Then branch
        self.cg.builder.position_at_end(then_bb);
        let then_val = self.evaluate_expression(true_value)?;
        let then_result = then_val.value();
        let then_end_bb = self.cg.builder.get_insert_block().unwrap();
        self.cg
            .builder
            .build_unconditional_branch(merge_bb)
            .unwrap();

        // Else branch
        self.cg.builder.position_at_end(else_bb);
        let else_val = self.evaluate_expression(false_value)?;
        let else_result = else_val.value();
        let else_end_bb = self.cg.builder.get_insert_block().unwrap();
        self.cg
            .builder
            .build_unconditional_branch(merge_bb)
            .unwrap();

        // Merge
        self.cg.builder.position_at_end(merge_bb);

        // Create PHI node for the result
        let result_type = then_val.ty().clone();
        let phi = self
            .cg
            .builder
            .build_phi(then_result.get_type(), "ternary_result")
            .unwrap();
        phi.add_incoming(&[(&then_result, then_end_bb), (&else_result, else_end_bb)]);

        Ok(PyValue::new(phi.as_basic_value(), result_type, None))
    }

    /// Create a tuple literal: (1, 2, 3)
    pub(crate) fn visit_tuple_lit_impl(
        &mut self,
        elements: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        // Create a new tuple with the right size
        let len = self
            .cg
            .ctx
            .i64_type()
            .const_int(elements.len() as u64, false);
        let tuple_new_fn = self.get_or_declare_c_builtin("tuple_new");
        let call_site = self
            .cg
            .builder
            .build_call(tuple_new_fn, &[len.into()], "tuple_new")
            .unwrap();
        let tuple_ptr = self.extract_ptr_call_result(call_site).value();

        // Collect element types for heterogeneous tuple type
        let mut elem_types = Vec::new();

        // Set each element
        let tuple_setitem_fn = self.get_or_declare_c_builtin("tuple_setitem");
        for (i, elem) in elements.iter().enumerate() {
            let elem_val = self.evaluate_expression(elem)?;
            elem_types.push(elem_val.ty().clone());

            // Convert value to i64 for storage
            let value_as_i64 = match elem_val.ty() {
                PyType::Float => {
                    // Bitcast f64 to i64
                    self.cg
                        .builder
                        .build_bit_cast(elem_val.value(), self.cg.ctx.i64_type(), "float_as_i64")
                        .unwrap()
                        .into_int_value()
                }
                PyType::Bool => {
                    // Zero-extend i1 to i64
                    self.cg
                        .builder
                        .build_int_z_extend(
                            elem_val.value().into_int_value(),
                            self.cg.ctx.i64_type(),
                            "bool_as_i64",
                        )
                        .unwrap()
                }
                PyType::Str
                | PyType::Bytes
                | PyType::List(_)
                | PyType::Dict(_, _)
                | PyType::Set(_)
                | PyType::Tuple(_) => {
                    // Pointer to int for reference types
                    self.cg
                        .builder
                        .build_ptr_to_int(
                            elem_val.value().into_pointer_value(),
                            self.cg.ctx.i64_type(),
                            "ptr_as_i64",
                        )
                        .unwrap()
                }
                _ => {
                    // Int and other types - use directly
                    elem_val.value().into_int_value()
                }
            };

            let index = self.cg.ctx.i64_type().const_int(i as u64, false);
            self.cg
                .builder
                .build_call(
                    tuple_setitem_fn,
                    &[tuple_ptr.into(), index.into(), value_as_i64.into()],
                    "tuple_setitem",
                )
                .unwrap();
        }

        Ok(PyValue::new(tuple_ptr, PyType::Tuple(elem_types), None))
    }
}
