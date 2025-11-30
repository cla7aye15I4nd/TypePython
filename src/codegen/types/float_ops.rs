//! Float operations for PyValue
//!
//! Binary and unary operations for Python float type.

use crate::ast::{BinaryOp, UnaryOp};
use inkwell::FloatPredicate;

use super::value::{CgCtx, PyType, PyValue};

/// Binary operations for Float type
pub fn binary_op<'ctx>(
    lhs: &PyValue<'ctx>,
    cg: &CgCtx<'ctx>,
    op: &BinaryOp,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let lhs_float = lhs.runtime_value().into_float_value();

    // Helper to coerce rhs to float
    let coerce_rhs = |rhs: &PyValue<'ctx>| -> Result<inkwell::values::FloatValue<'ctx>, String> {
        match &rhs.ty() {
            PyType::Float => Ok(rhs.runtime_value().into_float_value()),
            PyType::Int => Ok(cg
                .builder
                .build_signed_int_to_float(
                    rhs.runtime_value().into_int_value(),
                    cg.ctx.f64_type(),
                    "itof",
                )
                .unwrap()),
            PyType::Bool => {
                // Bool is i1, zero-extend to i64 then convert to float
                let int_val = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                Ok(cg
                    .builder
                    .build_signed_int_to_float(int_val, cg.ctx.f64_type(), "btof")
                    .unwrap())
            }
            _ => Err(format!("Cannot coerce {:?} to float", rhs.ty())),
        }
    };

    match op {
        // Arithmetic
        BinaryOp::Add => {
            let rhs_float = coerce_rhs(rhs)?;
            let result = cg
                .builder
                .build_float_add(lhs_float, rhs_float, "fadd")
                .unwrap();
            Ok(PyValue::float(result))
        }
        BinaryOp::Sub => {
            let rhs_float = coerce_rhs(rhs)?;
            let result = cg
                .builder
                .build_float_sub(lhs_float, rhs_float, "fsub")
                .unwrap();
            Ok(PyValue::float(result))
        }
        BinaryOp::Mul => {
            let rhs_float = coerce_rhs(rhs)?;
            let result = cg
                .builder
                .build_float_mul(lhs_float, rhs_float, "fmul")
                .unwrap();
            Ok(PyValue::float(result))
        }
        BinaryOp::Div => {
            let rhs_float = coerce_rhs(rhs)?;
            let result = cg
                .builder
                .build_float_div(lhs_float, rhs_float, "fdiv")
                .unwrap();
            Ok(PyValue::float(result))
        }
        BinaryOp::FloorDiv => {
            let rhs_float = coerce_rhs(rhs)?;
            let div_result = cg
                .builder
                .build_float_div(lhs_float, rhs_float, "fdiv")
                .unwrap();
            let floor_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "floor_float");
            let call_site = cg
                .builder
                .build_call(floor_fn, &[div_result.into()], "floor")
                .unwrap();
            Ok(PyValue::float(super::extract_float_result(
                call_site,
                "floor_float",
            )))
        }
        BinaryOp::Mod => {
            let rhs_float = coerce_rhs(rhs)?;
            let fmod_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "mod_float");
            let call_site = cg
                .builder
                .build_call(fmod_fn, &[lhs_float.into(), rhs_float.into()], "fmod")
                .unwrap();
            Ok(PyValue::float(super::extract_float_result(
                call_site,
                "mod_float",
            )))
        }
        BinaryOp::Pow => {
            let rhs_float = coerce_rhs(rhs)?;
            let pow_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, "pow_float");
            let call_site = cg
                .builder
                .build_call(pow_fn, &[lhs_float.into(), rhs_float.into()], "fpow")
                .unwrap();
            Ok(PyValue::float(super::extract_float_result(
                call_site,
                "pow_float",
            )))
        }

        // Comparison
        BinaryOp::Eq => {
            // If types are incompatible, return False directly (Python semantics)
            match coerce_rhs(rhs) {
                Ok(rhs_float) => Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OEQ, lhs_float, rhs_float, "feq")
                        .unwrap(),
                )),
                Err(_) => Ok(PyValue::bool(cg.ctx.bool_type().const_zero())),
            }
        }
        BinaryOp::Ne => {
            // If types are incompatible, return True directly (Python semantics)
            match coerce_rhs(rhs) {
                Ok(rhs_float) => Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::ONE, lhs_float, rhs_float, "fne")
                        .unwrap(),
                )),
                Err(_) => Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones())),
            }
        }
        BinaryOp::Lt => {
            let rhs_float = coerce_rhs(rhs)?;
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::OLT, lhs_float, rhs_float, "flt")
                    .unwrap(),
            ))
        }
        BinaryOp::Le => {
            let rhs_float = coerce_rhs(rhs)?;
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::OLE, lhs_float, rhs_float, "fle")
                    .unwrap(),
            ))
        }
        BinaryOp::Gt => {
            let rhs_float = coerce_rhs(rhs)?;
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::OGT, lhs_float, rhs_float, "fgt")
                    .unwrap(),
            ))
        }
        BinaryOp::Ge => {
            let rhs_float = coerce_rhs(rhs)?;
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::OGE, lhs_float, rhs_float, "fge")
                    .unwrap(),
            ))
        }
        BinaryOp::Is => match &rhs.ty() {
            PyType::Float => Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(
                        FloatPredicate::OEQ,
                        lhs_float,
                        rhs.runtime_value().into_float_value(),
                        "is",
                    )
                    .unwrap(),
            )),
            // Different types are never identical
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_zero())),
        },
        BinaryOp::IsNot => match &rhs.ty() {
            PyType::Float => Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(
                        FloatPredicate::ONE,
                        lhs_float,
                        rhs.runtime_value().into_float_value(),
                        "isnot",
                    )
                    .unwrap(),
            )),
            // Different types are never identical, so is not returns true
            _ => Ok(PyValue::bool(cg.ctx.bool_type().const_all_ones())),
        },

        // Logical and/or - same type returns same type, different types return bool
        BinaryOp::And => {
            let zero = cg.ctx.f64_type().const_zero();
            match &rhs.ty() {
                PyType::Float => {
                    // Float and Float -> Float (Python semantics: return first falsy or last)
                    let rhs_float = rhs.runtime_value().into_float_value();
                    let lhs_is_zero = cg
                        .builder
                        .build_float_compare(FloatPredicate::OEQ, lhs_float, zero, "is_zero")
                        .unwrap();
                    let result = cg
                        .builder
                        .build_select(lhs_is_zero, lhs_float, rhs_float, "and")
                        .unwrap();
                    Ok(PyValue::float(result.into_float_value()))
                }
                _ => {
                    // Different types -> convert both to bool and return bool
                    let lhs_bool = cg
                        .builder
                        .build_float_compare(FloatPredicate::ONE, lhs_float, zero, "to_bool")
                        .unwrap();
                    let rhs_bool = cg.value_to_bool(rhs);
                    let result = cg.builder.build_and(lhs_bool, rhs_bool, "and").unwrap();
                    Ok(PyValue::bool(result))
                }
            }
        }
        BinaryOp::Or => {
            let zero = cg.ctx.f64_type().const_zero();
            match &rhs.ty() {
                PyType::Float => {
                    // Float or Float -> Float (Python semantics: return first truthy or last)
                    let rhs_float = rhs.runtime_value().into_float_value();
                    let lhs_is_nonzero = cg
                        .builder
                        .build_float_compare(FloatPredicate::ONE, lhs_float, zero, "is_nonzero")
                        .unwrap();
                    let result = cg
                        .builder
                        .build_select(lhs_is_nonzero, lhs_float, rhs_float, "or")
                        .unwrap();
                    Ok(PyValue::float(result.into_float_value()))
                }
                _ => {
                    // Different types -> convert both to bool and return bool
                    let lhs_bool = cg
                        .builder
                        .build_float_compare(FloatPredicate::ONE, lhs_float, zero, "to_bool")
                        .unwrap();
                    let rhs_bool = cg.value_to_bool(rhs);
                    let result = cg.builder.build_or(lhs_bool, rhs_bool, "or").unwrap();
                    Ok(PyValue::bool(result))
                }
            }
        }
        BinaryOp::BitOr
        | BinaryOp::BitXor
        | BinaryOp::BitAnd
        | BinaryOp::LShift
        | BinaryOp::RShift => Err(format!("Bitwise operator {:?} not supported on floats", op)),
        BinaryOp::In | BinaryOp::NotIn => {
            // Float in list/dict/set
            let (fn_name, label) = match &rhs.ty() {
                PyType::List(_) => ("list_contains_float", "list_contains"),
                PyType::Dict(_, _) => ("dict_contains_float", "dict_contains"),
                PyType::Set(_) => ("set_contains_float", "set_contains"),
                _ => {
                    return Err(format!(
                        "Membership operator {:?} not supported for float in {:?}",
                        op,
                        rhs.ty()
                    ))
                }
            };
            let contains_fn = super::get_or_declare_builtin(&cg.module, cg.ctx, fn_name);
            let call = cg
                .builder
                .build_call(
                    contains_fn,
                    &[rhs.runtime_value().into(), lhs_float.into()],
                    label,
                )
                .unwrap();
            let result_i64 = super::extract_int_result(call, fn_name);
            // Convert i64 to i1 (bool): compare != 0
            let zero = cg.ctx.i64_type().const_zero();
            let bool_result = cg
                .builder
                .build_int_compare(inkwell::IntPredicate::NE, result_i64, zero, "tobool")
                .unwrap();
            if matches!(op, BinaryOp::NotIn) {
                Ok(PyValue::bool(
                    cg.builder.build_not(bool_result, "not").unwrap(),
                ))
            } else {
                Ok(PyValue::bool(bool_result))
            }
        }
    }
}

/// Unary operations for Float type
pub fn unary_op<'ctx>(
    val: &PyValue<'ctx>,
    cg: &CgCtx<'ctx>,
    op: &UnaryOp,
) -> Result<PyValue<'ctx>, String> {
    let float_val = val.runtime_value().into_float_value();
    match op {
        UnaryOp::Neg => Ok(PyValue::float(
            cg.builder.build_float_neg(float_val, "fneg").unwrap(),
        )),
        UnaryOp::Pos => Ok(PyValue::float(val.float_value())),
        UnaryOp::Not => {
            // not float: true if float == 0.0, false otherwise
            let zero = cg.ctx.f64_type().const_zero();
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::OEQ, float_val, zero, "fnot")
                    .unwrap(),
            ))
        }
        UnaryOp::BitNot => Err(format!("Operator {:?} not supported on floats", op)),
    }
}
