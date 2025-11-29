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
            _ => Err(format!("Cannot compare None with {:?}", rhs.ty)),
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
            _ => Err(format!("Cannot compare None with {:?}", rhs.ty)),
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
            _ => Err(format!("Cannot use 'is' between None and {:?}", rhs.ty)),
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
            _ => Err(format!("Cannot use 'is not' between None and {:?}", rhs.ty)),
        },
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
