//! Str operations: method calls, slicing, and indexing
//!
//! This module provides all str-related operations:
//! - Method lookup (upper, lower, ljust, etc.)
//! - Subscript operations (str[i])
//! - Slice operations (str[start:stop:step])
//!
//! All C runtime functions are in src/runtime/builtins/str.c
//! and discovered automatically by build.rs.

use crate::codegen::CodeGen;
use crate::types::{FunctionInfo, PyType, PyValue};
use inkwell::values::BasicValueEnum;

/// Maps method name to (builtin_symbol, return_type)
fn get_str_method_info(name: &str) -> Option<(&'static str, PyType)> {
    match name {
        // Case conversion - return str
        "upper" => Some(("str_upper", PyType::Str)),
        "lower" => Some(("str_lower", PyType::Str)),
        "capitalize" => Some(("str_capitalize", PyType::Str)),
        "title" => Some(("str_title", PyType::Str)),
        "swapcase" => Some(("str_swapcase", PyType::Str)),

        // Padding/alignment - return str
        "ljust" => Some(("str_ljust", PyType::Str)),
        "rjust" => Some(("str_rjust", PyType::Str)),
        "center" => Some(("str_center", PyType::Str)),
        "zfill" => Some(("str_zfill", PyType::Str)),

        // Stripping - return str
        "strip" => Some(("str_strip", PyType::Str)),
        "lstrip" => Some(("str_lstrip", PyType::Str)),
        "rstrip" => Some(("str_rstrip", PyType::Str)),

        // Search - return int
        "find" => Some(("str_find", PyType::Int)),
        "count" => Some(("str_count", PyType::Int)),

        // Predicates - return bool
        "startswith" => Some(("str_startswith", PyType::Bool)),
        "endswith" => Some(("str_endswith", PyType::Bool)),
        "isalnum" => Some(("str_isalnum", PyType::Bool)),
        "isalpha" => Some(("str_isalpha", PyType::Bool)),
        "isdigit" => Some(("str_isdigit", PyType::Bool)),
        "isspace" => Some(("str_isspace", PyType::Bool)),
        "islower" => Some(("str_islower", PyType::Bool)),
        "isupper" => Some(("str_isupper", PyType::Bool)),

        // Transform - return str
        "replace" => Some(("str_replace", PyType::Str)),

        _ => None,
    }
}

impl<'ctx> CodeGen<'ctx> {
    // ========================================================================
    // Method calls (e.g., b"hello".upper())
    // ========================================================================

    /// Get a str method as a function with the receiver pre-bound
    pub fn get_str_method(
        &mut self,
        receiver: &PyValue<'ctx>,
        method_name: &str,
    ) -> Result<PyValue<'ctx>, String> {
        let (symbol, return_type) = get_str_method_info(method_name)
            .ok_or_else(|| format!("str has no method '{}'", method_name))?;

        let function = self.get_or_declare_c_builtin(symbol);

        Ok(PyValue::function(FunctionInfo {
            mangled_name: symbol.to_string(),
            function,
            param_types: vec![], // Not needed for builtins
            return_type,
            bound_args: vec![receiver.value()],
        }))
    }

    // ========================================================================
    // Subscript operations (e.g., b"hello"[0])
    // ========================================================================

    /// Get a byte at index: str[i] -> int
    pub fn str_getitem(
        &mut self,
        str_val: BasicValueEnum<'ctx>,
        index: BasicValueEnum<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let getitem_fn = self.get_or_declare_c_builtin("str_getitem");
        let call = self
            .builder
            .build_call(getitem_fn, &[str_val.into(), index.into()], "str_getitem")
            .unwrap();
        Ok(self.extract_int_call_result(call))
    }

    // ========================================================================
    // Slice operations (e.g., b"hello"[1:4], b"hello"[::2])
    // ========================================================================

    /// Get the length of str
    pub fn str_len(&mut self, str_val: BasicValueEnum<'ctx>) -> Result<PyValue<'ctx>, String> {
        let len_fn = self.get_or_declare_c_builtin("str_len");
        let call = self
            .builder
            .build_call(len_fn, &[str_val.into()], "str_len")
            .unwrap();
        Ok(self.extract_int_call_result(call))
    }

    /// Slice str without step: str[start:stop] -> str
    pub fn str_slice(
        &mut self,
        str_val: BasicValueEnum<'ctx>,
        start: BasicValueEnum<'ctx>,
        stop: BasicValueEnum<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let slice_fn = self.get_or_declare_c_builtin("str_slice");
        let call = self
            .builder
            .build_call(
                slice_fn,
                &[str_val.into(), start.into(), stop.into()],
                "str_slice",
            )
            .unwrap();
        Ok(self.extract_ptr_call_result(call))
    }

    /// Slice str with step: str[start:stop:step] -> str
    pub fn str_slice_step(
        &mut self,
        str_val: BasicValueEnum<'ctx>,
        start: BasicValueEnum<'ctx>,
        stop: BasicValueEnum<'ctx>,
        step: BasicValueEnum<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let slice_fn = self.get_or_declare_c_builtin("str_slice_step");
        let call = self
            .builder
            .build_call(
                slice_fn,
                &[str_val.into(), start.into(), stop.into(), step.into()],
                "str_slice_step",
            )
            .unwrap();
        Ok(self.extract_ptr_call_result(call))
    }
}
