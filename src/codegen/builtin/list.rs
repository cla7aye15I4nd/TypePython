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
use crate::codegen::types::{
    get_field_type, is_range, iter_names, BoolStorage, EnumerateSource, PtrStorage,
};
use crate::codegen::CodeGen;
use crate::types::{InstanceType, PyType, PyValue};
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

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
        use inkwell::values::AnyValue;

        // Select the appropriate getitem function based on element type
        // Each list type has its own storage (int64_t for int/ptr, double for float, int8_t for bool)
        match elem_type {
            PyType::Bool => {
                // Bool lists use int8_t storage
                let getitem_fn = self.get_or_declare_c_builtin("bool_list_getitem");
                let call = self
                    .cg
                    .builder
                    .build_call(
                        getitem_fn,
                        &[list_val.into(), index.into()],
                        "bool_list_getitem",
                    )
                    .unwrap();
                // Returns i8, truncate to i1
                let result_i8 = call.as_any_value_enum().into_int_value();
                let bool_val = self
                    .cg
                    .builder
                    .build_int_truncate(result_i8, self.cg.ctx.bool_type(), "elem_bool")
                    .unwrap();
                Ok(PyValue::new(bool_val.into(), elem_type.clone(), None))
            }
            PyType::Float => {
                // Float lists use double storage
                let getitem_fn = self.get_or_declare_c_builtin("float_list_getitem");
                let call = self
                    .cg
                    .builder
                    .build_call(
                        getitem_fn,
                        &[list_val.into(), index.into()],
                        "float_list_getitem",
                    )
                    .unwrap();
                let float_val = call.as_any_value_enum().into_float_value();
                Ok(PyValue::new(float_val.into(), elem_type.clone(), None))
            }
            PyType::Str => {
                // String lists use pointer storage
                let getitem_fn = self.get_or_declare_c_builtin("str_list_getitem");
                let call = self
                    .cg
                    .builder
                    .build_call(
                        getitem_fn,
                        &[list_val.into(), index.into()],
                        "str_list_getitem",
                    )
                    .unwrap();
                let ptr_val = call.as_any_value_enum().into_pointer_value();
                Ok(PyValue::new(ptr_val.into(), elem_type.clone(), None))
            }
            PyType::Bytes
            | PyType::List(_)
            | PyType::Dict(_, _)
            | PyType::Set(_)
            | PyType::Tuple(_) => {
                // Other pointer types use int64_t storage
                let getitem_fn = self.get_or_declare_c_builtin("list_getitem");
                let call = self
                    .cg
                    .builder
                    .build_call(getitem_fn, &[list_val.into(), index.into()], "list_getitem")
                    .unwrap();
                let result = self.extract_int_call_result(call);
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
            _ => {
                // For Int, use int64_t storage
                let getitem_fn = self.get_or_declare_c_builtin("list_getitem");
                let call = self
                    .cg
                    .builder
                    .build_call(getitem_fn, &[list_val.into(), index.into()], "list_getitem")
                    .unwrap();
                let result = self.extract_int_call_result(call);
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

    // ========================================================================
    // any() and all() builtins
    // ========================================================================

    /// Generate any() builtin call
    /// any(iterable) -> True if any element is truthy
    pub fn generate_any_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("any() takes exactly 1 argument".to_string());
        }

        let arg_val = self.evaluate_expression(&args[0])?;

        let fn_name = match arg_val.ty() {
            PyType::List(elem_type) => match elem_type.as_ref() {
                PyType::Int => "list_any",
                PyType::Float => "float_list_any",
                PyType::Bool => "bool_list_any",
                PyType::Str => "str_list_any",
                _ => {
                    return Err(format!(
                        "any() not supported for list element type {:?}",
                        elem_type
                    ))
                }
            },
            PyType::Set(elem_type) => match elem_type.as_ref() {
                PyType::Int => "set_any",
                PyType::Str => "str_set_any",
                _ => {
                    return Err(format!(
                        "any() not supported for set element type {:?}",
                        elem_type
                    ))
                }
            },
            PyType::Dict(key_type, _) => match key_type.as_ref() {
                PyType::Int => "dict_any",
                PyType::Str => "str_dict_any",
                _ => {
                    return Err(format!(
                        "any() not supported for dict key type {:?}",
                        key_type
                    ))
                }
            },
            PyType::Str => "str_any",
            PyType::Bytes => "bytes_any",
            _ => return Err(format!("any() not supported for type {:?}", arg_val.ty())),
        };

        let any_fn = self.get_or_declare_c_builtin(fn_name);
        let call_site = self
            .cg
            .builder
            .build_call(any_fn, &[arg_val.value().into()], "any_result")
            .unwrap();
        let result = self.extract_int_call_result(call_site);

        // Convert i64 result to i1 bool
        let bool_val = self
            .cg
            .builder
            .build_int_compare(
                IntPredicate::NE,
                result.value().into_int_value(),
                self.cg.ctx.i64_type().const_int(0, false),
                "any_bool",
            )
            .unwrap();

        Ok(PyValue::bool(bool_val))
    }

    /// Generate all() builtin call
    /// all(iterable) -> True if all elements are truthy
    pub fn generate_all_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("all() takes exactly 1 argument".to_string());
        }

        let arg_val = self.evaluate_expression(&args[0])?;

        let fn_name = match arg_val.ty() {
            PyType::List(elem_type) => match elem_type.as_ref() {
                PyType::Int => "list_all",
                PyType::Float => "float_list_all",
                PyType::Bool => "bool_list_all",
                PyType::Str => "str_list_all",
                _ => {
                    return Err(format!(
                        "all() not supported for list element type {:?}",
                        elem_type
                    ))
                }
            },
            PyType::Set(elem_type) => match elem_type.as_ref() {
                PyType::Int => "set_all",
                PyType::Str => "str_set_all",
                _ => {
                    return Err(format!(
                        "all() not supported for set element type {:?}",
                        elem_type
                    ))
                }
            },
            PyType::Dict(key_type, _) => match key_type.as_ref() {
                PyType::Int => "dict_all",
                PyType::Str => "str_dict_all",
                _ => {
                    return Err(format!(
                        "all() not supported for dict key type {:?}",
                        key_type
                    ))
                }
            },
            PyType::Str => "str_all",
            PyType::Bytes => "bytes_all",
            _ => return Err(format!("all() not supported for type {:?}", arg_val.ty())),
        };

        let all_fn = self.get_or_declare_c_builtin(fn_name);
        let call_site = self
            .cg
            .builder
            .build_call(all_fn, &[arg_val.value().into()], "all_result")
            .unwrap();
        let result = self.extract_int_call_result(call_site);

        // Convert i64 result to i1 bool
        let bool_val = self
            .cg
            .builder
            .build_int_compare(
                IntPredicate::NE,
                result.value().into_int_value(),
                self.cg.ctx.i64_type().const_int(0, false),
                "all_bool",
            )
            .unwrap();

        Ok(PyValue::bool(bool_val))
    }

    // ========================================================================
    // enumerate(), zip(), filter(), iter() builtins
    // ========================================================================

    /// Generate enumerate() builtin call
    /// enumerate(iterable) -> enumerate iterator yielding (index, value) pairs
    pub fn generate_enumerate_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.is_empty() || args.len() > 2 {
            return Err("enumerate() takes 1 or 2 arguments".to_string());
        }

        let arg_val = self.evaluate_expression(&args[0])?;

        // Get the element type from the iterable
        let elem_type = match arg_val.ty() {
            PyType::List(elem) => (*elem).clone(),
            PyType::Set(elem) => (*elem).clone(),
            PyType::Dict(key, _) => (*key).clone(),
            PyType::Str => PyType::Str,   // Characters
            PyType::Bytes => PyType::Int, // Byte values
            _ => PyType::Int,             // Default
        };

        let (fn_name, source) = match arg_val.ty() {
            PyType::List(_) => ("enumerate_list", EnumerateSource::List),
            PyType::Str => ("enumerate_str", EnumerateSource::Str),
            PyType::Bytes => ("enumerate_bytes", EnumerateSource::Bytes),
            _ => ("enumerate_list", EnumerateSource::List), // Use list version as generic fallback
        };

        // Get the start parameter (default to 0)
        let start_val = if args.len() > 1 {
            let start = self.evaluate_expression(&args[1])?;
            start.value()
        } else {
            self.cg.ctx.i64_type().const_int(0, false).into()
        };

        let enumerate_fn = self.get_or_declare_c_builtin(fn_name);
        let call_site = self
            .cg
            .builder
            .build_call(
                enumerate_fn,
                &[arg_val.value().into(), start_val.into()],
                "enumerate",
            )
            .unwrap();
        let iter_ptr = self.extract_ptr_call_result(call_site);

        // Store source type as a string field for iteration dispatch
        let source_str = match source {
            EnumerateSource::List => "list",
            EnumerateSource::Str => "str",
            EnumerateSource::Bytes => "bytes",
        };
        Ok(PyValue::Instance(
            PtrStorage::Direct(iter_ptr.value().into_pointer_value()),
            InstanceType::new(
                iter_names::ENUMERATE.to_string(),
                vec![
                    ("element_type".to_string(), elem_type),
                    (
                        "source".to_string(),
                        PyType::Instance(InstanceType::new(source_str.to_string(), vec![])),
                    ),
                ],
            ),
        ))
    }

    /// Generate zip() builtin call
    /// zip(iterable1, iterable2, ...) -> zip iterator
    pub fn generate_zip_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.is_empty() {
            return Err("zip() requires at least 1 argument".to_string());
        }

        // Collect element types from all iterables
        let mut elem_types: Vec<PyType> = Vec::new();
        let mut arg_vals: Vec<PyValue<'ctx>> = Vec::new();

        for arg in args {
            let val = self.evaluate_expression(arg)?;
            let elem_type = match val.ty() {
                PyType::List(elem) => (*elem).clone(),
                PyType::Set(elem) => (*elem).clone(),
                PyType::Dict(key, _) => (*key).clone(),
                PyType::Str => PyType::Str,
                PyType::Bytes => PyType::Int,
                _ => PyType::Int,
            };
            elem_types.push(elem_type);
            arg_vals.push(val);
        }

        // Handle based on number of arguments
        let iter_ptr = if args.len() == 1 {
            let zip_fn = self.get_or_declare_c_builtin("zip_single");
            let call_site = self
                .cg
                .builder
                .build_call(zip_fn, &[arg_vals[0].value().into()], "zip")
                .unwrap();
            self.extract_ptr_call_result(call_site)
        } else if args.len() == 2 {
            let zip_fn = self.get_or_declare_c_builtin("zip_two");
            let call_site = self
                .cg
                .builder
                .build_call(
                    zip_fn,
                    &[arg_vals[0].value().into(), arg_vals[1].value().into()],
                    "zip",
                )
                .unwrap();
            self.extract_ptr_call_result(call_site)
        } else if args.len() == 3 {
            let zip_fn = self.get_or_declare_c_builtin("zip_three");
            let call_site = self
                .cg
                .builder
                .build_call(
                    zip_fn,
                    &[
                        arg_vals[0].value().into(),
                        arg_vals[1].value().into(),
                        arg_vals[2].value().into(),
                    ],
                    "zip",
                )
                .unwrap();
            self.extract_ptr_call_result(call_site)
        } else {
            return Err("zip() with more than 3 iterables not yet supported".to_string());
        };

        // Store element types as tuple type for unpacking
        Ok(PyValue::Instance(
            PtrStorage::Direct(iter_ptr.value().into_pointer_value()),
            InstanceType::new(
                iter_names::ZIP.to_string(),
                vec![("element_types".to_string(), PyType::Tuple(elem_types))],
            ),
        ))
    }

    /// Generate filter() builtin call
    /// filter(function_or_none, iterable) -> filter iterator
    pub fn generate_filter_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.len() != 2 {
            return Err("filter() takes exactly 2 arguments".to_string());
        }

        // For the test cases, the first argument is often None
        let iterable_val = self.evaluate_expression(&args[1])?;

        let elem_type = match iterable_val.ty() {
            PyType::List(elem) => (*elem).clone(),
            PyType::Set(elem) => (*elem).clone(),
            PyType::Dict(key, _) => (*key).clone(),
            PyType::Str => PyType::Str,
            PyType::Bytes => PyType::Int,
            _ => PyType::Int,
        };

        let filter_fn = self.get_or_declare_c_builtin("filter_none");
        let call_site = self
            .cg
            .builder
            .build_call(filter_fn, &[iterable_val.value().into()], "filter")
            .unwrap();
        let iter_ptr = self.extract_ptr_call_result(call_site);

        Ok(PyValue::Instance(
            PtrStorage::Direct(iter_ptr.value().into_pointer_value()),
            InstanceType::new(
                iter_names::FILTER.to_string(),
                vec![("element_type".to_string(), elem_type)],
            ),
        ))
    }

    /// Generate iter() builtin call
    /// iter(iterable) -> iterator
    pub fn generate_iter_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        use inkwell::values::AnyValue;

        if args.is_empty() || args.len() > 2 {
            return Err("iter() takes 1 or 2 arguments".to_string());
        }

        let arg_val = self.evaluate_expression(&args[0])?;

        // Handle Range specially - it returns a range iterator
        if is_range(&arg_val.ty()) {
            let iter_fn = self.get_or_declare_c_builtin("range_iter");
            let call_site = self
                .cg
                .builder
                .build_call(iter_fn, &[arg_val.value().into()], "range_iter")
                .unwrap();
            let iter_ptr = self.extract_ptr_call_result(call_site);
            return Ok(PyValue::Instance(
                PtrStorage::Direct(iter_ptr.value().into_pointer_value()),
                InstanceType::new(
                    iter_names::RANGE_ITERATOR.to_string(),
                    vec![("element_type".to_string(), PyType::Int)],
                ),
            ));
        }

        // Handle user-defined classes with __iter__ method
        if let PyType::Instance(inst) = arg_val.ty() {
            // Check if this class has __iter__ method
            if let Some(class_info) = self.classes.get(&inst.class_name).cloned() {
                if class_info.methods.contains(&"__iter__".to_string()) {
                    // Call __iter__ method on the instance
                    let iter_fn_name = format!("__main___{}___iter__", inst.class_name);
                    let iter_fn = self.cg.module.get_function(&iter_fn_name).ok_or_else(|| {
                        format!("__iter__ method not found for class {}", inst.class_name)
                    })?;

                    let iter_ptr = arg_val.value().into_pointer_value();
                    let iter_call = self
                        .cg
                        .builder
                        .build_call(iter_fn, &[iter_ptr.into()], "iter")
                        .unwrap();
                    let iterator_ptr = iter_call.as_any_value_enum().into_pointer_value();

                    // Determine return type from __next__ method
                    let next_fn_name = format!("__main___{}___next__", inst.class_name);
                    let elem_type =
                        if let Some(next_fn) = self.cg.module.get_function(&next_fn_name) {
                            if let Some(ret_ty) = next_fn.get_type().get_return_type() {
                                if ret_ty.is_int_type() {
                                    PyType::Int
                                } else if ret_ty.is_pointer_type() {
                                    PyType::Str
                                } else if ret_ty.is_float_type() {
                                    PyType::Float
                                } else {
                                    PyType::Int
                                }
                            } else {
                                PyType::Int
                            }
                        } else {
                            PyType::Int
                        };

                    // Return the iterator as an instance of the same class
                    // (since __iter__ returns self in most iterator implementations)
                    return Ok(PyValue::Instance(
                        PtrStorage::Direct(iterator_ptr),
                        InstanceType::new(
                            inst.class_name.clone(),
                            vec![("element_type".to_string(), elem_type)],
                        ),
                    ));
                }
            }
        }

        let elem_type = match arg_val.ty() {
            PyType::List(elem) => (*elem).clone(),
            PyType::Set(elem) => (*elem).clone(),
            PyType::Dict(key, _) => (*key).clone(),
            PyType::Str => PyType::Str,
            PyType::Bytes => PyType::Int,
            _ => PyType::Int,
        };

        let fn_name = match arg_val.ty() {
            PyType::List(_) => "iter_list",
            PyType::Str => "iter_str",
            PyType::Bytes => "iter_bytes",
            _ => "iter_list",
        };

        let iter_fn = self.get_or_declare_c_builtin(fn_name);
        let call_site = self
            .cg
            .builder
            .build_call(iter_fn, &[arg_val.value().into()], "iter")
            .unwrap();
        let iter_ptr = self.extract_ptr_call_result(call_site);

        // Use appropriate iterator class name based on source type
        let iter_class = match arg_val.ty() {
            PyType::List(_) => iter_names::LIST_ITERATOR,
            PyType::Str => iter_names::STR_ITERATOR,
            PyType::Bytes => iter_names::BYTES_ITERATOR,
            _ => iter_names::LIST_ITERATOR,
        };
        Ok(PyValue::Instance(
            PtrStorage::Direct(iter_ptr.value().into_pointer_value()),
            InstanceType::new(
                iter_class.to_string(),
                vec![("element_type".to_string(), elem_type)],
            ),
        ))
    }

    // ========================================================================
    // next() builtin - get next value from iterator
    // ========================================================================

    /// Generate next() builtin call
    /// next(iterator) -> value or raises StopIteration
    /// next(iterator, default) -> value or default
    pub fn generate_next_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.is_empty() || args.len() > 2 {
            return Err("next() takes 1 or 2 arguments".to_string());
        }

        let iter_val = self.evaluate_expression(&args[0])?;

        // Get or use default value
        let default_val = if args.len() == 2 {
            self.evaluate_expression(&args[1])?
        } else {
            // No default - use 0 as sentinel (in real Python this would raise StopIteration)
            PyValue::int(self.cg.ctx.i64_type().const_int(0, false))
        };

        let default_i64 = match default_val.ty() {
            PyType::Int => default_val.value().into_int_value(),
            _ => self.cg.ctx.i64_type().const_int(0, false),
        };

        // Create null pointer for exhausted flag (we'll ignore it for now)
        let null_ptr = self
            .cg
            .ctx
            .ptr_type(inkwell::AddressSpace::default())
            .const_null();

        // Handle Instance-based iterators
        if let PyValue::Instance(storage, inst) = &iter_val {
            let iter_ptr: inkwell::values::BasicValueEnum = match storage {
                PtrStorage::Direct(ptr) => (*ptr).into(),
                PtrStorage::Alloca(ptr) => self
                    .cg
                    .builder
                    .build_load(
                        self.cg.ctx.ptr_type(inkwell::AddressSpace::default()),
                        *ptr,
                        "load_iter",
                    )
                    .unwrap(),
            };

            // Get element type from fields
            let elem_type = get_field_type(&inst.fields, "element_type")
                .cloned()
                .unwrap_or(PyType::Int);

            match inst.class_name.as_str() {
                iter_names::RANGE_ITERATOR => {
                    let next_fn = self.get_or_declare_c_builtin("range_iter_next_default");
                    let call_site = self
                        .cg
                        .builder
                        .build_call(
                            next_fn,
                            &[iter_ptr.into(), default_i64.into(), null_ptr.into()],
                            "next",
                        )
                        .unwrap();
                    let result = self.extract_int_call_result(call_site);
                    return Ok(PyValue::int(result.value().into_int_value()));
                }
                iter_names::STR_ITERATOR => {
                    let next_fn = self.get_or_declare_c_builtin("iter_next_str");
                    let default_ptr = self
                        .cg
                        .ctx
                        .ptr_type(inkwell::AddressSpace::default())
                        .const_null();
                    let call_site = self
                        .cg
                        .builder
                        .build_call(
                            next_fn,
                            &[iter_ptr.into(), default_ptr.into(), null_ptr.into()],
                            "next",
                        )
                        .unwrap();
                    let result = self.extract_ptr_call_result(call_site);
                    return Ok(PyValue::new_str(result.value().into_pointer_value()));
                }
                iter_names::LIST_ITERATOR | iter_names::BYTES_ITERATOR => {
                    let next_fn = self.get_or_declare_c_builtin("iter_next_list");
                    let call_site = self
                        .cg
                        .builder
                        .build_call(
                            next_fn,
                            &[iter_ptr.into(), default_i64.into(), null_ptr.into()],
                            "next",
                        )
                        .unwrap();
                    let result = self.extract_int_call_result(call_site);
                    return match elem_type {
                        PyType::Int => Ok(PyValue::int(result.value().into_int_value())),
                        _ => Ok(PyValue::int(result.value().into_int_value())),
                    };
                }
                _ => {
                    // Check if this is a user-defined iterator class with __next__ method
                    use inkwell::values::AnyValue;
                    if let Some(class_info) = self.classes.get(&inst.class_name).cloned() {
                        if class_info.methods.contains(&"__next__".to_string()) {
                            // Call __next__ method on the iterator
                            let next_fn_name = format!("__main___{}___next__", inst.class_name);
                            let next_fn =
                                self.cg.module.get_function(&next_fn_name).ok_or_else(|| {
                                    format!(
                                        "__next__ method not found for class {}",
                                        inst.class_name
                                    )
                                })?;

                            // Get or declare the stop iteration flag
                            let stop_flag_global = self
                                .cg
                                .module
                                .get_global("__stop_iteration_flag")
                                .unwrap_or_else(|| {
                                    let g = self.cg.module.add_global(
                                        self.cg.ctx.bool_type(),
                                        None,
                                        "__stop_iteration_flag",
                                    );
                                    g.set_initializer(&self.cg.ctx.bool_type().const_zero());
                                    g
                                });

                            // Reset the flag before calling __next__
                            self.cg
                                .builder
                                .build_store(
                                    stop_flag_global.as_pointer_value(),
                                    self.cg.ctx.bool_type().const_zero(),
                                )
                                .unwrap();

                            // Call __next__
                            let next_call = self
                                .cg
                                .builder
                                .build_call(next_fn, &[iter_ptr.into()], "next_val")
                                .unwrap();

                            // Extract the return value based on element type
                            let next_value: inkwell::values::BasicValueEnum = match elem_type {
                                PyType::Int | PyType::Bool => {
                                    next_call.as_any_value_enum().into_int_value().into()
                                }
                                PyType::Float => {
                                    next_call.as_any_value_enum().into_float_value().into()
                                }
                                _ => next_call.as_any_value_enum().into_pointer_value().into(),
                            };

                            // Return value based on element type
                            return match elem_type {
                                PyType::Int => Ok(PyValue::int(next_value.into_int_value())),
                                PyType::Float => Ok(PyValue::float(next_value.into_float_value())),
                                PyType::Str => {
                                    Ok(PyValue::new_str(next_value.into_pointer_value()))
                                }
                                _ => Ok(PyValue::int(next_value.into_int_value())),
                            };
                        }
                    }
                    return Err(format!(
                        "next() not supported for iterator type: {}",
                        inst.class_name
                    ));
                }
            }
        }

        Err(format!(
            "next() requires an iterator, got {:?}",
            iter_val.ty()
        ))
    }

    // ========================================================================
    // id() builtin - returns memory address as integer
    // ========================================================================

    pub fn generate_id_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("id() takes exactly 1 argument".to_string());
        }

        let arg_val = self.evaluate_expression(&args[0])?;

        match arg_val.ty() {
            PyType::Int => {
                // For integers, use the value itself as ID
                let id_fn = self.get_or_declare_c_builtin("id_int");
                let call_site = self
                    .cg
                    .builder
                    .build_call(id_fn, &[arg_val.value().into()], "id")
                    .unwrap();
                let result = self.extract_int_call_result(call_site);
                Ok(PyValue::int(result.value().into_int_value()))
            }
            PyType::Float => {
                // For floats, use bit pattern
                let id_fn = self.get_or_declare_c_builtin("id_float");
                let call_site = self
                    .cg
                    .builder
                    .build_call(id_fn, &[arg_val.value().into()], "id")
                    .unwrap();
                let result = self.extract_int_call_result(call_site);
                Ok(PyValue::int(result.value().into_int_value()))
            }
            PyType::Bool => {
                // For bools, extend to i64
                let bool_val = arg_val.value().into_int_value();
                let extended = self
                    .cg
                    .builder
                    .build_int_z_extend(bool_val, self.cg.ctx.i64_type(), "bool_to_i64")
                    .unwrap();
                Ok(PyValue::int(extended))
            }
            PyType::None => {
                // None has a fixed ID of 0
                let zero = self.cg.ctx.i64_type().const_int(0, false);
                Ok(PyValue::int(zero))
            }
            _ => {
                // For pointer types (str, bytes, list, dict, set), use pointer value
                let id_fn = self.get_or_declare_c_builtin("id_ptr");
                let call_site = self
                    .cg
                    .builder
                    .build_call(id_fn, &[arg_val.value().into()], "id")
                    .unwrap();
                let result = self.extract_int_call_result(call_site);
                Ok(PyValue::int(result.value().into_int_value()))
            }
        }
    }

    // ========================================================================
    // repr() builtin - returns string representation
    // ========================================================================

    pub fn generate_repr_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("repr() takes exactly 1 argument".to_string());
        }

        let arg_val = self.evaluate_expression(&args[0])?;

        let fn_name = match arg_val.ty() {
            PyType::Int => "repr_int",
            PyType::Float => "repr_float",
            PyType::Bool => "repr_bool",
            PyType::Str => "repr_str",
            PyType::Bytes => "repr_bytes",
            PyType::None => "repr_none",
            PyType::List(_) => "repr_list",
            PyType::Dict(_, _) => "repr_str_dict",
            PyType::Set(_) => "repr_set",
            _ => return Err(format!("repr() not supported for {:?}", arg_val.ty())),
        };

        let repr_fn = self.get_or_declare_c_builtin(fn_name);

        // Handle special cases for arg passing
        let call_args: Vec<inkwell::values::BasicMetadataValueEnum> = match arg_val.ty() {
            PyType::Bool => {
                // Extend bool to i64 for repr_bool
                let bool_val = arg_val.value().into_int_value();
                let extended = self
                    .cg
                    .builder
                    .build_int_z_extend(bool_val, self.cg.ctx.i64_type(), "bool_to_i64")
                    .unwrap();
                vec![extended.into()]
            }
            PyType::None => {
                // repr_none takes no arguments
                vec![]
            }
            _ => {
                vec![arg_val.value().into()]
            }
        };

        let call_site = self
            .cg
            .builder
            .build_call(repr_fn, &call_args, "repr")
            .unwrap();
        let result = self.extract_ptr_call_result(call_site);

        Ok(PyValue::new_str(result.value().into_pointer_value()))
    }

    // ========================================================================
    // frozenset() builtin - creates an immutable set
    // ========================================================================

    pub fn generate_frozenset_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.is_empty() {
            // Create empty frozenset (just a set)
            let set_new_fn = self.get_or_declare_c_builtin("set_new");
            let call_site = self
                .cg
                .builder
                .build_call(set_new_fn, &[], "frozenset_new")
                .unwrap();
            let set_ptr = self.extract_ptr_call_result(call_site);
            return Ok(PyValue::Set(
                PtrStorage::Direct(set_ptr.value().into_pointer_value()),
                Box::new(PyType::Int),
            ));
        }

        if args.len() != 1 {
            return Err("frozenset() takes at most 1 argument".to_string());
        }

        let arg_val = self.evaluate_expression(&args[0])?;

        let (fn_name, elem_type) = match arg_val.ty() {
            PyType::Str => ("frozenset_from_str", PyType::Int),
            PyType::Bytes => ("frozenset_from_bytes", PyType::Int),
            PyType::List(elem) => ("frozenset_from_list", (*elem).clone()),
            PyType::Set(elem) => ("frozenset_from_set", (*elem).clone()),
            PyType::Dict(key, _) => ("frozenset_from_dict", (*key).clone()),
            _ => return Err(format!("frozenset() not supported for {:?}", arg_val.ty())),
        };

        let frozenset_fn = self.get_or_declare_c_builtin(fn_name);
        let call_site = self
            .cg
            .builder
            .build_call(frozenset_fn, &[arg_val.value().into()], "frozenset")
            .unwrap();
        let set_ptr = self.extract_ptr_call_result(call_site);

        Ok(PyValue::Set(
            PtrStorage::Direct(set_ptr.value().into_pointer_value()),
            Box::new(elem_type),
        ))
    }

    // ========================================================================
    // getattr() / hasattr() builtins
    // ========================================================================

    /// Generate getattr(obj, name, default) builtin call
    /// For primitive types that don't have attributes, always returns the default
    pub fn generate_getattr_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.len() != 3 {
            return Err("getattr() requires exactly 3 arguments (obj, name, default)".to_string());
        }

        // Evaluate arguments - we ignore the first two for primitive types
        let _obj_val = self.evaluate_expression(&args[0])?;
        let _name_val = self.evaluate_expression(&args[1])?;
        let default_val = self.evaluate_expression(&args[2])?;

        // For all primitive types (int, float, bool, str, bytes, list, dict, set, None),
        // we don't support attribute lookup, so just return the default value
        Ok(default_val)
    }

    /// Generate hasattr(obj, name) builtin call
    /// For primitive types that don't have attributes, always returns False
    pub fn generate_hasattr_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.len() != 2 {
            return Err("hasattr() requires exactly 2 arguments (obj, name)".to_string());
        }

        // Evaluate arguments - we ignore them for primitive types
        let _obj_val = self.evaluate_expression(&args[0])?;
        let _name_val = self.evaluate_expression(&args[1])?;

        // For all primitive types, we don't support attributes, so always return False
        let false_val = self.cg.ctx.bool_type().const_int(0, false);
        Ok(PyValue::Bool(BoolStorage::Immediate(false_val)))
    }
}
