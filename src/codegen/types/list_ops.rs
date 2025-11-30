//! List operations for PyValue
//!
//! Binary and unary operations for Python list type.

use crate::ast::{BinaryOp, UnaryOp};
use inkwell::IntPredicate;

use super::value::{CgCtx, PyType, PyValue};
use super::{extract_int_result, extract_ptr_result, get_or_declare_builtin};

/// Binary operations for list type
pub fn binary_op<'ctx>(
    lhs: &PyValue<'ctx>,
    cg: &CgCtx<'ctx>,
    op: &BinaryOp,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let lhs_ptr = lhs.runtime_value().into_pointer_value();
    let lhs_elem_type = match &lhs.ty() {
        PyType::List(elem) => elem.as_ref().clone(),
        _ => return Err("Expected list type".to_string()),
    };

    match op {
        // List concatenation: [1, 2] + [3, 4] = [1, 2, 3, 4]
        BinaryOp::Add => match &rhs.ty() {
            PyType::List(rhs_elem) => {
                if rhs_elem.as_ref() != &lhs_elem_type {
                    return Err(format!(
                        "Cannot concatenate list[{:?}] with list[{:?}]",
                        lhs_elem_type,
                        rhs_elem.as_ref()
                    ));
                }
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let concat_fn = get_or_declare_builtin(&cg.module, cg.ctx, "list_concat");
                let call_site = cg
                    .builder
                    .build_call(concat_fn, &[lhs_ptr.into(), rhs_ptr.into()], "list_concat")
                    .unwrap();
                let result = extract_ptr_result(call_site, "list_concat");
                Ok(PyValue::new(
                    result.into(),
                    PyType::List(Box::new(lhs_elem_type)),
                    None,
                ))
            }
            _ => Err(format!("Cannot add list and {:?}", rhs.ty())),
        },

        // List repetition: [1, 2] * 3 = [1, 2, 1, 2, 1, 2]
        BinaryOp::Mul => match &rhs.ty() {
            PyType::Int => {
                let rhs_int = rhs.runtime_value().into_int_value();
                let repeat_fn = get_or_declare_builtin(&cg.module, cg.ctx, "list_repeat");
                let call_site = cg
                    .builder
                    .build_call(repeat_fn, &[lhs_ptr.into(), rhs_int.into()], "list_repeat")
                    .unwrap();
                let result = extract_ptr_result(call_site, "list_repeat");
                Ok(PyValue::new(
                    result.into(),
                    PyType::List(Box::new(lhs_elem_type)),
                    None,
                ))
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
                let repeat_fn = get_or_declare_builtin(&cg.module, cg.ctx, "list_repeat");
                let call_site = cg
                    .builder
                    .build_call(
                        repeat_fn,
                        &[lhs_ptr.into(), repeat_count.into()],
                        "list_repeat",
                    )
                    .unwrap();
                let result = extract_ptr_result(call_site, "list_repeat");
                Ok(PyValue::new(
                    result.into(),
                    PyType::List(Box::new(lhs_elem_type)),
                    None,
                ))
            }
            _ => Err(format!("Cannot multiply list by {:?}", rhs.ty())),
        },

        // List equality: [1, 2] == [1, 2]
        BinaryOp::Eq => match &rhs.ty() {
            PyType::List(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let eq_fn = get_or_declare_builtin(&cg.module, cg.ctx, "list_eq");
                let call_site = cg
                    .builder
                    .build_call(eq_fn, &[lhs_ptr.into(), rhs_ptr.into()], "list_eq")
                    .unwrap();
                let result = extract_int_result(call_site, "list_eq");
                // Convert i64 to i1 bool
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        result,
                        cg.ctx.i64_type().const_zero(),
                        "list_eq_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val))
            }
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero())),
        },

        // List inequality: [1, 2] != [1, 3]
        BinaryOp::Ne => match &rhs.ty() {
            PyType::List(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let eq_fn = get_or_declare_builtin(&cg.module, cg.ctx, "list_eq");
                let call_site = cg
                    .builder
                    .build_call(eq_fn, &[lhs_ptr.into(), rhs_ptr.into()], "list_eq")
                    .unwrap();
                let result = extract_int_result(call_site, "list_eq");
                // Convert i64 to i1 bool and negate
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        result,
                        cg.ctx.i64_type().const_zero(),
                        "list_ne_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val))
            }
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_int(1, false))),
        },

        // List comparisons: [1, 2] < [1, 3], etc. (lexicographic)
        BinaryOp::Lt => match &rhs.ty() {
            PyType::List(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let cmp_fn = get_or_declare_builtin(&cg.module, cg.ctx, "list_cmp");
                let call_site = cg
                    .builder
                    .build_call(cmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "list_cmp")
                    .unwrap();
                let result = extract_int_result(call_site, "list_cmp");
                // result < 0 means lhs < rhs
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::SLT,
                        result,
                        cg.ctx.i64_type().const_zero(),
                        "list_lt",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val))
            }
            _ => Err(format!("Cannot compare list with {:?}", rhs.ty())),
        },
        BinaryOp::Le => match &rhs.ty() {
            PyType::List(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let cmp_fn = get_or_declare_builtin(&cg.module, cg.ctx, "list_cmp");
                let call_site = cg
                    .builder
                    .build_call(cmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "list_cmp")
                    .unwrap();
                let result = extract_int_result(call_site, "list_cmp");
                // result <= 0 means lhs <= rhs
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::SLE,
                        result,
                        cg.ctx.i64_type().const_zero(),
                        "list_le",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val))
            }
            _ => Err(format!("Cannot compare list with {:?}", rhs.ty())),
        },
        BinaryOp::Gt => match &rhs.ty() {
            PyType::List(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let cmp_fn = get_or_declare_builtin(&cg.module, cg.ctx, "list_cmp");
                let call_site = cg
                    .builder
                    .build_call(cmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "list_cmp")
                    .unwrap();
                let result = extract_int_result(call_site, "list_cmp");
                // result > 0 means lhs > rhs
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::SGT,
                        result,
                        cg.ctx.i64_type().const_zero(),
                        "list_gt",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val))
            }
            _ => Err(format!("Cannot compare list with {:?}", rhs.ty())),
        },
        BinaryOp::Ge => match &rhs.ty() {
            PyType::List(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let cmp_fn = get_or_declare_builtin(&cg.module, cg.ctx, "list_cmp");
                let call_site = cg
                    .builder
                    .build_call(cmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "list_cmp")
                    .unwrap();
                let result = extract_int_result(call_site, "list_cmp");
                // result >= 0 means lhs >= rhs
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::SGE,
                        result,
                        cg.ctx.i64_type().const_zero(),
                        "list_ge",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val))
            }
            _ => Err(format!("Cannot compare list with {:?}", rhs.ty())),
        },

        // Membership: list in list is checking if the left list is an element of the right list
        // For homogeneous int lists, this is always False (can't have nested lists)
        BinaryOp::In => match &rhs.ty() {
            PyType::List(_) => {
                // list in list - always False for int lists (no nesting)
                Ok(PyValue::bool(cg.ctx.bool_type().const_zero()))
            }
            _ => Err(format!("Cannot use 'in' with list and {:?}", rhs.ty())),
        },
        BinaryOp::NotIn => match &rhs.ty() {
            PyType::List(_) => {
                // list not in list - always True for int lists (no nesting)
                Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones()))
            }
            _ => Err(format!("Cannot use 'not in' with list and {:?}", rhs.ty())),
        },

        // Logical and/or - same type returns same type, different types return bool
        BinaryOp::And => {
            // Get list length to determine truthiness
            let len_fn = get_or_declare_builtin(&cg.module, cg.ctx, "list_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[lhs_ptr.into()], "list_len")
                .unwrap();
            let len = extract_int_result(len_call, "list_len");
            let zero = cg.ctx.i64_type().const_zero();
            let lhs_bool = cg
                .builder
                .build_int_compare(IntPredicate::NE, len, zero, "to_bool")
                .unwrap();

            match &rhs.ty() {
                PyType::List(elem_ty) => {
                    // List and List -> List (Python semantics: return first falsy or last)
                    let rhs_ptr = rhs.runtime_value().into_pointer_value();
                    let result = cg
                        .builder
                        .build_select(lhs_bool, rhs_ptr, lhs_ptr, "and")
                        .unwrap();
                    Ok(PyValue::new(result, PyType::List(elem_ty.clone()), None))
                }
                _ => {
                    // Different types -> convert both to bool and return bool
                    let rhs_bool = cg.value_to_bool(rhs);
                    let result = cg.builder.build_and(lhs_bool, rhs_bool, "and").unwrap();
                    Ok(PyValue::bool(result))
                }
            }
        }
        BinaryOp::Or => {
            // Get list length to determine truthiness
            let len_fn = get_or_declare_builtin(&cg.module, cg.ctx, "list_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[lhs_ptr.into()], "list_len")
                .unwrap();
            let len = extract_int_result(len_call, "list_len");
            let zero = cg.ctx.i64_type().const_zero();
            let lhs_bool = cg
                .builder
                .build_int_compare(IntPredicate::NE, len, zero, "to_bool")
                .unwrap();

            match &rhs.ty() {
                PyType::List(elem_ty) => {
                    // List or List -> List (Python semantics: return first truthy or last)
                    let rhs_ptr = rhs.runtime_value().into_pointer_value();
                    let result = cg
                        .builder
                        .build_select(lhs_bool, lhs_ptr, rhs_ptr, "or")
                        .unwrap();
                    Ok(PyValue::new(result, PyType::List(elem_ty.clone()), None))
                }
                _ => {
                    // Different types -> convert both to bool and return bool
                    let rhs_bool = cg.value_to_bool(rhs);
                    let result = cg.builder.build_or(lhs_bool, rhs_bool, "or").unwrap();
                    Ok(PyValue::bool(result))
                }
            }
        }

        // Identity operators - list is list compares pointer identity
        BinaryOp::Is => match &rhs.ty() {
            PyType::List(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(inkwell::IntPredicate::EQ, lhs_ptr, rhs_ptr, "is")
                        .unwrap(),
                ))
            }
            // Different types are never identical
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero())),
        },
        BinaryOp::IsNot => match &rhs.ty() {
            PyType::List(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(inkwell::IntPredicate::NE, lhs_ptr, rhs_ptr, "isnot")
                        .unwrap(),
                ))
            }
            // Different types are never identical, so is not returns true
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones())),
        },

        _ => Err(format!("Operator {:?} not supported on list", op)),
    }
}

/// Unary operations for list type
pub fn unary_op<'ctx>(
    val: &PyValue<'ctx>,
    cg: &CgCtx<'ctx>,
    op: &UnaryOp,
) -> Result<PyValue<'ctx>, String> {
    match op {
        UnaryOp::Not => {
            // not list: true if list is empty, false otherwise
            let ptr = val.runtime_value().into_pointer_value();
            let len_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "list_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[ptr.into()], "list_len")
                .unwrap();
            let len = super::extract_int_result(len_call, "list_len");
            let zero = cg.ctx.i64_type().const_zero();
            // not list is true when len == 0
            Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(inkwell::IntPredicate::EQ, len, zero, "list_not")
                    .unwrap(),
            ))
        }
        _ => Err(format!("Unary operator {:?} not supported on list", op)),
    }
}
