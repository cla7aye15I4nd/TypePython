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
pub use codegen::{CodeGenOps, PyType, PyValue};
pub use float::FloatType;
pub use int::IntType;
pub use none::NoneType;

use crate::ast::{BinaryOp, Type, UnaryOp};
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

    /// Check if a binary operation is supported between this type and another
    fn supports_binary_op(&self, op: &BinaryOp, other: &Type) -> bool;

    /// Check if a unary operation is supported for this type
    fn supports_unary_op(&self, op: &UnaryOp) -> bool;

    /// Get the result type of a binary operation (for type checking)
    fn binary_op_result_type(&self, op: &BinaryOp, other: &Type) -> Option<Type>;

    /// Get the result type of a unary operation (for type checking)
    fn unary_op_result_type(&self, op: &UnaryOp) -> Option<Type>;

    /// Get the builtin function name for printing this type
    fn print_function_name(&self) -> &'static str;

    /// Check if this type can be coerced to another type
    fn can_coerce_to(&self, target: &Type) -> bool;
}

/// Get a TypeInfo implementation for an AST Type
pub fn get_type_info(ty: &Type) -> Box<dyn for<'ctx> TypeInfo<'ctx>> {
    match ty {
        Type::Int => Box::new(IntType),
        Type::Float => Box::new(FloatType),
        Type::Bool => Box::new(BoolType),
        Type::Bytes => Box::new(BytesType),
        Type::None => Box::new(NoneType),
        Type::Str => todo!("Str type not yet implemented"),
        Type::List(_) => todo!("List type not yet implemented"),
        Type::Dict(_, _) => todo!("Dict type not yet implemented"),
        Type::Set(_) => todo!("Set type not yet implemented"),
        Type::Tuple(_) => todo!("Tuple type not yet implemented"),
        Type::Custom(_) => todo!("Custom types not yet implemented"),
    }
}

/// Determine the common type for a binary operation between two types.
/// Returns the type that both operands should be coerced to.
pub fn common_type(left: &Type, right: &Type) -> Option<Type> {
    match (left, right) {
        // Same types - no coercion needed
        (Type::Int, Type::Int) => Some(Type::Int),
        (Type::Float, Type::Float) => Some(Type::Float),
        (Type::Bool, Type::Bool) => Some(Type::Bool),
        (Type::Bytes, Type::Bytes) => Some(Type::Bytes),

        // Int/Float coercion - promote to Float
        (Type::Int, Type::Float) | (Type::Float, Type::Int) => Some(Type::Float),

        // Bool can be treated as Int in arithmetic
        (Type::Bool, Type::Int) | (Type::Int, Type::Bool) => Some(Type::Int),
        (Type::Bool, Type::Float) | (Type::Float, Type::Bool) => Some(Type::Float),

        _ => None,
    }
}

/// Check if two types are compatible for a given binary operation
pub fn types_compatible_for_op(left: &Type, right: &Type, op: &BinaryOp) -> bool {
    match op {
        // Arithmetic operations - numeric types only
        BinaryOp::Add => {
            matches!(
                (left, right),
                (Type::Int, Type::Int)
                    | (Type::Float, Type::Float)
                    | (Type::Int, Type::Float)
                    | (Type::Float, Type::Int)
                    | (Type::Bytes, Type::Bytes) // bytes concatenation
            )
        }
        BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::FloorDiv | BinaryOp::Mod => {
            matches!(
                (left, right),
                (Type::Int, Type::Int)
                    | (Type::Float, Type::Float)
                    | (Type::Int, Type::Float)
                    | (Type::Float, Type::Int)
            )
        }
        BinaryOp::Pow => {
            matches!(
                (left, right),
                (Type::Int, Type::Int)
                    | (Type::Float, Type::Float)
                    | (Type::Int, Type::Float)
                    | (Type::Float, Type::Int)
            )
        }

        // Comparison operations - same type or compatible numeric
        BinaryOp::Eq | BinaryOp::Ne => {
            left == right
                || matches!(
                    (left, right),
                    (Type::Int, Type::Float)
                        | (Type::Float, Type::Int)
                        | (Type::Int, Type::Bool)
                        | (Type::Bool, Type::Int)
                )
        }
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
            matches!(
                (left, right),
                (Type::Int, Type::Int)
                    | (Type::Float, Type::Float)
                    | (Type::Int, Type::Float)
                    | (Type::Float, Type::Int)
                    | (Type::Bytes, Type::Bytes)
            )
        }

        // Logical operations - boolean only
        BinaryOp::And | BinaryOp::Or => {
            matches!((left, right), (Type::Bool, Type::Bool))
        }

        // Bitwise operations - integer only
        BinaryOp::BitOr | BinaryOp::BitXor | BinaryOp::BitAnd => {
            matches!(
                (left, right),
                (Type::Int, Type::Int) | (Type::Bool, Type::Bool)
            )
        }
        BinaryOp::LShift | BinaryOp::RShift => {
            matches!((left, right), (Type::Int, Type::Int))
        }

        // Identity operations
        BinaryOp::Is | BinaryOp::IsNot => true,

        // Membership operations - need container support
        BinaryOp::In | BinaryOp::NotIn => false,
    }
}
