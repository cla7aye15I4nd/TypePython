//! List operations for PyValue
//!
//! Binary and unary operations for Python list type.

use crate::ast::{BinaryOp, UnaryOp};
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

use super::value::{CgCtx, PyType, PyValue};
use super::{extract_int_result, extract_ptr_result, get_or_declare_builtin};

/// Binary operations for list type
pub fn binary_op<'a, 'ctx>(
    lhs: &PyValue<'ctx>,
    cg: &CgCtx<'a, 'ctx>,
    op: &BinaryOp,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let lhs_ptr = lhs.runtime_value().into_pointer_value();
    let lhs_elem_type = match &lhs.ty {
        PyType::List(elem) => elem.as_ref().clone(),
        _ => return Err("Expected list type".to_string()),
    };

    match op {
        // List concatenation: [1, 2] + [3, 4] = [1, 2, 3, 4]
        BinaryOp::Add => match &rhs.ty {
            PyType::List(rhs_elem) => {
                if rhs_elem.as_ref() != &lhs_elem_type {
                    return Err(format!(
                        "Cannot concatenate list[{:?}] with list[{:?}]",
                        lhs_elem_type,
                        rhs_elem.as_ref()
                    ));
                }
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let concat_fn = get_or_declare_builtin(cg.module, cg.ctx, "list_concat");
                let call_site = cg
                    .builder
                    .build_call(concat_fn, &[lhs_ptr.into(), rhs_ptr.into()], "list_concat")
                    .unwrap();
                let result = extract_ptr_result(call_site, "list_concat");
                Ok(PyValue::new(
                    result,
                    PyType::List(Box::new(lhs_elem_type)),
                    None,
                ))
            }
            _ => Err(format!("Cannot add list and {:?}", rhs.ty)),
        },

        // List repetition: [1, 2] * 3 = [1, 2, 1, 2, 1, 2]
        BinaryOp::Mul => match &rhs.ty {
            PyType::Int => {
                let rhs_int = rhs.runtime_value().into_int_value();
                let repeat_fn = get_or_declare_builtin(cg.module, cg.ctx, "list_repeat");
                let call_site = cg
                    .builder
                    .build_call(repeat_fn, &[lhs_ptr.into(), rhs_int.into()], "list_repeat")
                    .unwrap();
                let result = extract_ptr_result(call_site, "list_repeat");
                Ok(PyValue::new(
                    result,
                    PyType::List(Box::new(lhs_elem_type)),
                    None,
                ))
            }
            _ => Err(format!("Cannot multiply list by {:?}", rhs.ty)),
        },

        // List equality: [1, 2] == [1, 2]
        BinaryOp::Eq => match &rhs.ty {
            PyType::List(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let eq_fn = get_or_declare_builtin(cg.module, cg.ctx, "list_eq");
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
                        result.into_int_value(),
                        cg.ctx.i64_type().const_zero(),
                        "list_eq_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero().into())),
        },

        // List inequality: [1, 2] != [1, 3]
        BinaryOp::Ne => match &rhs.ty {
            PyType::List(_) => {
                let rhs_ptr = rhs.runtime_value().into_pointer_value();
                let eq_fn = get_or_declare_builtin(cg.module, cg.ctx, "list_eq");
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
                        result.into_int_value(),
                        cg.ctx.i64_type().const_zero(),
                        "list_ne_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_int(1, false).into())),
        },

        _ => Err(format!("Operator {:?} not supported on list", op)),
    }
}

/// Unary operations for list type
pub fn unary_op<'a, 'ctx>(
    _val: &PyValue<'ctx>,
    _cg: &CgCtx<'a, 'ctx>,
    op: &UnaryOp,
) -> Result<BasicValueEnum<'ctx>, String> {
    Err(format!("Unary operator {:?} not supported on list", op))
}
