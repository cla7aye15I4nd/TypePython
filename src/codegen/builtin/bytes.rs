//! Bytes operations: method calls, slicing, and indexing
//!
//! This module provides all bytes-related operations:
//! - Method lookup (upper, lower, ljust, etc.)
//! - Subscript operations (bytes[i])
//! - Slice operations (bytes[start:stop:step])
//!
//! All C runtime functions are in src/runtime/builtins/bytes.c
//! and discovered automatically by build.rs.

use crate::codegen::CodeGen;
use crate::types::{FunctionInfo, PyType, PyValue};
use inkwell::values::BasicValueEnum;

/// Maps method name to (builtin_symbol, return_type)
fn get_bytes_method_info(name: &str) -> Option<(&'static str, PyType)> {
    match name {
        // Case conversion - return bytes
        "upper" => Some(("bytes_upper", PyType::Bytes)),
        "lower" => Some(("bytes_lower", PyType::Bytes)),

        // Padding/alignment - return bytes
        "ljust" => Some(("bytes_ljust", PyType::Bytes)),
        "rjust" => Some(("bytes_rjust", PyType::Bytes)),
        "center" => Some(("bytes_center", PyType::Bytes)),
        "zfill" => Some(("bytes_zfill", PyType::Bytes)),

        // Stripping - return bytes
        "strip" => Some(("bytes_strip", PyType::Bytes)),
        "lstrip" => Some(("bytes_lstrip", PyType::Bytes)),
        "rstrip" => Some(("bytes_rstrip", PyType::Bytes)),

        // Search - return int
        "find" => Some(("bytes_find", PyType::Int)),
        "count" => Some(("bytes_count", PyType::Int)),

        // Predicates - return bool (as int)
        "startswith" => Some(("bytes_startswith", PyType::Int)),
        "endswith" => Some(("bytes_endswith", PyType::Int)),
        "isalnum" => Some(("bytes_isalnum", PyType::Int)),
        "isalpha" => Some(("bytes_isalpha", PyType::Int)),
        "isdigit" => Some(("bytes_isdigit", PyType::Int)),
        "isspace" => Some(("bytes_isspace", PyType::Int)),
        "islower" => Some(("bytes_islower", PyType::Int)),
        "isupper" => Some(("bytes_isupper", PyType::Int)),

        // Transform - return bytes
        "replace" => Some(("bytes_replace", PyType::Bytes)),

        _ => None,
    }
}

impl<'ctx> CodeGen<'ctx> {
    // ========================================================================
    // Method calls (e.g., b"hello".upper())
    // ========================================================================

    /// Get a bytes method as a function with the receiver pre-bound
    pub fn get_bytes_method(
        &mut self,
        receiver: &PyValue<'ctx>,
        method_name: &str,
    ) -> Result<PyValue<'ctx>, String> {
        let (symbol, return_type) = get_bytes_method_info(method_name)
            .ok_or_else(|| format!("bytes has no method '{}'", method_name))?;

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

    /// Get a byte at index: bytes[i] -> int
    pub fn bytes_getitem(
        &mut self,
        bytes_val: BasicValueEnum<'ctx>,
        index: BasicValueEnum<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let getitem_fn = self.get_or_declare_c_builtin("bytes_getitem");
        let call = self
            .builder
            .build_call(
                getitem_fn,
                &[bytes_val.into(), index.into()],
                "bytes_getitem",
            )
            .unwrap();
        self.extract_int_call_result(call)
    }

    // ========================================================================
    // Slice operations (e.g., b"hello"[1:4], b"hello"[::2])
    // ========================================================================

    /// Get the length of bytes
    pub fn bytes_len(&mut self, bytes_val: BasicValueEnum<'ctx>) -> Result<PyValue<'ctx>, String> {
        let len_fn = self.get_or_declare_c_builtin("bytes_len");
        let call = self
            .builder
            .build_call(len_fn, &[bytes_val.into()], "bytes_len")
            .unwrap();
        self.extract_int_call_result(call)
    }

    /// Slice bytes without step: bytes[start:stop] -> bytes
    pub fn bytes_slice(
        &mut self,
        bytes_val: BasicValueEnum<'ctx>,
        start: BasicValueEnum<'ctx>,
        stop: BasicValueEnum<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let slice_fn = self.get_or_declare_c_builtin("bytes_slice");
        let call = self
            .builder
            .build_call(
                slice_fn,
                &[bytes_val.into(), start.into(), stop.into()],
                "bytes_slice",
            )
            .unwrap();
        self.extract_bytes_call_result(call)
    }

    /// Slice bytes with step: bytes[start:stop:step] -> bytes
    pub fn bytes_slice_step(
        &mut self,
        bytes_val: BasicValueEnum<'ctx>,
        start: BasicValueEnum<'ctx>,
        stop: BasicValueEnum<'ctx>,
        step: BasicValueEnum<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let slice_fn = self.get_or_declare_c_builtin("bytes_slice_step");
        let call = self
            .builder
            .build_call(
                slice_fn,
                &[bytes_val.into(), start.into(), stop.into(), step.into()],
                "bytes_slice_step",
            )
            .unwrap();
        self.extract_bytes_call_result(call)
    }
}
