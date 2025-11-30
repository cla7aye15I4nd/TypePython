//! List operations: slicing, indexing, and list() builtin
//!
//! This module provides list-related operations:
//! - Subscript operations (list[i])
//! - Slice operations (list[start:stop:step])
//! - list() builtin function
//!
//! Method lookup is handled by the unified method registry in types/methods.rs
//!
//! All C runtime functions are in src/runtime/builtins/list.c
//! and discovered automatically by build.rs.

use crate::ast::Expression;
use crate::codegen::CodeGen;
use crate::types::{PyType, PyValue};
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGen<'ctx> {
    // ========================================================================
    // Subscript operations (e.g., my_list[0])
    // ========================================================================

    /// Get an item at index: list[i] -> element
    pub fn list_getitem(
        &mut self,
        list_val: BasicValueEnum<'ctx>,
        index: BasicValueEnum<'ctx>,
        elem_type: &PyType,
    ) -> Result<PyValue<'ctx>, String> {
        let getitem_fn = self.get_or_declare_c_builtin("list_getitem");
        let call = self
            .cg
            .builder
            .build_call(getitem_fn, &[list_val.into(), index.into()], "list_getitem")
            .unwrap();
        let result = self.extract_int_call_result(call);

        // Handle different element types - C runtime stores all as i64
        // For pointer types, we need to cast the i64 back to a pointer
        // For float, we need to bitcast the i64 bit pattern back to f64
        match elem_type {
            PyType::Str | PyType::Bytes | PyType::List(_) | PyType::Dict(_, _) | PyType::Set(_) => {
                // Cast i64 to pointer for reference types
                let ptr_val = self
                    .cg
                    .builder
                    .build_int_to_ptr(
                        result.value().into_int_value(),
                        self.cg.ctx.ptr_type(inkwell::AddressSpace::default()),
                        "elem_ptr",
                    )
                    .unwrap();
                Ok(PyValue::new(ptr_val.into(), elem_type.clone(), None))
            }
            PyType::Float => {
                // Bitcast i64 to f64 (preserves bit pattern)
                let float_val = self
                    .cg
                    .builder
                    .build_bit_cast(
                        result.value().into_int_value(),
                        self.cg.ctx.f64_type(),
                        "elem_float",
                    )
                    .unwrap();
                Ok(PyValue::new(float_val, elem_type.clone(), None))
            }
            PyType::Bool => {
                // Truncate i64 to i1 for booleans
                let bool_val = self
                    .cg
                    .builder
                    .build_int_truncate(
                        result.value().into_int_value(),
                        self.cg.ctx.bool_type(),
                        "elem_bool",
                    )
                    .unwrap();
                Ok(PyValue::new(bool_val.into(), elem_type.clone(), None))
            }
            _ => {
                // For Int, use the value directly
                Ok(PyValue::new(result.value(), elem_type.clone(), None))
            }
        }
    }

    /// Set an item at index: list[i] = value
    pub fn list_setitem(
        &mut self,
        list_val: BasicValueEnum<'ctx>,
        index: BasicValueEnum<'ctx>,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let setitem_fn = self.get_or_declare_c_builtin("list_setitem");
        self.cg
            .builder
            .build_call(
                setitem_fn,
                &[list_val.into(), index.into(), value.into()],
                "list_setitem",
            )
            .unwrap();
        Ok(())
    }

    /// Delete an item at index: del list[i]
    pub fn list_delitem(
        &mut self,
        list_val: BasicValueEnum<'ctx>,
        index: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let delitem_fn = self.get_or_declare_c_builtin("list_delitem");
        self.cg
            .builder
            .build_call(delitem_fn, &[list_val.into(), index.into()], "list_delitem")
            .unwrap();
        Ok(())
    }

    // ========================================================================
    // Slice operations (e.g., my_list[1:4], my_list[::2])
    // ========================================================================

    /// Get the length of a list
    pub fn list_len(&mut self, list_val: BasicValueEnum<'ctx>) -> Result<PyValue<'ctx>, String> {
        let len_fn = self.get_or_declare_c_builtin("list_len");
        let call = self
            .cg
            .builder
            .build_call(len_fn, &[list_val.into()], "list_len")
            .unwrap();
        Ok(self.extract_int_call_result(call))
    }

    /// Slice list without step: list[start:stop] -> list
    pub fn list_slice(
        &mut self,
        list_val: BasicValueEnum<'ctx>,
        start: BasicValueEnum<'ctx>,
        stop: BasicValueEnum<'ctx>,
        elem_type: &PyType,
    ) -> Result<PyValue<'ctx>, String> {
        let slice_fn = self.get_or_declare_c_builtin("list_slice");
        let call = self
            .cg
            .builder
            .build_call(
                slice_fn,
                &[list_val.into(), start.into(), stop.into()],
                "list_slice",
            )
            .unwrap();
        let result = self.extract_ptr_call_result(call);
        Ok(PyValue::new(
            result.value(),
            PyType::List(Box::new(elem_type.clone())),
            None,
        ))
    }

    /// Slice list with step: list[start:stop:step] -> list
    pub fn list_slice_step(
        &mut self,
        list_val: BasicValueEnum<'ctx>,
        start: BasicValueEnum<'ctx>,
        stop: BasicValueEnum<'ctx>,
        step: BasicValueEnum<'ctx>,
        elem_type: &PyType,
    ) -> Result<PyValue<'ctx>, String> {
        let slice_fn = self.get_or_declare_c_builtin("list_slice_step");
        let call = self
            .cg
            .builder
            .build_call(
                slice_fn,
                &[list_val.into(), start.into(), stop.into(), step.into()],
                "list_slice_step",
            )
            .unwrap();
        let result = self.extract_ptr_call_result(call);
        Ok(PyValue::new(
            result.value(),
            PyType::List(Box::new(elem_type.clone())),
            None,
        ))
    }

    // ========================================================================
    // list() builtin function
    // ========================================================================

    /// Generate list() builtin call
    /// list() -> empty list[int] (default element type)
    /// list(str) -> list of character ordinal values
    /// list(bytes) -> list of byte values
    /// list(list) -> copy of list
    /// list(set) -> list of set elements
    /// list(dict) -> list of dict keys
    pub fn generate_list_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.is_empty() {
            // Create empty list with default int element type
            let list_new_fn = self.get_or_declare_c_builtin("list_new");
            let call_site = self
                .cg
                .builder
                .build_call(list_new_fn, &[], "list_new")
                .unwrap();
            let list_ptr = self.extract_ptr_call_result(call_site);

            return Ok(PyValue::new(
                list_ptr.value(),
                PyType::List(Box::new(PyType::Int)),
                None,
            ));
        }

        if args.len() != 1 {
            return Err("list() takes at most 1 argument".to_string());
        }

        // Generate the argument value
        let arg_val = self.evaluate_expression(&args[0])?;

        match &arg_val.ty() {
            PyType::List(elem_type) => {
                // Copy existing list
                let list_from_list_fn = self.get_or_declare_c_builtin("list_from_list");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        list_from_list_fn,
                        &[arg_val.value().into()],
                        "list_from_list",
                    )
                    .unwrap();
                let list_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::new(
                    list_ptr.value(),
                    PyType::List(elem_type.clone()),
                    None,
                ))
            }
            PyType::Str => {
                // list("hello") -> ['h', 'e', 'l', 'l', 'o'] (single-char strings)
                let list_from_str_fn = self.get_or_declare_c_builtin("str_list_from_str");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        list_from_str_fn,
                        &[arg_val.value().into()],
                        "str_list_from_str",
                    )
                    .unwrap();
                let list_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::new(
                    list_ptr.value(),
                    PyType::List(Box::new(PyType::Str)),
                    None,
                ))
            }
            PyType::Bytes => {
                // list(b"hello") -> [104, 101, 108, 108, 111] (byte values)
                let list_from_bytes_fn = self.get_or_declare_c_builtin("list_from_bytes");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        list_from_bytes_fn,
                        &[arg_val.value().into()],
                        "list_from_bytes",
                    )
                    .unwrap();
                let list_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::new(
                    list_ptr.value(),
                    PyType::List(Box::new(PyType::Int)),
                    None,
                ))
            }
            PyType::Set(elem_type) => {
                // list({1, 2, 3}) -> [1, 2, 3]
                let list_from_set_fn = self.get_or_declare_c_builtin("list_from_set");
                let call_site = self
                    .cg
                    .builder
                    .build_call(list_from_set_fn, &[arg_val.value().into()], "list_from_set")
                    .unwrap();
                let list_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::new(
                    list_ptr.value(),
                    PyType::List(elem_type.clone()),
                    None,
                ))
            }
            PyType::Dict(key_type, _) => {
                // list({"a": 1}) -> ["a"] (list of keys)
                // For string-keyed dicts, use str_list_from_str_dict
                if matches!(key_type.as_ref(), PyType::Str) {
                    let list_from_dict_fn = self.get_or_declare_c_builtin("str_list_from_str_dict");
                    let call_site = self
                        .cg
                        .builder
                        .build_call(
                            list_from_dict_fn,
                            &[arg_val.value().into()],
                            "str_list_from_str_dict",
                        )
                        .unwrap();
                    let list_ptr = self.extract_ptr_call_result(call_site);
                    Ok(PyValue::new(
                        list_ptr.value(),
                        PyType::List(Box::new(PyType::Str)),
                        None,
                    ))
                } else {
                    let list_from_dict_fn = self.get_or_declare_c_builtin("list_from_dict");
                    let call_site = self
                        .cg
                        .builder
                        .build_call(
                            list_from_dict_fn,
                            &[arg_val.value().into()],
                            "list_from_dict",
                        )
                        .unwrap();
                    let list_ptr = self.extract_ptr_call_result(call_site);
                    Ok(PyValue::new(
                        list_ptr.value(),
                        PyType::List(key_type.clone()),
                        None,
                    ))
                }
            }
            _ => Err(format!(
                "list() argument must be an iterable, got {:?}",
                arg_val.ty()
            )),
        }
    }
}
