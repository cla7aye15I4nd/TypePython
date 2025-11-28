//! Bytes type implementation (C-style null-terminated strings)

use super::TypeInfo;
use crate::ast::Type;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

/// Bytes type (pointer to null-terminated byte sequence)
#[derive(Clone, Debug, PartialEq)]
pub struct BytesType;

impl<'ctx> TypeInfo<'ctx> for BytesType {
    fn llvm_type(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx> {
        ctx.ptr_type(inkwell::AddressSpace::default()).into()
    }

    fn default_value(&self, ctx: &'ctx Context) -> BasicValueEnum<'ctx> {
        ctx.ptr_type(inkwell::AddressSpace::default())
            .const_null()
            .into()
    }

    fn print_function_name(&self) -> &'static str {
        "print_bytes"
    }

    fn can_coerce_to(&self, target: &Type) -> bool {
        matches!(target, Type::Bytes)
    }
}

impl BytesType {
    /// Get the builtin function name for bytes concatenation
    pub fn concat_function_name() -> &'static str {
        "strcat_bytes"
    }

    /// Get the builtin function name for bytes comparison
    pub fn compare_function_name() -> &'static str {
        "strcmp_bytes"
    }

    /// Get the builtin function name for bytes repetition
    pub fn repeat_function_name() -> &'static str {
        "strrepeat_bytes"
    }
}
