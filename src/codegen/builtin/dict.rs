//! Dict operations: subscript access and dict() builtin
//!
//! This module provides dict-related operations:
//! - Subscript operations (dict[key])
//! - dict() builtin function
//!
//! Method lookup is handled by the unified method registry in types/methods.rs
//!
//! All C runtime functions are in src/runtime/builtins/dict.c
//! and discovered automatically by build.rs.

use crate::ast::Expression;
use crate::codegen::CodeGen;
use crate::types::{PyType, PyValue};
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGen<'ctx> {
    // ========================================================================
    // Subscript operations (e.g., my_dict[key])
    // ========================================================================

    /// Get an item by key: dict[key] -> value
    /// Uses the appropriate C function based on key type
    pub fn dict_getitem_typed(
        &mut self,
        dict_val: BasicValueEnum<'ctx>,
        key: BasicValueEnum<'ctx>,
        key_type: &PyType,
        val_type: &PyType,
    ) -> Result<PyValue<'ctx>, String> {
        // Select the appropriate getitem function based on key type
        let getitem_fn = if matches!(key_type, PyType::Str) {
            self.get_or_declare_c_builtin("str_dict_getitem")
        } else {
            self.get_or_declare_c_builtin("dict_getitem")
        };

        let call = self
            .cg
            .builder
            .build_call(getitem_fn, &[dict_val.into(), key.into()], "dict_getitem")
            .unwrap();
        let result = self.extract_int_call_result(call);

        // Handle different value types - C runtime stores all as i64
        // For pointer types, we need to cast the i64 back to a pointer
        // For float, we need to bitcast the i64 bit pattern back to f64
        match val_type {
            PyType::Str | PyType::Bytes | PyType::List(_) | PyType::Dict(_, _) | PyType::Set(_) => {
                // Cast i64 to pointer for reference types
                let ptr_val = self
                    .cg
                    .builder
                    .build_int_to_ptr(
                        result.value().into_int_value(),
                        self.cg.ctx.ptr_type(inkwell::AddressSpace::default()),
                        "val_ptr",
                    )
                    .unwrap();
                Ok(PyValue::new(ptr_val.into(), val_type.clone(), None))
            }
            PyType::Float => {
                // Bitcast i64 to f64 (preserves bit pattern)
                let float_val = self
                    .cg
                    .builder
                    .build_bit_cast(
                        result.value().into_int_value(),
                        self.cg.ctx.f64_type(),
                        "val_float",
                    )
                    .unwrap();
                Ok(PyValue::new(float_val, val_type.clone(), None))
            }
            PyType::Bool => {
                // Truncate i64 to i1 for booleans
                let bool_val = self
                    .cg
                    .builder
                    .build_int_truncate(
                        result.value().into_int_value(),
                        self.cg.ctx.bool_type(),
                        "val_bool",
                    )
                    .unwrap();
                Ok(PyValue::new(bool_val.into(), val_type.clone(), None))
            }
            _ => {
                // For Int, use the value directly
                Ok(PyValue::new(result.value(), val_type.clone(), None))
            }
        }
    }

    /// Get an item by key: dict[key] -> value (for int-keyed dicts)
    #[allow(dead_code)]
    pub fn dict_getitem(
        &mut self,
        dict_val: BasicValueEnum<'ctx>,
        key: BasicValueEnum<'ctx>,
        val_type: &PyType,
    ) -> Result<PyValue<'ctx>, String> {
        self.dict_getitem_typed(dict_val, key, &PyType::Int, val_type)
    }

    /// Set an item by key: dict[key] = value
    pub fn dict_setitem(
        &mut self,
        dict_val: BasicValueEnum<'ctx>,
        key: BasicValueEnum<'ctx>,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let setitem_fn = self.get_or_declare_c_builtin("dict_setitem");
        self.cg
            .builder
            .build_call(
                setitem_fn,
                &[dict_val.into(), key.into(), value.into()],
                "dict_setitem",
            )
            .unwrap();
        Ok(())
    }

    /// Delete an item by key: del dict[key]
    pub fn dict_delitem(
        &mut self,
        dict_val: BasicValueEnum<'ctx>,
        key: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let delitem_fn = self.get_or_declare_c_builtin("dict_delitem");
        self.cg
            .builder
            .build_call(delitem_fn, &[dict_val.into(), key.into()], "dict_delitem")
            .unwrap();
        Ok(())
    }

    // ========================================================================
    // dict() builtin function
    // ========================================================================

    /// Generate dict() builtin call - creates an empty dict
    /// dict() -> empty dict[int, int] (default key/value types)
    pub fn generate_dict_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if !args.is_empty() {
            return Err(
                "dict() takes no arguments (use dict literal {1: 2, 3: 4} instead)".to_string(),
            );
        }

        // Create empty dict with default int key/value types
        let dict_new_fn = self.get_or_declare_c_builtin("dict_new");
        let call_site = self
            .cg
            .builder
            .build_call(dict_new_fn, &[], "dict_new")
            .unwrap();
        let dict_ptr = self.extract_ptr_call_result(call_site);

        Ok(PyValue::new(
            dict_ptr.value(),
            PyType::Dict(Box::new(PyType::Int), Box::new(PyType::Int)),
            None,
        ))
    }
}
