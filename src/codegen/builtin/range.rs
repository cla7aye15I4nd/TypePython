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
        // Helper to coerce Bool to Int
        let coerce_to_int = |val: PyValue<'ctx>,
                             cg: &crate::codegen::CgCtx<'ctx>|
         -> Result<inkwell::values::IntValue<'ctx>, String> {
            match val.ty() {
                PyType::Int => Ok(val.value().into_int_value()),
                PyType::Bool => {
                    // Zero-extend i1 to i64
                    Ok(cg
                        .builder
                        .build_int_z_extend(
                            val.value().into_int_value(),
                            cg.ctx.i64_type(),
                            "bool_to_int",
                        )
                        .unwrap())
                }
                _ => Err("range() arguments must be integers".to_string()),
            }
        };

        match args.len() {
            1 => {
                // range(stop)
                let stop_val = self.evaluate_expression(&args[0])?;
                let stop_int = coerce_to_int(stop_val, &self.cg)?;

                let range_new_fn = self.get_or_declare_c_builtin("range_new1");
                let call_site = self
                    .cg
                    .builder
                    .build_call(range_new_fn, &[stop_int.into()], "range_new")
                    .unwrap();
                let range_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::range(range_ptr.value().into_pointer_value()))
            }
            2 => {
                // range(start, stop)
                let start_val = self.evaluate_expression(&args[0])?;
                let stop_val = self.evaluate_expression(&args[1])?;

                let start_int = coerce_to_int(start_val, &self.cg)?;
                let stop_int = coerce_to_int(stop_val, &self.cg)?;

                let range_new_fn = self.get_or_declare_c_builtin("range_new2");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        range_new_fn,
                        &[start_int.into(), stop_int.into()],
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

                let start_int = coerce_to_int(start_val, &self.cg)?;
                let stop_int = coerce_to_int(stop_val, &self.cg)?;
                let step_int = coerce_to_int(step_val, &self.cg)?;

                let range_new_fn = self.get_or_declare_c_builtin("range_new");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        range_new_fn,
                        &[start_int.into(), stop_int.into(), step_int.into()],
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

    // ========================================================================
    // range subscript: range[index]
    // ========================================================================

    /// Get element at index: range[i] -> int
    pub fn range_getitem(
        &mut self,
        range_val: inkwell::values::BasicValueEnum<'ctx>,
        index: inkwell::values::BasicValueEnum<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        use inkwell::values::AnyValue;

        // Call range_getitem which returns the value at the given index
        let getitem_fn = self.get_or_declare_c_builtin("range_getitem");
        let call = self
            .cg
            .builder
            .build_call(
                getitem_fn,
                &[range_val.into(), index.into()],
                "range_getitem",
            )
            .unwrap();
        let result = call.as_any_value_enum().into_int_value();
        Ok(PyValue::int(result))
    }
}
