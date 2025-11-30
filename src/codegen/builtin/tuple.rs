//! Tuple operations: indexing and tuple() builtin
//!
//! This module provides tuple-related operations:
//! - Subscript operations (tuple[i])
//! - tuple() builtin function
//!
//! All C runtime functions are in src/runtime/builtins/tuple.c
//! and discovered automatically by build.rs.

use crate::ast::Expression;
use crate::codegen::types::PtrStorage;
use crate::codegen::CodeGen;
use crate::types::{PyType, PyValue};
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGen<'ctx> {
    // ========================================================================
    // Subscript operations (e.g., my_tuple[0])
    // ========================================================================

    /// Get an item at index: tuple[i] -> element
    pub fn tuple_getitem(
        &mut self,
        tuple_val: BasicValueEnum<'ctx>,
        index: BasicValueEnum<'ctx>,
        elem_type: &PyType,
    ) -> Result<PyValue<'ctx>, String> {
        let getitem_fn = self.get_or_declare_c_builtin("tuple_getitem");
        let call = self
            .cg
            .builder
            .build_call(
                getitem_fn,
                &[tuple_val.into(), index.into()],
                "tuple_getitem",
            )
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

    // ========================================================================
    // tuple() builtin function
    // ========================================================================

    /// Generate tuple() builtin call
    /// tuple() -> empty tuple
    /// tuple(str) -> tuple of character ordinals
    /// tuple(bytes) -> tuple of byte values
    /// tuple(list) -> tuple from list elements
    /// tuple(set) -> tuple from set elements
    /// tuple(dict) -> tuple from dict keys
    pub fn generate_tuple_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        if args.is_empty() {
            // Create empty tuple
            let tuple_new_fn = self.get_or_declare_c_builtin("tuple_new");
            let zero = self.cg.ctx.i64_type().const_int(0, false);
            let call_site = self
                .cg
                .builder
                .build_call(tuple_new_fn, &[zero.into()], "tuple_new")
                .unwrap();
            let tuple_ptr = self.extract_ptr_call_result(call_site);

            return Ok(PyValue::Tuple(
                PtrStorage::Direct(tuple_ptr.value().into_pointer_value()),
                vec![],
            ));
        }

        if args.len() != 1 {
            return Err("tuple() takes at most 1 argument".to_string());
        }

        // Generate the argument value
        let arg_val = self.evaluate_expression(&args[0])?;

        match arg_val.ty() {
            PyType::Str => {
                // tuple("hello") -> (104, 101, 108, 108, 111) (character ordinals)
                let tuple_from_str_fn = self.get_or_declare_c_builtin("tuple_from_str");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        tuple_from_str_fn,
                        &[arg_val.value().into()],
                        "tuple_from_str",
                    )
                    .unwrap();
                let tuple_ptr = self.extract_ptr_call_result(call_site);
                // Element types are ints (character ordinals)
                Ok(PyValue::Tuple(
                    PtrStorage::Direct(tuple_ptr.value().into_pointer_value()),
                    vec![PyType::Int],
                ))
            }
            PyType::Bytes => {
                // tuple(b"hello") -> (104, 101, 108, 108, 111) (byte values)
                let tuple_from_bytes_fn = self.get_or_declare_c_builtin("tuple_from_bytes");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        tuple_from_bytes_fn,
                        &[arg_val.value().into()],
                        "tuple_from_bytes",
                    )
                    .unwrap();
                let tuple_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::Tuple(
                    PtrStorage::Direct(tuple_ptr.value().into_pointer_value()),
                    vec![PyType::Int],
                ))
            }
            PyType::List(elem_type) => {
                // tuple([1, 2, 3]) -> (1, 2, 3)
                let tuple_from_list_fn = self.get_or_declare_c_builtin("tuple_from_list");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        tuple_from_list_fn,
                        &[arg_val.value().into()],
                        "tuple_from_list",
                    )
                    .unwrap();
                let tuple_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::Tuple(
                    PtrStorage::Direct(tuple_ptr.value().into_pointer_value()),
                    vec![(*elem_type).clone()],
                ))
            }
            PyType::Set(elem_type) => {
                // tuple({1, 2, 3}) -> (1, 2, 3)
                let tuple_from_set_fn = self.get_or_declare_c_builtin("tuple_from_set");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        tuple_from_set_fn,
                        &[arg_val.value().into()],
                        "tuple_from_set",
                    )
                    .unwrap();
                let tuple_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::Tuple(
                    PtrStorage::Direct(tuple_ptr.value().into_pointer_value()),
                    vec![(*elem_type).clone()],
                ))
            }
            PyType::Dict(key_type, _) => {
                // tuple({"a": 1}) -> ("a",) (tuple of keys)
                let tuple_from_dict_fn = self.get_or_declare_c_builtin("tuple_from_dict");
                let call_site = self
                    .cg
                    .builder
                    .build_call(
                        tuple_from_dict_fn,
                        &[arg_val.value().into()],
                        "tuple_from_dict",
                    )
                    .unwrap();
                let tuple_ptr = self.extract_ptr_call_result(call_site);
                Ok(PyValue::Tuple(
                    PtrStorage::Direct(tuple_ptr.value().into_pointer_value()),
                    vec![(*key_type).clone()],
                ))
            }
            _ => Err(format!(
                "tuple() argument must be an iterable, got {:?}",
                arg_val.ty()
            )),
        }
    }
}
