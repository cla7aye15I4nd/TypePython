//! Bytes operations for PyValue
//!
//! Binary and unary operations for Python bytes type.

use crate::ast::{BinaryOp, UnaryOp};

use super::value::{CgCtx, PyType, PyValue};

/// Binary operations for Bytes type
pub fn binary_op<'ctx>(
    lhs: &PyValue<'ctx>,
    cg: &CgCtx<'ctx>,
    op: &BinaryOp,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let lhs_ptr = lhs.runtime_value().into_pointer_value();

    match op {
        // Concatenation
        BinaryOp::Add => match &rhs.ty {
            PyType::Bytes => {
                let strcat_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "strcat_bytes");
                let call_site = cg
                    .builder
                    .build_call(
                        strcat_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "bytescat",
                    )
                    .unwrap();
                Ok(PyValue::bytes(super::extract_ptr_result(
                    call_site,
                    "strcat_bytes",
                )))
            }
            _ => Err(format!("Cannot concatenate Bytes and {:?}", rhs.ty)),
        },

        // Repetition
        BinaryOp::Mul => match &rhs.ty {
            PyType::Int => {
                let repeat_fn =
                    super::get_or_declare_builtin(&cg.module, cg.ctx, "strrepeat_bytes");
                let call_site = cg
                    .builder
                    .build_call(
                        repeat_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "bytes_repeat",
                    )
                    .unwrap();
                Ok(PyValue::bytes(super::extract_ptr_result(
                    call_site,
                    "strrepeat_bytes",
                )))
            }
            PyType::Bool => {
                // Coerce Bool to Int and repeat
                let repeat_count = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let repeat_fn =
                    super::get_or_declare_builtin(&cg.module, cg.ctx, "strrepeat_bytes");
                let call_site = cg
                    .builder
                    .build_call(
                        repeat_fn,
                        &[lhs_ptr.into(), repeat_count.into()],
                        "bytes_repeat",
                    )
                    .unwrap();
                Ok(PyValue::bytes(super::extract_ptr_result(
                    call_site,
                    "strrepeat_bytes",
                )))
            }
            _ => Err(format!("Cannot multiply Bytes by {:?}", rhs.ty)),
        },

        // Comparison
        BinaryOp::Eq => match &rhs.ty {
            PyType::Bytes => {
                let strcmp_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "strcmp_bytes");
                let call_site = cg
                    .builder
                    .build_call(
                        strcmp_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "bytescmp",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "strcmp_bytes");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            // Different types are never equal
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into())),
        },
        BinaryOp::Ne => match &rhs.ty {
            PyType::Bytes => {
                let strcmp_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "strcmp_bytes");
                let call_site = cg
                    .builder
                    .build_call(
                        strcmp_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "bytescmp",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "strcmp_bytes");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                let negated = cg.builder.build_not(bool_val, "ne").unwrap();
                Ok(PyValue::bool(negated.into()))
            }
            // Different types are never equal
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones().into())),
        },
        BinaryOp::Lt => match &rhs.ty {
            PyType::Bytes => {
                let cmp_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "bytes_lt");
                let call_site = cg
                    .builder
                    .build_call(
                        cmp_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "bytes_lt",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "bytes_lt");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
        },
        BinaryOp::Le => match &rhs.ty {
            PyType::Bytes => {
                let cmp_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "bytes_le");
                let call_site = cg
                    .builder
                    .build_call(
                        cmp_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "bytes_le",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "bytes_le");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
        },
        BinaryOp::Gt => match &rhs.ty {
            PyType::Bytes => {
                let cmp_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "bytes_gt");
                let call_site = cg
                    .builder
                    .build_call(
                        cmp_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "bytes_gt",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "bytes_gt");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
        },
        BinaryOp::Ge => match &rhs.ty {
            PyType::Bytes => {
                let cmp_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "bytes_ge");
                let call_site = cg
                    .builder
                    .build_call(
                        cmp_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "bytes_ge",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "bytes_ge");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
        },

        // Membership
        BinaryOp::In => match &rhs.ty {
            PyType::Bytes => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let contains_fn =
                    super::get_or_declare_builtin(&cg.module, cg.ctx, "bytes_contains");
                let call_site = cg
                    .builder
                    .build_call(
                        contains_fn,
                        &[rhs_ptr.into(), lhs_ptr.into()],
                        "bytes_contains",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "bytes_contains");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot use 'in' with Bytes and {:?}", rhs.ty)),
        },
        BinaryOp::NotIn => match &rhs.ty {
            PyType::Bytes => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let contains_fn =
                    super::get_or_declare_builtin(&cg.module, cg.ctx, "bytes_contains");
                let call_site = cg
                    .builder
                    .build_call(
                        contains_fn,
                        &[rhs_ptr.into(), lhs_ptr.into()],
                        "bytes_contains",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "bytes_contains");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                let negated = cg.builder.build_not(bool_val, "not_in").unwrap();
                Ok(PyValue::bool(negated.into()))
            }
            _ => Err(format!("Cannot use 'not in' with Bytes and {:?}", rhs.ty)),
        },

        // Logical and/or - same type returns same type, different types return bool
        BinaryOp::And => {
            // Get bytes length to determine truthiness
            let len_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "bytes_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[lhs_ptr.into()], "bytes_len")
                .unwrap();
            let len = super::extract_int_result(len_call, "bytes_len").into_int_value();
            let zero = cg.ctx.i64_type().const_zero();
            let lhs_bool = cg
                .builder
                .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                .unwrap();

            match &rhs.ty {
                PyType::Bytes => {
                    // Bytes and Bytes -> Bytes (Python semantics: return first falsy or last)
                    let rhs_ptr = rhs.runtime_value().into_pointer_value();
                    let result = cg
                        .builder
                        .build_select(lhs_bool, rhs_ptr, lhs_ptr, "and")
                        .unwrap();
                    Ok(PyValue::bytes(result))
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
            // Get bytes length to determine truthiness
            let len_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "bytes_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[lhs_ptr.into()], "bytes_len")
                .unwrap();
            let len = super::extract_int_result(len_call, "bytes_len").into_int_value();
            let zero = cg.ctx.i64_type().const_zero();
            let lhs_bool = cg
                .builder
                .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                .unwrap();

            match &rhs.ty {
                PyType::Bytes => {
                    // Bytes or Bytes -> Bytes (Python semantics: return first truthy or last)
                    let rhs_ptr = rhs.runtime_value().into_pointer_value();
                    let result = cg
                        .builder
                        .build_select(lhs_bool, lhs_ptr, rhs_ptr, "or")
                        .unwrap();
                    Ok(PyValue::bytes(result))
                }
                _ => {
                    // Different types -> convert both to bool and return bool
                    let rhs_bool = cg.value_to_bool(rhs);
                    let result = cg.builder.build_or(lhs_bool, rhs_bool, "or").unwrap();
                    Ok(PyValue::bool(result.into()))
                }
            }
        }

        // Identity operators - bytes is bytes compares pointer identity
        BinaryOp::Is => match &rhs.ty {
            PyType::Bytes => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(inkwell::IntPredicate::EQ, lhs_ptr, rhs_ptr, "is")
                        .unwrap()
                        .into(),
                ))
            }
            // Different types are never identical
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into())),
        },
        BinaryOp::IsNot => match &rhs.ty {
            PyType::Bytes => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(inkwell::IntPredicate::NE, lhs_ptr, rhs_ptr, "isnot")
                        .unwrap()
                        .into(),
                ))
            }
            // Different types are never identical, so is not returns true
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones().into())),
        },

        _ => Err(format!("Operator {:?} not supported for bytes type", op)),
    }
}

/// Unary operations for Bytes type
pub fn unary_op<'ctx>(
    val: &PyValue<'ctx>,
    cg: &CgCtx<'ctx>,
    op: &UnaryOp,
) -> Result<PyValue<'ctx>, String> {
    match op {
        UnaryOp::Not => {
            // not bytes: true if bytes is empty, false otherwise
            let ptr = val.runtime_value().into_pointer_value();
            let len_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "bytes_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[ptr.into()], "bytes_len")
                .unwrap();
            let len = super::extract_int_result(len_call, "bytes_len").into_int_value();
            let zero = cg.ctx.i64_type().const_zero();
            // not bytes is true when len == 0
            Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(inkwell::IntPredicate::EQ, len, zero, "bytes_not")
                    .unwrap()
                    .into(),
            ))
        }
        _ => Err(format!("Unary operator {:?} not supported on bytes", op)),
    }
}
