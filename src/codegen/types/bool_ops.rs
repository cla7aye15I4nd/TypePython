//! Bool operations for PyValue
//!
//! Binary and unary operations for Python bool type.

use crate::ast::{BinaryOp, UnaryOp};
use inkwell::IntPredicate;

use super::value::{CgCtx, PyType, PyValue};

/// Binary operations for Bool type
pub fn binary_op<'ctx>(
    lhs: &PyValue<'ctx>,
    cg: &CgCtx<'ctx>,
    op: &BinaryOp,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let lhs_bool = lhs.runtime_value().into_int_value();

    match op {
        // For arithmetic, coerce to int first
        BinaryOp::Add
        | BinaryOp::Sub
        | BinaryOp::Mul
        | BinaryOp::Div
        | BinaryOp::FloorDiv
        | BinaryOp::Mod
        | BinaryOp::Pow
        | BinaryOp::LShift
        | BinaryOp::RShift => {
            let lhs_int = cg
                .builder
                .build_int_z_extend(lhs_bool, cg.ctx.i64_type(), "btoi")
                .unwrap();

            // If rhs is Float, coerce to float
            if let PyType::Float = &rhs.ty() {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                return super::float_ops::binary_op(&PyValue::float(lhs_float), cg, op, rhs);
            }

            super::int_ops::binary_op(&PyValue::int(lhs_int), cg, op, rhs)
        }

        // Bitwise - coerce to int when RHS is int
        BinaryOp::BitAnd => match &rhs.ty() {
            PyType::Bool => {
                let result = cg
                    .builder
                    .build_and(lhs_bool, rhs.runtime_value().into_int_value(), "bitand")
                    .unwrap();
                Ok(PyValue::bool(result))
            }
            PyType::Int => {
                // Coerce bool to int and delegate
                let lhs_int = cg
                    .builder
                    .build_int_z_extend(lhs_bool, cg.ctx.i64_type(), "btoi")
                    .unwrap();
                super::int_ops::binary_op(&PyValue::int(lhs_int), cg, op, rhs)
            }
            _ => Err(format!("Cannot bitwise AND Bool and {:?}", rhs.ty())),
        },
        BinaryOp::BitOr => match &rhs.ty() {
            PyType::Bool => {
                let result = cg
                    .builder
                    .build_or(lhs_bool, rhs.runtime_value().into_int_value(), "bitor")
                    .unwrap();
                Ok(PyValue::bool(result))
            }
            PyType::Int => {
                let lhs_int = cg
                    .builder
                    .build_int_z_extend(lhs_bool, cg.ctx.i64_type(), "btoi")
                    .unwrap();
                super::int_ops::binary_op(&PyValue::int(lhs_int), cg, op, rhs)
            }
            _ => Err(format!("Cannot bitwise OR Bool and {:?}", rhs.ty())),
        },
        BinaryOp::BitXor => match &rhs.ty() {
            PyType::Bool => {
                let result = cg
                    .builder
                    .build_xor(lhs_bool, rhs.runtime_value().into_int_value(), "bitxor")
                    .unwrap();
                Ok(PyValue::bool(result))
            }
            PyType::Int => {
                let lhs_int = cg
                    .builder
                    .build_int_z_extend(lhs_bool, cg.ctx.i64_type(), "btoi")
                    .unwrap();
                super::int_ops::binary_op(&PyValue::int(lhs_int), cg, op, rhs)
            }
            _ => Err(format!("Cannot bitwise XOR Bool and {:?}", rhs.ty())),
        },

        // Comparison - coerce to int when RHS is int, return false for other types
        BinaryOp::Eq => match &rhs.ty() {
            PyType::Bool => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        lhs_bool,
                        rhs.runtime_value().into_int_value(),
                        "eq",
                    )
                    .unwrap(),
            )),
            PyType::Int | PyType::Float => {
                let lhs_int = cg
                    .builder
                    .build_int_z_extend(lhs_bool, cg.ctx.i64_type(), "btoi")
                    .unwrap();
                super::int_ops::binary_op(&PyValue::int(lhs_int), cg, op, rhs)
            }
            // Different types are never equal
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero())),
        },
        BinaryOp::Ne => match &rhs.ty() {
            PyType::Bool => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::NE,
                        lhs_bool,
                        rhs.runtime_value().into_int_value(),
                        "ne",
                    )
                    .unwrap(),
            )),
            PyType::Int | PyType::Float => {
                let lhs_int = cg
                    .builder
                    .build_int_z_extend(lhs_bool, cg.ctx.i64_type(), "btoi")
                    .unwrap();
                super::int_ops::binary_op(&PyValue::int(lhs_int), cg, op, rhs)
            }
            // Different types are never equal
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones())),
        },
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
            // Coerce to int for ordering comparisons
            let lhs_int = cg
                .builder
                .build_int_z_extend(lhs_bool, cg.ctx.i64_type(), "btoi")
                .unwrap();
            super::int_ops::binary_op(&PyValue::int(lhs_int), cg, op, rhs)
        }
        BinaryOp::Is => match &rhs.ty() {
            PyType::Bool => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        lhs_bool,
                        rhs.runtime_value().into_int_value(),
                        "is",
                    )
                    .unwrap(),
            )),
            // Different types are never identical
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero())),
        },
        BinaryOp::IsNot => match &rhs.ty() {
            PyType::Bool => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::NE,
                        lhs_bool,
                        rhs.runtime_value().into_int_value(),
                        "isnot",
                    )
                    .unwrap(),
            )),
            // Different types are never identical
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones())),
        },

        // Logical and/or - same type returns same type, different types return bool
        BinaryOp::And => {
            match &rhs.ty() {
                PyType::Bool => {
                    // Bool and Bool -> Bool (Python semantics)
                    let rhs_bool = rhs.runtime_value().into_int_value();
                    let result = cg.builder.build_and(lhs_bool, rhs_bool, "and").unwrap();
                    Ok(PyValue::bool(result))
                }
                _ => {
                    // Different types -> convert rhs to bool and return bool
                    let rhs_bool = cg.value_to_bool(rhs);
                    let result = cg.builder.build_and(lhs_bool, rhs_bool, "and").unwrap();
                    Ok(PyValue::bool(result))
                }
            }
        }
        BinaryOp::Or => {
            match &rhs.ty() {
                PyType::Bool => {
                    // Bool or Bool -> Bool (Python semantics)
                    let rhs_bool = rhs.runtime_value().into_int_value();
                    let result = cg.builder.build_or(lhs_bool, rhs_bool, "or").unwrap();
                    Ok(PyValue::bool(result))
                }
                _ => {
                    // Different types -> convert rhs to bool and return bool
                    let rhs_bool = cg.value_to_bool(rhs);
                    let result = cg.builder.build_or(lhs_bool, rhs_bool, "or").unwrap();
                    Ok(PyValue::bool(result))
                }
            }
        }

        BinaryOp::In | BinaryOp::NotIn => {
            // Coerce Bool to Int and delegate to int_ops
            let lhs_int = cg
                .builder
                .build_int_z_extend(lhs_bool, cg.ctx.i64_type(), "btoi")
                .unwrap();
            super::int_ops::binary_op(&PyValue::int(lhs_int), cg, op, rhs)
        }
    }
}

/// Unary operations for Bool type
pub fn unary_op<'ctx>(
    val: &PyValue<'ctx>,
    cg: &CgCtx<'ctx>,
    op: &UnaryOp,
) -> Result<PyValue<'ctx>, String> {
    let bool_val = val.runtime_value().into_int_value();
    match op {
        UnaryOp::Not => Ok(PyValue::bool(
            cg.builder.build_not(bool_val, "not").unwrap(),
        )),
        // For bitwise NOT, unary minus, and unary plus, coerce to int first
        UnaryOp::BitNot | UnaryOp::Neg | UnaryOp::Pos => {
            let int_val = cg
                .builder
                .build_int_z_extend(bool_val, cg.ctx.i64_type(), "btoi")
                .unwrap();
            super::int_ops::unary_op(&PyValue::int(int_val), cg, op)
        }
    }
}
