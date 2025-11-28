//! Bool type implementation

use super::TypeInfo;
use crate::ast::{BinaryOp, Type, UnaryOp};
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

/// Boolean type (i1)
pub struct BoolType;

impl<'ctx> TypeInfo<'ctx> for BoolType {
    fn llvm_type(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx> {
        ctx.bool_type().into()
    }

    fn default_value(&self, ctx: &'ctx Context) -> BasicValueEnum<'ctx> {
        ctx.bool_type().const_zero().into()
    }

    fn supports_binary_op(&self, op: &BinaryOp, other: &Type) -> bool {
        match other {
            Type::Bool => true, // All ops supported between bools
            Type::Int => matches!(
                op,
                BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge
                    | BinaryOp::BitOr
                    | BinaryOp::BitXor
                    | BinaryOp::BitAnd
            ),
            Type::Float => matches!(
                op,
                BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
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
        matches!(op, UnaryOp::Not | UnaryOp::Neg | UnaryOp::Pos)
    }

    fn binary_op_result_type(&self, op: &BinaryOp, other: &Type) -> Option<Type> {
        match other {
            Type::Bool => match op {
                // Logical operations return Bool
                BinaryOp::And | BinaryOp::Or => Some(Type::Bool),
                // Comparison operations return Bool
                BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge
                | BinaryOp::Is
                | BinaryOp::IsNot => Some(Type::Bool),
                // Bitwise operations on bools return Bool
                BinaryOp::BitOr | BinaryOp::BitXor | BinaryOp::BitAnd => Some(Type::Bool),
                // Arithmetic operations promote to Int
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::FloorDiv => {
                    Some(Type::Int)
                }
                BinaryOp::Div => Some(Type::Float),
                _ => None,
            },
            Type::Int => match op {
                // Mixed bool/int arithmetic returns Int
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::FloorDiv => {
                    Some(Type::Int)
                }
                BinaryOp::Div => Some(Type::Float),
                // Comparisons return Bool
                BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge => Some(Type::Bool),
                // Bitwise with int returns Int
                BinaryOp::BitOr | BinaryOp::BitXor | BinaryOp::BitAnd => Some(Type::Int),
                _ => None,
            },
            Type::Float => match op {
                // Mixed bool/float arithmetic returns Float
                BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div
                | BinaryOp::FloorDiv => Some(Type::Float),
                // Comparisons return Bool
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
            UnaryOp::Not => Some(Type::Bool),
            UnaryOp::Neg | UnaryOp::Pos => Some(Type::Int), // -True = -1, +True = 1
            UnaryOp::BitNot => Some(Type::Int),             // ~True = -2
        }
    }

    fn print_function_name(&self) -> &'static str {
        "print_bool"
    }

    fn can_coerce_to(&self, target: &Type) -> bool {
        matches!(target, Type::Bool | Type::Int | Type::Float)
    }
}
