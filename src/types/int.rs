//! Integer type implementation

use super::TypeInfo;
use crate::ast::Type;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

/// Integer type (i64)
#[derive(Clone, Debug, PartialEq)]
pub struct IntType;

impl<'ctx> TypeInfo<'ctx> for IntType {
    fn llvm_type(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx> {
        ctx.i64_type().into()
    }

    fn default_value(&self, ctx: &'ctx Context) -> BasicValueEnum<'ctx> {
        ctx.i64_type().const_zero().into()
    }

    fn print_function_name(&self) -> &'static str {
        "print_int"
    }

    fn can_coerce_to(&self, target: &Type) -> bool {
        matches!(target, Type::Int | Type::Float)
    }
}
