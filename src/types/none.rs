//! None type implementation and codegen operations

use crate::ast::BinaryOp;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FunctionValue, IntValue};
use inkwell::IntPredicate;

use super::traits::{EqualityComparable, IdentityComparable, Printable, ToBool};
use super::value::{CgCtx, PyType, PyValue};

/// Wrapper for Python None values
pub struct PyNone<'ctx> {
    pub value: IntValue<'ctx>,
}

impl<'ctx> PyNone<'ctx> {
    pub fn new(value: IntValue<'ctx>) -> Self {
        Self { value }
    }

    pub fn from_basic(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value.into_int_value())
    }
}

impl<'ctx> Printable<'ctx> for PyNone<'ctx> {
    fn print(&self, builder: &Builder<'ctx>, print_fn: FunctionValue<'ctx>) -> Result<(), String> {
        builder.build_call(print_fn, &[], "print_none").unwrap();
        Ok(())
    }

    fn print_function_name(&self) -> &'static str {
        "print_none"
    }
}

// ============================================================================
// Comparison Operations
// ============================================================================

impl<'ctx> EqualityComparable<'ctx> for PyNone<'ctx> {
    fn eq<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::None => Ok(PyValue::bool(
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
            _ => Err(format!("Cannot compare None with {:?}", rhs.ty)),
        }
    }

    fn ne<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::None => Ok(PyValue::bool(
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
            _ => Err(format!("Cannot compare None with {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> IdentityComparable<'ctx> for PyNone<'ctx> {
    fn is<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::None => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::EQ,
                        self.value,
                        rhs.value.into_int_value(),
                        "is_none",
                    )
                    .unwrap()
                    .into(),
            )),
            _ => Err(format!("Cannot use 'is' between None and {:?}", rhs.ty)),
        }
    }

    fn is_not<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::None => Ok(PyValue::bool(
                cg.builder
                    .build_int_compare(
                        IntPredicate::NE,
                        self.value,
                        rhs.value.into_int_value(),
                        "isnot_none",
                    )
                    .unwrap()
                    .into(),
            )),
            _ => Err(format!("Cannot use 'is not' between None and {:?}", rhs.ty)),
        }
    }
}

// ============================================================================
// Type Conversion
// ============================================================================

impl<'ctx> ToBool<'ctx> for PyNone<'ctx> {
    fn to_bool<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        // None is always falsy
        Ok(cg.ctx.bool_type().const_zero().into())
    }
}

// ============================================================================
// Dispatch function for PyValue
// ============================================================================

/// Dispatch binary operation for none type
pub fn none_binary_op<'a, 'ctx>(
    cg: &CgCtx<'a, 'ctx>,
    op: &BinaryOp,
    lhs: BasicValueEnum<'ctx>,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let py_none = PyNone::from_basic(lhs);
    match op {
        BinaryOp::Eq => py_none.eq(cg, rhs),
        BinaryOp::Ne => py_none.ne(cg, rhs),
        BinaryOp::Is => py_none.is(cg, rhs),
        BinaryOp::IsNot => py_none.is_not(cg, rhs),
        _ => Err(format!("Operator {:?} not supported on None", op)),
    }
}
