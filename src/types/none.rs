//! None type implementation

use super::TypeInfo;
use crate::ast::{BinaryOp, Type, UnaryOp};
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

/// None type (null pointer)
#[derive(Clone, Debug, PartialEq)]
pub struct NoneType;

impl<'ctx> TypeInfo<'ctx> for NoneType {
    fn llvm_type(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx> {
        // None is represented as a null pointer
        ctx.ptr_type(inkwell::AddressSpace::default()).into()
    }

    fn default_value(&self, ctx: &'ctx Context) -> BasicValueEnum<'ctx> {
        ctx.ptr_type(inkwell::AddressSpace::default())
            .const_null()
            .into()
    }

    fn supports_binary_op(&self, op: &BinaryOp, other: &Type) -> bool {
        // None only supports identity comparison
        matches!(
            op,
            BinaryOp::Is | BinaryOp::IsNot | BinaryOp::Eq | BinaryOp::Ne
        ) && matches!(other, Type::None)
    }

    fn supports_unary_op(&self, _op: &UnaryOp) -> bool {
        false
    }

    fn binary_op_result_type(&self, op: &BinaryOp, other: &Type) -> Option<Type> {
        match other {
            Type::None => match op {
                BinaryOp::Is | BinaryOp::IsNot | BinaryOp::Eq | BinaryOp::Ne => Some(Type::Bool),
                _ => None,
            },
            _ => None,
        }
    }

    fn unary_op_result_type(&self, _op: &UnaryOp) -> Option<Type> {
        None
    }

    fn print_function_name(&self) -> &'static str {
        "print_none"
    }

    fn can_coerce_to(&self, target: &Type) -> bool {
        matches!(target, Type::None)
    }
}
