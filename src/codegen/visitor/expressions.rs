/// Expression visitor implementation for code generation
use super::super::CodeGen;
use crate::types::PyValue;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn visit_int_lit_impl(&mut self, val: i64) -> Result<PyValue<'ctx>, String> {
        let ir_val = self.context.i64_type().const_int(val as u64, false).into();
        Ok(PyValue::int(ir_val))
    }

    pub(crate) fn visit_float_lit_impl(&mut self, val: f64) -> Result<PyValue<'ctx>, String> {
        let ir_val = self.context.f64_type().const_float(val).into();
        Ok(PyValue::float(ir_val))
    }

    pub(crate) fn visit_str_lit_impl(&mut self, _val: &str) -> Result<PyValue<'ctx>, String> {
        todo!("str type (use bytes instead)")
    }

    pub(crate) fn visit_bytes_lit_impl(&mut self, val: &str) -> Result<PyValue<'ctx>, String> {
        // Bytes literals are the same as string literals in C (char*)
        // They're both null-terminated byte sequences
        let str_name = if let Some(&id) = self.strings.get(val) {
            format!(".bytes_{}", id)
        } else {
            let id = self.strings.len() as u64;
            self.strings.insert(val.to_string(), id);
            format!(".bytes_{}", id)
        };
        let str_const = self
            .builder
            .build_global_string_ptr(val, &str_name)
            .unwrap();
        Ok(PyValue::bytes(str_const.as_pointer_value().into()))
    }

    pub(crate) fn visit_bool_lit_impl(&mut self, val: bool) -> Result<PyValue<'ctx>, String> {
        let ir_val = self.context.bool_type().const_int(val as u64, false).into();
        Ok(PyValue::bool(ir_val))
    }

    pub(crate) fn visit_none_lit_impl(&mut self) -> Result<PyValue<'ctx>, String> {
        // Represent None as i32(0) - consistent with type_to_llvm(Type::None)
        let ir_val = self.context.i32_type().const_zero().into();
        Ok(PyValue::none(ir_val))
    }

    pub(crate) fn visit_var_impl(&mut self, name: &str) -> Result<PyValue<'ctx>, String> {
        // First check local variables (stack allocations)
        if let Some(var) = self.variables.get(name) {
            return Ok(var.load(&self.builder, name));
        }

        // Check if it's a function in the current LLVM module (local functions)
        // This must come before global_variables because global_variables contains
        // placeholder functions from preprocessing
        let mangled_name = self.mangle_function_name(&self.module_name, name);
        if let Some(function) = self.module.get_function(&mangled_name) {
            // Get type info from global_variables if available (for correct return type)
            let (param_types, return_type) = if let Some(global) = self.global_variables.get(name) {
                if let Ok(info) = global.get_function() {
                    (info.param_types.clone(), info.return_type.clone())
                } else {
                    (vec![], crate::types::PyType::None)
                }
            } else {
                (vec![], crate::types::PyType::None)
            };

            return Ok(PyValue::function(crate::types::FunctionInfo {
                mangled_name,
                function,
                param_types,
                return_type,
                bound_args: vec![],
            }));
        }

        // Then check global variables (imported modules)
        if let Some(global) = self.global_variables.get(name) {
            return Ok(global.clone());
        }

        // Check if it's a builtin function
        if let Some(func_value) = self.get_builtin_function(name) {
            return Ok(func_value);
        }

        Err(format!("Variable {} not found", name))
    }
}
