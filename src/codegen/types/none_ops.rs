//! None operations for PyValue
//!
//! Binary and unary operations for Python None type.

use crate::ast::{BinaryOp, UnaryOp};
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

use super::value::{CgCtx, PyType, PyValue};

/// Binary operations for None type
pub fn binary_op<'a, 'ctx>(
    lhs: &PyValue<'ctx>,
    cg: &CgCtx<'a, 'ctx>,
    op: &BinaryOp,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let lhs_int = lhs.runtime_value().into_int_value();

    match op {
        BinaryOp::Eq => match &rhs.ty {
            PyType::None => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        "eq",
                    )
                    .unwrap()
                    .into(),
            )),
            // None is never equal to other types
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into())),
        },
        BinaryOp::Ne => match &rhs.ty {
            PyType::None => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::NE,
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        "ne",
                    )
                    .unwrap()
                    .into(),
            )),
            // None is always not equal to other types
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones().into())),
        },
        BinaryOp::Is => match &rhs.ty {
            PyType::None => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        "is_none",
                    )
                    .unwrap()
                    .into(),
            )),
            // Different types are never identical
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into())),
        },
        BinaryOp::IsNot => match &rhs.ty {
            PyType::None => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::NE,
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        "isnot_none",
                    )
                    .unwrap()
                    .into(),
            )),
            // Different types are never identical, so is not returns true
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones().into())),
        },
        // Membership: None in list/dict/set - always False (can't have None in int collections)
        BinaryOp::In => match &rhs.ty {
            PyType::List(_) | PyType::Dict(_, _) | PyType::Set(_) => {
                Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into()))
            }
            _ => Err(format!("Cannot use 'in' with None and {:?}", rhs.ty)),
        },
        BinaryOp::NotIn => match &rhs.ty {
            PyType::List(_) | PyType::Dict(_, _) | PyType::Set(_) => {
                Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones().into()))
            }
            _ => Err(format!("Cannot use 'not in' with None and {:?}", rhs.ty)),
        },

        // Logical and/or - None is always falsy
        BinaryOp::And => {
            // None is always falsy, so "None and X" returns None (first falsy)
            match &rhs.ty {
                PyType::None => {
                    // None and None -> None
                    Ok(PyValue::none(lhs_int.into()))
                }
                _ => {
                    // Different types -> convert both to bool and return bool
                    // None is falsy, so result is always False
                    Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into()))
                }
            }
        }
        BinaryOp::Or => {
            // None is always falsy, so "None or X" returns X (first truthy or last)
            match &rhs.ty {
                PyType::None => {
                    // None or None -> None (returns last)
                    Ok(PyValue::none(rhs.runtime_value()))
                }
                _ => {
                    // Different types -> convert both to bool and return bool
                    let rhs_bool = cg.value_to_bool(rhs);
                    Ok(PyValue::bool(rhs_bool.into()))
                }
            }
        }

        _ => Err(format!("Operator {:?} not supported on None", op)),
    }
}

/// Unary operations for None type
pub fn unary_op<'a, 'ctx>(
    _val: &PyValue<'ctx>,
    _cg: &CgCtx<'a, 'ctx>,
    op: &UnaryOp,
) -> Result<BasicValueEnum<'ctx>, String> {
    Err(format!("Unary operator {:?} not supported on None", op))
}
