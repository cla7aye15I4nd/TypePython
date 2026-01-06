//! Runtime function declarations for code generation

use inkwell::AddressSpace;

use super::context::CodegenContext;

impl<'ctx> CodegenContext<'ctx> {
    /// Declare runtime functions
    pub fn declare_runtime_functions(&mut self) {
        /// Macro to simplify runtime function declarations
        macro_rules! declare_fn {
            ($ret_type:expr, $name:expr $(, $arg:expr)*) => {
                let fn_type = $ret_type.fn_type(&[$($arg.into()),*], false);
                self.module.add_function($name, fn_type, None);
            };
        }
        let i64_type = self.context.i64_type();
        let void_type = self.context.void_type();
        let i8_ptr_type = self.context.ptr_type(AddressSpace::default());

        // List type: { i64*, i64, i64 }
        let _list_type = self.context.struct_type(
            &[
                i8_ptr_type.into(), // data pointer (cast from i64*)
                i64_type.into(),    // len
                i64_type.into(),    // cap
            ],
            false,
        );
        let list_ptr_type = self.context.ptr_type(AddressSpace::default());

        // list_new() -> List*
        declare_fn!(list_ptr_type, "__pyc___builtin___list___init__");

        // list_append(List*, i64) -> void
        declare_fn!(
            void_type,
            "__pyc___builtin___list_append",
            list_ptr_type,
            i64_type
        );

        // list_getitem(List*, i64) -> i64
        declare_fn!(
            i64_type,
            "__pyc___builtin___list___getitem__",
            list_ptr_type,
            i64_type
        );

        // list_setitem(List*, i64, i64) -> void
        declare_fn!(
            void_type,
            "__pyc___builtin___list___setitem__",
            list_ptr_type,
            i64_type,
            i64_type
        );

        // list_len(List*) -> i64
        declare_fn!(i64_type, "__pyc___builtin___list___len__", list_ptr_type);

        let i8_type = self.context.i8_type();

        // class_new(i64) -> void*
        declare_fn!(i8_ptr_type, "class_new", i64_type);

        // ByteArray type: { u8*, i64, i64 } (same layout as List but with u8 elements)
        let bytearray_ptr_type = self.context.ptr_type(AddressSpace::default());

        // bytearray_new() -> ByteArray*
        declare_fn!(bytearray_ptr_type, "__pyc___builtin___bytearray___init__");

        // bytearray_append(ByteArray*, i64) -> void
        declare_fn!(
            void_type,
            "__pyc___builtin___bytearray_append",
            bytearray_ptr_type,
            i64_type
        );

        // bytearray_getitem(ByteArray*, i64) -> i64
        declare_fn!(
            i64_type,
            "__pyc___builtin___bytearray___getitem__",
            bytearray_ptr_type,
            i64_type
        );

        // bytearray_setitem(ByteArray*, i64, i64) -> void
        declare_fn!(
            void_type,
            "__pyc___builtin___bytearray___setitem__",
            bytearray_ptr_type,
            i64_type,
            i64_type
        );

        // bytearray_len(ByteArray*) -> i64
        declare_fn!(
            i64_type,
            "__pyc___builtin___bytearray___len__",
            bytearray_ptr_type
        );

        // Bytes* type (same layout as other pointer types)
        let bytes_ptr_type = self.context.ptr_type(AddressSpace::default());

        // bytes_len(Bytes*) -> i64
        declare_fn!(i64_type, "__pyc___builtin___bytes___len__", bytes_ptr_type);

        // bytes_getitem(Bytes*, i64 index) -> i64
        declare_fn!(
            i64_type,
            "__pyc___builtin___bytes___getitem__",
            bytes_ptr_type,
            i64_type
        );

        // String* type (same layout as other pointer types)
        let string_ptr_type = self.context.ptr_type(AddressSpace::default());

        // str_len(String*) -> i64
        declare_fn!(i64_type, "__pyc___builtin___str___len__", string_ptr_type);

        // str_str(String*) -> String* (identity for __str__)
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___str___str__",
            string_ptr_type
        );

        // str_repr(String*) -> String* (returns quoted string with escapes)
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___str___repr__",
            string_ptr_type
        );

        // str_getitem(String*, i64) -> i64 (returns Unicode codepoint at index)
        declare_fn!(
            i64_type,
            "__pyc___builtin___str___getitem__",
            string_ptr_type,
            i64_type
        );

        // str_add(String*, String*) -> String* (string concatenation)
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___str___add__",
            string_ptr_type,
            string_ptr_type
        );

        // String comparison operators
        declare_fn!(
            i8_type,
            "__pyc___builtin___str___eq__",
            string_ptr_type,
            string_ptr_type
        );

        declare_fn!(
            i8_type,
            "__pyc___builtin___str___ne__",
            string_ptr_type,
            string_ptr_type
        );

        declare_fn!(
            i8_type,
            "__pyc___builtin___str___lt__",
            string_ptr_type,
            string_ptr_type
        );

        declare_fn!(
            i8_type,
            "__pyc___builtin___str___le__",
            string_ptr_type,
            string_ptr_type
        );

        declare_fn!(
            i8_type,
            "__pyc___builtin___str___gt__",
            string_ptr_type,
            string_ptr_type
        );

        declare_fn!(
            i8_type,
            "__pyc___builtin___str___ge__",
            string_ptr_type,
            string_ptr_type
        );

        // Phase 3: String methods

        // Case conversion
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___str_lower",
            string_ptr_type
        );

        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___str_upper",
            string_ptr_type
        );

        // Whitespace operations
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___str_strip",
            string_ptr_type
        );

        // String search
        declare_fn!(
            i64_type,
            "__pyc___builtin___str_find",
            string_ptr_type,
            string_ptr_type
        );

        declare_fn!(
            i8_type,
            "__pyc___builtin___str_startswith",
            string_ptr_type,
            string_ptr_type
        );

        declare_fn!(
            i8_type,
            "__pyc___builtin___str_endswith",
            string_ptr_type,
            string_ptr_type
        );

        // String modification
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___str_replace",
            string_ptr_type,
            string_ptr_type,
            string_ptr_type
        );

        // Character classification
        declare_fn!(i8_type, "__pyc___builtin___str_isalpha", string_ptr_type);

        declare_fn!(i8_type, "__pyc___builtin___str_isdigit", string_ptr_type);

        declare_fn!(i8_type, "__pyc___builtin___str_isspace", string_ptr_type);

        // int.__print__(i64) -> void (prints int without newline)
        declare_fn!(void_type, "__pyc___builtin___int___print__", i64_type);

        // bool.__print__(i8) -> void (prints bool without newline)
        declare_fn!(void_type, "__pyc___builtin___bool___print__", i8_type);

        // float.__print__(f64) -> void (prints float without newline)
        let f64_type = self.context.f64_type();
        declare_fn!(void_type, "__pyc___builtin___float___print__", f64_type);

        // bytes.__str__(Bytes*) -> String*
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___bytes___str__",
            bytes_ptr_type
        );

        // bytes.__repr__(Bytes*) -> String*
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___bytes___repr__",
            bytes_ptr_type
        );

        // bytearray.__str__(ByteArray*) -> String*
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___bytearray___str__",
            bytearray_ptr_type
        );

        // bytearray.__repr__(ByteArray*) -> String*
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___bytearray___repr__",
            bytearray_ptr_type
        );

        // list.__str__(List*) -> String*
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___list___str__",
            list_ptr_type
        );

        // list.__repr__(List*) -> String*
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___list___repr__",
            list_ptr_type
        );

        // Low-level I/O functions (no newlines)
        // write_str_impl(const char*) -> void
        declare_fn!(void_type, "write_str_impl", i8_ptr_type);

        // write_string_impl(String*) -> void
        declare_fn!(void_type, "write_string_impl", string_ptr_type);

        // write_char_impl(char) -> void
        declare_fn!(void_type, "write_char_impl", i8_type);

        // write_newline_impl() -> void
        declare_fn!(void_type, "write_newline_impl");

        // write_space_impl() -> void
        declare_fn!(void_type, "write_space_impl");

        // int64_to_str_impl(i64, char*) -> char*
        declare_fn!(i8_ptr_type, "int64_to_str_impl", i64_type, i8_ptr_type);

        // ================================================================
        // Exception handling runtime functions
        // ================================================================

        // Exception* type (pointer to Exception struct)
        let exception_ptr_type = self.context.ptr_type(AddressSpace::default());
        let i32_type = self.context.i32_type();

        // __pyc_setjmp(JmpBuf*) -> i32
        // Returns 0 on direct call, non-zero on longjmp
        declare_fn!(i32_type, "__pyc_setjmp", i8_ptr_type);

        // __pyc_longjmp(JmpBuf*, i32) -> void (noreturn)
        declare_fn!(void_type, "__pyc_longjmp", i8_ptr_type, i32_type);

        // __pyc_push_exception_frame(ExceptionFrame*) -> void
        declare_fn!(void_type, "__pyc_push_exception_frame", i8_ptr_type);

        // __pyc_pop_exception_frame() -> void
        declare_fn!(void_type, "__pyc_pop_exception_frame");

        // __pyc_get_exception() -> Exception*
        declare_fn!(exception_ptr_type, "__pyc_get_exception");

        // __pyc_set_exception(Exception*) -> void
        declare_fn!(void_type, "__pyc_set_exception", exception_ptr_type);

        // __pyc_clear_exception() -> void
        declare_fn!(void_type, "__pyc_clear_exception");

        // __pyc_has_exception() -> i32 (boolean)
        declare_fn!(i32_type, "__pyc_has_exception");

        // __pyc_raise(Exception*) -> void (noreturn)
        declare_fn!(void_type, "__pyc_raise", exception_ptr_type);

        // __pyc_reraise() -> void (noreturn)
        declare_fn!(void_type, "__pyc_reraise");

        // Exception.__init__(String* message) -> Exception*
        declare_fn!(
            exception_ptr_type,
            "__pyc___builtin___Exception___init__",
            string_ptr_type
        );

        // __pyc_exception_new(String* type_name, String* message, String* parent_types) -> Exception*
        declare_fn!(
            exception_ptr_type,
            "__pyc_exception_new",
            string_ptr_type,
            string_ptr_type,
            string_ptr_type
        );

        // Exception.__str__(Exception*) -> String*
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___Exception___str__",
            exception_ptr_type
        );

        // Exception.__repr__(Exception*) -> String*
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___Exception___repr__",
            exception_ptr_type
        );

        // __pyc_exception_matches(Exception*, const char* type_name) -> i32
        declare_fn!(
            i32_type,
            "__pyc_exception_matches",
            exception_ptr_type,
            i8_ptr_type
        );

        // ================================================================
        // Range iterator runtime functions
        // ================================================================

        // Range* type (pointer to Range struct)
        let range_ptr_type = self.context.ptr_type(AddressSpace::default());

        // range(stop) -> Range*
        declare_fn!(range_ptr_type, "__pyc___builtin___range_1", i64_type);

        // range(start, stop) -> Range*
        declare_fn!(
            range_ptr_type,
            "__pyc___builtin___range_2",
            i64_type,
            i64_type
        );

        // range(start, stop, step) -> Range*
        declare_fn!(
            range_ptr_type,
            "__pyc___builtin___range_3",
            i64_type,
            i64_type,
            i64_type
        );

        // range.__iter__(Range*) -> Range*
        declare_fn!(
            range_ptr_type,
            "__pyc___builtin___range___iter__",
            range_ptr_type
        );

        // range.__next__(Range*) -> i64
        declare_fn!(i64_type, "__pyc___builtin___range___next__", range_ptr_type);

        // range.__len__(Range*) -> i64
        declare_fn!(i64_type, "__pyc___builtin___range___len__", range_ptr_type);

        // range.__str__(Range*) -> String*
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___range___str__",
            range_ptr_type
        );

        // range.__repr__(Range*) -> String*
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___range___repr__",
            range_ptr_type
        );

        // range.__dealloc__(Range*) -> void
        declare_fn!(
            void_type,
            "__pyc___builtin___range___dealloc__",
            range_ptr_type
        );

        // ================================================================
        // List iterator runtime functions
        // ================================================================

        // ListIterator* type (pointer to ListIterator struct)
        let list_iterator_ptr_type = self.context.ptr_type(AddressSpace::default());

        // list.__iter__(List*) -> ListIterator*
        declare_fn!(
            list_iterator_ptr_type,
            "__pyc___builtin___list___iter__",
            list_ptr_type
        );

        // list_iterator.__iter__(ListIterator*) -> ListIterator*
        declare_fn!(
            list_iterator_ptr_type,
            "__pyc___builtin___list_iterator___iter__",
            list_iterator_ptr_type
        );

        // list_iterator.__next__(ListIterator*) -> i64
        declare_fn!(
            i64_type,
            "__pyc___builtin___list_iterator___next__",
            list_iterator_ptr_type
        );

        // list_iterator.__dealloc__(ListIterator*) -> void
        declare_fn!(
            void_type,
            "__pyc___builtin___list_iterator___dealloc__",
            list_iterator_ptr_type
        );

        // ================================================================
        // StopIteration exception runtime functions
        // ================================================================

        // __pyc_stop_iteration() -> Exception* (singleton)
        declare_fn!(exception_ptr_type, "__pyc_stop_iteration");

        // StopIteration.__init__() -> Exception*
        declare_fn!(
            exception_ptr_type,
            "__pyc___builtin___StopIteration___init__"
        );

        // StopIteration.__str__(Exception*) -> String*
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___StopIteration___str__",
            exception_ptr_type
        );

        // StopIteration.__repr__(Exception*) -> String*
        declare_fn!(
            string_ptr_type,
            "__pyc___builtin___StopIteration___repr__",
            exception_ptr_type
        );
    }
}
