//! Set operations: method calls
//!
//! This module provides all set-related operations:
//! - Method lookup (add, remove, discard, etc.)
//!
//! All C runtime functions are in src/runtime/builtins/set.c
//! and discovered automatically by build.rs.

use crate::ast::Expression;
use crate::codegen::CodeGen;
use crate::types::{FunctionInfo, PyType, PyValue};
use inkwell::values::BasicValueEnum;

/// Maps method name to (builtin_symbol, return_type)
fn get_set_method_info(name: &str, elem_type: &PyType) -> Option<(&'static str, PyType)> {
    match name {
        // Void methods (mutating in-place)
        "add" => Some(("set_add", PyType::None)),
        "remove" => Some(("set_remove", PyType::None)),
        "discard" => Some(("set_discard", PyType::None)),
        "clear" => Some(("set_clear", PyType::None)),
        "update" => Some(("set_update", PyType::None)),
        "difference_update" => Some(("set_difference_update", PyType::None)),
        "intersection_update" => Some(("set_intersection_update", PyType::None)),
        "symmetric_difference_update" => Some(("set_symmetric_difference_update", PyType::None)),

        // Methods returning an element
        "pop" => Some(("set_pop", elem_type.clone())),

        // Methods returning new set
        "copy" => Some(("set_copy", PyType::Set(Box::new(elem_type.clone())))),
        "union" => Some(("set_union", PyType::Set(Box::new(elem_type.clone())))),
        "intersection" => Some(("set_intersection", PyType::Set(Box::new(elem_type.clone())))),
        "difference" => Some(("set_difference", PyType::Set(Box::new(elem_type.clone())))),
        "symmetric_difference" => Some((
            "set_symmetric_difference",
            PyType::Set(Box::new(elem_type.clone())),
        )),

        // Methods returning bool
        "issubset" => Some(("set_issubset", PyType::Bool)),
        "issuperset" => Some(("set_issuperset", PyType::Bool)),
        "isdisjoint" => Some(("set_isdisjoint", PyType::Bool)),

        _ => None,
    }
}

impl<'ctx> CodeGen<'ctx> {
    // ========================================================================
    // Method calls (e.g., my_set.add(42))
    // ========================================================================

    /// Get a set method as a function with the receiver pre-bound
    pub fn get_set_method(
        &mut self,
        receiver_value: BasicValueEnum<'ctx>,
        method_name: &str,
        elem_type: &PyType,
    ) -> Result<PyValue<'ctx>, String> {
        let (symbol, return_type) = get_set_method_info(method_name, elem_type)
            .ok_or_else(|| format!("set has no method '{}'", method_name))?;

        let function = self.get_or_declare_c_builtin(symbol);

        Ok(PyValue::function(FunctionInfo {
            mangled_name: symbol.to_string(),
            function,
            param_types: vec![],
            return_type,
            bound_args: vec![receiver_value],
        }))
    }

    // ========================================================================
    // Helper operations
    // ========================================================================

    /// Get the length of a set
    pub fn set_len(&mut self, set_val: BasicValueEnum<'ctx>) -> Result<PyValue<'ctx>, String> {
        let len_fn = self.get_or_declare_c_builtin("set_len");
        let call = self
            .builder
            .build_call(len_fn, &[set_val.into()], "set_len")
            .unwrap();
        Ok(self.extract_int_call_result(call))
    }

    // ========================================================================
    // set() builtin function
    // ========================================================================

    /// Generate set() builtin call
    /// set() -> empty set[int] (default element type)
    /// set(existing_set) -> copy of existing_set
    pub fn generate_set_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.is_empty() {
            // Create empty set with default int element type
            let set_new_fn = self.get_or_declare_c_builtin("set_new");
            let call_site = self.builder.build_call(set_new_fn, &[], "set_new").unwrap();
            let set_ptr = self.extract_ptr_call_result(call_site);

            return Ok(PyValue::new(
                set_ptr.value(),
                PyType::Set(Box::new(PyType::Int)),
                None,
            ));
        }

        if args.len() != 1 {
            return Err("set() takes at most 1 argument".to_string());
        }

        // Generate the argument value
        let arg_val = self.evaluate_expression(&args[0])?;

        // If argument is a set, create a copy
        if let PyType::Set(elem_type) = &arg_val.ty {
            let set_copy_fn = self.get_or_declare_c_builtin("set_copy");
            let call_site = self
                .builder
                .build_call(set_copy_fn, &[arg_val.value().into()], "set_copy")
                .unwrap();
            let set_ptr = self.extract_ptr_call_result(call_site);

            return Ok(PyValue::new(
                set_ptr.value(),
                PyType::Set(elem_type.clone()),
                None,
            ));
        }

        Err(format!(
            "set() argument must be a set, got {:?}",
            arg_val.ty
        ))
    }
}
