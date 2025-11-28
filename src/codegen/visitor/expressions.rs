/// Expression visitor implementation for code generation
use super::super::CodeGen;
use crate::ast::*;
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
        // Represent None as a null pointer
        let ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
        Ok(PyValue::none(ptr_type.const_null().into()))
    }

    pub(crate) fn visit_var_impl(&mut self, name: &str) -> Result<PyValue<'ctx>, String> {
        let (var, load_type, py_type) = self
            .variables
            .get(name)
            .ok_or_else(|| format!("Variable {} not found", name))?
            .clone();
        let ir_val = self.builder.build_load(load_type, var, name).unwrap();
        Ok(PyValue::new(ir_val, py_type))
    }

    pub(crate) fn visit_binop_impl(
        &mut self,
        op: &BinaryOp,
        left: &Expression,
        right: &Expression,
    ) -> Result<(), String> {
        self.generate_binary_op(op, left, right)?;
        Ok(())
    }

    pub(crate) fn visit_unaryop_impl(
        &mut self,
        op: &UnaryOp,
        operand: &Expression,
    ) -> Result<(), String> {
        self.generate_unary_op(op, operand)?;
        Ok(())
    }

    pub(crate) fn visit_call_impl(
        &mut self,
        func: &Expression,
        args: &[Expression],
    ) -> Result<(), String> {
        match func {
            // Simple function call: function_name()
            Expression::Var(name) => {
                self.generate_call(name, args)?;
                Ok(())
            }
            // Qualified call: module.function()
            Expression::Attribute { object, attr } => {
                if let Expression::Var(module_name) = object.as_ref() {
                    let qualified_name = format!("{}.{}", module_name, attr);
                    self.generate_call(&qualified_name, args)?;
                    Ok(())
                } else {
                    Err("Only simple module.function() calls are supported".to_string())
                }
            }
            _ => Err(
                "Only simple function calls and module.function() calls are supported".to_string(),
            ),
        }
    }
}
