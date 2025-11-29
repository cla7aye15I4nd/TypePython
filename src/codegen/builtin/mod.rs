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
mod str;

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
            // Type conversion builtins
            "int" => MacroKind::Int,
            "float" => MacroKind::Float,
            "bool" => MacroKind::Bool,
            "str" => MacroKind::Str,
            // String representation builtins
            "bin" => MacroKind::Bin,
            "hex" => MacroKind::Hex,
            "oct" => MacroKind::Oct,
            "chr" => MacroKind::Chr,
            "ord" => MacroKind::Ord,
            "ascii" => MacroKind::Ascii,
            // Sequence builtins
            "sum" => MacroKind::Sum,
            "sorted" => MacroKind::Sorted,
            "reversed" => MacroKind::Reversed,
            // Math builtins
            "divmod" => MacroKind::Divmod,
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
            // Type conversion builtins
            MacroKind::Int => self.generate_int_call(args),
            MacroKind::Float => self.generate_float_call(args),
            MacroKind::Bool => self.generate_bool_call(args),
            MacroKind::Str => self.generate_str_call(args),
            // String representation builtins
            MacroKind::Bin => self.generate_bin_call(args),
            MacroKind::Hex => self.generate_hex_call(args),
            MacroKind::Oct => self.generate_oct_call(args),
            MacroKind::Chr => self.generate_chr_call(args),
            MacroKind::Ord => self.generate_ord_call(args),
            MacroKind::Ascii => self.generate_ascii_call(args),
            // Sequence builtins
            MacroKind::Sum => self.generate_sum_call(args),
            MacroKind::Sorted => self.generate_sorted_call(args),
            MacroKind::Reversed => self.generate_reversed_call(args),
            // Math builtins
            MacroKind::Divmod => self.generate_divmod_call(args),
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
