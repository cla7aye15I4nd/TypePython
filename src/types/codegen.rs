//! Code generation operations for types
//!
//! This module provides the `CodeGenOps` trait that each type implements
//! to generate LLVM instructions for operations.

use crate::ast::{BinaryOp, Type, UnaryOp};
use crate::codegen::builtins::BUILTIN_TABLE;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue};
use inkwell::{FloatPredicate, IntPredicate};

use super::{BoolType, BytesType, FloatType, IntType, NoneType};

/// Code generation context bundling LLVM context, builder, and module.
/// This reduces parameter passing in codegen operations.
pub struct CgCtx<'a, 'ctx> {
    pub ctx: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
}

impl<'a, 'ctx> CgCtx<'a, 'ctx> {
    pub fn new(ctx: &'ctx Context, builder: &'a Builder<'ctx>, module: &'a Module<'ctx>) -> Self {
        Self {
            ctx,
            builder,
            module,
        }
    }
}

/// Trait for type-specific LLVM code generation.
///
/// Each type implements this to generate its own instructions for operations,
/// eliminating the need for if/else chains in the main codegen.
///
/// Internal trait used by PyValue - external code should use PyValue::binary_op().
pub(super) trait CodeGenOps<'a, 'ctx> {
    /// Generate code for a binary operation: `lhs op rhs`
    /// Internal implementation - called by PyValue::binary_op()
    fn binary_op_impl(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: &super::PyValue<'ctx>,
    ) -> Result<super::PyValue<'ctx>, String>;

    /// Generate code for a unary operation on this type.
    fn unary_op(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &UnaryOp,
        val: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String>;

    /// Generate a print call for a value of this type.
    fn print(
        &self,
        builder: &Builder<'ctx>,
        print_fn: FunctionValue<'ctx>,
        val: BasicValueEnum<'ctx>,
    ) -> Result<(), String>;

    /// Coerce a value to the target type (e.g., int to float).
    fn coerce_to(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        val: BasicValueEnum<'ctx>,
        target: &crate::ast::Type,
    ) -> Result<BasicValueEnum<'ctx>, String>;
}

// ============================================================================
// IntType CodeGen Implementation
// ============================================================================

impl<'a, 'ctx> CodeGenOps<'a, 'ctx> for super::IntType {
    fn binary_op_impl(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: &super::PyValue<'ctx>,
    ) -> Result<super::PyValue<'ctx>, String> {
        use super::{PyType, PyValue};

        // Handle Int op Int case
        if let PyType::Int(_) = &rhs.ty {
            let lhs_int = lhs.into_int_value();
            let rhs_int = rhs.value.into_int_value();

            let result = match op {
                BinaryOp::Add => cg.builder.build_int_add(lhs_int, rhs_int, "add").unwrap(),
                BinaryOp::Sub => cg.builder.build_int_sub(lhs_int, rhs_int, "sub").unwrap(),
                BinaryOp::Mul => cg.builder.build_int_mul(lhs_int, rhs_int, "mul").unwrap(),
                BinaryOp::Div => {
                    return Ok(PyValue::int(
                        cg.builder
                            .build_int_signed_div(lhs_int, rhs_int, "div")
                            .unwrap()
                            .into(),
                    ))
                }
                BinaryOp::FloorDiv => {
                    let floordiv_fn = get_or_declare_builtin(cg.module, cg.ctx, "floordiv_int");
                    let call_site = cg
                        .builder
                        .build_call(floordiv_fn, &[lhs_int.into(), rhs_int.into()], "floordiv")
                        .unwrap();
                    return Ok(PyValue::int(extract_int_result(call_site, "floordiv_int")?));
                }
                BinaryOp::Mod => {
                    let mod_fn = get_or_declare_builtin(cg.module, cg.ctx, "mod_int");
                    let call_site = cg
                        .builder
                        .build_call(mod_fn, &[lhs_int.into(), rhs_int.into()], "mod")
                        .unwrap();
                    return Ok(PyValue::int(extract_int_result(call_site, "mod_int")?));
                }
                BinaryOp::Pow => {
                    let pow_fn = get_or_declare_builtin(cg.module, cg.ctx, "pow_int");
                    let call_site = cg
                        .builder
                        .build_call(pow_fn, &[lhs_int.into(), rhs_int.into()], "ipow")
                        .unwrap();
                    return Ok(PyValue::int(extract_int_result(call_site, "pow_int")?));
                }
                BinaryOp::Eq => {
                    return Ok(PyValue::bool(
                        cg.builder
                            .build_int_compare(IntPredicate::EQ, lhs_int, rhs_int, "eq")
                            .unwrap()
                            .into(),
                    ))
                }
                BinaryOp::Ne => {
                    return Ok(PyValue::bool(
                        cg.builder
                            .build_int_compare(IntPredicate::NE, lhs_int, rhs_int, "ne")
                            .unwrap()
                            .into(),
                    ))
                }
                BinaryOp::Lt => {
                    return Ok(PyValue::bool(
                        cg.builder
                            .build_int_compare(IntPredicate::SLT, lhs_int, rhs_int, "lt")
                            .unwrap()
                            .into(),
                    ))
                }
                BinaryOp::Le => {
                    return Ok(PyValue::bool(
                        cg.builder
                            .build_int_compare(IntPredicate::SLE, lhs_int, rhs_int, "le")
                            .unwrap()
                            .into(),
                    ))
                }
                BinaryOp::Gt => {
                    return Ok(PyValue::bool(
                        cg.builder
                            .build_int_compare(IntPredicate::SGT, lhs_int, rhs_int, "gt")
                            .unwrap()
                            .into(),
                    ))
                }
                BinaryOp::Ge => {
                    return Ok(PyValue::bool(
                        cg.builder
                            .build_int_compare(IntPredicate::SGE, lhs_int, rhs_int, "ge")
                            .unwrap()
                            .into(),
                    ))
                }
                BinaryOp::And => cg.builder.build_and(lhs_int, rhs_int, "and").unwrap(),
                BinaryOp::Or => cg.builder.build_or(lhs_int, rhs_int, "or").unwrap(),
                BinaryOp::BitOr => cg.builder.build_or(lhs_int, rhs_int, "bitor").unwrap(),
                BinaryOp::BitXor => cg.builder.build_xor(lhs_int, rhs_int, "bitxor").unwrap(),
                BinaryOp::BitAnd => cg.builder.build_and(lhs_int, rhs_int, "bitand").unwrap(),
                BinaryOp::LShift => cg
                    .builder
                    .build_left_shift(lhs_int, rhs_int, "lshift")
                    .unwrap(),
                BinaryOp::RShift => cg
                    .builder
                    .build_right_shift(lhs_int, rhs_int, true, "rshift")
                    .unwrap(),
                BinaryOp::Is => {
                    return Ok(PyValue::bool(
                        cg.builder
                            .build_int_compare(IntPredicate::EQ, lhs_int, rhs_int, "is")
                            .unwrap()
                            .into(),
                    ))
                }
                BinaryOp::IsNot => {
                    return Ok(PyValue::bool(
                        cg.builder
                            .build_int_compare(IntPredicate::NE, lhs_int, rhs_int, "isnot")
                            .unwrap()
                            .into(),
                    ))
                }
                BinaryOp::In | BinaryOp::NotIn => {
                    return Err(format!(
                        "Membership operator {:?} requires container support",
                        op
                    ))
                }
            };
            return Ok(PyValue::int(result.into()));
        }

        // Handle Int op Float -> Float (coerce int to float)
        if let PyType::Float(_) = &rhs.ty {
            let lhs_float = cg
                .builder
                .build_signed_int_to_float(lhs.into_int_value(), cg.ctx.f64_type(), "itof")
                .unwrap();
            return FloatType.binary_op_impl(cg, op, lhs_float.into(), rhs);
        }

        Err(format!(
            "Unsupported binary operation {:?} between Int and {:?}",
            op, rhs.ty
        ))
    }

    fn unary_op(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &UnaryOp,
        val: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let int_val = val.into_int_value();
        match op {
            UnaryOp::Neg => Ok(cg.builder.build_int_neg(int_val, "neg").unwrap().into()),
            UnaryOp::Pos => Ok(val), // no-op
            UnaryOp::Not => Ok(cg.builder.build_not(int_val, "not").unwrap().into()),
            UnaryOp::BitNot => Ok(cg.builder.build_not(int_val, "bitnot").unwrap().into()),
        }
    }

    fn print(
        &self,
        builder: &Builder<'ctx>,
        print_fn: FunctionValue<'ctx>,
        val: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let int_val = val.into_int_value();
        builder
            .build_call(print_fn, &[int_val.into()], "print_int")
            .unwrap();
        Ok(())
    }

    fn coerce_to(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        val: BasicValueEnum<'ctx>,
        target: &crate::ast::Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match target {
            crate::ast::Type::Int => Ok(val),
            crate::ast::Type::Float => {
                let int_val = val.into_int_value();
                Ok(cg
                    .builder
                    .build_signed_int_to_float(int_val, cg.ctx.f64_type(), "itof")
                    .unwrap()
                    .into())
            }
            _ => Err(format!("Cannot coerce Int to {:?}", target)),
        }
    }
}

// ============================================================================
// FloatType CodeGen Implementation
// ============================================================================

impl<'a, 'ctx> CodeGenOps<'a, 'ctx> for super::FloatType {
    fn binary_op_impl(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: &super::PyValue<'ctx>,
    ) -> Result<super::PyValue<'ctx>, String> {
        use super::{PyType, PyValue};

        // Coerce rhs to float if it's an int
        let rhs_float = match &rhs.ty {
            PyType::Float(_) => rhs.value.into_float_value(),
            PyType::Int(_) => cg
                .builder
                .build_signed_int_to_float(rhs.value.into_int_value(), cg.ctx.f64_type(), "itof")
                .unwrap(),
            _ => {
                return Err(format!(
                    "Unsupported binary operation {:?} between Float and {:?}",
                    op, rhs.ty
                ))
            }
        };

        let lhs_float = lhs.into_float_value();

        let result = match op {
            BinaryOp::Add => cg
                .builder
                .build_float_add(lhs_float, rhs_float, "fadd")
                .unwrap(),
            BinaryOp::Sub => cg
                .builder
                .build_float_sub(lhs_float, rhs_float, "fsub")
                .unwrap(),
            BinaryOp::Mul => cg
                .builder
                .build_float_mul(lhs_float, rhs_float, "fmul")
                .unwrap(),
            BinaryOp::Div => cg
                .builder
                .build_float_div(lhs_float, rhs_float, "fdiv")
                .unwrap(),
            BinaryOp::Mod => {
                let fmod_fn = get_or_declare_builtin(cg.module, cg.ctx, "mod_float");
                let call_site = cg
                    .builder
                    .build_call(fmod_fn, &[lhs_float.into(), rhs_float.into()], "fmod")
                    .unwrap();
                return Ok(PyValue::float(extract_float_result(
                    call_site,
                    "mod_float",
                )?));
            }
            BinaryOp::Pow => {
                let pow_fn = get_or_declare_builtin(cg.module, cg.ctx, "pow_float");
                let call_site = cg
                    .builder
                    .build_call(pow_fn, &[lhs_float.into(), rhs_float.into()], "fpow")
                    .unwrap();
                return Ok(PyValue::float(extract_float_result(
                    call_site,
                    "pow_float",
                )?));
            }
            BinaryOp::FloorDiv => {
                let div_result = cg
                    .builder
                    .build_float_div(lhs_float, rhs_float, "fdiv")
                    .unwrap();
                let floor_fn = get_or_declare_builtin(cg.module, cg.ctx, "floor_float");
                let call_site = cg
                    .builder
                    .build_call(floor_fn, &[div_result.into()], "floor")
                    .unwrap();
                return Ok(PyValue::float(extract_float_result(
                    call_site,
                    "floor_float",
                )?));
            }
            BinaryOp::Eq => {
                return Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OEQ, lhs_float, rhs_float, "feq")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::Ne => {
                return Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::ONE, lhs_float, rhs_float, "fne")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::Lt => {
                return Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OLT, lhs_float, rhs_float, "flt")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::Le => {
                return Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OLE, lhs_float, rhs_float, "fle")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::Gt => {
                return Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OGT, lhs_float, rhs_float, "fgt")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::Ge => {
                return Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OGE, lhs_float, rhs_float, "fge")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::Is => {
                return Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OEQ, lhs_float, rhs_float, "is")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::IsNot => {
                return Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::ONE, lhs_float, rhs_float, "isnot")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::And | BinaryOp::Or => {
                return Err("Logical operators not supported on floats".to_string())
            }
            BinaryOp::BitOr
            | BinaryOp::BitXor
            | BinaryOp::BitAnd
            | BinaryOp::LShift
            | BinaryOp::RShift => {
                return Err(format!("Bitwise operator {:?} not supported on floats", op))
            }
            BinaryOp::In | BinaryOp::NotIn => {
                return Err(format!(
                    "Membership operator {:?} requires container support",
                    op
                ))
            }
        };
        Ok(PyValue::float(result.into()))
    }

    fn unary_op(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &UnaryOp,
        val: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let float_val = val.into_float_value();
        match op {
            UnaryOp::Neg => Ok(cg
                .builder
                .build_float_neg(float_val, "fneg")
                .unwrap()
                .into()),
            UnaryOp::Pos => Ok(val), // no-op
            UnaryOp::Not | UnaryOp::BitNot => {
                Err(format!("Operator {:?} not supported on floats", op))
            }
        }
    }

    fn print(
        &self,
        builder: &Builder<'ctx>,
        print_fn: FunctionValue<'ctx>,
        val: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let float_val = val.into_float_value();
        builder
            .build_call(print_fn, &[float_val.into()], "print_float")
            .unwrap();
        Ok(())
    }

    fn coerce_to(
        &self,
        _cg: &CgCtx<'a, 'ctx>,
        val: BasicValueEnum<'ctx>,
        target: &crate::ast::Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match target {
            crate::ast::Type::Float => Ok(val),
            _ => Err(format!("Cannot coerce Float to {:?}", target)),
        }
    }
}

// ============================================================================
// BoolType CodeGen Implementation
// ============================================================================

impl<'a, 'ctx> CodeGenOps<'a, 'ctx> for super::BoolType {
    fn binary_op_impl(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: &super::PyValue<'ctx>,
    ) -> Result<super::PyValue<'ctx>, String> {
        use super::{PyType, PyValue};

        // Handle Bool op Bool case
        if let PyType::Bool(_) = &rhs.ty {
            let lhs_bool = lhs.into_int_value();
            let rhs_bool = rhs.value.into_int_value();

            return match op {
                BinaryOp::And => Ok(PyValue::bool(
                    cg.builder
                        .build_and(lhs_bool, rhs_bool, "and")
                        .unwrap()
                        .into(),
                )),
                BinaryOp::Or => Ok(PyValue::bool(
                    cg.builder
                        .build_or(lhs_bool, rhs_bool, "or")
                        .unwrap()
                        .into(),
                )),
                BinaryOp::Eq => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::EQ, lhs_bool, rhs_bool, "eq")
                        .unwrap()
                        .into(),
                )),
                BinaryOp::Ne => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::NE, lhs_bool, rhs_bool, "ne")
                        .unwrap()
                        .into(),
                )),
                BinaryOp::BitOr => Ok(PyValue::bool(
                    cg.builder
                        .build_or(lhs_bool, rhs_bool, "bitor")
                        .unwrap()
                        .into(),
                )),
                BinaryOp::BitXor => Ok(PyValue::bool(
                    cg.builder
                        .build_xor(lhs_bool, rhs_bool, "bitxor")
                        .unwrap()
                        .into(),
                )),
                BinaryOp::BitAnd => Ok(PyValue::bool(
                    cg.builder
                        .build_and(lhs_bool, rhs_bool, "bitand")
                        .unwrap()
                        .into(),
                )),
                BinaryOp::Is => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::EQ, lhs_bool, rhs_bool, "is")
                        .unwrap()
                        .into(),
                )),
                BinaryOp::IsNot => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(IntPredicate::NE, lhs_bool, rhs_bool, "isnot")
                        .unwrap()
                        .into(),
                )),
                _ => Err(format!("Operator {:?} not supported on bools directly", op)),
            };
        }

        // Handle Bool op Int -> Int (coerce bool to int)
        if let PyType::Int(_) = &rhs.ty {
            let lhs_int = cg
                .builder
                .build_int_z_extend(lhs.into_int_value(), cg.ctx.i64_type(), "btoi")
                .unwrap();
            return IntType.binary_op_impl(cg, op, lhs_int.into(), rhs);
        }

        // Handle Bool op Float -> Float
        if let PyType::Float(_) = &rhs.ty {
            let lhs_int = cg
                .builder
                .build_int_z_extend(lhs.into_int_value(), cg.ctx.i64_type(), "btoi")
                .unwrap();
            let lhs_float = cg
                .builder
                .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                .unwrap();
            return FloatType.binary_op_impl(cg, op, lhs_float.into(), rhs);
        }

        Err(format!(
            "Unsupported binary operation {:?} between Bool and {:?}",
            op, rhs.ty
        ))
    }

    fn unary_op(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &UnaryOp,
        val: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let bool_val = val.into_int_value();
        match op {
            UnaryOp::Not => Ok(cg.builder.build_not(bool_val, "not").unwrap().into()),
            _ => Err(format!("Operator {:?} not supported on bools", op)),
        }
    }

    fn print(
        &self,
        builder: &Builder<'ctx>,
        print_fn: FunctionValue<'ctx>,
        val: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let bool_val = val.into_int_value();
        builder
            .build_call(print_fn, &[bool_val.into()], "print_bool")
            .unwrap();
        Ok(())
    }

    fn coerce_to(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        val: BasicValueEnum<'ctx>,
        target: &crate::ast::Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let bool_val = val.into_int_value();
        match target {
            crate::ast::Type::Bool => Ok(val),
            crate::ast::Type::Int => {
                // Zero-extend bool to i64
                Ok(cg
                    .builder
                    .build_int_z_extend(bool_val, cg.ctx.i64_type(), "btoi")
                    .unwrap()
                    .into())
            }
            crate::ast::Type::Float => {
                // First extend to i64, then convert to float
                let int_val = cg
                    .builder
                    .build_int_z_extend(bool_val, cg.ctx.i64_type(), "btoi")
                    .unwrap();
                Ok(cg
                    .builder
                    .build_signed_int_to_float(int_val, cg.ctx.f64_type(), "itof")
                    .unwrap()
                    .into())
            }
            _ => Err(format!("Cannot coerce Bool to {:?}", target)),
        }
    }
}

// ============================================================================
// BytesType CodeGen Implementation
// ============================================================================

impl<'a, 'ctx> CodeGenOps<'a, 'ctx> for super::BytesType {
    fn binary_op_impl(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: &super::PyValue<'ctx>,
    ) -> Result<super::PyValue<'ctx>, String> {
        use super::{PyType, PyValue};
        let lhs_ptr = lhs.into_pointer_value();

        // Handle Bytes op Bytes case
        if let PyType::Bytes(_) = &rhs.ty {
            let rhs_ptr = rhs.value.into_pointer_value();

            return match op {
                BinaryOp::Add => {
                    // Bytes concatenation
                    let strcat_fn = get_or_declare_builtin(cg.module, cg.ctx, "strcat_bytes");
                    let call_site = cg
                        .builder
                        .build_call(strcat_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytescat")
                        .unwrap();
                    Ok(PyValue::bytes(extract_ptr_result(
                        call_site,
                        "strcat_bytes",
                    )?))
                }
                BinaryOp::Eq => {
                    let strcmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "strcmp_bytes");
                    let call_site = cg
                        .builder
                        .build_call(strcmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytescmp")
                        .unwrap();
                    let result = extract_int_result(call_site, "strcmp_bytes")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
                BinaryOp::Ne => {
                    let strcmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "strcmp_bytes");
                    let call_site = cg
                        .builder
                        .build_call(strcmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytescmp")
                        .unwrap();
                    let result = extract_int_result(call_site, "strcmp_bytes")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    let negated = cg.builder.build_not(bool_val, "ne").unwrap();
                    Ok(PyValue::bool(negated.into()))
                }
                BinaryOp::Lt => {
                    let cmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_lt");
                    let call_site = cg
                        .builder
                        .build_call(cmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytes_lt")
                        .unwrap();
                    let result = extract_int_result(call_site, "bytes_lt")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
                BinaryOp::Le => {
                    let cmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_le");
                    let call_site = cg
                        .builder
                        .build_call(cmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytes_le")
                        .unwrap();
                    let result = extract_int_result(call_site, "bytes_le")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
                BinaryOp::Gt => {
                    let cmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_gt");
                    let call_site = cg
                        .builder
                        .build_call(cmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytes_gt")
                        .unwrap();
                    let result = extract_int_result(call_site, "bytes_gt")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
                BinaryOp::Ge => {
                    let cmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_ge");
                    let call_site = cg
                        .builder
                        .build_call(cmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytes_ge")
                        .unwrap();
                    let result = extract_int_result(call_site, "bytes_ge")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
                BinaryOp::In => {
                    // b"x" in b"xyz" -> bytes_contains(b"xyz", b"x")
                    let contains_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_contains");
                    let call_site = cg
                        .builder
                        .build_call(
                            contains_fn,
                            &[rhs_ptr.into(), lhs_ptr.into()],
                            "bytes_contains",
                        )
                        .unwrap();
                    let result = extract_int_result(call_site, "bytes_contains")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
                BinaryOp::NotIn => {
                    let contains_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_contains");
                    let call_site = cg
                        .builder
                        .build_call(
                            contains_fn,
                            &[rhs_ptr.into(), lhs_ptr.into()],
                            "bytes_contains",
                        )
                        .unwrap();
                    let result = extract_int_result(call_site, "bytes_contains")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    let negated = cg.builder.build_not(bool_val, "not_in").unwrap();
                    Ok(PyValue::bool(negated.into()))
                }
                _ => Err(format!("Operator {:?} not supported for bytes type", op)),
            };
        }

        // Handle Bytes op Int case (repetition: b"ab" * 3 -> b"ababab")
        if let PyType::Int(_) = &rhs.ty {
            if let BinaryOp::Mul = op {
                let repeat_fn = get_or_declare_builtin(cg.module, cg.ctx, "strrepeat_bytes");
                let rhs_int = rhs.value.into_int_value();
                let call_site = cg
                    .builder
                    .build_call(repeat_fn, &[lhs_ptr.into(), rhs_int.into()], "bytes_repeat")
                    .unwrap();
                return Ok(PyValue::bytes(extract_ptr_result(
                    call_site,
                    "strrepeat_bytes",
                )?));
            }
        }

        Err(format!(
            "Unsupported binary operation {:?} between Bytes and {:?}",
            op, rhs.ty
        ))
    }

    fn unary_op(
        &self,
        _cg: &CgCtx<'a, 'ctx>,
        op: &UnaryOp,
        _val: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        Err(format!("Unary operator {:?} not supported on bytes", op))
    }

    fn print(
        &self,
        builder: &Builder<'ctx>,
        print_fn: FunctionValue<'ctx>,
        val: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let ptr_val = val.into_pointer_value();
        builder
            .build_call(print_fn, &[ptr_val.into()], "print_bytes")
            .unwrap();
        Ok(())
    }

    fn coerce_to(
        &self,
        _cg: &CgCtx<'a, 'ctx>,
        val: BasicValueEnum<'ctx>,
        target: &crate::ast::Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match target {
            crate::ast::Type::Bytes => Ok(val),
            _ => Err(format!("Cannot coerce Bytes to {:?}", target)),
        }
    }
}

// ============================================================================
// NoneType CodeGen Implementation
// ============================================================================

impl<'a, 'ctx> CodeGenOps<'a, 'ctx> for super::NoneType {
    fn binary_op_impl(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: &super::PyValue<'ctx>,
    ) -> Result<super::PyValue<'ctx>, String> {
        use super::{PyType, PyValue};

        // Only None op None is supported
        if !matches!(&rhs.ty, PyType::None(_)) {
            return Err(format!(
                "Unsupported binary operation {:?} between None and {:?}",
                op, rhs.ty
            ));
        }

        let lhs_ptr = lhs.into_pointer_value();
        let rhs_ptr = rhs.value.into_pointer_value();

        // Convert pointers to integers for comparison
        let ptr_int_type = cg.ctx.i64_type();
        let lhs_int = cg
            .builder
            .build_ptr_to_int(lhs_ptr, ptr_int_type, "ptr_to_int")
            .unwrap();
        let rhs_int = cg
            .builder
            .build_ptr_to_int(rhs_ptr, ptr_int_type, "ptr_to_int")
            .unwrap();

        match op {
            BinaryOp::Is | BinaryOp::Eq => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(IntPredicate::EQ, lhs_int, rhs_int, "is_none")
                    .unwrap()
                    .into(),
            )),
            BinaryOp::IsNot | BinaryOp::Ne => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(IntPredicate::NE, lhs_int, rhs_int, "isnot_none")
                    .unwrap()
                    .into(),
            )),
            _ => Err(format!("Operator {:?} not supported on None", op)),
        }
    }

    fn unary_op(
        &self,
        _cg: &CgCtx<'a, 'ctx>,
        op: &UnaryOp,
        _val: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        Err(format!("Unary operator {:?} not supported on None", op))
    }

    fn print(
        &self,
        builder: &Builder<'ctx>,
        print_fn: FunctionValue<'ctx>,
        _val: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        builder.build_call(print_fn, &[], "print_none").unwrap();
        Ok(())
    }

    fn coerce_to(
        &self,
        _cg: &CgCtx<'a, 'ctx>,
        val: BasicValueEnum<'ctx>,
        target: &crate::ast::Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match target {
            crate::ast::Type::None => Ok(val),
            _ => Err(format!("Cannot coerce None to {:?}", target)),
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Get or declare a builtin function using the auto-generated table
///
/// Note: This function does NOT track builtin module usage. For proper tracking,
/// use CodeGen::get_or_declare_builtin_function() instead when possible.
/// This function exists for type operations that don't have direct access to CodeGen.
fn get_or_declare_builtin<'ctx>(
    module: &Module<'ctx>,
    ctx: &'ctx Context,
    name: &str,
) -> FunctionValue<'ctx> {
    // Look up the builtin function in the generated table
    let builtin = BUILTIN_TABLE
        .get(name)
        .unwrap_or_else(|| panic!("Unknown builtin function: {}", name));

    // Check if already declared (use the symbol name, which is the actual C function name)
    if let Some(func) = module.get_function(builtin.symbol) {
        return func;
    }

    // Declare the function using the signature from the table
    let fn_type = builtin.to_llvm_fn_type(ctx);
    module.add_function(builtin.symbol, fn_type, None)
}

/// Extract int result from a call site
fn extract_int_result<'ctx>(
    call_site: inkwell::values::CallSiteValue<'ctx>,
    fn_name: &str,
) -> Result<BasicValueEnum<'ctx>, String> {
    use inkwell::values::AnyValue;
    let any_val = call_site.as_any_value_enum();
    if let inkwell::values::AnyValueEnum::IntValue(iv) = any_val {
        Ok(iv.into())
    } else {
        Err(format!("{} did not return an int value", fn_name))
    }
}

/// Extract float result from a call site
fn extract_float_result<'ctx>(
    call_site: inkwell::values::CallSiteValue<'ctx>,
    fn_name: &str,
) -> Result<BasicValueEnum<'ctx>, String> {
    use inkwell::values::AnyValue;
    let any_val = call_site.as_any_value_enum();
    if let inkwell::values::AnyValueEnum::FloatValue(fv) = any_val {
        Ok(fv.into())
    } else {
        Err(format!("{} did not return a float value", fn_name))
    }
}

/// Extract pointer result from a call site
fn extract_ptr_result<'ctx>(
    call_site: inkwell::values::CallSiteValue<'ctx>,
    fn_name: &str,
) -> Result<BasicValueEnum<'ctx>, String> {
    use inkwell::values::AnyValue;
    let any_val = call_site.as_any_value_enum();
    if let inkwell::values::AnyValueEnum::PointerValue(pv) = any_val {
        Ok(pv.into())
    } else {
        Err(format!("{} did not return a pointer value", fn_name))
    }
}

// ============================================================================
// PyType - Python semantic type for codegen
// ============================================================================

/// Python semantic type for codegen.
///
/// This represents the Python-level type, not the LLVM IR type.
/// Different PyTypes may map to the same IR type (e.g., List, Dict, bytes, None
/// all map to pointer in LLVM IR).
#[derive(Clone, Debug, PartialEq)]
pub enum PyType {
    Int(IntType),
    Float(FloatType),
    Bool(BoolType),
    Bytes(BytesType),
    None(NoneType),
}

impl PyType {
    /// Get the PyType for a given AST Type
    pub fn from_ast_type(ty: &Type) -> Result<Self, String> {
        match ty {
            Type::Int => Ok(PyType::Int(IntType)),
            Type::Float => Ok(PyType::Float(FloatType)),
            Type::Bool => Ok(PyType::Bool(BoolType)),
            Type::Bytes => Ok(PyType::Bytes(BytesType)),
            Type::None => Ok(PyType::None(NoneType)),
            Type::Str => Err("Str type not yet implemented (use Bytes)".to_string()),
            Type::List(_) => Err("List type not yet implemented".to_string()),
            Type::Dict(_, _) => Err("Dict type not yet implemented".to_string()),
            Type::Set(_) => Err("Set type not yet implemented".to_string()),
            Type::Tuple(_) => Err("Tuple type not yet implemented".to_string()),
            Type::Custom(name) => Err(format!("Custom type '{}' not yet implemented", name)),
        }
    }

    /// Dispatch binary operation to the appropriate type implementation (internal)
    fn binary_op_impl<'a, 'ctx>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match self {
            PyType::Int(t) => t.binary_op_impl(cg, op, lhs, rhs),
            PyType::Float(t) => t.binary_op_impl(cg, op, lhs, rhs),
            PyType::Bool(t) => t.binary_op_impl(cg, op, lhs, rhs),
            PyType::Bytes(t) => t.binary_op_impl(cg, op, lhs, rhs),
            PyType::None(t) => t.binary_op_impl(cg, op, lhs, rhs),
        }
    }

    /// Dispatch unary operation to the appropriate type implementation
    pub fn unary_op<'a, 'ctx>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &UnaryOp,
        val: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match self {
            PyType::Int(t) => t.unary_op(cg, op, val),
            PyType::Float(t) => t.unary_op(cg, op, val),
            PyType::Bool(t) => t.unary_op(cg, op, val),
            PyType::Bytes(t) => t.unary_op(cg, op, val),
            PyType::None(t) => t.unary_op(cg, op, val),
        }
    }

    /// Dispatch print to the appropriate type implementation
    pub fn print<'ctx>(
        &self,
        builder: &Builder<'ctx>,
        print_fn: FunctionValue<'ctx>,
        val: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        match self {
            PyType::Int(t) => t.print(builder, print_fn, val),
            PyType::Float(t) => t.print(builder, print_fn, val),
            PyType::Bool(t) => t.print(builder, print_fn, val),
            PyType::Bytes(t) => t.print(builder, print_fn, val),
            PyType::None(t) => t.print(builder, print_fn, val),
        }
    }

    /// Dispatch coercion to the appropriate type implementation
    pub fn coerce_to<'a, 'ctx>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        val: BasicValueEnum<'ctx>,
        target: &Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match self {
            PyType::Int(t) => t.coerce_to(cg, val, target),
            PyType::Float(t) => t.coerce_to(cg, val, target),
            PyType::Bool(t) => t.coerce_to(cg, val, target),
            PyType::Bytes(t) => t.coerce_to(cg, val, target),
            PyType::None(t) => t.coerce_to(cg, val, target),
        }
    }

    /// Get the print function name for this type
    pub fn print_function_name(&self) -> &'static str {
        match self {
            PyType::Int(_) => "print_int",
            PyType::Float(_) => "print_float",
            PyType::Bool(_) => "print_bool",
            PyType::Bytes(_) => "print_bytes",
            PyType::None(_) => "print_none",
        }
    }

    /// Convert to AST Type
    pub fn to_ast_type(&self) -> Type {
        match self {
            PyType::Int(_) => Type::Int,
            PyType::Float(_) => Type::Float,
            PyType::Bool(_) => Type::Bool,
            PyType::Bytes(_) => Type::Bytes,
            PyType::None(_) => Type::None,
        }
    }
}

// ============================================================================
// PyValue - Python value with its type information
// ============================================================================

/// A Python value paired with its type information.
/// This eliminates the need to infer types from LLVM IR values.
#[derive(Clone)]
pub struct PyValue<'ctx> {
    pub value: BasicValueEnum<'ctx>,
    pub ty: PyType,
}

impl<'ctx> PyValue<'ctx> {
    /// Create a new Python value
    pub fn new(value: BasicValueEnum<'ctx>, ty: PyType) -> Self {
        Self { value, ty }
    }

    /// Create a Python int value
    pub fn int(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Int(IntType))
    }

    /// Create a Python float value
    pub fn float(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Float(FloatType))
    }

    /// Create a Python bool value
    pub fn bool(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Bool(BoolType))
    }

    /// Create a Python bytes value
    pub fn bytes(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Bytes(BytesType))
    }

    /// Create a Python none value
    pub fn none(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::None(NoneType))
    }

    /// Check if this is an int type
    pub fn is_int(&self) -> bool {
        matches!(self.ty, PyType::Int(_))
    }

    /// Check if this is a float type
    pub fn is_float(&self) -> bool {
        matches!(self.ty, PyType::Float(_))
    }

    /// Check if this is a bool type
    pub fn is_bool(&self) -> bool {
        matches!(self.ty, PyType::Bool(_))
    }

    /// Check if this is a bytes type
    pub fn is_bytes(&self) -> bool {
        matches!(self.ty, PyType::Bytes(_))
    }

    /// Check if this is a none type
    pub fn is_none(&self) -> bool {
        matches!(self.ty, PyType::None(_))
    }

    /// Get the AST type for this value
    pub fn ast_type(&self) -> Type {
        self.ty.to_ast_type()
    }

    /// Perform a binary operation: self op rhs
    /// This is the main entry point for binary operations on PyValue.
    pub fn binary_op<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        self.ty.binary_op_impl(cg, op, self.value, rhs)
    }
}
