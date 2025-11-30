//! Str operations for PyValue
//!
//! Binary and unary operations for Python str type.

use crate::ast::{BinaryOp, UnaryOp};
use inkwell::values::BasicValueEnum;

use super::value::{CgCtx, PyType, PyValue};

/// Binary operations for Str type
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
            PyType::Str => {
                let strcat_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "strcat_str");
                let call_site = cg
                    .builder
                    .build_call(
                        strcat_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "strcat",
                    )
                    .unwrap();
                Ok(PyValue::new_str(super::extract_ptr_result(
                    call_site,
                    "strcat_str",
                )))
            }
            _ => Err(format!("Cannot concatenate Str and {:?}", rhs.ty)),
        },

        // String formatting (% operator)
        BinaryOp::Mod => match &rhs.ty {
            PyType::Int => {
                let format_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_format_int");
                let call_site = cg
                    .builder
                    .build_call(
                        format_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "str_format",
                    )
                    .unwrap();
                Ok(PyValue::new_str(super::extract_ptr_result(
                    call_site,
                    "str_format_int",
                )))
            }
            PyType::Float => {
                let format_fn =
                    super::get_or_declare_builtin(cg.module, cg.ctx, "str_format_float");
                let call_site = cg
                    .builder
                    .build_call(
                        format_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "str_format",
                    )
                    .unwrap();
                Ok(PyValue::new_str(super::extract_ptr_result(
                    call_site,
                    "str_format_float",
                )))
            }
            PyType::Bool => {
                let format_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_format_bool");
                // Bool is i1, need to convert to i64
                let bool_val = rhs.runtime_value().into_int_value();
                let int_val = cg
                    .builder
                    .build_int_z_extend(bool_val, cg.ctx.i64_type(), "bool_to_int")
                    .unwrap();
                let call_site = cg
                    .builder
                    .build_call(format_fn, &[lhs_ptr.into(), int_val.into()], "str_format")
                    .unwrap();
                Ok(PyValue::new_str(super::extract_ptr_result(
                    call_site,
                    "str_format_bool",
                )))
            }
            PyType::Str => {
                let format_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_format_str");
                let call_site = cg
                    .builder
                    .build_call(
                        format_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "str_format",
                    )
                    .unwrap();
                Ok(PyValue::new_str(super::extract_ptr_result(
                    call_site,
                    "str_format_str",
                )))
            }
            PyType::Bytes => {
                let format_fn =
                    super::get_or_declare_builtin(cg.module, cg.ctx, "str_format_bytes");
                let call_site = cg
                    .builder
                    .build_call(
                        format_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "str_format",
                    )
                    .unwrap();
                Ok(PyValue::new_str(super::extract_ptr_result(
                    call_site,
                    "str_format_bytes",
                )))
            }
            PyType::None => {
                let format_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_format_none");
                let call_site = cg
                    .builder
                    .build_call(format_fn, &[lhs_ptr.into()], "str_format")
                    .unwrap();
                Ok(PyValue::new_str(super::extract_ptr_result(
                    call_site,
                    "str_format_none",
                )))
            }
            PyType::List(_) => {
                let format_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_format_list");
                let call_site = cg
                    .builder
                    .build_call(
                        format_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "str_format",
                    )
                    .unwrap();
                Ok(PyValue::new_str(super::extract_ptr_result(
                    call_site,
                    "str_format_list",
                )))
            }
            PyType::Dict(_, _) => {
                let format_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_format_dict");
                let call_site = cg
                    .builder
                    .build_call(
                        format_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "str_format",
                    )
                    .unwrap();
                Ok(PyValue::new_str(super::extract_ptr_result(
                    call_site,
                    "str_format_dict",
                )))
            }
            PyType::Set(_) => {
                let format_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_format_set");
                let call_site = cg
                    .builder
                    .build_call(
                        format_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "str_format",
                    )
                    .unwrap();
                Ok(PyValue::new_str(super::extract_ptr_result(
                    call_site,
                    "str_format_set",
                )))
            }
            _ => Err(format!("Cannot format Str with {:?}", rhs.ty)),
        },

        // Repetition
        BinaryOp::Mul => match &rhs.ty {
            PyType::Int => {
                let repeat_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "strrepeat_str");
                let call_site = cg
                    .builder
                    .build_call(
                        repeat_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "str_repeat",
                    )
                    .unwrap();
                Ok(PyValue::new_str(super::extract_ptr_result(
                    call_site,
                    "strrepeat_str",
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
                let repeat_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "strrepeat_str");
                let call_site = cg
                    .builder
                    .build_call(
                        repeat_fn,
                        &[lhs_ptr.into(), repeat_count.into()],
                        "str_repeat",
                    )
                    .unwrap();
                Ok(PyValue::new_str(super::extract_ptr_result(
                    call_site,
                    "strrepeat_str",
                )))
            }
            _ => Err(format!("Cannot multiply Str by {:?}", rhs.ty)),
        },

        // Comparison
        BinaryOp::Eq => match &rhs.ty {
            PyType::Str => {
                let strcmp_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "strcmp_str");
                let call_site = cg
                    .builder
                    .build_call(
                        strcmp_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "strcmp",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "strcmp_str");
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
            PyType::Str => {
                let strcmp_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "strcmp_str");
                let call_site = cg
                    .builder
                    .build_call(
                        strcmp_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "strcmp",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "strcmp_str");
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
            PyType::Str => {
                let cmp_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_lt");
                let call_site = cg
                    .builder
                    .build_call(
                        cmp_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "str_lt",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "str_lt");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Str with {:?}", rhs.ty)),
        },
        BinaryOp::Le => match &rhs.ty {
            PyType::Str => {
                let cmp_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_le");
                let call_site = cg
                    .builder
                    .build_call(
                        cmp_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "str_le",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "str_le");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Str with {:?}", rhs.ty)),
        },
        BinaryOp::Gt => match &rhs.ty {
            PyType::Str => {
                let cmp_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_gt");
                let call_site = cg
                    .builder
                    .build_call(
                        cmp_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "str_gt",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "str_gt");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Str with {:?}", rhs.ty)),
        },
        BinaryOp::Ge => match &rhs.ty {
            PyType::Str => {
                let cmp_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_ge");
                let call_site = cg
                    .builder
                    .build_call(
                        cmp_fn,
                        &[lhs_ptr.into(), rhs.runtime_value().into()],
                        "str_ge",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "str_ge");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Str with {:?}", rhs.ty)),
        },

        // Membership
        BinaryOp::In => match &rhs.ty {
            PyType::Str => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let contains_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_contains");
                let call_site = cg
                    .builder
                    .build_call(
                        contains_fn,
                        &[rhs_ptr.into(), lhs_ptr.into()],
                        "str_contains",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "str_contains");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot use 'in' with Str and {:?}", rhs.ty)),
        },
        BinaryOp::NotIn => match &rhs.ty {
            PyType::Str => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let contains_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_contains");
                let call_site = cg
                    .builder
                    .build_call(
                        contains_fn,
                        &[rhs_ptr.into(), lhs_ptr.into()],
                        "str_contains",
                    )
                    .unwrap();
                let result = super::extract_int_result(call_site, "str_contains");
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                let negated = cg.builder.build_not(bool_val, "not_in").unwrap();
                Ok(PyValue::bool(negated.into()))
            }
            _ => Err(format!("Cannot use 'not in' with Str and {:?}", rhs.ty)),
        },

        // Logical and/or - same type returns same type, different types return bool
        BinaryOp::And => {
            // Get str length to determine truthiness
            let len_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[lhs_ptr.into()], "str_len")
                .unwrap();
            let len = super::extract_int_result(len_call, "str_len").into_int_value();
            let zero = cg.ctx.i64_type().const_zero();
            let lhs_bool = cg
                .builder
                .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                .unwrap();

            match &rhs.ty {
                PyType::Str => {
                    // Str and Str -> Str (Python semantics: return first falsy or last)
                    let rhs_ptr = rhs.runtime_value().into_pointer_value();
                    let result = cg
                        .builder
                        .build_select(lhs_bool, rhs_ptr, lhs_ptr, "and")
                        .unwrap();
                    Ok(PyValue::new_str(result))
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
            // Get str length to determine truthiness
            let len_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[lhs_ptr.into()], "str_len")
                .unwrap();
            let len = super::extract_int_result(len_call, "str_len").into_int_value();
            let zero = cg.ctx.i64_type().const_zero();
            let lhs_bool = cg
                .builder
                .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                .unwrap();

            match &rhs.ty {
                PyType::Str => {
                    // Str or Str -> Str (Python semantics: return first truthy or last)
                    let rhs_ptr = rhs.runtime_value().into_pointer_value();
                    let result = cg
                        .builder
                        .build_select(lhs_bool, lhs_ptr, rhs_ptr, "or")
                        .unwrap();
                    Ok(PyValue::new_str(result))
                }
                _ => {
                    // Different types -> convert both to bool and return bool
                    let rhs_bool = cg.value_to_bool(rhs);
                    let result = cg.builder.build_or(lhs_bool, rhs_bool, "or").unwrap();
                    Ok(PyValue::bool(result.into()))
                }
            }
        }

        // Identity operators - str is str compares pointer identity
        BinaryOp::Is => match &rhs.ty {
            PyType::Str => {
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
            PyType::Str => {
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

        _ => Err(format!("Operator {:?} not supported for str type", op)),
    }
}

/// Unary operations for Str type
pub fn unary_op<'a, 'ctx>(
    val: &PyValue<'ctx>,
    cg: &CgCtx<'a, 'ctx>,
    op: &UnaryOp,
) -> Result<BasicValueEnum<'ctx>, String> {
    match op {
        UnaryOp::Not => {
            // not str: true if str is empty, false otherwise
            let ptr = val.runtime_value().into_pointer_value();
            let len_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "str_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[ptr.into()], "str_len")
                .unwrap();
            let len = super::extract_int_result(len_call, "str_len").into_int_value();
            let zero = cg.ctx.i64_type().const_zero();
            // not str is true when len == 0
            Ok(cg
                .builder
                .build_int_compare(inkwell::IntPredicate::EQ, len, zero, "str_not")
                .unwrap()
                .into())
        }
        _ => Err(format!("Unary operator {:?} not supported on str", op)),
    }
}
