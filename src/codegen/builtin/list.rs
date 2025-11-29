//! List operations: method calls, slicing, and indexing
//!
//! This module provides all list-related operations:
//! - Method lookup (append, pop, insert, etc.)
//! - Subscript operations (list[i])
//! - Slice operations (list[start:stop:step])
//!
//! All C runtime functions are in src/runtime/builtins/list.c
//! and discovered automatically by build.rs.

use crate::ast::Expression;
use crate::codegen::CodeGen;
use crate::types::{FunctionInfo, PyType, PyValue};
use inkwell::values::BasicValueEnum;

/// Maps method name to (builtin_symbol, return_type, mutates_self)
/// mutates_self indicates if the method returns the modified list pointer
fn get_list_method_info(name: &str, elem_type: &PyType) -> Option<(&'static str, PyType, bool)> {
    match name {
        // Mutating methods that return None (like Python)
        "append" => Some(("list_append", PyType::None, true)),
        "insert" => Some(("list_insert", PyType::None, true)),
        "extend" => Some(("list_extend", PyType::None, true)),
        "remove" => Some(("list_remove", PyType::None, false)),
        "clear" => Some(("list_clear", PyType::None, false)),
        "reverse" => Some(("list_reverse", PyType::None, false)),
        "sort" => Some(("list_sort", PyType::None, false)),

        // Methods returning values
        "pop" => Some(("list_pop", PyType::Int, false)),
        "index" => Some(("list_index", PyType::Int, false)),
        "count" => Some(("list_count", PyType::Int, false)),

        // Methods returning new list
        "copy" => Some((
            "list_copy",
            PyType::List(Box::new(elem_type.clone())),
            false,
        )),

        _ => None,
    }
}

impl<'ctx> CodeGen<'ctx> {
    // ========================================================================
    // Method calls (e.g., my_list.append(42))
    // ========================================================================

    /// Get a list method as a function with the receiver pre-bound
    pub fn get_list_method(
        &mut self,
        receiver: &PyValue<'ctx>,
        method_name: &str,
    ) -> Result<PyValue<'ctx>, String> {
        let elem_type = match &receiver.ty {
            PyType::List(elem) => elem.as_ref(),
            _ => return Err("Expected list type".to_string()),
        };

        let (symbol, return_type, _mutates) = get_list_method_info(method_name, elem_type)
            .ok_or_else(|| format!("list has no method '{}'", method_name))?;

        let function = self.get_or_declare_c_builtin(symbol);

        Ok(PyValue::function(FunctionInfo {
            mangled_name: symbol.to_string(),
            function,
            param_types: vec![],
            return_type,
            bound_args: vec![receiver.value()],
        }))
    }

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
            .builder
            .build_call(getitem_fn, &[list_val.into(), index.into()], "list_getitem")
            .unwrap();
        let result = self.extract_int_call_result(call)?;
        // For now, all elements are stored as i64
        // TODO: Handle different element types properly
        Ok(PyValue::new(result.value(), elem_type.clone(), None))
    }

    /// Set an item at index: list[i] = value
    pub fn list_setitem(
        &mut self,
        list_val: BasicValueEnum<'ctx>,
        index: BasicValueEnum<'ctx>,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let setitem_fn = self.get_or_declare_c_builtin("list_setitem");
        self.builder
            .build_call(
                setitem_fn,
                &[list_val.into(), index.into(), value.into()],
                "list_setitem",
            )
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
            .builder
            .build_call(len_fn, &[list_val.into()], "list_len")
            .unwrap();
        self.extract_int_call_result(call)
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
            .builder
            .build_call(
                slice_fn,
                &[list_val.into(), start.into(), stop.into()],
                "list_slice",
            )
            .unwrap();
        let result = self.extract_ptr_call_result(call)?;
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
            .builder
            .build_call(
                slice_fn,
                &[list_val.into(), start.into(), stop.into(), step.into()],
                "list_slice_step",
            )
            .unwrap();
        let result = self.extract_ptr_call_result(call)?;
        Ok(PyValue::new(
            result.value(),
            PyType::List(Box::new(elem_type.clone())),
            None,
        ))
    }

    // ========================================================================
    // list() builtin function
    // ========================================================================

    /// Generate list() builtin call - creates an empty list
    /// list() -> empty list[int] (default element type)
    pub fn generate_list_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if !args.is_empty() {
            return Err(
                "list() takes no arguments (use list literal [1, 2, 3] instead)".to_string(),
            );
        }

        // Create empty list with default int element type
        let list_new_fn = self.get_or_declare_c_builtin("list_new");
        let call_site = self
            .builder
            .build_call(list_new_fn, &[], "list_new")
            .unwrap();
        let list_ptr = self.extract_ptr_call_result(call_site)?;

        Ok(PyValue::new(
            list_ptr.value(),
            PyType::List(Box::new(PyType::Int)),
            None,
        ))
    }
}
