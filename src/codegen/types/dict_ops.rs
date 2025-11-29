//! Dict operations for PyValue
//!
//! Binary and unary operations for Python dict type.

use crate::ast::{BinaryOp, UnaryOp};
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

use super::value::{CgCtx, PyType, PyValue};
use super::{extract_int_result, get_or_declare_builtin};

/// Binary operations for dict type
pub fn binary_op<'a, 'ctx>(
    lhs: &PyValue<'ctx>,
    cg: &CgCtx<'a, 'ctx>,
    op: &BinaryOp,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let lhs_ptr = lhs.runtime_value().into_pointer_value();

    match op {
        // Dict equality: {1: 2} == {1: 2}
        BinaryOp::Eq => match &rhs.ty {
            PyType::Dict(_, _) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let eq_fn = get_or_declare_builtin(cg.module, cg.ctx, "dict_eq");
                let call_site = cg
                    .builder
                    .build_call(eq_fn, &[lhs_ptr.into(), rhs_ptr.into()], "dict_eq")
                    .unwrap();
                let result = extract_int_result(call_site, "dict_eq")?;
                // Convert i64 to i1 bool
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        result.into_int_value(),
                        cg.ctx.i64_type().const_zero(),
                        "dict_eq_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into())),
        },

        // Dict inequality: {1: 2} != {1: 3}
        BinaryOp::Ne => match &rhs.ty {
            PyType::Dict(_, _) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let eq_fn = get_or_declare_builtin(cg.module, cg.ctx, "dict_eq");
                let call_site = cg
                    .builder
                    .build_call(eq_fn, &[lhs_ptr.into(), rhs_ptr.into()], "dict_eq")
                    .unwrap();
                let result = extract_int_result(call_site, "dict_eq")?;
                // Convert i64 to i1 bool and negate
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        result.into_int_value(),
                        cg.ctx.i64_type().const_zero(),
                        "dict_ne_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_int(1, false).into())),
        },

        // Dict merge with | operator (Python 3.9+): {1: 2} | {3: 4}
        BinaryOp::BitOr => match &rhs.ty {
            PyType::Dict(_, _) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let merge_fn = get_or_declare_builtin(cg.module, cg.ctx, "dict_merge");
                let call_site = cg
                    .builder
                    .build_call(merge_fn, &[lhs_ptr.into(), rhs_ptr.into()], "dict_merge")
                    .unwrap();
                let result = super::extract_ptr_result(call_site, "dict_merge")?;
                Ok(PyValue::new(result, lhs.ty.clone(), None))
            }
            _ => Err(format!("Cannot use | between dict and {:?}", rhs.ty)),
        },

        _ => Err(format!("Operator {:?} not supported on dict", op)),
    }
}

/// Unary operations for dict type
pub fn unary_op<'a, 'ctx>(
    _val: &PyValue<'ctx>,
    _cg: &CgCtx<'a, 'ctx>,
    op: &UnaryOp,
) -> Result<BasicValueEnum<'ctx>, String> {
    Err(format!("Unary operator {:?} not supported on dict", op))
}
