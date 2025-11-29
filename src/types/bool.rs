//! Bool type implementation and codegen operations

use crate::ast::BinaryOp;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FunctionValue, IntValue};
use inkwell::IntPredicate;

use super::traits::{
    Addable, BitAndable, BitOrable, BitXorable, EqualityComparable, IdentityComparable,
    LogicalAndable, LogicalNegatable, LogicalOrable, Printable, ToBool,
};
use super::value::{CgCtx, PyType, PyValue};

/// Wrapper for Python bool values
pub struct PyBool<'ctx> {
    pub value: IntValue<'ctx>,
}

impl<'ctx> PyBool<'ctx> {
    pub fn new(value: IntValue<'ctx>) -> Self {
        Self { value }
    }

    pub fn from_basic(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value.into_int_value())
    }
}

impl<'ctx> Printable<'ctx> for PyBool<'ctx> {
    fn print(&self, builder: &Builder<'ctx>, print_fn: FunctionValue<'ctx>) -> Result<(), String> {
        builder
            .build_call(print_fn, &[self.value.into()], "print_bool")
            .unwrap();
        Ok(())
    }

    fn print_function_name(&self) -> &'static str {
        "print_bool"
    }
}

// ============================================================================
// Arithmetic Operations (coerce to int/float)
// ============================================================================

impl<'ctx> Addable<'ctx> for PyBool<'ctx> {
    fn add<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        // Coerce bool to int and delegate
        let lhs_int = cg
            .builder
            .build_int_z_extend(self.value, cg.ctx.i64_type(), "btoi")
            .unwrap();
        let py_int = super::int::PyInt::new(lhs_int);
        py_int.add(cg, rhs)
    }
}

// ============================================================================
// Bitwise Operations
// ============================================================================

impl<'ctx> BitAndable<'ctx> for PyBool<'ctx> {
    fn bitand<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bool => {
                let result = cg
                    .builder
                    .build_and(self.value, rhs.value.into_int_value(), "bitand")
                    .unwrap();
                Ok(PyValue::bool(result.into()))
            }
            _ => Err(format!("Cannot bitwise AND Bool and {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> BitOrable<'ctx> for PyBool<'ctx> {
    fn bitor<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bool => {
                let result = cg
                    .builder
                    .build_or(self.value, rhs.value.into_int_value(), "bitor")
                    .unwrap();
                Ok(PyValue::bool(result.into()))
            }
            _ => Err(format!("Cannot bitwise OR Bool and {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> BitXorable<'ctx> for PyBool<'ctx> {
    fn bitxor<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bool => {
                let result = cg
                    .builder
                    .build_xor(self.value, rhs.value.into_int_value(), "bitxor")
                    .unwrap();
                Ok(PyValue::bool(result.into()))
            }
            _ => Err(format!("Cannot bitwise XOR Bool and {:?}", rhs.ty)),
        }
    }
}

// ============================================================================
// Comparison Operations
// ============================================================================

impl<'ctx> EqualityComparable<'ctx> for PyBool<'ctx> {
    fn eq<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bool => Ok(PyValue::bool(
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
            _ => Err(format!("Cannot compare Bool with {:?}", rhs.ty)),
        }
    }

    fn ne<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bool => Ok(PyValue::bool(
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
            _ => Err(format!("Cannot compare Bool with {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> IdentityComparable<'ctx> for PyBool<'ctx> {
    fn is<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bool => Ok(PyValue::bool(
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
            _ => Err(format!("Cannot use 'is' between Bool and {:?}", rhs.ty)),
        }
    }

    fn is_not<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bool => Ok(PyValue::bool(
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
            _ => Err(format!("Cannot use 'is not' between Bool and {:?}", rhs.ty)),
        }
    }
}

// ============================================================================
// Logical Operations
// ============================================================================

impl<'ctx> LogicalAndable<'ctx> for PyBool<'ctx> {
    fn logical_and<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bool => {
                let result = cg
                    .builder
                    .build_and(self.value, rhs.value.into_int_value(), "and")
                    .unwrap();
                Ok(PyValue::bool(result.into()))
            }
            _ => Err(format!("Cannot logical AND Bool and {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> LogicalOrable<'ctx> for PyBool<'ctx> {
    fn logical_or<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bool => {
                let result = cg
                    .builder
                    .build_or(self.value, rhs.value.into_int_value(), "or")
                    .unwrap();
                Ok(PyValue::bool(result.into()))
            }
            _ => Err(format!("Cannot logical OR Bool and {:?}", rhs.ty)),
        }
    }
}

// ============================================================================
// Unary Operations
// ============================================================================

impl<'ctx> LogicalNegatable<'ctx> for PyBool<'ctx> {
    fn logical_not<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(cg.builder.build_not(self.value, "not").unwrap().into())
    }
}

// ============================================================================
// Type Conversion
// ============================================================================

impl<'ctx> ToBool<'ctx> for PyBool<'ctx> {
    fn to_bool<'a>(&self, _cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(self.value.into())
    }
}

// ============================================================================
// Dispatch function for PyValue
// ============================================================================

/// Dispatch binary operation for bool type
pub fn bool_binary_op<'a, 'ctx>(
    cg: &CgCtx<'a, 'ctx>,
    op: &BinaryOp,
    lhs: BasicValueEnum<'ctx>,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let py_bool = PyBool::from_basic(lhs);

    // For arithmetic operations, coerce to int first
    match op {
        BinaryOp::Add
        | BinaryOp::Sub
        | BinaryOp::Mul
        | BinaryOp::Div
        | BinaryOp::FloorDiv
        | BinaryOp::Mod
        | BinaryOp::Pow
        | BinaryOp::LShift
        | BinaryOp::RShift => {
            // Coerce bool to int and delegate
            let lhs_int = cg
                .builder
                .build_int_z_extend(py_bool.value, cg.ctx.i64_type(), "btoi")
                .unwrap();

            // If rhs is Float, coerce to float
            if let PyType::Float = &rhs.ty {
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                    .unwrap();
                return super::float::float_binary_op(cg, op, lhs_float.into(), rhs);
            }

            super::int::int_binary_op(cg, op, lhs_int.into(), rhs)
        }
        BinaryOp::BitAnd => py_bool.bitand(cg, rhs),
        BinaryOp::BitOr => py_bool.bitor(cg, rhs),
        BinaryOp::BitXor => py_bool.bitxor(cg, rhs),
        BinaryOp::Eq => py_bool.eq(cg, rhs),
        BinaryOp::Ne => py_bool.ne(cg, rhs),
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
            // Coerce to int for ordering comparisons
            let lhs_int = cg
                .builder
                .build_int_z_extend(py_bool.value, cg.ctx.i64_type(), "btoi")
                .unwrap();
            super::int::int_binary_op(cg, op, lhs_int.into(), rhs)
        }
        BinaryOp::Is => py_bool.is(cg, rhs),
        BinaryOp::IsNot => py_bool.is_not(cg, rhs),
        BinaryOp::And => py_bool.logical_and(cg, rhs),
        BinaryOp::Or => py_bool.logical_or(cg, rhs),
        BinaryOp::In | BinaryOp::NotIn => Err(format!(
            "Membership operator {:?} not supported on Bool",
            op
        )),
    }
}
