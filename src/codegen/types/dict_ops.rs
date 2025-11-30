//! Dict operations for PyValue
//!
//! Binary and unary operations for Python dict type.

use crate::ast::{BinaryOp, UnaryOp};
use inkwell::IntPredicate;

use super::value::{CgCtx, PyType, PyValue};
use super::{extract_int_result, get_or_declare_builtin};

/// Binary operations for dict type
pub fn binary_op<'ctx>(
    lhs: &PyValue<'ctx>,
    cg: &CgCtx<'ctx>,
    op: &BinaryOp,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let lhs_ptr = lhs.runtime_value().into_pointer_value();

    match op {
        // Dict equality: {1: 2} == {1: 2}
        BinaryOp::Eq => match &rhs.ty() {
            PyType::Dict(_, _) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let eq_fn = get_or_declare_builtin(&cg.module, cg.ctx, "dict_eq");
                let call_site = cg
                    .builder
                    .build_call(eq_fn, &[lhs_ptr.into(), rhs_ptr.into()], "dict_eq")
                    .unwrap();
                let result = extract_int_result(call_site, "dict_eq");
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
        BinaryOp::Ne => match &rhs.ty() {
            PyType::Dict(_, _) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let eq_fn = get_or_declare_builtin(&cg.module, cg.ctx, "dict_eq");
                let call_site = cg
                    .builder
                    .build_call(eq_fn, &[lhs_ptr.into(), rhs_ptr.into()], "dict_eq")
                    .unwrap();
                let result = extract_int_result(call_site, "dict_eq");
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
        BinaryOp::BitOr => match &rhs.ty() {
            PyType::Dict(rhs_key_type, _) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                // Use str_dict_merge for string-keyed dicts
                let merge_fn_name = match (&lhs.ty(), rhs_key_type.as_ref()) {
                    (PyType::Dict(lhs_key_type, _), _)
                        if matches!(lhs_key_type.as_ref(), PyType::Str) =>
                    {
                        "str_dict_merge"
                    }
                    _ => "dict_merge",
                };
                let merge_fn = get_or_declare_builtin(&cg.module, cg.ctx, merge_fn_name);
                let call_site = cg
                    .builder
                    .build_call(merge_fn, &[lhs_ptr.into(), rhs_ptr.into()], "dict_merge")
                    .unwrap();
                let result = super::extract_ptr_result(call_site, "dict_merge");
                Ok(PyValue::new(result, lhs.ty().clone(), None))
            }
            _ => Err(format!("Cannot use | between dict and {:?}", rhs.ty())),
        },

        // Membership: dict in list is checking if the dict is an element of the list
        // For lists of int, this is always False
        BinaryOp::In => match &rhs.ty() {
            PyType::List(_) => {
                // dict in list - always False (can't have dicts in int lists)
                Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into()))
            }
            _ => Err(format!("Cannot use 'in' with dict and {:?}", rhs.ty())),
        },
        BinaryOp::NotIn => match &rhs.ty() {
            PyType::List(_) => {
                // dict not in list - always True (can't have dicts in int lists)
                Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones().into()))
            }
            _ => Err(format!("Cannot use 'not in' with dict and {:?}", rhs.ty())),
        },

        // Logical and/or - same type returns same type, different types return bool
        BinaryOp::And => {
            // Get dict length to determine truthiness
            let len_fn = get_or_declare_builtin(&cg.module, cg.ctx, "dict_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[lhs_ptr.into()], "dict_len")
                .unwrap();
            let len = extract_int_result(len_call, "dict_len").into_int_value();
            let zero = cg.ctx.i64_type().const_zero();
            let lhs_bool = cg
                .builder
                .build_int_compare(IntPredicate::NE, len, zero, "to_bool")
                .unwrap();

            match &rhs.ty() {
                PyType::Dict(key_ty, val_ty) => {
                    // Dict and Dict -> Dict (Python semantics: return first falsy or last)
                    let rhs_ptr = rhs.runtime_value().into_pointer_value();
                    let result = cg
                        .builder
                        .build_select(lhs_bool, rhs_ptr, lhs_ptr, "and")
                        .unwrap();
                    Ok(PyValue::new(
                        result,
                        PyType::Dict(key_ty.clone(), val_ty.clone()),
                        None,
                    ))
                }
                _ => {
                    // Different types -> convert both to bool and return bool
                    let rhs_bool = cg.value_to_bool(rhs);
                    let result = cg.builder.build_and(lhs_bool, rhs_bool, "and").unwrap();
                    Ok(PyValue::bool(result.into()))
                }
            }
        }
        BinaryOp::Or => {
            // Get dict length to determine truthiness
            let len_fn = get_or_declare_builtin(&cg.module, cg.ctx, "dict_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[lhs_ptr.into()], "dict_len")
                .unwrap();
            let len = extract_int_result(len_call, "dict_len").into_int_value();
            let zero = cg.ctx.i64_type().const_zero();
            let lhs_bool = cg
                .builder
                .build_int_compare(IntPredicate::NE, len, zero, "to_bool")
                .unwrap();

            match &rhs.ty() {
                PyType::Dict(key_ty, val_ty) => {
                    // Dict or Dict -> Dict (Python semantics: return first truthy or last)
                    let rhs_ptr = rhs.runtime_value().into_pointer_value();
                    let result = cg
                        .builder
                        .build_select(lhs_bool, lhs_ptr, rhs_ptr, "or")
                        .unwrap();
                    Ok(PyValue::new(
                        result,
                        PyType::Dict(key_ty.clone(), val_ty.clone()),
                        None,
                    ))
                }
                _ => {
                    // Different types -> convert both to bool and return bool
                    let rhs_bool = cg.value_to_bool(rhs);
                    let result = cg.builder.build_or(lhs_bool, rhs_bool, "or").unwrap();
                    Ok(PyValue::bool(result.into()))
                }
            }
        }

        // Identity operators - dict is dict compares pointer identity
        BinaryOp::Is => match &rhs.ty() {
            PyType::Dict(_, _) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::EQ, lhs_ptr, rhs_ptr, "is")
                        .unwrap()
                        .into(),
                ))
            }
            // Different types are never identical
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into())),
        },
        BinaryOp::IsNot => match &rhs.ty() {
            PyType::Dict(_, _) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::NE, lhs_ptr, rhs_ptr, "isnot")
                        .unwrap()
                        .into(),
                ))
            }
            // Different types are never identical, so is not returns true
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones().into())),
        },

        _ => Err(format!("Operator {:?} not supported on dict", op)),
    }
}

/// Unary operations for dict type
pub fn unary_op<'ctx>(
    val: &PyValue<'ctx>,
    cg: &CgCtx<'ctx>,
    op: &UnaryOp,
) -> Result<PyValue<'ctx>, String> {
    match op {
        UnaryOp::Not => {
            // not dict: true if dict is empty, false otherwise
            let ptr = val.runtime_value().into_pointer_value();
            let len_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "dict_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[ptr.into()], "dict_len")
                .unwrap();
            let len = super::extract_int_result(len_call, "dict_len").into_int_value();
            let zero = cg.ctx.i64_type().const_zero();
            // not dict is true when len == 0
            Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(inkwell::IntPredicate::EQ, len, zero, "dict_not")
                    .unwrap()
                    .into(),
            ))
        }
        _ => Err(format!("Unary operator {:?} not supported on dict", op)),
    }
}
