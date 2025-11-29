//! Dict operations: method calls and subscript access
//!
//! This module provides all dict-related operations:
//! - Method lookup (get, keys, values, items, etc.)
//! - Subscript operations (dict[key])
//!
//! All C runtime functions are in src/runtime/builtins/dict.c
//! and discovered automatically by build.rs.

use crate::ast::Expression;
use crate::codegen::CodeGen;
use crate::types::{FunctionInfo, PyType, PyValue};
use inkwell::values::BasicValueEnum;

/// Maps method name to (builtin_symbol, return_type)
fn get_dict_method_info(
    name: &str,
    _key_type: &PyType,
    val_type: &PyType,
) -> Option<(&'static str, PyType)> {
    match name {
        // Methods returning values
        "get" => Some(("dict_get", val_type.clone())),
        "pop" => Some(("dict_pop", val_type.clone())),
        "setdefault" => Some(("dict_setdefault", val_type.clone())),

        // Void methods
        "clear" => Some(("dict_clear", PyType::None)),
        "update" => Some(("dict_update", PyType::None)),

        // Methods returning new dict
        "copy" => Some((
            "dict_copy",
            PyType::Dict(Box::new(PyType::Int), Box::new(val_type.clone())),
        )),

        _ => None,
    }
}

impl<'ctx> CodeGen<'ctx> {
    // ========================================================================
    // Method calls (e.g., my_dict.get(key))
    // ========================================================================

    /// Get a dict method as a function with the receiver pre-bound
    pub fn get_dict_method(
        &mut self,
        receiver_value: BasicValueEnum<'ctx>,
        method_name: &str,
        key_type: &PyType,
        val_type: &PyType,
    ) -> Result<PyValue<'ctx>, String> {
        let (symbol, return_type) = get_dict_method_info(method_name, key_type, val_type)
            .ok_or_else(|| format!("dict has no method '{}'", method_name))?;

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
    // Subscript operations (e.g., my_dict[key])
    // ========================================================================

    /// Get an item by key: dict[key] -> value
    pub fn dict_getitem(
        &mut self,
        dict_val: BasicValueEnum<'ctx>,
        key: BasicValueEnum<'ctx>,
        val_type: &PyType,
    ) -> Result<PyValue<'ctx>, String> {
        let getitem_fn = self.get_or_declare_c_builtin("dict_getitem");
        let call = self
            .builder
            .build_call(getitem_fn, &[dict_val.into(), key.into()], "dict_getitem")
            .unwrap();
        let result = self.extract_int_call_result(call);
        Ok(PyValue::new(result.value(), val_type.clone(), None))
    }

    /// Set an item by key: dict[key] = value
    pub fn dict_setitem(
        &mut self,
        dict_val: BasicValueEnum<'ctx>,
        key: BasicValueEnum<'ctx>,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let setitem_fn = self.get_or_declare_c_builtin("dict_setitem");
        self.builder
            .build_call(
                setitem_fn,
                &[dict_val.into(), key.into(), value.into()],
                "dict_setitem",
            )
            .unwrap();
        Ok(())
    }

    /// Delete an item by key: del dict[key]
    pub fn dict_delitem(
        &mut self,
        dict_val: BasicValueEnum<'ctx>,
        key: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let delitem_fn = self.get_or_declare_c_builtin("dict_delitem");
        self.builder
            .build_call(delitem_fn, &[dict_val.into(), key.into()], "dict_delitem")
            .unwrap();
        Ok(())
    }

    /// Get the length of a dict
    pub fn dict_len(&mut self, dict_val: BasicValueEnum<'ctx>) -> Result<PyValue<'ctx>, String> {
        let len_fn = self.get_or_declare_c_builtin("dict_len");
        let call = self
            .builder
            .build_call(len_fn, &[dict_val.into()], "dict_len")
            .unwrap();
        Ok(self.extract_int_call_result(call))
    }

    // ========================================================================
    // dict() builtin function
    // ========================================================================

    /// Generate dict() builtin call - creates an empty dict
    /// dict() -> empty dict[int, int] (default key/value types)
    pub fn generate_dict_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if !args.is_empty() {
            return Err(
                "dict() takes no arguments (use dict literal {1: 2, 3: 4} instead)".to_string(),
            );
        }

        // Create empty dict with default int key/value types
        let dict_new_fn = self.get_or_declare_c_builtin("dict_new");
        let call_site = self
            .builder
            .build_call(dict_new_fn, &[], "dict_new")
            .unwrap();
        let dict_ptr = self.extract_ptr_call_result(call_site);

        Ok(PyValue::new(
            dict_ptr.value(),
            PyType::Dict(Box::new(PyType::Int), Box::new(PyType::Int)),
            None,
        ))
    }
}
