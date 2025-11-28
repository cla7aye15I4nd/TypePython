//! Float type implementation

use super::TypeInfo;
use crate::ast::Type;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

/// Float type (f64)
#[derive(Clone, Debug, PartialEq)]
pub struct FloatType;

impl<'ctx> TypeInfo<'ctx> for FloatType {
    fn llvm_type(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx> {
        ctx.f64_type().into()
    }

    fn default_value(&self, ctx: &'ctx Context) -> BasicValueEnum<'ctx> {
        ctx.f64_type().const_zero().into()
    }

    fn print_function_name(&self) -> &'static str {
        "print_float"
    }

    fn can_coerce_to(&self, target: &Type) -> bool {
        matches!(target, Type::Float)
    }
}
