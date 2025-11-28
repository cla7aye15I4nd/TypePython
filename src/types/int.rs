//! Integer type implementation

use super::TypeInfo;
use crate::ast::{BinaryOp, Type, UnaryOp};
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

/// Integer type (i64)
pub struct IntType;

impl<'ctx> TypeInfo<'ctx> for IntType {
    fn llvm_type(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx> {
        ctx.i64_type().into()
    }

    fn default_value(&self, ctx: &'ctx Context) -> BasicValueEnum<'ctx> {
        ctx.i64_type().const_zero().into()
    }

    fn supports_binary_op(&self, op: &BinaryOp, other: &Type) -> bool {
        match other {
            Type::Int | Type::Bool => true,
            Type::Float => matches!(
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
            ),
            _ => false,
        }
    }

    fn supports_unary_op(&self, op: &UnaryOp) -> bool {
        matches!(op, UnaryOp::Neg | UnaryOp::Pos | UnaryOp::BitNot)
    }

    fn binary_op_result_type(&self, op: &BinaryOp, other: &Type) -> Option<Type> {
        match other {
            Type::Int | Type::Bool => match op {
                // Arithmetic operations return Int
                BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::FloorDiv
                | BinaryOp::Mod
                | BinaryOp::Pow => Some(Type::Int),
                // Division always returns Float in Python semantics
                BinaryOp::Div => Some(Type::Float),
                // Comparison operations return Bool
                BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge
                | BinaryOp::Is
                | BinaryOp::IsNot => Some(Type::Bool),
                // Bitwise operations return Int
                BinaryOp::BitOr
                | BinaryOp::BitXor
                | BinaryOp::BitAnd
                | BinaryOp::LShift
                | BinaryOp::RShift => Some(Type::Int),
                // Logical operations on ints return Int (Python semantics)
                BinaryOp::And | BinaryOp::Or => Some(Type::Int),
                _ => None,
            },
            Type::Float => match op {
                // Operations with float promote to float
                BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div
                | BinaryOp::FloorDiv
                | BinaryOp::Mod
                | BinaryOp::Pow => Some(Type::Float),
                // Comparisons still return Bool
                BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge => Some(Type::Bool),
                _ => None,
            },
            _ => None,
        }
    }

    fn unary_op_result_type(&self, op: &UnaryOp) -> Option<Type> {
        match op {
            UnaryOp::Neg | UnaryOp::Pos | UnaryOp::BitNot => Some(Type::Int),
            UnaryOp::Not => Some(Type::Bool),
        }
    }

    fn print_function_name(&self) -> &'static str {
        "tpy_print_int"
    }

    fn can_coerce_to(&self, target: &Type) -> bool {
        matches!(target, Type::Int | Type::Float)
    }
}
