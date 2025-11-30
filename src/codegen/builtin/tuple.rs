//! Tuple operations: indexing and tuple() builtin
//!
//! This module provides tuple-related operations:
//! - Subscript operations (tuple[i])
//! - tuple() builtin function
//!
//! All C runtime functions are in src/runtime/builtins/tuple.c
//! and discovered automatically by build.rs.

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
}
