//! Bytes type implementation and codegen operations

use crate::ast::BinaryOp;
use crate::codegen::builtins::BUILTIN_TABLE;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};

use super::traits::{
    Addable, EqualityComparable, MembershipTestable, Multipliable, OrderComparable, Printable,
    ToBool,
};
use super::value::{CgCtx, PyType, PyValue};

/// Wrapper for Python bytes values
pub struct PyBytes<'ctx> {
    pub value: PointerValue<'ctx>,
}

impl<'ctx> PyBytes<'ctx> {
    pub fn new(value: PointerValue<'ctx>) -> Self {
        Self { value }
    }

    pub fn from_basic(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value.into_pointer_value())
    }
}

impl<'ctx> Printable<'ctx> for PyBytes<'ctx> {
    fn print(&self, builder: &Builder<'ctx>, print_fn: FunctionValue<'ctx>) -> Result<(), String> {
        builder
            .build_call(print_fn, &[self.value.into()], "print_bytes")
            .unwrap();
        Ok(())
    }

    fn print_function_name(&self) -> &'static str {
        "print_bytes"
    }
}

// ============================================================================
// Concatenation (bytes + bytes)
// ============================================================================

impl<'ctx> Addable<'ctx> for PyBytes<'ctx> {
    fn add<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bytes => {
                let strcat_fn = get_or_declare_builtin(cg.module, cg.ctx, "strcat_bytes");
                let call_site = cg
                    .builder
                    .build_call(
                        strcat_fn,
                        &[self.value.into(), rhs.value.into()],
                        "bytescat",
                    )
                    .unwrap();
                Ok(PyValue::bytes(extract_ptr_result(
                    call_site,
                    "strcat_bytes",
                )?))
            }
            _ => Err(format!("Cannot concatenate Bytes and {:?}", rhs.ty)),
        }
    }
}

// ============================================================================
// Repetition (bytes * int)
// ============================================================================

impl<'ctx> Multipliable<'ctx> for PyBytes<'ctx> {
    fn mul<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Int => {
                let repeat_fn = get_or_declare_builtin(cg.module, cg.ctx, "strrepeat_bytes");
                let call_site = cg
                    .builder
                    .build_call(
                        repeat_fn,
                        &[self.value.into(), rhs.value.into()],
                        "bytes_repeat",
                    )
                    .unwrap();
                Ok(PyValue::bytes(extract_ptr_result(
                    call_site,
                    "strrepeat_bytes",
                )?))
            }
            _ => Err(format!("Cannot multiply Bytes by {:?}", rhs.ty)),
        }
    }
}

// ============================================================================
// Comparison Operations
// ============================================================================

impl<'ctx> EqualityComparable<'ctx> for PyBytes<'ctx> {
    fn eq<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bytes => {
                let strcmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "strcmp_bytes");
                let call_site = cg
                    .builder
                    .build_call(
                        strcmp_fn,
                        &[self.value.into(), rhs.value.into()],
                        "bytescmp",
                    )
                    .unwrap();
                let result = extract_int_result(call_site, "strcmp_bytes")?;
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
        }
    }

    fn ne<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bytes => {
                let strcmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "strcmp_bytes");
                let call_site = cg
                    .builder
                    .build_call(
                        strcmp_fn,
                        &[self.value.into(), rhs.value.into()],
                        "bytescmp",
                    )
                    .unwrap();
                let result = extract_int_result(call_site, "strcmp_bytes")?;
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                let negated = cg.builder.build_not(bool_val, "ne").unwrap();
                Ok(PyValue::bool(negated.into()))
            }
            _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
        }
    }
}

impl<'ctx> OrderComparable<'ctx> for PyBytes<'ctx> {
    fn lt<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bytes => {
                let cmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_lt");
                let call_site = cg
                    .builder
                    .build_call(cmp_fn, &[self.value.into(), rhs.value.into()], "bytes_lt")
                    .unwrap();
                let result = extract_int_result(call_site, "bytes_lt")?;
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
        }
    }

    fn le<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bytes => {
                let cmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_le");
                let call_site = cg
                    .builder
                    .build_call(cmp_fn, &[self.value.into(), rhs.value.into()], "bytes_le")
                    .unwrap();
                let result = extract_int_result(call_site, "bytes_le")?;
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
        }
    }

    fn gt<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bytes => {
                let cmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_gt");
                let call_site = cg
                    .builder
                    .build_call(cmp_fn, &[self.value.into(), rhs.value.into()], "bytes_gt")
                    .unwrap();
                let result = extract_int_result(call_site, "bytes_gt")?;
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
        }
    }

    fn ge<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String> {
        match &rhs.ty {
            PyType::Bytes => {
                let cmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_ge");
                let call_site = cg
                    .builder
                    .build_call(cmp_fn, &[self.value.into(), rhs.value.into()], "bytes_ge")
                    .unwrap();
                let result = extract_int_result(call_site, "bytes_ge")?;
                let bool_val = cg
                    .builder
                    .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                    .unwrap();
                Ok(PyValue::bool(bool_val.into()))
            }
            _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
        }
    }
}

// ============================================================================
// Membership Operations
// ============================================================================

impl<'ctx> MembershipTestable<'ctx> for PyBytes<'ctx> {
    fn contains<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        item: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &item.ty {
            PyType::Bytes => {
                // item in self -> bytes_contains(self, item)
                let contains_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_contains");
                let call_site = cg
                    .builder
                    .build_call(
                        contains_fn,
                        &[self.value.into(), item.value.into()],
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
            _ => Err(format!("Cannot test membership of {:?} in Bytes", item.ty)),
        }
    }
}

// ============================================================================
// Type Conversion
// ============================================================================

impl<'ctx> ToBool<'ctx> for PyBytes<'ctx> {
    fn to_bool<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        let strlen_fn = get_or_declare_builtin(cg.module, cg.ctx, "strlen_bytes");
        let call_site = cg
            .builder
            .build_call(strlen_fn, &[self.value.into()], "strlen")
            .unwrap();
        let len = extract_int_result(call_site, "strlen_bytes")?;
        let zero = cg.ctx.i64_type().const_zero();
        Ok(cg
            .builder
            .build_int_compare(
                inkwell::IntPredicate::NE,
                len.into_int_value(),
                zero,
                "bytes_to_bool",
            )
            .unwrap()
            .into())
    }
}

// ============================================================================
// Dispatch function for PyValue
// ============================================================================

/// Dispatch binary operation for bytes type
pub fn bytes_binary_op<'a, 'ctx>(
    cg: &CgCtx<'a, 'ctx>,
    op: &BinaryOp,
    lhs: BasicValueEnum<'ctx>,
    rhs: &PyValue<'ctx>,
) -> Result<PyValue<'ctx>, String> {
    let py_bytes = PyBytes::from_basic(lhs);
    match op {
        BinaryOp::Add => py_bytes.add(cg, rhs),
        BinaryOp::Mul => py_bytes.mul(cg, rhs),
        BinaryOp::Eq => py_bytes.eq(cg, rhs),
        BinaryOp::Ne => py_bytes.ne(cg, rhs),
        BinaryOp::Lt => py_bytes.lt(cg, rhs),
        BinaryOp::Le => py_bytes.le(cg, rhs),
        BinaryOp::Gt => py_bytes.gt(cg, rhs),
        BinaryOp::Ge => py_bytes.ge(cg, rhs),
        BinaryOp::In => {
            // lhs in rhs -> rhs.contains(lhs)
            // But here lhs is our bytes, so we need to check if lhs is in rhs
            match &rhs.ty {
                PyType::Bytes => {
                    let rhs_bytes = PyBytes::from_basic(rhs.value);
                    let lhs_val = PyValue::bytes(lhs);
                    rhs_bytes.contains(cg, &lhs_val)
                }
                _ => Err(format!("Cannot use 'in' with Bytes and {:?}", rhs.ty)),
            }
        }
        BinaryOp::NotIn => match &rhs.ty {
            PyType::Bytes => {
                let rhs_bytes = PyBytes::from_basic(rhs.value);
                let lhs_val = PyValue::bytes(lhs);
                let contains_result = rhs_bytes.contains(cg, &lhs_val)?;
                let bool_val = contains_result.value.into_int_value();
                let negated = cg.builder.build_not(bool_val, "not_in").unwrap();
                Ok(PyValue::bool(negated.into()))
            }
            _ => Err(format!("Cannot use 'not in' with Bytes and {:?}", rhs.ty)),
        },
        _ => Err(format!("Operator {:?} not supported for bytes type", op)),
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
