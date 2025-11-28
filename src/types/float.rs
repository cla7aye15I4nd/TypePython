//! Float type implementation

use super::TypeInfo;
use crate::ast::{BinaryOp, Type, UnaryOp};
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

    fn supports_binary_op(&self, op: &BinaryOp, other: &Type) -> bool {
        match other {
            Type::Float | Type::Int | Type::Bool => matches!(
                op,
                BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::FloorDiv
                    | BinaryOp::Mod
                    | BinaryOp::Pow
                    | BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge
                    | BinaryOp::Is
                    | BinaryOp::IsNot
            ),
            _ => false,
        }
    }

    fn supports_unary_op(&self, op: &UnaryOp) -> bool {
        matches!(op, UnaryOp::Neg | UnaryOp::Pos)
    }

    fn binary_op_result_type(&self, op: &BinaryOp, other: &Type) -> Option<Type> {
        match other {
            Type::Float | Type::Int | Type::Bool => match op {
                // Arithmetic operations return Float
                BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div
                | BinaryOp::FloorDiv
                | BinaryOp::Mod
                | BinaryOp::Pow => Some(Type::Float),
                // Comparison operations return Bool
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
            _ => None,
        }
    }

    fn unary_op_result_type(&self, op: &UnaryOp) -> Option<Type> {
        match op {
            UnaryOp::Neg | UnaryOp::Pos => Some(Type::Float),
            UnaryOp::Not => Some(Type::Bool),
            UnaryOp::BitNot => None, // Bitwise not doesn't work on floats
        }
    }

    fn print_function_name(&self) -> &'static str {
        "print_float"
    }

    fn can_coerce_to(&self, target: &Type) -> bool {
        matches!(target, Type::Float)
    }
}
