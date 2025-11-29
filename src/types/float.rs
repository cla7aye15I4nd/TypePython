//! Float type implementation and codegen operations

use crate::ast::BinaryOp;
use crate::codegen::builtins::BUILTIN_TABLE;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue};
use inkwell::FloatPredicate;

use super::traits::{
    Addable, Divisible, EqualityComparable, FloorDivisible, IdentityComparable, Modulo,
    Multipliable, Negatable, OrderComparable, Powable, Printable, Subtractable, ToBool,
    UnaryPlusable,
};
use super::value::{CgCtx, PyType, PyValue};

/// Wrapper for Python float values
pub struct PyFloat<'ctx> {
    pub value: FloatValue<'ctx>,
}

impl<'ctx> PyFloat<'ctx> {
    pub fn new(value: FloatValue<'ctx>) -> Self {
        Self { value }
    }

    pub fn from_basic(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value.into_float_value())
    }

    /// Helper to coerce rhs to float
    fn coerce_rhs_to_float<'a>(
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<FloatValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Float => Ok(rhs.value.into_float_value()),
            PyType::Int => Ok(cg
                .builder
                .build_signed_int_to_float(rhs.value.into_int_value(), cg.ctx.f64_type(), "itof")
                .unwrap()),
            _ => Err(format!("Cannot coerce {:?} to float", rhs.ty)),
        }
    }
}

impl<'ctx> Printable<'ctx> for PyFloat<'ctx> {
    fn print(&self, builder: &Builder<'ctx>, print_fn: FunctionValue<'ctx>) -> Result<(), String> {
        builder
            .build_call(print_fn, &[self.value.into()], "print_float")
            .unwrap();
        Ok(())
    }

    fn print_function_name(&self) -> &'static str {
        "print_float"
    }
}

// ============================================================================
// Arithmetic Operations
// ============================================================================

impl<'ctx> Addable<'ctx> for PyFloat<'ctx> {
    fn add<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        let result = cg
            .builder
            .build_float_add(self.value, rhs_float, "fadd")
            .unwrap();
        Ok(PyValue::float(result.into()))
    }
}

impl<'ctx> Subtractable<'ctx> for PyFloat<'ctx> {
    fn sub<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        let result = cg
            .builder
            .build_float_sub(self.value, rhs_float, "fsub")
            .unwrap();
        Ok(PyValue::float(result.into()))
    }
}

impl<'ctx> Multipliable<'ctx> for PyFloat<'ctx> {
    fn mul<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        let result = cg
            .builder
            .build_float_mul(self.value, rhs_float, "fmul")
            .unwrap();
        Ok(PyValue::float(result.into()))
    }
}

impl<'ctx> Divisible<'ctx> for PyFloat<'ctx> {
    fn div<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        let result = cg
            .builder
            .build_float_div(self.value, rhs_float, "fdiv")
            .unwrap();
        Ok(PyValue::float(result.into()))
    }
}

impl<'ctx> FloorDivisible<'ctx> for PyFloat<'ctx> {
    fn floordiv<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        let div_result = cg
            .builder
            .build_float_div(self.value, rhs_float, "fdiv")
            .unwrap();
        let floor_fn = get_or_declare_builtin(cg.module, cg.ctx, "floor_float");
        let call_site = cg
            .builder
            .build_call(floor_fn, &[div_result.into()], "floor")
            .unwrap();
        Ok(PyValue::float(extract_float_result(
            call_site,
            "floor_float",
        )?))
    }
}

impl<'ctx> Modulo<'ctx> for PyFloat<'ctx> {
    fn modulo<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        let fmod_fn = get_or_declare_builtin(cg.module, cg.ctx, "mod_float");
        let call_site = cg
            .builder
            .build_call(fmod_fn, &[self.value.into(), rhs_float.into()], "fmod")
            .unwrap();
        Ok(PyValue::float(extract_float_result(
            call_site,
            "mod_float",
        )?))
    }
}

impl<'ctx> Powable<'ctx> for PyFloat<'ctx> {
    fn pow<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        let pow_fn = get_or_declare_builtin(cg.module, cg.ctx, "pow_float");
        let call_site = cg
            .builder
            .build_call(pow_fn, &[self.value.into(), rhs_float.into()], "fpow")
            .unwrap();
        Ok(PyValue::float(extract_float_result(
            call_site,
            "pow_float",
        )?))
    }
}

// ============================================================================
// Comparison Operations
// ============================================================================

impl<'ctx> EqualityComparable<'ctx> for PyFloat<'ctx> {
    fn eq<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        Ok(PyValue::bool(
            cg.builder
                .build_float_compare(FloatPredicate::OEQ, self.value, rhs_float, "feq")
                .unwrap()
                .into(),
        ))
    }

    fn ne<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        Ok(PyValue::bool(
            cg.builder
                .build_float_compare(FloatPredicate::ONE, self.value, rhs_float, "fne")
                .unwrap()
                .into(),
        ))
    }
}

impl<'ctx> OrderComparable<'ctx> for PyFloat<'ctx> {
    fn lt<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        Ok(PyValue::bool(
            cg.builder
                .build_float_compare(FloatPredicate::OLT, self.value, rhs_float, "flt")
                .unwrap()
                .into(),
        ))
    }

    fn le<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        Ok(PyValue::bool(
            cg.builder
                .build_float_compare(FloatPredicate::OLE, self.value, rhs_float, "fle")
                .unwrap()
                .into(),
        ))
    }

    fn gt<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        Ok(PyValue::bool(
            cg.builder
                .build_float_compare(FloatPredicate::OGT, self.value, rhs_float, "fgt")
                .unwrap()
                .into(),
        ))
    }

    fn ge<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        Ok(PyValue::bool(
            cg.builder
                .build_float_compare(FloatPredicate::OGE, self.value, rhs_float, "fge")
                .unwrap()
                .into(),
        ))
    }
}

impl<'ctx> IdentityComparable<'ctx> for PyFloat<'ctx> {
    fn is<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        Ok(PyValue::bool(
            cg.builder
                .build_float_compare(FloatPredicate::OEQ, self.value, rhs_float, "is")
                .unwrap()
                .into(),
        ))
    }

    fn is_not<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let rhs_float = Self::coerce_rhs_to_float(cg, rhs)?;
        Ok(PyValue::bool(
            cg.builder
                .build_float_compare(FloatPredicate::ONE, self.value, rhs_float, "isnot")
                .unwrap()
                .into(),
        ))
    }
}

// ============================================================================
// Unary Operations
// ============================================================================

impl<'ctx> Negatable<'ctx> for PyFloat<'ctx> {
    fn neg<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(cg
            .builder
            .build_float_neg(self.value, "fneg")
            .unwrap()
            .into())
    }
}

impl<'ctx> UnaryPlusable<'ctx> for PyFloat<'ctx> {
    fn pos<'a>(&self, _cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(self.value.into())
    }
}

// ============================================================================
// Type Conversion
// ============================================================================

impl<'ctx> ToBool<'ctx> for PyFloat<'ctx> {
    fn to_bool<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        let zero = cg.ctx.f64_type().const_zero();
        Ok(cg
            .builder
            .build_float_compare(FloatPredicate::ONE, self.value, zero, "float_to_bool")
            .unwrap()
            .into())
    }
}

// ============================================================================
// Dispatch function for PyValue
// ============================================================================

/// Dispatch binary operation for float type
pub fn float_binary_op<'a, 'ctx>(
    cg: &CgCtx<'a, 'ctx>,
    op: &BinaryOp,
    lhs: BasicValueEnum<'ctx>,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let py_float = PyFloat::from_basic(lhs);
    match op {
        BinaryOp::Add => py_float.add(cg, rhs),
        BinaryOp::Sub => py_float.sub(cg, rhs),
        BinaryOp::Mul => py_float.mul(cg, rhs),
        BinaryOp::Div => py_float.div(cg, rhs),
        BinaryOp::FloorDiv => py_float.floordiv(cg, rhs),
        BinaryOp::Mod => py_float.modulo(cg, rhs),
        BinaryOp::Pow => py_float.pow(cg, rhs),
        BinaryOp::Eq => py_float.eq(cg, rhs),
        BinaryOp::Ne => py_float.ne(cg, rhs),
        BinaryOp::Lt => py_float.lt(cg, rhs),
        BinaryOp::Le => py_float.le(cg, rhs),
        BinaryOp::Gt => py_float.gt(cg, rhs),
        BinaryOp::Ge => py_float.ge(cg, rhs),
        BinaryOp::Is => py_float.is(cg, rhs),
        BinaryOp::IsNot => py_float.is_not(cg, rhs),
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

// ============================================================================
// Helper Functions
// ============================================================================

fn get_or_declare_builtin<'ctx>(
    module: &Module<'ctx>,
    ctx: &'ctx Context,
    name: &str,
) -> FunctionValue<'ctx> {
    let builtin = BUILTIN_TABLE
        .get(name)
        .unwrap_or_else(|| panic!("Unknown builtin function: {}", name));

    if let Some(func) = module.get_function(builtin.symbol) {
        return func;
    }

    let fn_type = builtin.to_llvm_fn_type(ctx);
    module.add_function(builtin.symbol, fn_type, None)
}

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
