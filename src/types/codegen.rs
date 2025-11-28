//! Code generation operations for types
//!
//! This module provides the `CodeGenOps` trait that each type implements
//! to generate LLVM instructions for operations.

use crate::ast::{BinaryOp, Type, UnaryOp};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue};
use inkwell::{FloatPredicate, IntPredicate};

use super::{BoolType, BytesType, FloatType, IntType, NoneType};

/// Trait for type-specific LLVM code generation.
///
/// Each type implements this to generate its own instructions for operations,
/// eliminating the need for if/else chains in the main codegen.
pub trait CodeGenOps<'ctx> {
    /// Generate code for a binary operation where both operands are this type.
    /// Returns the result value.
    fn binary_op(
        &self,
        ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        module: &Module<'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String>;

    /// Generate code for a unary operation on this type.
    fn unary_op(
        &self,
        ctx: &'ctx Context,
        builder: &Builder<'ctx>,
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
        ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        val: BasicValueEnum<'ctx>,
        target: &crate::ast::Type,
    ) -> Result<BasicValueEnum<'ctx>, String>;
}

// ============================================================================
// IntType CodeGen Implementation
// ============================================================================

impl<'ctx> CodeGenOps<'ctx> for super::IntType {
    fn binary_op(
        &self,
        ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        module: &Module<'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let lhs_int = lhs.into_int_value();
        let rhs_int = rhs.into_int_value();

        let result = match op {
            BinaryOp::Add => builder.build_int_add(lhs_int, rhs_int, "add").unwrap(),
            BinaryOp::Sub => builder.build_int_sub(lhs_int, rhs_int, "sub").unwrap(),
            BinaryOp::Mul => builder.build_int_mul(lhs_int, rhs_int, "mul").unwrap(),
            BinaryOp::Div => {
                return Ok(builder
                    .build_int_signed_div(lhs_int, rhs_int, "div")
                    .unwrap()
                    .into())
            }
            BinaryOp::FloorDiv => {
                // Call tpy_floordiv_int for Python-style floor division
                let floordiv_fn = get_or_declare_builtin(module, ctx, "tpy_floordiv_int");
                let call_site = builder
                    .build_call(floordiv_fn, &[lhs_int.into(), rhs_int.into()], "floordiv")
                    .unwrap();
                return extract_int_result(call_site, "tpy_floordiv_int");
            }
            BinaryOp::Mod => {
                // Call tpy_mod_int for Python-style modulo
                let mod_fn = get_or_declare_builtin(module, ctx, "tpy_mod_int");
                let call_site = builder
                    .build_call(mod_fn, &[lhs_int.into(), rhs_int.into()], "mod")
                    .unwrap();
                return extract_int_result(call_site, "tpy_mod_int");
            }
            BinaryOp::Pow => {
                // Call tpy_pow_int builtin
                let pow_fn = get_or_declare_builtin(module, ctx, "tpy_pow_int");
                let call_site = builder
                    .build_call(pow_fn, &[lhs_int.into(), rhs_int.into()], "ipow")
                    .unwrap();
                return extract_int_result(call_site, "tpy_pow_int");
            }
            BinaryOp::Eq => {
                return Ok(builder
                    .build_int_compare(IntPredicate::EQ, lhs_int, rhs_int, "eq")
                    .unwrap()
                    .into())
            }
            BinaryOp::Ne => {
                return Ok(builder
                    .build_int_compare(IntPredicate::NE, lhs_int, rhs_int, "ne")
                    .unwrap()
                    .into())
            }
            BinaryOp::Lt => {
                return Ok(builder
                    .build_int_compare(IntPredicate::SLT, lhs_int, rhs_int, "lt")
                    .unwrap()
                    .into())
            }
            BinaryOp::Le => {
                return Ok(builder
                    .build_int_compare(IntPredicate::SLE, lhs_int, rhs_int, "le")
                    .unwrap()
                    .into())
            }
            BinaryOp::Gt => {
                return Ok(builder
                    .build_int_compare(IntPredicate::SGT, lhs_int, rhs_int, "gt")
                    .unwrap()
                    .into())
            }
            BinaryOp::Ge => {
                return Ok(builder
                    .build_int_compare(IntPredicate::SGE, lhs_int, rhs_int, "ge")
                    .unwrap()
                    .into())
            }
            BinaryOp::And => builder.build_and(lhs_int, rhs_int, "and").unwrap(),
            BinaryOp::Or => builder.build_or(lhs_int, rhs_int, "or").unwrap(),
            BinaryOp::BitOr => builder.build_or(lhs_int, rhs_int, "bitor").unwrap(),
            BinaryOp::BitXor => builder.build_xor(lhs_int, rhs_int, "bitxor").unwrap(),
            BinaryOp::BitAnd => builder.build_and(lhs_int, rhs_int, "bitand").unwrap(),
            BinaryOp::LShift => builder
                .build_left_shift(lhs_int, rhs_int, "lshift")
                .unwrap(),
            BinaryOp::RShift => builder
                .build_right_shift(lhs_int, rhs_int, true, "rshift")
                .unwrap(),
            BinaryOp::Is => {
                return Ok(builder
                    .build_int_compare(IntPredicate::EQ, lhs_int, rhs_int, "is")
                    .unwrap()
                    .into())
            }
            BinaryOp::IsNot => {
                return Ok(builder
                    .build_int_compare(IntPredicate::NE, lhs_int, rhs_int, "isnot")
                    .unwrap()
                    .into())
            }
            BinaryOp::In | BinaryOp::NotIn => {
                return Err(format!(
                    "Membership operator {:?} requires container support",
                    op
                ))
            }
        };
        Ok(result.into())
    }

    fn unary_op(
        &self,
        _ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        op: &UnaryOp,
        val: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let int_val = val.into_int_value();
        match op {
            UnaryOp::Neg => Ok(builder.build_int_neg(int_val, "neg").unwrap().into()),
            UnaryOp::Pos => Ok(val), // no-op
            UnaryOp::Not => Ok(builder.build_not(int_val, "not").unwrap().into()),
            UnaryOp::BitNot => Ok(builder.build_not(int_val, "bitnot").unwrap().into()),
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
        ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        val: BasicValueEnum<'ctx>,
        target: &crate::ast::Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match target {
            crate::ast::Type::Int => Ok(val),
            crate::ast::Type::Float => {
                let int_val = val.into_int_value();
                Ok(builder
                    .build_signed_int_to_float(int_val, ctx.f64_type(), "itof")
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

impl<'ctx> CodeGenOps<'ctx> for super::FloatType {
    fn binary_op(
        &self,
        ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        module: &Module<'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let lhs_float = lhs.into_float_value();
        let rhs_float = rhs.into_float_value();

        let result = match op {
            BinaryOp::Add => builder
                .build_float_add(lhs_float, rhs_float, "fadd")
                .unwrap(),
            BinaryOp::Sub => builder
                .build_float_sub(lhs_float, rhs_float, "fsub")
                .unwrap(),
            BinaryOp::Mul => builder
                .build_float_mul(lhs_float, rhs_float, "fmul")
                .unwrap(),
            BinaryOp::Div => builder
                .build_float_div(lhs_float, rhs_float, "fdiv")
                .unwrap(),
            BinaryOp::Mod => {
                // Call tpy_fmod for Python-style float modulo
                let fmod_fn = get_or_declare_builtin(module, ctx, "tpy_fmod");
                let call_site = builder
                    .build_call(fmod_fn, &[lhs_float.into(), rhs_float.into()], "fmod")
                    .unwrap();
                return extract_float_result(call_site, "tpy_fmod");
            }
            BinaryOp::Pow => {
                let pow_fn = get_or_declare_builtin(module, ctx, "tpy_pow");
                let call_site = builder
                    .build_call(pow_fn, &[lhs_float.into(), rhs_float.into()], "fpow")
                    .unwrap();
                return extract_float_result(call_site, "tpy_pow");
            }
            BinaryOp::FloorDiv => {
                let div_result = builder
                    .build_float_div(lhs_float, rhs_float, "fdiv")
                    .unwrap();
                let floor_fn = get_or_declare_builtin(module, ctx, "tpy_floor");
                let call_site = builder
                    .build_call(floor_fn, &[div_result.into()], "floor")
                    .unwrap();
                return extract_float_result(call_site, "tpy_floor");
            }
            BinaryOp::Eq => {
                return Ok(builder
                    .build_float_compare(FloatPredicate::OEQ, lhs_float, rhs_float, "feq")
                    .unwrap()
                    .into())
            }
            BinaryOp::Ne => {
                return Ok(builder
                    .build_float_compare(FloatPredicate::ONE, lhs_float, rhs_float, "fne")
                    .unwrap()
                    .into())
            }
            BinaryOp::Lt => {
                return Ok(builder
                    .build_float_compare(FloatPredicate::OLT, lhs_float, rhs_float, "flt")
                    .unwrap()
                    .into())
            }
            BinaryOp::Le => {
                return Ok(builder
                    .build_float_compare(FloatPredicate::OLE, lhs_float, rhs_float, "fle")
                    .unwrap()
                    .into())
            }
            BinaryOp::Gt => {
                return Ok(builder
                    .build_float_compare(FloatPredicate::OGT, lhs_float, rhs_float, "fgt")
                    .unwrap()
                    .into())
            }
            BinaryOp::Ge => {
                return Ok(builder
                    .build_float_compare(FloatPredicate::OGE, lhs_float, rhs_float, "fge")
                    .unwrap()
                    .into())
            }
            BinaryOp::Is => {
                return Ok(builder
                    .build_float_compare(FloatPredicate::OEQ, lhs_float, rhs_float, "is")
                    .unwrap()
                    .into())
            }
            BinaryOp::IsNot => {
                return Ok(builder
                    .build_float_compare(FloatPredicate::ONE, lhs_float, rhs_float, "isnot")
                    .unwrap()
                    .into())
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
        Ok(result.into())
    }

    fn unary_op(
        &self,
        _ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        op: &UnaryOp,
        val: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let float_val = val.into_float_value();
        match op {
            UnaryOp::Neg => Ok(builder.build_float_neg(float_val, "fneg").unwrap().into()),
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
        _ctx: &'ctx Context,
        _builder: &Builder<'ctx>,
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

impl<'ctx> CodeGenOps<'ctx> for super::BoolType {
    fn binary_op(
        &self,
        _ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        _module: &Module<'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let lhs_bool = lhs.into_int_value();
        let rhs_bool = rhs.into_int_value();

        match op {
            BinaryOp::And => Ok(builder.build_and(lhs_bool, rhs_bool, "and").unwrap().into()),
            BinaryOp::Or => Ok(builder.build_or(lhs_bool, rhs_bool, "or").unwrap().into()),
            BinaryOp::Eq => Ok(builder
                .build_int_compare(IntPredicate::EQ, lhs_bool, rhs_bool, "eq")
                .unwrap()
                .into()),
            BinaryOp::Ne => Ok(builder
                .build_int_compare(IntPredicate::NE, lhs_bool, rhs_bool, "ne")
                .unwrap()
                .into()),
            BinaryOp::BitOr => Ok(builder
                .build_or(lhs_bool, rhs_bool, "bitor")
                .unwrap()
                .into()),
            BinaryOp::BitXor => Ok(builder
                .build_xor(lhs_bool, rhs_bool, "bitxor")
                .unwrap()
                .into()),
            BinaryOp::BitAnd => Ok(builder
                .build_and(lhs_bool, rhs_bool, "bitand")
                .unwrap()
                .into()),
            BinaryOp::Is => Ok(builder
                .build_int_compare(IntPredicate::EQ, lhs_bool, rhs_bool, "is")
                .unwrap()
                .into()),
            BinaryOp::IsNot => Ok(builder
                .build_int_compare(IntPredicate::NE, lhs_bool, rhs_bool, "isnot")
                .unwrap()
                .into()),
            _ => Err(format!("Operator {:?} not supported on bools directly", op)),
        }
    }

    fn unary_op(
        &self,
        _ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        op: &UnaryOp,
        val: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let bool_val = val.into_int_value();
        match op {
            UnaryOp::Not => Ok(builder.build_not(bool_val, "not").unwrap().into()),
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
        ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        val: BasicValueEnum<'ctx>,
        target: &crate::ast::Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let bool_val = val.into_int_value();
        match target {
            crate::ast::Type::Bool => Ok(val),
            crate::ast::Type::Int => {
                // Zero-extend bool to i64
                Ok(builder
                    .build_int_z_extend(bool_val, ctx.i64_type(), "btoi")
                    .unwrap()
                    .into())
            }
            crate::ast::Type::Float => {
                // First extend to i64, then convert to float
                let int_val = builder
                    .build_int_z_extend(bool_val, ctx.i64_type(), "btoi")
                    .unwrap();
                Ok(builder
                    .build_signed_int_to_float(int_val, ctx.f64_type(), "itof")
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

impl<'ctx> CodeGenOps<'ctx> for super::BytesType {
    fn binary_op(
        &self,
        ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        module: &Module<'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let lhs_ptr = lhs.into_pointer_value();
        let rhs_ptr = rhs.into_pointer_value();

        match op {
            BinaryOp::Add => {
                // Bytes concatenation
                let strcat_fn = get_or_declare_builtin(module, ctx, "tpy_strcat");
                let call_site = builder
                    .build_call(strcat_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytescat")
                    .unwrap();
                extract_ptr_result(call_site, "tpy_strcat")
            }
            BinaryOp::Eq => {
                let strcmp_fn = get_or_declare_builtin(module, ctx, "tpy_strcmp");
                let call_site = builder
                    .build_call(strcmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytescmp")
                    .unwrap();
                let result = extract_int_result(call_site, "tpy_strcmp")?;
                // Convert i64 to i1 (boolean) by truncating
                let bool_val = builder
                    .build_int_truncate(result.into_int_value(), ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(bool_val.into())
            }
            BinaryOp::Ne => {
                let strcmp_fn = get_or_declare_builtin(module, ctx, "tpy_strcmp");
                let call_site = builder
                    .build_call(strcmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytescmp")
                    .unwrap();
                let result = extract_int_result(call_site, "tpy_strcmp")?;
                let bool_val = builder
                    .build_int_truncate(result.into_int_value(), ctx.bool_type(), "to_bool")
                    .unwrap();
                let negated = builder.build_not(bool_val, "ne").unwrap();
                Ok(negated.into())
            }
            _ => Err(format!("Operator {:?} not supported for bytes type", op)),
        }
    }

    fn unary_op(
        &self,
        _ctx: &'ctx Context,
        _builder: &Builder<'ctx>,
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
        _ctx: &'ctx Context,
        _builder: &Builder<'ctx>,
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

impl<'ctx> CodeGenOps<'ctx> for super::NoneType {
    fn binary_op(
        &self,
        ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        _module: &Module<'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let lhs_ptr = lhs.into_pointer_value();
        let rhs_ptr = rhs.into_pointer_value();

        // Convert pointers to integers for comparison
        let ptr_int_type = ctx.i64_type();
        let lhs_int = builder
            .build_ptr_to_int(lhs_ptr, ptr_int_type, "ptr_to_int")
            .unwrap();
        let rhs_int = builder
            .build_ptr_to_int(rhs_ptr, ptr_int_type, "ptr_to_int")
            .unwrap();

        match op {
            BinaryOp::Is | BinaryOp::Eq => Ok(builder
                .build_int_compare(IntPredicate::EQ, lhs_int, rhs_int, "is_none")
                .unwrap()
                .into()),
            BinaryOp::IsNot | BinaryOp::Ne => Ok(builder
                .build_int_compare(IntPredicate::NE, lhs_int, rhs_int, "isnot_none")
                .unwrap()
                .into()),
            _ => Err(format!("Operator {:?} not supported on None", op)),
        }
    }

    fn unary_op(
        &self,
        _ctx: &'ctx Context,
        _builder: &Builder<'ctx>,
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
        _ctx: &'ctx Context,
        _builder: &Builder<'ctx>,
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

/// Get or declare a builtin function
fn get_or_declare_builtin<'ctx>(
    module: &Module<'ctx>,
    ctx: &'ctx Context,
    name: &str,
) -> FunctionValue<'ctx> {
    if let Some(func) = module.get_function(name) {
        return func;
    }

    let i64_type = ctx.i64_type();
    let f64_type = ctx.f64_type();
    let str_type = ctx.ptr_type(inkwell::AddressSpace::default());

    match name {
        "tpy_pow" => {
            let fn_type = f64_type.fn_type(&[f64_type.into(), f64_type.into()], false);
            module.add_function(name, fn_type, None)
        }
        "tpy_pow_int" => {
            let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);
            module.add_function(name, fn_type, None)
        }
        "tpy_floor" => {
            let fn_type = f64_type.fn_type(&[f64_type.into()], false);
            module.add_function(name, fn_type, None)
        }
        "tpy_floordiv_int" => {
            let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);
            module.add_function(name, fn_type, None)
        }
        "tpy_mod_int" => {
            let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);
            module.add_function(name, fn_type, None)
        }
        "tpy_fmod" => {
            let fn_type = f64_type.fn_type(&[f64_type.into(), f64_type.into()], false);
            module.add_function(name, fn_type, None)
        }
        "tpy_strcat" => {
            let fn_type = str_type.fn_type(&[str_type.into(), str_type.into()], false);
            module.add_function(name, fn_type, None)
        }
        "tpy_strcmp" => {
            let fn_type = i64_type.fn_type(&[str_type.into(), str_type.into()], false);
            module.add_function(name, fn_type, None)
        }
        _ => panic!("Unknown builtin function: {}", name),
    }
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
// Type Dispatch
// ============================================================================

/// Dispatch enum that wraps type-specific CodeGenOps implementations.
/// This allows us to return a concrete type from get_codegen_ops().
pub enum TypeCodeGen {
    Int(IntType),
    Float(FloatType),
    Bool(BoolType),
    Bytes(BytesType),
    None(NoneType),
}

impl TypeCodeGen {
    /// Get the CodeGenOps implementation for a given AST Type
    pub fn from_ast_type(ty: &Type) -> Result<Self, String> {
        match ty {
            Type::Int => Ok(TypeCodeGen::Int(IntType)),
            Type::Float => Ok(TypeCodeGen::Float(FloatType)),
            Type::Bool => Ok(TypeCodeGen::Bool(BoolType)),
            Type::Bytes => Ok(TypeCodeGen::Bytes(BytesType)),
            Type::None => Ok(TypeCodeGen::None(NoneType)),
            Type::Str => Err("Str type not yet implemented (use Bytes)".to_string()),
            Type::List(_) => Err("List type not yet implemented".to_string()),
            Type::Dict(_, _) => Err("Dict type not yet implemented".to_string()),
            Type::Set(_) => Err("Set type not yet implemented".to_string()),
            Type::Tuple(_) => Err("Tuple type not yet implemented".to_string()),
            Type::Custom(name) => Err(format!("Custom type '{}' not yet implemented", name)),
        }
    }

    /// Dispatch binary operation to the appropriate type implementation
    pub fn binary_op<'ctx>(
        &self,
        ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        module: &Module<'ctx>,
        op: &BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match self {
            TypeCodeGen::Int(t) => t.binary_op(ctx, builder, module, op, lhs, rhs),
            TypeCodeGen::Float(t) => t.binary_op(ctx, builder, module, op, lhs, rhs),
            TypeCodeGen::Bool(t) => t.binary_op(ctx, builder, module, op, lhs, rhs),
            TypeCodeGen::Bytes(t) => t.binary_op(ctx, builder, module, op, lhs, rhs),
            TypeCodeGen::None(t) => t.binary_op(ctx, builder, module, op, lhs, rhs),
        }
    }

    /// Dispatch unary operation to the appropriate type implementation
    pub fn unary_op<'ctx>(
        &self,
        ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        op: &UnaryOp,
        val: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match self {
            TypeCodeGen::Int(t) => t.unary_op(ctx, builder, op, val),
            TypeCodeGen::Float(t) => t.unary_op(ctx, builder, op, val),
            TypeCodeGen::Bool(t) => t.unary_op(ctx, builder, op, val),
            TypeCodeGen::Bytes(t) => t.unary_op(ctx, builder, op, val),
            TypeCodeGen::None(t) => t.unary_op(ctx, builder, op, val),
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
            TypeCodeGen::Int(t) => t.print(builder, print_fn, val),
            TypeCodeGen::Float(t) => t.print(builder, print_fn, val),
            TypeCodeGen::Bool(t) => t.print(builder, print_fn, val),
            TypeCodeGen::Bytes(t) => t.print(builder, print_fn, val),
            TypeCodeGen::None(t) => t.print(builder, print_fn, val),
        }
    }

    /// Dispatch coercion to the appropriate type implementation
    pub fn coerce_to<'ctx>(
        &self,
        ctx: &'ctx Context,
        builder: &Builder<'ctx>,
        val: BasicValueEnum<'ctx>,
        target: &Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match self {
            TypeCodeGen::Int(t) => t.coerce_to(ctx, builder, val, target),
            TypeCodeGen::Float(t) => t.coerce_to(ctx, builder, val, target),
            TypeCodeGen::Bool(t) => t.coerce_to(ctx, builder, val, target),
            TypeCodeGen::Bytes(t) => t.coerce_to(ctx, builder, val, target),
            TypeCodeGen::None(t) => t.coerce_to(ctx, builder, val, target),
        }
    }

    /// Get the print function name for this type
    pub fn print_function_name(&self) -> &'static str {
        match self {
            TypeCodeGen::Int(_) => "tpy_print_int",
            TypeCodeGen::Float(_) => "tpy_print_float",
            TypeCodeGen::Bool(_) => "tpy_print_bool",
            TypeCodeGen::Bytes(_) => "tpy_print_str",
            TypeCodeGen::None(_) => "tpy_print_none",
        }
    }
}

/// Infer the type from an LLVM value at runtime.
/// This is used when we don't have static type information.
pub fn infer_type_from_value(val: &BasicValueEnum) -> Result<TypeCodeGen, String> {
    if val.is_int_value() {
        let int_val = val.into_int_value();
        if int_val.get_type().get_bit_width() == 1 {
            Ok(TypeCodeGen::Bool(BoolType))
        } else {
            Ok(TypeCodeGen::Int(IntType))
        }
    } else if val.is_float_value() {
        Ok(TypeCodeGen::Float(FloatType))
    } else if val.is_pointer_value() {
        // Could be Bytes or None - default to Bytes for now
        Ok(TypeCodeGen::Bytes(BytesType))
    } else {
        Err("Cannot infer type from value".to_string())
    }
}
