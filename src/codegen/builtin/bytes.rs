//! Bytes operations: slicing and indexing
//!
//! This module provides bytes-related operations:
//! - Subscript operations (bytes[i])
//! - Slice operations (bytes[start:stop:step])
//!
//! Method lookup is handled by the unified method registry in types/methods.rs
//!
//! All C runtime functions are in src/runtime/builtins/bytes.c
//! and discovered automatically by build.rs.

use crate::codegen::CodeGen;
use crate::types::PyValue;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGen<'ctx> {
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
            .cg
            .builder
            .build_call(
                getitem_fn,
                &[bytes_val.into(), index.into()],
                "bytes_getitem",
            )
            .unwrap();
        Ok(self.extract_int_call_result(call))
    }

    // ========================================================================
    // Slice operations (e.g., b"hello"[1:4], b"hello"[::2])
    // ========================================================================

    /// Get the length of bytes
    pub fn bytes_len(&mut self, bytes_val: BasicValueEnum<'ctx>) -> Result<PyValue<'ctx>, String> {
        let len_fn = self.get_or_declare_c_builtin("bytes_len");
        let call = self
            .cg
            .builder
            .build_call(len_fn, &[bytes_val.into()], "bytes_len")
            .unwrap();
        Ok(self.extract_int_call_result(call))
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
            .cg
            .builder
            .build_call(
                slice_fn,
                &[bytes_val.into(), start.into(), stop.into()],
                "bytes_slice",
            )
            .unwrap();
        Ok(self.extract_bytes_call_result(call))
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
            .cg
            .builder
            .build_call(
                slice_fn,
                &[bytes_val.into(), start.into(), stop.into(), step.into()],
                "bytes_slice_step",
            )
            .unwrap();
        Ok(self.extract_bytes_call_result(call))
    }
}
