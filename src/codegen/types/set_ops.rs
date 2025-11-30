//! Set operations for PyValue
//!
//! Binary and unary operations for Python set type.

use crate::ast::{BinaryOp, UnaryOp};
use inkwell::IntPredicate;

use super::value::{CgCtx, PyType, PyValue};
use super::{extract_int_result, extract_ptr_result, get_or_declare_builtin};

/// Binary operations for set type
pub fn binary_op<'ctx>(
    lhs: &PyValue<'ctx>,
    cg: &CgCtx<'ctx>,
    op: &BinaryOp,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let lhs_ptr = lhs.runtime_value().into_pointer_value();
    let lhs_elem_type = match &lhs.ty() {
        PyType::Set(elem) => elem.as_ref().clone(),
        _ => return Err("Expected set type".to_string()),
    };

    match op {
        // Set difference: {1, 2, 3} - {2} = {1, 3}
        BinaryOp::Sub => match &rhs.ty() {
            PyType::Set(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let diff_fn = get_or_declare_builtin(&cg.module, cg.ctx, "set_difference");
                let call_site = cg
                    .builder
                    .build_call(diff_fn, &[lhs_ptr.into(), rhs_ptr.into()], "set_difference")
                    .unwrap();
                let result = extract_ptr_result(call_site, "set_difference");
                Ok(PyValue::new(
                    result.into(),
                    PyType::Set(Box::new(lhs_elem_type)),
                    None,
                ))
            }
            _ => Err(format!("Cannot subtract {:?} from set", rhs.ty())),
        },

        // Set union: {1, 2} | {2, 3} = {1, 2, 3}
        BinaryOp::BitOr => match &rhs.ty() {
            PyType::Set(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let union_fn = get_or_declare_builtin(&cg.module, cg.ctx, "set_union");
                let call_site = cg
                    .builder
                    .build_call(union_fn, &[lhs_ptr.into(), rhs_ptr.into()], "set_union")
                    .unwrap();
                let result = extract_ptr_result(call_site, "set_union");
                Ok(PyValue::new(
                    result.into(),
                    PyType::Set(Box::new(lhs_elem_type)),
                    None,
                ))
            }
            _ => Err(format!("Cannot use | between set and {:?}", rhs.ty())),
        },

        // Set intersection: {1, 2, 3} & {2, 3, 4} = {2, 3}
        BinaryOp::BitAnd => match &rhs.ty() {
            PyType::Set(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let intersect_fn = get_or_declare_builtin(&cg.module, cg.ctx, "set_intersection");
                let call_site = cg
                    .builder
                    .build_call(
                        intersect_fn,
                        &[lhs_ptr.into(), rhs_ptr.into()],
                        "set_intersection",
                    )
                    .unwrap();
                let result = extract_ptr_result(call_site, "set_intersection");
                Ok(PyValue::new(
                    result.into(),
                    PyType::Set(Box::new(lhs_elem_type)),
                    None,
                ))
            }
            _ => Err(format!("Cannot use & between set and {:?}", rhs.ty())),
        },

        // Set symmetric difference: {1, 2, 3} ^ {2, 3, 4} = {1, 4}
        BinaryOp::BitXor => match &rhs.ty() {
            PyType::Set(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let sym_diff_fn =
                    get_or_declare_builtin(&cg.module, cg.ctx, "set_symmetric_difference");
                let call_site = cg
                    .builder
                    .build_call(
                        sym_diff_fn,
                        &[lhs_ptr.into(), rhs_ptr.into()],
                        "set_symmetric_difference",
                    )
                    .unwrap();
                let result = extract_ptr_result(call_site, "set_symmetric_difference");
                Ok(PyValue::new(
                    result.into(),
                    PyType::Set(Box::new(lhs_elem_type)),
                    None,
                ))
            }
            _ => Err(format!("Cannot use ^ between set and {:?}", rhs.ty())),
        },

        // Set equality: {1, 2} == {1, 2}
        BinaryOp::Eq => match &rhs.ty() {
            PyType::Set(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let eq_fn = get_or_declare_builtin(&cg.module, cg.ctx, "set_eq");
                let call_site = cg
                    .builder
                    .build_call(eq_fn, &[lhs_ptr.into(), rhs_ptr.into()], "set_eq")
                    .unwrap();
                let result = extract_int_result(call_site, "set_eq");
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        result,
                        cg.ctx.i64_type().const_zero(),
                        "set_eq_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val))
            }
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero())),
        },

        // Set inequality
        BinaryOp::Ne => match &rhs.ty() {
            PyType::Set(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let eq_fn = get_or_declare_builtin(&cg.module, cg.ctx, "set_eq");
                let call_site = cg
                    .builder
                    .build_call(eq_fn, &[lhs_ptr.into(), rhs_ptr.into()], "set_eq")
                    .unwrap();
                let result = extract_int_result(call_site, "set_eq");
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        result,
                        cg.ctx.i64_type().const_zero(),
                        "set_ne_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val))
            }
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_int(1, false))),
        },

        // Proper subset: {1} < {1, 2}
        BinaryOp::Lt => match &rhs.ty() {
            PyType::Set(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let subset_fn = get_or_declare_builtin(&cg.module, cg.ctx, "set_is_proper_subset");
                let call_site = cg
                    .builder
                    .build_call(
                        subset_fn,
                        &[lhs_ptr.into(), rhs_ptr.into()],
                        "set_proper_subset",
                    )
                    .unwrap();
                let result = extract_int_result(call_site, "set_is_proper_subset");
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        result,
                        cg.ctx.i64_type().const_zero(),
                        "set_lt_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val))
            }
            _ => Err(format!("Cannot compare set with {:?}", rhs.ty())),
        },

        // Subset: {1} <= {1, 2}
        BinaryOp::Le => match &rhs.ty() {
            PyType::Set(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let subset_fn = get_or_declare_builtin(&cg.module, cg.ctx, "set_issubset");
                let call_site = cg
                    .builder
                    .build_call(subset_fn, &[lhs_ptr.into(), rhs_ptr.into()], "set_issubset")
                    .unwrap();
                let result = extract_int_result(call_site, "set_issubset");
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        result,
                        cg.ctx.i64_type().const_zero(),
                        "set_le_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val))
            }
            _ => Err(format!("Cannot compare set with {:?}", rhs.ty())),
        },

        // Proper superset: {1, 2} > {1}
        BinaryOp::Gt => match &rhs.ty() {
            PyType::Set(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let superset_fn =
                    get_or_declare_builtin(&cg.module, cg.ctx, "set_is_proper_superset");
                let call_site = cg
                    .builder
                    .build_call(
                        superset_fn,
                        &[lhs_ptr.into(), rhs_ptr.into()],
                        "set_proper_superset",
                    )
                    .unwrap();
                let result = extract_int_result(call_site, "set_is_proper_superset");
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        result,
                        cg.ctx.i64_type().const_zero(),
                        "set_gt_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val))
            }
            _ => Err(format!("Cannot compare set with {:?}", rhs.ty())),
        },

        // Superset: {1, 2} >= {1}
        BinaryOp::Ge => match &rhs.ty() {
            PyType::Set(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let superset_fn = get_or_declare_builtin(&cg.module, cg.ctx, "set_issuperset");
                let call_site = cg
                    .builder
                    .build_call(
                        superset_fn,
                        &[lhs_ptr.into(), rhs_ptr.into()],
                        "set_issuperset",
                    )
                    .unwrap();
                let result = extract_int_result(call_site, "set_issuperset");
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        result,
                        cg.ctx.i64_type().const_zero(),
                        "set_ge_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val))
            }
            _ => Err(format!("Cannot compare set with {:?}", rhs.ty())),
        },

        // Membership: set in list/set - checking if the set is an element
        // For int sets/lists, this is always False
        BinaryOp::In => match &rhs.ty() {
            PyType::List(_) | PyType::Set(_) => {
                // set in list/set - always False (sets can't be elements of int collections)
                Ok(PyValue::bool(cg.ctx.bool_type().const_zero()))
            }
            _ => Err(format!("Cannot use 'in' with set and {:?}", rhs.ty())),
        },
        BinaryOp::NotIn => match &rhs.ty() {
            PyType::List(_) | PyType::Set(_) => {
                // set not in list/set - always True
                Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones()))
            }
            _ => Err(format!("Cannot use 'not in' with set and {:?}", rhs.ty())),
        },

        // Logical and/or - same type returns same type, different types return bool
        BinaryOp::And => {
            // Get set length to determine truthiness
            let len_fn = get_or_declare_builtin(&cg.module, cg.ctx, "set_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[lhs_ptr.into()], "set_len")
                .unwrap();
            let len = extract_int_result(len_call, "set_len");
            let zero = cg.ctx.i64_type().const_zero();
            let lhs_bool = cg
                .builder
                .build_int_compare(IntPredicate::NE, len, zero, "to_bool")
                .unwrap();

            match &rhs.ty() {
                PyType::Set(elem_ty) => {
                    // Set and Set -> Set (Python semantics: return first falsy or last)
                    let rhs_ptr = rhs.runtime_value().into_pointer_value();
                    let result = cg
                        .builder
                        .build_select(lhs_bool, rhs_ptr, lhs_ptr, "and")
                        .unwrap();
                    Ok(PyValue::new(result, PyType::Set(elem_ty.clone()), None))
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
            // Get set length to determine truthiness
            let len_fn = get_or_declare_builtin(&cg.module, cg.ctx, "set_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[lhs_ptr.into()], "set_len")
                .unwrap();
            let len = extract_int_result(len_call, "set_len");
            let zero = cg.ctx.i64_type().const_zero();
            let lhs_bool = cg
                .builder
                .build_int_compare(IntPredicate::NE, len, zero, "to_bool")
                .unwrap();

            match &rhs.ty() {
                PyType::Set(elem_ty) => {
                    // Set or Set -> Set (Python semantics: return first truthy or last)
                    let rhs_ptr = rhs.runtime_value().into_pointer_value();
                    let result = cg
                        .builder
                        .build_select(lhs_bool, lhs_ptr, rhs_ptr, "or")
                        .unwrap();
                    Ok(PyValue::new(result, PyType::Set(elem_ty.clone()), None))
                }
                _ => {
                    // Different types -> convert both to bool and return bool
                    let rhs_bool = cg.value_to_bool(rhs);
                    let result = cg.builder.build_or(lhs_bool, rhs_bool, "or").unwrap();
                    Ok(PyValue::bool(result))
                }
            }
        }

        // Identity operators - set is set compares pointer identity
        BinaryOp::Is => match &rhs.ty() {
            PyType::Set(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::EQ, lhs_ptr, rhs_ptr, "is")
                        .unwrap(),
                ))
            }
            // Different types are never identical
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero())),
        },
        BinaryOp::IsNot => match &rhs.ty() {
            PyType::Set(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::NE, lhs_ptr, rhs_ptr, "isnot")
                        .unwrap(),
                ))
            }
            // Different types are never identical, so is not returns true
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones())),
        },

        _ => Err(format!("Operator {:?} not supported on set", op)),
    }
}

/// Unary operations for set type
pub fn unary_op<'ctx>(
    val: &PyValue<'ctx>,
    cg: &CgCtx<'ctx>,
    op: &UnaryOp,
) -> Result<PyValue<'ctx>, String> {
    match op {
        UnaryOp::Not => {
            // not set: true if set is empty, false otherwise
            let ptr = val.runtime_value().into_pointer_value();
            let len_fn = get_or_declare_builtin(&cg.module, cg.ctx, "set_len");
            let len_call = cg
                .builder
                .build_call(len_fn, &[ptr.into()], "set_len")
                .unwrap();
            let len = extract_int_result(len_call, "set_len");
            let zero = cg.ctx.i64_type().const_zero();
            // not set is true when len == 0
            Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(IntPredicate::EQ, len, zero, "set_not")
                    .unwrap(),
            ))
        }
        _ => Err(format!("Unary operator {:?} not supported on set", op)),
    }
}
