//! Int operations for PyValue
//!
//! Binary and unary operations for Python int type.

use crate::ast::{BinaryOp, UnaryOp};
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

use super::value::{CgCtx, PyType, PyValue};
use super::{extract_int_result, get_or_declare_builtin};

/// Binary operations for Int type
pub fn binary_op<'a, 'ctx>(
    lhs: &PyValue<'ctx>,
    cg: &CgCtx<'a, 'ctx>,
    op: &BinaryOp,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let lhs_int = lhs.runtime_value().into_int_value();

    match op {
        // Arithmetic
        BinaryOp::Add => match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_int_add(lhs_int, rhs.runtime_value().into_int_value(), "add")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let result = cg.builder.build_int_add(lhs_int, rhs_int, "add").unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
            }
            _ => Err(format!("Cannot add Int and {:?}", rhs.ty)),
        },
        BinaryOp::Sub => match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_int_sub(lhs_int, rhs.runtime_value().into_int_value(), "sub")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let result = cg.builder.build_int_sub(lhs_int, rhs_int, "sub").unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
            }
            _ => Err(format!("Cannot subtract {:?} from Int", rhs.ty)),
        },
        BinaryOp::Mul => match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_int_mul(lhs_int, rhs.runtime_value().into_int_value(), "mul")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let result = cg.builder.build_int_mul(lhs_int, rhs_int, "mul").unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
            }
            _ => Err(format!("Cannot multiply Int and {:?}", rhs.ty)),
        },
        BinaryOp::Div => {
            // Python 3 semantics: int / int -> float
            let lhs_float = cg
                .builder
                .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "lhs_itof")
                .unwrap();
            match &rhs.ty {
                PyType::Int => {
                    let rhs_float = cg
                        .builder
                        .build_signed_int_to_float(
                            rhs.runtime_value().into_int_value(),
                            cg.ctx.f64_type(),
                            "rhs_itof",
                        )
                        .unwrap();
                    let result = cg
                        .builder
                        .build_float_div(lhs_float, rhs_float, "fdiv")
                        .unwrap();
                    Ok(PyValue::float(result.into()))
                }
                PyType::Bool => {
                    let rhs_int = cg
                        .builder
                        .build_int_z_extend(
                            rhs.runtime_value().into_int_value(),
                            cg.ctx.i64_type(),
                            "btoi",
                        )
                        .unwrap();
                    let rhs_float = cg
                        .builder
                        .build_signed_int_to_float(rhs_int, cg.ctx.f64_type(), "rhs_itof")
                        .unwrap();
                    let result = cg
                        .builder
                        .build_float_div(lhs_float, rhs_float, "fdiv")
                        .unwrap();
                    Ok(PyValue::float(result.into()))
                }
                PyType::Float => {
                    super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
                }
                _ => Err(format!("Cannot divide Int by {:?}", rhs.ty)),
            }
        }
        BinaryOp::FloorDiv => match &rhs.ty {
            PyType::Int => {
                let floordiv_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "floordiv_int");
                let call_site = cg
                    .builder
                    .build_call(
                        floordiv_fn,
                        &[lhs_int.into(), rhs.runtime_value().into()],
                        "floordiv",
                    )
                    .unwrap();
                Ok(PyValue::int(super::extract_int_result(
                    call_site,
                    "floordiv_int",
                )))
            }
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let floordiv_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "floordiv_int");
                let call_site = cg
                    .builder
                    .build_call(floordiv_fn, &[lhs_int.into(), rhs_int.into()], "floordiv")
                    .unwrap();
                Ok(PyValue::int(super::extract_int_result(
                    call_site,
                    "floordiv_int",
                )))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
            }
            _ => Err(format!("Cannot floor divide Int by {:?}", rhs.ty)),
        },
        BinaryOp::Mod => match &rhs.ty {
            PyType::Int => {
                let mod_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "mod_int");
                let call_site = cg
                    .builder
                    .build_call(mod_fn, &[lhs_int.into(), rhs.runtime_value().into()], "mod")
                    .unwrap();
                Ok(PyValue::int(super::extract_int_result(
                    call_site, "mod_int",
                )))
            }
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let mod_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "mod_int");
                let call_site = cg
                    .builder
                    .build_call(mod_fn, &[lhs_int.into(), rhs_int.into()], "mod")
                    .unwrap();
                Ok(PyValue::int(super::extract_int_result(
                    call_site, "mod_int",
                )))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
            }
            _ => Err(format!("Cannot compute Int modulo {:?}", rhs.ty)),
        },
        BinaryOp::Pow => match &rhs.ty {
            PyType::Int => {
                let pow_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "pow_int");
                let call_site = cg
                    .builder
                    .build_call(
                        pow_fn,
                        &[lhs_int.into(), rhs.runtime_value().into()],
                        "ipow",
                    )
                    .unwrap();
                Ok(PyValue::int(super::extract_int_result(
                    call_site, "pow_int",
                )))
            }
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let pow_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "pow_int");
                let call_site = cg
                    .builder
                    .build_call(pow_fn, &[lhs_int.into(), rhs_int.into()], "ipow")
                    .unwrap();
                Ok(PyValue::int(super::extract_int_result(
                    call_site, "pow_int",
                )))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
            }
            _ => Err(format!("Cannot raise Int to power {:?}", rhs.ty)),
        },

        // Bitwise
        BinaryOp::BitAnd => match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_and(lhs_int, rhs.runtime_value().into_int_value(), "bitand")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let result = cg.builder.build_and(lhs_int, rhs_int, "bitand").unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot bitwise AND Int and {:?}", rhs.ty)),
        },
        BinaryOp::BitOr => match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_or(lhs_int, rhs.runtime_value().into_int_value(), "bitor")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let result = cg.builder.build_or(lhs_int, rhs_int, "bitor").unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot bitwise OR Int and {:?}", rhs.ty)),
        },
        BinaryOp::BitXor => match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_xor(lhs_int, rhs.runtime_value().into_int_value(), "bitxor")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let result = cg.builder.build_xor(lhs_int, rhs_int, "bitxor").unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot bitwise XOR Int and {:?}", rhs.ty)),
        },
        BinaryOp::LShift => match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_left_shift(lhs_int, rhs.runtime_value().into_int_value(), "lshift")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let result = cg.builder.build_left_shift(lhs_int, rhs_int, "lshift").unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot left shift Int by {:?}", rhs.ty)),
        },
        BinaryOp::RShift => match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_right_shift(
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        true,
                        "rshift",
                    )
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let result = cg.builder.build_right_shift(lhs_int, rhs_int, true, "rshift").unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot right shift Int by {:?}", rhs.ty)),
        },

        // Comparison
        BinaryOp::Eq => match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        "eq",
                    )
                    .unwrap()
                    .into(),
            )),
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::EQ, lhs_int, rhs_int, "eq")
                        .unwrap()
                        .into(),
                ))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
            }
            _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
        },
        BinaryOp::Ne => match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::NE,
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        "ne",
                    )
                    .unwrap()
                    .into(),
            )),
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::NE, lhs_int, rhs_int, "ne")
                        .unwrap()
                        .into(),
                ))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
            }
            _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
        },
        BinaryOp::Lt => match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::SLT,
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        "lt",
                    )
                    .unwrap()
                    .into(),
            )),
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::SLT, lhs_int, rhs_int, "lt")
                        .unwrap()
                        .into(),
                ))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
            }
            _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
        },
        BinaryOp::Le => match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::SLE,
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        "le",
                    )
                    .unwrap()
                    .into(),
            )),
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::SLE, lhs_int, rhs_int, "le")
                        .unwrap()
                        .into(),
                ))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
            }
            _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
        },
        BinaryOp::Gt => match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::SGT,
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        "gt",
                    )
                    .unwrap()
                    .into(),
            )),
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::SGT, lhs_int, rhs_int, "gt")
                        .unwrap()
                        .into(),
                ))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
            }
            _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
        },
        BinaryOp::Ge => match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::SGE,
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        "ge",
                    )
                    .unwrap()
                    .into(),
            )),
            PyType::Bool => {
                let rhs_int = cg
                    .builder
                    .build_int_z_extend(
                        rhs.runtime_value().into_int_value(),
                        cg.ctx.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::SGE, lhs_int, rhs_int, "ge")
                        .unwrap()
                        .into(),
                ))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                super::float_ops::binary_op(&PyValue::float(lhs_float.into()), cg, op, rhs)
            }
            _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
        },
        BinaryOp::Is => match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        "is",
                    )
                    .unwrap()
                    .into(),
            )),
            _ => Err(format!("Cannot use 'is' between Int and {:?}", rhs.ty)),
        },
        BinaryOp::IsNot => match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::NE,
                        lhs_int,
                        rhs.runtime_value().into_int_value(),
                        "isnot",
                    )
                    .unwrap()
                    .into(),
            )),
            _ => Err(format!("Cannot use 'is not' between Int and {:?}", rhs.ty)),
        },

        // Logical
        BinaryOp::And => match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_and(lhs_int, rhs.runtime_value().into_int_value(), "and")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot logical AND Int and {:?}", rhs.ty)),
        },
        BinaryOp::Or => match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_or(lhs_int, rhs.runtime_value().into_int_value(), "or")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot logical OR Int and {:?}", rhs.ty)),
        },

        BinaryOp::In => match &rhs.ty {
            PyType::List(_) => {
                let contains_fn = get_or_declare_builtin(cg.module, cg.ctx, "list_contains");
                let call_site = cg
                    .builder
                    .build_call(
                        contains_fn,
                        &[rhs.runtime_value().into(), lhs_int.into()],
                        "list_contains",
                    )
                    .unwrap();
                let result = extract_int_result(call_site, "list_contains");
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        result.into_int_value(),
                        cg.ctx.i64_type().const_zero(),
                        "in_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            PyType::Set(_) => {
                let contains_fn = get_or_declare_builtin(cg.module, cg.ctx, "set_contains");
                let call_site = cg
                    .builder
                    .build_call(
                        contains_fn,
                        &[rhs.runtime_value().into(), lhs_int.into()],
                        "set_contains",
                    )
                    .unwrap();
                let result = extract_int_result(call_site, "set_contains");
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        result.into_int_value(),
                        cg.ctx.i64_type().const_zero(),
                        "in_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            PyType::Dict(_, _) => {
                let contains_fn = get_or_declare_builtin(cg.module, cg.ctx, "dict_contains");
                let call_site = cg
                    .builder
                    .build_call(
                        contains_fn,
                        &[rhs.runtime_value().into(), lhs_int.into()],
                        "dict_contains",
                    )
                    .unwrap();
                let result = extract_int_result(call_site, "dict_contains");
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        result.into_int_value(),
                        cg.ctx.i64_type().const_zero(),
                        "in_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot use 'in' with Int and {:?}", rhs.ty)),
        },

        BinaryOp::NotIn => match &rhs.ty {
            PyType::List(_) => {
                let contains_fn = get_or_declare_builtin(cg.module, cg.ctx, "list_contains");
                let call_site = cg
                    .builder
                    .build_call(
                        contains_fn,
                        &[rhs.runtime_value().into(), lhs_int.into()],
                        "list_contains",
                    )
                    .unwrap();
                let result = extract_int_result(call_site, "list_contains");
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        result.into_int_value(),
                        cg.ctx.i64_type().const_zero(),
                        "not_in_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            PyType::Set(_) => {
                let contains_fn = get_or_declare_builtin(cg.module, cg.ctx, "set_contains");
                let call_site = cg
                    .builder
                    .build_call(
                        contains_fn,
                        &[rhs.runtime_value().into(), lhs_int.into()],
                        "set_contains",
                    )
                    .unwrap();
                let result = extract_int_result(call_site, "set_contains");
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        result.into_int_value(),
                        cg.ctx.i64_type().const_zero(),
                        "not_in_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            PyType::Dict(_, _) => {
                let contains_fn = get_or_declare_builtin(cg.module, cg.ctx, "dict_contains");
                let call_site = cg
                    .builder
                    .build_call(
                        contains_fn,
                        &[rhs.runtime_value().into(), lhs_int.into()],
                        "dict_contains",
                    )
                    .unwrap();
                let result = extract_int_result(call_site, "dict_contains");
                let bool_val = cg
                    .builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        result.into_int_value(),
                        cg.ctx.i64_type().const_zero(),
                        "not_in_bool",
                    )
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot use 'not in' with Int and {:?}", rhs.ty)),
        },
    }
}

/// Unary operations for Int type
pub fn unary_op<'a, 'ctx>(
    val: &PyValue<'ctx>,
    cg: &CgCtx<'a, 'ctx>,
    op: &UnaryOp,
) -> Result<BasicValueEnum<'ctx>, String> {
    let int_val = val.runtime_value().into_int_value();
    match op {
        UnaryOp::Neg => Ok(cg.builder.build_int_neg(int_val, "neg").unwrap().into()),
        UnaryOp::Pos => Ok(val.runtime_value()),
        UnaryOp::Not => Ok(cg.builder.build_not(int_val, "not").unwrap().into()),
        UnaryOp::BitNot => Ok(cg.builder.build_not(int_val, "bitnot").unwrap().into()),
    }
}
