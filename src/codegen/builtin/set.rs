//! Set operations: set() builtin
//!
//! This module provides the set() builtin function.
//!
//! Method lookup is handled by the unified method registry in types/methods.rs
//!
//! All C runtime functions are in src/runtime/builtins/set.c
//! and discovered automatically by build.rs.

use crate::ast::Expression;
use crate::codegen::CodeGen;
use crate::types::{PyType, PyValue};

impl<'ctx> CodeGen<'ctx> {
    // ========================================================================
    // set() builtin function
    // ========================================================================

    /// Generate set() builtin call
    /// set() -> empty set[int] (default element type)
    /// set(existing_set) -> copy of existing_set
    /// set(str) -> set of character ordinal values
    /// set(bytes) -> set of byte values
    /// set(list) -> set of list elements
    /// set(dict) -> set of dict keys
    pub fn generate_set_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.is_empty() {
            // Create empty set with default int element type
            let set_new_fn = self.get_or_declare_c_builtin("set_new");
            let call_site = self
                .cg
                .builder
                .build_call(set_new_fn, &[], "set_new")
                .unwrap();
            let set_ptr = self.extract_ptr_call_result(call_site);

            return Ok(PyValue::new(
                set_ptr.value(),
                PyType::Set(Box::new(PyType::Int)),
                None,
            ));
        }

        if args.len() != 1 {
            return Err("set() takes at most 1 argument".to_string());
        }

        // Generate the argument value
        let arg_val = self.evaluate_expression(&args[0])?;

        match &arg_val.ty() {
            PyType::Set(elem_type) => {
                // Copy existing set
                let set_copy_fn = self.get_or_declare_c_builtin("set_copy");
                let call_site = self
                    .cg
                    .builder
                    .build_call(set_copy_fn, &[arg_val.value().into()], "set_copy")
                    .unwrap();
                let set_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::new(
                    set_ptr.value(),
                    PyType::Set(elem_type.clone()),
                    None,
                ))
            }
            PyType::Str => {
                // set("hello") -> {'h', 'e', 'l', 'o'} (single-char strings)
                let set_from_str_fn = self.get_or_declare_c_builtin("str_set_from_str");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        set_from_str_fn,
                        &[arg_val.value().into()],
                        "str_set_from_str",
                    )
                    .unwrap();
                let set_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::new(
                    set_ptr.value(),
                    PyType::Set(Box::new(PyType::Str)),
                    None,
                ))
            }
            PyType::Bytes => {
                // set(b"hello") -> {104, 101, 108, 111} (byte values)
                let set_from_bytes_fn = self.get_or_declare_c_builtin("set_from_bytes");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        set_from_bytes_fn,
                        &[arg_val.value().into()],
                        "set_from_bytes",
                    )
                    .unwrap();
                let set_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::new(
                    set_ptr.value(),
                    PyType::Set(Box::new(PyType::Int)),
                    None,
                ))
            }
            PyType::List(elem_type) => {
                // set([1, 2, 3]) -> {1, 2, 3}
                let set_from_list_fn = self.get_or_declare_c_builtin("set_from_list");
                let call_site = self
                    .cg
                    .builder
                    .build_call(set_from_list_fn, &[arg_val.value().into()], "set_from_list")
                    .unwrap();
                let set_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::new(
                    set_ptr.value(),
                    PyType::Set(elem_type.clone()),
                    None,
                ))
            }
            PyType::Dict(key_type, _) => {
                // set({"a": 1}) -> {"a"} (set of keys)
                let set_from_dict_fn = self.get_or_declare_c_builtin("set_from_dict");
                let call_site = self
                    .cg
                    .builder
                    .build_call(set_from_dict_fn, &[arg_val.value().into()], "set_from_dict")
                    .unwrap();
                let set_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::new(
                    set_ptr.value(),
                    PyType::Set(key_type.clone()),
                    None,
                ))
            }
            _ => Err(format!(
                "set() argument must be an iterable, got {:?}",
                arg_val.ty()
            )),
        }
    }
}
