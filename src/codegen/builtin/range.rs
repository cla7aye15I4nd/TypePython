//! Range operations: range() builtin
//!
//! This module provides the range() builtin function.
//!
//! All C runtime functions are in src/runtime/builtins/range.c
//! and discovered automatically by build.rs.

use crate::ast::Expression;
use crate::codegen::CodeGen;
use crate::types::{PyType, PyValue};

impl<'ctx> CodeGen<'ctx> {
    // ========================================================================
    // range() builtin function
    // ========================================================================

    /// Generate range() builtin call
    /// range(stop) -> range from 0 to stop with step 1
    /// range(start, stop) -> range from start to stop with step 1
    /// range(start, stop, step) -> range from start to stop with given step
    pub fn generate_range_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        match args.len() {
            1 => {
                // range(stop)
                let stop_val = self.evaluate_expression(&args[0])?;
                if stop_val.ty() != PyType::Int {
                    return Err("range() argument must be an integer".to_string());
                }

                let range_new_fn = self.get_or_declare_c_builtin("range_new1");
                let call_site = self
                    .cg
                    .builder
                    .build_call(range_new_fn, &[stop_val.value().into()], "range_new")
                    .unwrap();
                let range_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::range(range_ptr.value().into_pointer_value()))
            }
            2 => {
                // range(start, stop)
                let start_val = self.evaluate_expression(&args[0])?;
                let stop_val = self.evaluate_expression(&args[1])?;

                if start_val.ty() != PyType::Int || stop_val.ty() != PyType::Int {
                    return Err("range() arguments must be integers".to_string());
                }

                let range_new_fn = self.get_or_declare_c_builtin("range_new2");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        range_new_fn,
                        &[start_val.value().into(), stop_val.value().into()],
                        "range_new",
                    )
                    .unwrap();
                let range_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::range(range_ptr.value().into_pointer_value()))
            }
            3 => {
                // range(start, stop, step)
                let start_val = self.evaluate_expression(&args[0])?;
                let stop_val = self.evaluate_expression(&args[1])?;
                let step_val = self.evaluate_expression(&args[2])?;

                if start_val.ty() != PyType::Int
                    || stop_val.ty() != PyType::Int
                    || step_val.ty() != PyType::Int
                {
                    return Err("range() arguments must be integers".to_string());
                }

                let range_new_fn = self.get_or_declare_c_builtin("range_new");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        range_new_fn,
                        &[
                            start_val.value().into(),
                            stop_val.value().into(),
                            step_val.value().into(),
                        ],
                        "range_new",
                    )
                    .unwrap();
                let range_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::range(range_ptr.value().into_pointer_value()))
            }
            0 => Err("range() requires at least 1 argument".to_string()),
            _ => Err("range() takes at most 3 arguments".to_string()),
        }
    }
}
