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

        _ => Err(format!("Operator {:?} not supported for str type", op)),
    }
}

/// Unary operations for Str type
pub fn unary_op<'a, 'ctx>(
    _val: &PyValue<'ctx>,
    _cg: &CgCtx<'a, 'ctx>,
    op: &UnaryOp,
) -> Result<BasicValueEnum<'ctx>, String> {
    Err(format!("Unary operator {:?} not supported on str", op))
}
