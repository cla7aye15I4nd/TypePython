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
                    if global.ty() == crate::types::PyType::Function {
                        let info = global.get_function();
                        (info.param_types.clone(), info.return_type.clone())
                    } else {
                        (vec![], crate::types::PyType::None)
                    }
                } else {
                    (vec![], crate::types::PyType::None)
                };

            // If it's a generator, wrap the return type in Generator
            if is_generator {
                return_type = PyType::Generator(Box::new(return_type));
            }

            return Ok(PyValue::function(crate::types::FunctionInfo {
                mangled_name,
                param_types,
                return_type,
                bound_args: vec![],
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

        Err(format!("Variable {} not found", name))
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
