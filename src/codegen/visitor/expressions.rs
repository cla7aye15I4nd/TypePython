/// Expression visitor implementation for code generation
use super::super::CodeGen;
use crate::ast::*;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn visit_int_lit_impl(&mut self, val: i64) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(self.context.i64_type().const_int(val as u64, false).into())
    }

    pub(crate) fn visit_float_lit_impl(
        &mut self,
        val: f64,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(self.context.f64_type().const_float(val).into())
    }

    pub(crate) fn visit_str_lit_impl(&mut self, val: &str) -> Result<BasicValueEnum<'ctx>, String> {
        let str_name = if let Some(&id) = self.strings.get(val) {
            format!(".str_{}", id)
        } else {
            let id = self.strings.len() as u64;
            self.strings.insert(val.to_string(), id);
            format!(".str_{}", id)
        };
        let str_const = self
            .builder
            .build_global_string_ptr(val, &str_name)
            .unwrap();
        Ok(str_const.as_pointer_value().into())
    }

    pub(crate) fn visit_bool_lit_impl(
        &mut self,
        val: bool,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(self.context.bool_type().const_int(val as u64, false).into())
    }

    pub(crate) fn visit_none_lit_impl(&mut self) -> Result<BasicValueEnum<'ctx>, String> {
        todo!("Handle None literal generation in better way");
    }

    pub(crate) fn visit_var_impl(&mut self, name: &str) -> Result<BasicValueEnum<'ctx>, String> {
        let (var, load_type) = *self
            .variables
            .get(name)
            .ok_or_else(|| format!("Variable {} not found", name))?;
        let val = self.builder.build_load(load_type, var, name).unwrap();
        Ok(val)
    }

    pub(crate) fn visit_binary_op_impl(
        &mut self,
        op: &BinaryOp,
        left: &Expression,
        right: &Expression,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        self.generate_binary_op(op, left, right)
    }

    pub(crate) fn visit_unary_op_impl(
        &mut self,
        op: &UnaryOp,
        operand: &Expression,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        self.generate_unary_op(op, operand)
    }

    pub(crate) fn visit_call_impl(
        &mut self,
        name: &str,
        args: &[Expression],
    ) -> Result<BasicValueEnum<'ctx>, String> {
        self.generate_call(name, args)
    }
}
