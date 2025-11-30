//! Str operations
//!
//! This module handles str-specific operations like subscript access.
//! Method lookup is handled by the unified method registry in types/methods.rs
//!
//! All C runtime functions are in src/runtime/builtins/str.c
//! and discovered automatically by build.rs.

use crate::codegen::types::{PyType, PyValue};
use crate::codegen::CodeGen;
use inkwell::values::{AnyValue, BasicValueEnum};

impl<'ctx> CodeGen<'ctx> {
    /// Get a character at index: str[i] -> str (single character)
    pub fn str_getitem(
        &mut self,
        str_val: BasicValueEnum<'ctx>,
        index: BasicValueEnum<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        // Call str_char_at which returns a new single-character string
        let getitem_fn = self.get_or_declare_c_builtin("str_char_at");
        let call = self
            .cg
            .builder
            .build_call(getitem_fn, &[str_val.into(), index.into()], "str_char_at")
            .unwrap();
        let result_ptr = call.as_any_value_enum().into_pointer_value();
        Ok(PyValue::new(result_ptr.into(), PyType::Str, None))
    }
}
