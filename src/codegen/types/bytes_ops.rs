//! Bytes operations for PyValue
//!
//! Binary and unary operations for Python bytes type.

use crate::ast::{BinaryOp, UnaryOp};
use inkwell::values::BasicValueEnum;

use super::value::{CgCtx, PyType, PyValue};

/// Binary operations for Bytes type
pub fn binary_op<'a, 'ctx>(
    lhs: &PyValue<'ctx>,
    cg: &CgCtx<'a, 'ctx>,
    op: &BinaryOp,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let lhs_ptr = lhs.runtime_value().into_pointer_value();

    match op {
        // Concatenation
        BinaryOp::Add => match &rhs.ty {
            PyType::Bytes => {
                let strcat_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "strcat_bytes");
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
                let repeat_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "strrepeat_bytes");
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
                let repeat_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "strrepeat_bytes");
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
                let strcmp_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "strcmp_bytes");
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
                let strcmp_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "strcmp_bytes");
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
                let cmp_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "bytes_lt");
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
                let cmp_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "bytes_le");
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
                let cmp_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "bytes_gt");
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
                let cmp_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "bytes_ge");
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
                    super::get_or_declare_builtin(cg.module, cg.ctx, "bytes_contains");
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
            // bytes in list[bytes], dict[bytes, *], set[bytes]
            PyType::List(elem_type) => {
                if matches!(elem_type.as_ref(), PyType::Bytes) {
                    // bytes in list[bytes] - call list_contains_bytes
                    let contains_fn =
                        super::get_or_declare_builtin(cg.module, cg.ctx, "list_contains_str");
                    let call_site = cg
                        .builder
                        .build_call(
                            contains_fn,
                            &[rhs.runtime_value().into(), lhs_ptr.into()],
                            "list_contains_bytes",
                        )
                        .unwrap();
                    let result = super::extract_int_result(call_site, "list_contains_str");
                    let zero = cg.ctx.i64_type().const_zero();
                    let bool_val = cg
                        .builder
                        .build_int_compare(
                            inkwell::IntPredicate::NE,
                            result.into_int_value(),
                            zero,
                            "tobool",
                        )
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                } else {
                    // bytes in list[non-bytes] - always False
                    Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into()))
                }
            }
            PyType::Dict(key_type, _) => {
                if matches!(key_type.as_ref(), PyType::Bytes) {
                    // bytes in dict[bytes, *] - call str_dict_contains (uses same fn)
                    let contains_fn =
                        super::get_or_declare_builtin(cg.module, cg.ctx, "str_dict_contains");
                    let call_site = cg
                        .builder
                        .build_call(
                            contains_fn,
                            &[rhs.runtime_value().into(), lhs_ptr.into()],
                            "dict_contains_bytes",
                        )
                        .unwrap();
                    let result = super::extract_int_result(call_site, "str_dict_contains");
                    let zero = cg.ctx.i64_type().const_zero();
                    let bool_val = cg
                        .builder
                        .build_int_compare(
                            inkwell::IntPredicate::NE,
                            result.into_int_value(),
                            zero,
                            "tobool",
                        )
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                } else {
                    // bytes in dict[non-bytes, *] - always False
                    Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into()))
                }
            }
            PyType::Set(elem_type) => {
                if matches!(elem_type.as_ref(), PyType::Bytes) {
                    // bytes in set[bytes] - call str_set_contains (uses same fn)
                    let contains_fn =
                        super::get_or_declare_builtin(cg.module, cg.ctx, "str_set_contains");
                    let call_site = cg
                        .builder
                        .build_call(
                            contains_fn,
                            &[rhs.runtime_value().into(), lhs_ptr.into()],
                            "set_contains_bytes",
                        )
                        .unwrap();
                    let result = super::extract_int_result(call_site, "str_set_contains");
                    let zero = cg.ctx.i64_type().const_zero();
                    let bool_val = cg
                        .builder
                        .build_int_compare(
                            inkwell::IntPredicate::NE,
                            result.into_int_value(),
                            zero,
                            "tobool",
                        )
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                } else {
                    // bytes in set[non-bytes] - always False
                    Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into()))
                }
            }
            _ => Err(format!("Cannot use 'in' with Bytes and {:?}", rhs.ty)),
        },
        BinaryOp::NotIn => match &rhs.ty {
            PyType::Bytes => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let contains_fn =
                    super::get_or_declare_builtin(cg.module, cg.ctx, "bytes_contains");
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
            // bytes not in list[bytes], dict[bytes, *], set[bytes]
            PyType::List(elem_type) => {
                if matches!(elem_type.as_ref(), PyType::Bytes) {
                    let contains_fn =
                        super::get_or_declare_builtin(cg.module, cg.ctx, "list_contains_str");
                    let call_site = cg
                        .builder
                        .build_call(
                            contains_fn,
                            &[rhs.runtime_value().into(), lhs_ptr.into()],
                            "list_contains_bytes",
                        )
                        .unwrap();
                    let result = super::extract_int_result(call_site, "list_contains_str");
                    let zero = cg.ctx.i64_type().const_zero();
                    let bool_val = cg
                        .builder
                        .build_int_compare(
                            inkwell::IntPredicate::NE,
                            result.into_int_value(),
                            zero,
                            "tobool",
                        )
                        .unwrap();
                    let negated = cg.builder.build_not(bool_val, "not_in").unwrap();
                    Ok(PyValue::bool(negated.into()))
                } else {
                    // bytes not in list[non-bytes] - always True
                    Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones().into()))
                }
            }
            PyType::Dict(key_type, _) => {
                if matches!(key_type.as_ref(), PyType::Bytes) {
                    let contains_fn =
                        super::get_or_declare_builtin(cg.module, cg.ctx, "str_dict_contains");
                    let call_site = cg
                        .builder
                        .build_call(
                            contains_fn,
                            &[rhs.runtime_value().into(), lhs_ptr.into()],
                            "dict_contains_bytes",
                        )
                        .unwrap();
                    let result = super::extract_int_result(call_site, "str_dict_contains");
                    let zero = cg.ctx.i64_type().const_zero();
                    let bool_val = cg
                        .builder
                        .build_int_compare(
                            inkwell::IntPredicate::NE,
                            result.into_int_value(),
                            zero,
                            "tobool",
                        )
                        .unwrap();
                    let negated = cg.builder.build_not(bool_val, "not_in").unwrap();
                    Ok(PyValue::bool(negated.into()))
                } else {
                    // bytes not in dict[non-bytes, *] - always True
                    Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones().into()))
                }
            }
            PyType::Set(elem_type) => {
                if matches!(elem_type.as_ref(), PyType::Bytes) {
                    let contains_fn =
                        super::get_or_declare_builtin(cg.module, cg.ctx, "str_set_contains");
                    let call_site = cg
                        .builder
                        .build_call(
                            contains_fn,
                            &[rhs.runtime_value().into(), lhs_ptr.into()],
                            "set_contains_bytes",
                        )
                        .unwrap();
                    let result = super::extract_int_result(call_site, "str_set_contains");
                    let zero = cg.ctx.i64_type().const_zero();
                    let bool_val = cg
                        .builder
                        .build_int_compare(
                            inkwell::IntPredicate::NE,
                            result.into_int_value(),
                            zero,
                            "tobool",
                        )
                        .unwrap();
                    let negated = cg.builder.build_not(bool_val, "not_in").unwrap();
                    Ok(PyValue::bool(negated.into()))
                } else {
                    // bytes not in set[non-bytes] - always True
                    Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones().into()))
                }
            }
            _ => Err(format!("Cannot use 'not in' with Bytes and {:?}", rhs.ty)),
        },

        // Modulo (bytes formatting) - only supported for list, dict (returns original bytes)
        BinaryOp::Mod => match &rhs.ty {
            PyType::List(_) | PyType::Dict(_, _) => {
                // No-op: return the original bytes (no actual format substitution)
                Ok(lhs.clone())
            }
            _ => Err(format!(
                "Bytes formatting with % is not supported for {:?}",
                rhs.ty
            )),
        },

        _ => Err(format!("Operator {:?} not supported for bytes type", op)),
    }
}

/// Unary operations for Bytes type
pub fn unary_op<'a, 'ctx>(
    _val: &PyValue<'ctx>,
    _cg: &CgCtx<'a, 'ctx>,
    op: &UnaryOp,
) -> Result<BasicValueEnum<'ctx>, String> {
    Err(format!("Unary operator {:?} not supported on bytes", op))
}
