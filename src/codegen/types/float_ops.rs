//! Float operations for PyValue
//!
//! Binary and unary operations for Python float type.

use crate::ast::{BinaryOp, UnaryOp};
use inkwell::values::BasicValueEnum;
use inkwell::FloatPredicate;

use super::value::{CgCtx, PyType, PyValue};

/// Binary operations for Float type
pub fn binary_op<'a, 'ctx>(
    lhs: &PyValue<'ctx>,
    cg: &CgCtx<'a, 'ctx>,
    op: &BinaryOp,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let lhs_float = lhs.runtime_value().into_float_value();

    // Helper to coerce rhs to float
    let coerce_rhs = |rhs: &PyValue<'ctx>| -> Result<inkwell::values::FloatValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Float => Ok(rhs.runtime_value().into_float_value()),
            PyType::Int => Ok(cg
                .builder
                .build_signed_int_to_float(
                    rhs.runtime_value().into_int_value(),
                    cg.ctx.f64_type(),
                    "itof",
                )
                .unwrap()),
            _ => Err(format!("Cannot coerce {:?} to float", rhs.ty)),
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
            Ok(PyValue::float(result.into()))
        }
        BinaryOp::Sub => {
            let rhs_float = coerce_rhs(rhs)?;
            let result = cg
                .builder
                .build_float_sub(lhs_float, rhs_float, "fsub")
                .unwrap();
            Ok(PyValue::float(result.into()))
        }
        BinaryOp::Mul => {
            let rhs_float = coerce_rhs(rhs)?;
            let result = cg
                .builder
                .build_float_mul(lhs_float, rhs_float, "fmul")
                .unwrap();
            Ok(PyValue::float(result.into()))
        }
        BinaryOp::Div => {
            let rhs_float = coerce_rhs(rhs)?;
            let result = cg
                .builder
                .build_float_div(lhs_float, rhs_float, "fdiv")
                .unwrap();
            Ok(PyValue::float(result.into()))
        }
        BinaryOp::FloorDiv => {
            let rhs_float = coerce_rhs(rhs)?;
            let div_result = cg
                .builder
                .build_float_div(lhs_float, rhs_float, "fdiv")
                .unwrap();
            let floor_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "floor_float");
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
            let fmod_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "mod_float");
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
            let pow_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "pow_float");
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
            let rhs_float = coerce_rhs(rhs)?;
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::OEQ, lhs_float, rhs_float, "feq")
                    .unwrap()
                    .into(),
            ))
        }
        BinaryOp::Ne => {
            let rhs_float = coerce_rhs(rhs)?;
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::ONE, lhs_float, rhs_float, "fne")
                    .unwrap()
                    .into(),
            ))
        }
        BinaryOp::Lt => {
            let rhs_float = coerce_rhs(rhs)?;
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::OLT, lhs_float, rhs_float, "flt")
                    .unwrap()
                    .into(),
            ))
        }
        BinaryOp::Le => {
            let rhs_float = coerce_rhs(rhs)?;
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::OLE, lhs_float, rhs_float, "fle")
                    .unwrap()
                    .into(),
            ))
        }
        BinaryOp::Gt => {
            let rhs_float = coerce_rhs(rhs)?;
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::OGT, lhs_float, rhs_float, "fgt")
                    .unwrap()
                    .into(),
            ))
        }
        BinaryOp::Ge => {
            let rhs_float = coerce_rhs(rhs)?;
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::OGE, lhs_float, rhs_float, "fge")
                    .unwrap()
                    .into(),
            ))
        }
        BinaryOp::Is => {
            let rhs_float = coerce_rhs(rhs)?;
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::OEQ, lhs_float, rhs_float, "is")
                    .unwrap()
                    .into(),
            ))
        }
        BinaryOp::IsNot => {
            let rhs_float = coerce_rhs(rhs)?;
            Ok(PyValue::bool(
                cg.builder
                    .build_float_compare(FloatPredicate::ONE, lhs_float, rhs_float, "isnot")
                    .unwrap()
                    .into(),
            ))
        }

        // Not supported on floats
        BinaryOp::And | BinaryOp::Or => {
            Err("Logical operators not supported on floats".to_string())
        }
        BinaryOp::BitOr
        | BinaryOp::BitXor
        | BinaryOp::BitAnd
        | BinaryOp::LShift
        | BinaryOp::RShift => Err(format!("Bitwise operator {:?} not supported on floats", op)),
        BinaryOp::In | BinaryOp::NotIn => Err(format!(
            "Membership operator {:?} requires container support",
            op
        )),
    }
}

/// Unary operations for Float type
pub fn unary_op<'a, 'ctx>(
    val: &PyValue<'ctx>,
    cg: &CgCtx<'a, 'ctx>,
    op: &UnaryOp,
) -> Result<BasicValueEnum<'ctx>, String> {
    let float_val = val.runtime_value().into_float_value();
    match op {
        UnaryOp::Neg => Ok(cg
            .builder
            .build_float_neg(float_val, "fneg")
            .unwrap()
            .into()),
        UnaryOp::Pos => Ok(val.runtime_value()),
        UnaryOp::Not | UnaryOp::BitNot => Err(format!("Operator {:?} not supported on floats", op)),
    }
}
