/// Expression visitor implementation for code generation
use super::super::CodeGen;
use crate::ast::Expression;
use crate::types::{PyType, PyValue};

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn visit_int_lit_impl(&mut self, val: i64) -> Result<PyValue<'ctx>, String> {
        let ir_val = self.context.i64_type().const_int(val as u64, false).into();
        Ok(PyValue::int(ir_val))
    }

    pub(crate) fn visit_float_lit_impl(&mut self, val: f64) -> Result<PyValue<'ctx>, String> {
        let ir_val = self.context.f64_type().const_float(val).into();
        Ok(PyValue::float(ir_val))
    }

    pub(crate) fn visit_str_lit_impl(&mut self, _val: &str) -> Result<PyValue<'ctx>, String> {
        todo!("str type (use bytes instead)")
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
            .builder
            .build_global_string_ptr(val, &str_name)
            .unwrap();
        Ok(PyValue::bytes(str_const.as_pointer_value().into()))
    }

    pub(crate) fn visit_bool_lit_impl(&mut self, val: bool) -> Result<PyValue<'ctx>, String> {
        let ir_val = self.context.bool_type().const_int(val as u64, false).into();
        Ok(PyValue::bool(ir_val))
    }

    pub(crate) fn visit_none_lit_impl(&mut self) -> Result<PyValue<'ctx>, String> {
        // Represent None as i32(0) - consistent with type_to_llvm(Type::None)
        let ir_val = self.context.i32_type().const_zero().into();
        Ok(PyValue::none(ir_val))
    }

    pub(crate) fn visit_var_impl(&mut self, name: &str) -> Result<PyValue<'ctx>, String> {
        // First check local variables (stack allocations)
        if let Some(var) = self.variables.get(name) {
            return Ok(var.load(&self.builder, name));
        }

        // Check if it's a function in the current LLVM module (local functions)
        // This must come before global_variables because global_variables contains
        // placeholder functions from preprocessing
        let mangled_name = self.mangle_function_name(&self.module_name, name);
        if let Some(function) = self.module.get_function(&mangled_name) {
            // Get type info from global_variables if available (for correct return type)
            let (param_types, return_type) = if let Some(global) = self.global_variables.get(name) {
                if let Ok(info) = global.get_function() {
                    (info.param_types.clone(), info.return_type.clone())
                } else {
                    (vec![], crate::types::PyType::None)
                }
            } else {
                (vec![], crate::types::PyType::None)
            };

            return Ok(PyValue::function(crate::types::FunctionInfo {
                mangled_name,
                function,
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
        // Create a new list with capacity for all elements
        let capacity = self
            .context
            .i64_type()
            .const_int(elements.len() as u64, false);
        let list_new_fn = self.get_or_declare_c_builtin("list_with_capacity");
        let call_site = self
            .builder
            .build_call(list_new_fn, &[capacity.into()], "list_new")
            .unwrap();
        let list_ptr = self.extract_ptr_call_result(call_site)?.value();

        // Infer element type from first element (or default to Int for empty list)
        let elem_type = if elements.is_empty() {
            PyType::Int
        } else {
            let first = self.evaluate_expression(&elements[0])?;
            first.ty.clone()
        };

        // Append each element (list_append mutates in place, returns void)
        let list_append_fn = self.get_or_declare_c_builtin("list_append");
        for elem in elements {
            let elem_val = self.evaluate_expression(elem)?;
            // TODO: Type check that elem_val.ty matches elem_type
            self.builder
                .build_call(
                    list_append_fn,
                    &[list_ptr.into(), elem_val.value().into()],
                    "list_append",
                )
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
        // Create a new empty dict
        let dict_new_fn = self.get_or_declare_c_builtin("dict_new");
        let call_site = self
            .builder
            .build_call(dict_new_fn, &[], "dict_new")
            .unwrap();
        let dict_ptr = self.extract_ptr_call_result(call_site)?.value();

        // Infer key/value types from first pair (or default to Int for empty dict)
        let (key_type, val_type) = if pairs.is_empty() {
            (PyType::Int, PyType::Int)
        } else {
            let first_key = self.evaluate_expression(&pairs[0].0)?;
            let first_val = self.evaluate_expression(&pairs[0].1)?;
            (first_key.ty.clone(), first_val.ty.clone())
        };

        // Set each key-value pair
        let dict_setitem_fn = self.get_or_declare_c_builtin("dict_setitem");
        for (key_expr, val_expr) in pairs {
            let key_val = self.evaluate_expression(key_expr)?;
            let val_val = self.evaluate_expression(val_expr)?;
            // TODO: Type check that key/val types match
            self.builder
                .build_call(
                    dict_setitem_fn,
                    &[
                        dict_ptr.into(),
                        key_val.value().into(),
                        val_val.value().into(),
                    ],
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
        // Create a new empty set
        let set_new_fn = self.get_or_declare_c_builtin("set_new");
        let call_site = self.builder.build_call(set_new_fn, &[], "set_new").unwrap();
        let set_ptr = self.extract_ptr_call_result(call_site)?.value();

        // Infer element type from first element (or default to Int for empty set)
        let elem_type = if elements.is_empty() {
            PyType::Int
        } else {
            let first = self.evaluate_expression(&elements[0])?;
            first.ty.clone()
        };

        // Add each element
        let set_add_fn = self.get_or_declare_c_builtin("set_add");
        for elem in elements {
            let elem_val = self.evaluate_expression(elem)?;
            // TODO: Type check that elem_val.ty matches elem_type
            self.builder
                .build_call(
                    set_add_fn,
                    &[set_ptr.into(), elem_val.value().into()],
                    "set_add",
                )
                .unwrap();
        }

        Ok(PyValue::new(
            set_ptr,
            PyType::Set(Box::new(elem_type)),
            None,
        ))
    }
}
