//! Builtin function handling for Python builtins like print, abs, min, max, etc.
//!
//! This module contains all the logic for handling Python builtin functions
//! that need special codegen treatment (type dispatch, multiple args, etc.)

mod bytes;
mod dict;
mod list;
mod math;
mod print;
mod set;

use super::CodeGen;
use crate::ast::Expression;
use crate::types::{MacroKind, PyValue};
use inkwell::values::FunctionValue;

impl<'ctx> CodeGen<'ctx> {
    /// Get a Python builtin function as a PyValue::Macro
    /// Returns None if the name is not a builtin function
    pub fn get_builtin_function(&mut self, name: &str) -> Option<PyValue<'ctx>> {
        let kind = match name {
            "print" => MacroKind::Print,
            "abs" => MacroKind::Abs,
            "round" => MacroKind::Round,
            "len" => MacroKind::Len,
            "min" => MacroKind::Min,
            "max" => MacroKind::Max,
            "pow" => MacroKind::Pow,
            "set" => MacroKind::Set,
            "list" => MacroKind::List,
            "dict" => MacroKind::Dict,
            _ => return None,
        };
        Some(PyValue::macro_fn(kind))
    }

    /// Expand a macro function call
    /// This is the main entry point for macro expansion at call time
    pub fn expand_macro(
        &mut self,
        macro_val: &PyValue<'ctx>,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        let kind = macro_val.get_macro_kind()?;
        match kind {
            MacroKind::Print => self.generate_print_call(args),
            MacroKind::Abs => self.generate_abs_call(args),
            MacroKind::Round => self.generate_round_call(args),
            MacroKind::Min => self.generate_min_call(args),
            MacroKind::Max => self.generate_max_call(args),
            MacroKind::Pow => self.generate_pow_call(args),
            MacroKind::Len => self.generate_len_call(args),
            MacroKind::Set => self.generate_set_call(args),
            MacroKind::List => self.generate_list_call(args),
            MacroKind::Dict => self.generate_dict_call(args),
        }
    }

    /// Get or declare a C builtin function from the builtin table
    pub fn get_or_declare_c_builtin(&mut self, name: &str) -> FunctionValue<'ctx> {
        use super::builtins;

        let builtin = builtins::BUILTIN_TABLE
            .get(name)
            .unwrap_or_else(|| panic!("Unknown builtin function: {}", name));

        self.used_builtin_modules.insert(builtin.module.to_string());

        if let Some(func) = self.module.get_function(builtin.symbol) {
            return func;
        }

        let fn_type = builtin.to_llvm_fn_type(self.context);
        self.module.add_function(builtin.symbol, fn_type, None)
    }
}
