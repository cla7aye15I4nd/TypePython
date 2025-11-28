//! Bytes type implementation (C-style null-terminated strings)

use super::TypeInfo;
use crate::ast::{BinaryOp, Type, UnaryOp};
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

/// Bytes type (pointer to null-terminated byte sequence)
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

    fn supports_binary_op(&self, op: &BinaryOp, other: &Type) -> bool {
        match other {
            Type::Bytes => matches!(
                op,
                BinaryOp::Add | // concatenation
                BinaryOp::Eq |
                BinaryOp::Ne |
                BinaryOp::Lt |
                BinaryOp::Le |
                BinaryOp::Gt |
                BinaryOp::Ge |
                BinaryOp::Is |
                BinaryOp::IsNot
            ),
            Type::Int => matches!(op, BinaryOp::Mul), // repeat: b"x" * 3
            _ => false,
        }
    }

    fn supports_unary_op(&self, _op: &UnaryOp) -> bool {
        false // No unary operations on bytes
    }

    fn binary_op_result_type(&self, op: &BinaryOp, other: &Type) -> Option<Type> {
        match other {
            Type::Bytes => match op {
                BinaryOp::Add => Some(Type::Bytes), // concatenation
                BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge
                | BinaryOp::Is
                | BinaryOp::IsNot => Some(Type::Bool),
                _ => None,
            },
            Type::Int => match op {
                BinaryOp::Mul => Some(Type::Bytes), // repeat
                _ => None,
            },
            _ => None,
        }
    }

    fn unary_op_result_type(&self, _op: &UnaryOp) -> Option<Type> {
        None
    }

    fn print_function_name(&self) -> &'static str {
        "print_str"
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
