//! Bool type implementation

use super::TypeInfo;
use crate::ast::Type;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

/// Boolean type (i1)
#[derive(Clone, Debug, PartialEq)]
pub struct BoolType;

impl<'ctx> TypeInfo<'ctx> for BoolType {
    fn llvm_type(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx> {
        ctx.bool_type().into()
    }

    fn default_value(&self, ctx: &'ctx Context) -> BasicValueEnum<'ctx> {
        ctx.bool_type().const_zero().into()
    }

    fn print_function_name(&self) -> &'static str {
        "print_bool"
    }

    fn can_coerce_to(&self, target: &Type) -> bool {
        matches!(target, Type::Bool | Type::Int | Type::Float)
    }
}
