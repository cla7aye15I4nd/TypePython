//! Type system for TypePython
//!
//! This module provides a trait-based type system where each type
//! implements its own logic for:
//! - LLVM type representation
//! - Binary and unary operations
//! - Type checking and coercion
//! - Code generation helpers

mod bool;
mod bytes;
pub mod codegen;
mod float;
mod int;
mod none;

pub use self::bool::BoolType;
pub use bytes::BytesType;
pub use codegen::{CgCtx, PyType, PyValue};
pub use float::FloatType;
pub use int::IntType;
pub use none::NoneType;

use crate::ast::Type;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

/// Core trait for type-specific code generation and type checking.
///
/// Each TypePython type (Int, Float, Bool, Bytes, etc.) implements this trait
/// to provide its own logic for operations, rather than using if/else chains.
pub trait TypeInfo<'ctx> {
    /// Get the LLVM type representation for this type
    fn llvm_type(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx>;

    /// Get the default/zero value for this type
    fn default_value(&self, ctx: &'ctx Context) -> BasicValueEnum<'ctx>;

    /// Get the builtin function name for printing this type
    fn print_function_name(&self) -> &'static str;

    /// Check if this type can be coerced to another type
    fn can_coerce_to(&self, target: &Type) -> bool;
}
