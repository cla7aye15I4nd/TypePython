//! Print builtin function implementation

use crate::ast::Expression;
use crate::codegen::CodeGen;
use crate::types::{PyType, PyValue};

impl<'ctx> CodeGen<'ctx> {
    /// Generate code for print() calls
    /// Handles multiple arguments with space separation and newline at end
    pub fn generate_print_call(&mut self, args: &[Expression]) -> Result<PyValue<'ctx>, String> {
        let print_space = self.get_or_declare_c_builtin("print_space");
        let print_newline = self.get_or_declare_c_builtin("print_newline");

        for (i, arg) in args.iter().enumerate() {
            let val = self.evaluate_expression(arg)?;
            let print_fn_name = self.get_print_fn_for_type(&val.ty);
            let print_fn = self.get_or_declare_c_builtin(print_fn_name);

            // print_none takes no arguments, others take the value
            if val.ty == PyType::None {
                self.builder.build_call(print_fn, &[], "print").unwrap();
            } else {
                self.builder
                    .build_call(print_fn, &[val.value().into()], "print")
                    .unwrap();
            }

            // Print space between arguments (not after last)
            if i < args.len() - 1 {
                self.builder
                    .build_call(print_space, &[], "print_space")
                    .unwrap();
            }
        }

        // Print newline at end
        self.builder
            .build_call(print_newline, &[], "print_newline")
            .unwrap();

        Ok(PyValue::none(
            self.context
                .ptr_type(inkwell::AddressSpace::default())
                .const_null()
                .into(),
        ))
    }

    /// Get the appropriate print function name for a type
    fn get_print_fn_for_type(&self, ty: &PyType) -> &'static str {
        match ty {
            PyType::Int => "print_int",
            PyType::Float => "print_float",
            PyType::Bool => "print_bool",
            PyType::Str => "print_str",
            PyType::Bytes => "print_bytes",
            PyType::None => "print_none",
            PyType::List(_) => "print_list",
            PyType::Dict(_, _) => "print_dict",
            PyType::Set(_) => "print_set",
            _ => "print_int", // fallback
        }
    }
}
