//! None type implementation

use super::TypeInfo;
use crate::ast::Type;
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

    fn print_function_name(&self) -> &'static str {
        "print_none"
    }

    fn can_coerce_to(&self, target: &Type) -> bool {
        matches!(target, Type::None)
    }
}
