//! Type system for TypePython
//!
//! This module provides Python types with code generation support.
//! Operations are organized by type for clarity.

mod bool_ops;
mod bytes_ops;
mod dict_ops;
mod float_ops;
mod int_ops;
mod list_ops;
mod none_ops;
mod set_ops;
mod str_ops;
mod value;

pub use value::{CgCtx, FunctionInfo, MacroKind, ModuleInfo, PyType, PyValue};

// ============================================================================
// Helper Functions (used by type operation modules)
// ============================================================================

use crate::codegen::builtins::BUILTIN_TABLE;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;

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
) -> inkwell::values::IntValue<'ctx> {
    use inkwell::values::AnyValue;
    let any_val = call_site.as_any_value_enum();
    if let inkwell::values::AnyValueEnum::IntValue(iv) = any_val {
        iv
    } else {
        panic!("{} did not return an int value", fn_name)
    }
}

fn extract_float_result<'ctx>(
    call_site: inkwell::values::CallSiteValue<'ctx>,
    fn_name: &str,
) -> inkwell::values::FloatValue<'ctx> {
    use inkwell::values::AnyValue;
    let any_val = call_site.as_any_value_enum();
    if let inkwell::values::AnyValueEnum::FloatValue(fv) = any_val {
        fv
    } else {
        panic!("{} did not return a float value", fn_name)
    }
}

fn extract_ptr_result<'ctx>(
    call_site: inkwell::values::CallSiteValue<'ctx>,
    fn_name: &str,
) -> inkwell::values::PointerValue<'ctx> {
    use inkwell::values::AnyValue;
    let any_val = call_site.as_any_value_enum();
    if let inkwell::values::AnyValueEnum::PointerValue(pv) = any_val {
        pv
    } else {
        panic!("{} did not return a pointer value", fn_name)
    }
}
