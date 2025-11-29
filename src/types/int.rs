//! Integer type implementation and codegen operations

use crate::ast::BinaryOp;
use crate::codegen::builtins::BUILTIN_TABLE;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue, IntValue};
use inkwell::IntPredicate;

use super::traits::{
    Addable, BitAndable, BitNegatable, BitOrable, BitXorable, Divisible, EqualityComparable,
    FloorDivisible, IdentityComparable, LeftShiftable, LogicalAndable, LogicalNegatable,
    LogicalOrable, Modulo, Multipliable, Negatable, OrderComparable, Powable, Printable,
    RightShiftable, Subtractable, ToBool, UnaryPlusable,
};
use super::value::{CgCtx, PyType, PyValue};

/// Wrapper for Python int values
pub struct PyInt<'ctx> {
    pub value: IntValue<'ctx>,
}

impl<'ctx> PyInt<'ctx> {
    pub fn new(value: IntValue<'ctx>) -> Self {
        Self { value }
    }

    pub fn from_basic(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value.into_int_value())
    }
}

impl<'ctx> Printable<'ctx> for PyInt<'ctx> {
    fn print(&self, builder: &Builder<'ctx>, print_fn: FunctionValue<'ctx>) -> Result<(), String> {
        builder
            .build_call(print_fn, &[self.value.into()], "print_int")
            .unwrap();
        Ok(())
    }

    fn print_function_name(&self) -> &'static str {
        "print_int"
    }
}

// ============================================================================
// Arithmetic Operations
// ============================================================================

impl<'ctx> Addable<'ctx> for PyInt<'ctx> {
    fn add<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_int_add(self.value, rhs.value.into_int_value(), "add")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "itof")
                    .unwrap();
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.add(cg, rhs)
            }
            _ => Err(format!("Cannot add Int and {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> Subtractable<'ctx> for PyInt<'ctx> {
    fn sub<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_int_sub(self.value, rhs.value.into_int_value(), "sub")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "itof")
                    .unwrap();
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.sub(cg, rhs)
            }
            _ => Err(format!("Cannot subtract {:?} from Int", rhs.ty)),
        }
    }
}

impl<'ctx> Multipliable<'ctx> for PyInt<'ctx> {
    fn mul<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_int_mul(self.value, rhs.value.into_int_value(), "mul")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "itof")
                    .unwrap();
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.mul(cg, rhs)
            }
            _ => Err(format!("Cannot multiply Int and {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> Divisible<'ctx> for PyInt<'ctx> {
    fn div<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        // Python 3 semantics: int / int -> float
        let lhs_float = cg
            .builder
            .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "lhs_itof")
            .unwrap();

        match &rhs.ty {
            PyType::Int => {
                let rhs_float = cg
                    .builder
                    .build_signed_int_to_float(
                        rhs.value.into_int_value(),
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
            PyType::Float => {
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.div(cg, rhs)
            }
            _ => Err(format!("Cannot divide Int by {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> FloorDivisible<'ctx> for PyInt<'ctx> {
    fn floordiv<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let floordiv_fn = get_or_declare_builtin(cg.module, cg.ctx, "floordiv_int");
                let call_site = cg
                    .builder
                    .build_call(
                        floordiv_fn,
                        &[self.value.into(), rhs.value.into()],
                        "floordiv",
                    )
                    .unwrap();
                Ok(PyValue::int(extract_int_result(call_site, "floordiv_int")?))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "itof")
                    .unwrap();
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.floordiv(cg, rhs)
            }
            _ => Err(format!("Cannot floor divide Int by {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> Modulo<'ctx> for PyInt<'ctx> {
    fn modulo<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let mod_fn = get_or_declare_builtin(cg.module, cg.ctx, "mod_int");
                let call_site = cg
                    .builder
                    .build_call(mod_fn, &[self.value.into(), rhs.value.into()], "mod")
                    .unwrap();
                Ok(PyValue::int(extract_int_result(call_site, "mod_int")?))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "itof")
                    .unwrap();
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.modulo(cg, rhs)
            }
            _ => Err(format!("Cannot compute Int modulo {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> Powable<'ctx> for PyInt<'ctx> {
    fn pow<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let pow_fn = get_or_declare_builtin(cg.module, cg.ctx, "pow_int");
                let call_site = cg
                    .builder
                    .build_call(pow_fn, &[self.value.into(), rhs.value.into()], "ipow")
                    .unwrap();
                Ok(PyValue::int(extract_int_result(call_site, "pow_int")?))
            }
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "itof")
                    .unwrap();
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.pow(cg, rhs)
            }
            _ => Err(format!("Cannot raise Int to power {:?}", rhs.ty)),
        }
    }
}

// ============================================================================
// Bitwise Operations
// ============================================================================

impl<'ctx> BitAndable<'ctx> for PyInt<'ctx> {
    fn bitand<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_and(self.value, rhs.value.into_int_value(), "bitand")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot bitwise AND Int and {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> BitOrable<'ctx> for PyInt<'ctx> {
    fn bitor<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_or(self.value, rhs.value.into_int_value(), "bitor")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot bitwise OR Int and {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> BitXorable<'ctx> for PyInt<'ctx> {
    fn bitxor<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_xor(self.value, rhs.value.into_int_value(), "bitxor")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot bitwise XOR Int and {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> LeftShiftable<'ctx> for PyInt<'ctx> {
    fn lshift<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_left_shift(self.value, rhs.value.into_int_value(), "lshift")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot left shift Int by {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> RightShiftable<'ctx> for PyInt<'ctx> {
    fn rshift<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_right_shift(self.value, rhs.value.into_int_value(), true, "rshift")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot right shift Int by {:?}", rhs.ty)),
        }
    }
}

// ============================================================================
// Comparison Operations
// ============================================================================

impl<'ctx> EqualityComparable<'ctx> for PyInt<'ctx> {
    fn eq<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        self.value,
                        rhs.value.into_int_value(),
                        "eq",
                    )
                    .unwrap()
                    .into(),
            )),
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "itof")
                    .unwrap();
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.eq(cg, rhs)
            }
            _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
        }
    }

    fn ne<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::NE,
                        self.value,
                        rhs.value.into_int_value(),
                        "ne",
                    )
                    .unwrap()
                    .into(),
            )),
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "itof")
                    .unwrap();
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.ne(cg, rhs)
            }
            _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> OrderComparable<'ctx> for PyInt<'ctx> {
    fn lt<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::SLT,
                        self.value,
                        rhs.value.into_int_value(),
                        "lt",
                    )
                    .unwrap()
                    .into(),
            )),
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "itof")
                    .unwrap();
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.lt(cg, rhs)
            }
            _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
        }
    }

    fn le<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::SLE,
                        self.value,
                        rhs.value.into_int_value(),
                        "le",
                    )
                    .unwrap()
                    .into(),
            )),
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "itof")
                    .unwrap();
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.le(cg, rhs)
            }
            _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
        }
    }

    fn gt<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::SGT,
                        self.value,
                        rhs.value.into_int_value(),
                        "gt",
                    )
                    .unwrap()
                    .into(),
            )),
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "itof")
                    .unwrap();
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.gt(cg, rhs)
            }
            _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
        }
    }

    fn ge<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::SGE,
                        self.value,
                        rhs.value.into_int_value(),
                        "ge",
                    )
                    .unwrap()
                    .into(),
            )),
            PyType::Float => {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(self.value, cg.ctx.f64_type(), "itof")
                    .unwrap();
                let py_float = super::float::PyFloat::new(lhs_float);
                py_float.ge(cg, rhs)
            }
            _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> IdentityComparable<'ctx> for PyInt<'ctx> {
    fn is<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        self.value,
                        rhs.value.into_int_value(),
                        "is",
                    )
                    .unwrap()
                    .into(),
            )),
            _ => Err(format!("Cannot use 'is' between Int and {:?}", rhs.ty)),
        }
    }

    fn is_not<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::NE,
                        self.value,
                        rhs.value.into_int_value(),
                        "isnot",
                    )
                    .unwrap()
                    .into(),
            )),
            _ => Err(format!("Cannot use 'is not' between Int and {:?}", rhs.ty)),
        }
    }
}

// ============================================================================
// Logical Operations
// ============================================================================

impl<'ctx> LogicalAndable<'ctx> for PyInt<'ctx> {
    fn logical_and<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_and(self.value, rhs.value.into_int_value(), "and")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot logical AND Int and {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> LogicalOrable<'ctx> for PyInt<'ctx> {
    fn logical_or<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let result = cg
                    .builder
                    .build_or(self.value, rhs.value.into_int_value(), "or")
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("Cannot logical OR Int and {:?}", rhs.ty)),
        }
    }
}

// ============================================================================
// Unary Operations
// ============================================================================

impl<'ctx> Negatable<'ctx> for PyInt<'ctx> {
    fn neg<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(cg.builder.build_int_neg(self.value, "neg").unwrap().into())
    }
}

impl<'ctx> UnaryPlusable<'ctx> for PyInt<'ctx> {
    fn pos<'a>(&self, _cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(self.value.into())
    }
}

impl<'ctx> LogicalNegatable<'ctx> for PyInt<'ctx> {
    fn logical_not<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(cg.builder.build_not(self.value, "not").unwrap().into())
    }
}

impl<'ctx> BitNegatable<'ctx> for PyInt<'ctx> {
    fn bitnot<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(cg.builder.build_not(self.value, "bitnot").unwrap().into())
    }
}

// ============================================================================
// Type Conversion
// ============================================================================

impl<'ctx> ToBool<'ctx> for PyInt<'ctx> {
    fn to_bool<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        let zero = cg.ctx.i64_type().const_zero();
        Ok(cg
            .builder
            .build_int_compare(IntPredicate::NE, self.value, zero, "int_to_bool")
            .unwrap()
            .into())
    }
}

// ============================================================================
// Dispatch function for PyValue
// ============================================================================

/// Dispatch binary operation for int type
pub fn int_binary_op<'a, 'ctx>(
    cg: &CgCtx<'a, 'ctx>,
    op: &BinaryOp,
    lhs: BasicValueEnum<'ctx>,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let py_int = PyInt::from_basic(lhs);
    match op {
        BinaryOp::Add => py_int.add(cg, rhs),
        BinaryOp::Sub => py_int.sub(cg, rhs),
        BinaryOp::Mul => py_int.mul(cg, rhs),
        BinaryOp::Div => py_int.div(cg, rhs),
        BinaryOp::FloorDiv => py_int.floordiv(cg, rhs),
        BinaryOp::Mod => py_int.modulo(cg, rhs),
        BinaryOp::Pow => py_int.pow(cg, rhs),
        BinaryOp::BitAnd => py_int.bitand(cg, rhs),
        BinaryOp::BitOr => py_int.bitor(cg, rhs),
        BinaryOp::BitXor => py_int.bitxor(cg, rhs),
        BinaryOp::LShift => py_int.lshift(cg, rhs),
        BinaryOp::RShift => py_int.rshift(cg, rhs),
        BinaryOp::Eq => py_int.eq(cg, rhs),
        BinaryOp::Ne => py_int.ne(cg, rhs),
        BinaryOp::Lt => py_int.lt(cg, rhs),
        BinaryOp::Le => py_int.le(cg, rhs),
        BinaryOp::Gt => py_int.gt(cg, rhs),
        BinaryOp::Ge => py_int.ge(cg, rhs),
        BinaryOp::Is => py_int.is(cg, rhs),
        BinaryOp::IsNot => py_int.is_not(cg, rhs),
        BinaryOp::And => py_int.logical_and(cg, rhs),
        BinaryOp::Or => py_int.logical_or(cg, rhs),
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
