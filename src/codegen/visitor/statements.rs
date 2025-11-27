/// Statement visitor implementation for code generation
use super::super::CodeGen;
use crate::ast::visitor::Visitor;
use crate::ast::*;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn visit_var_decl_impl(
        &mut self,
        name: &str,
        var_type: &Type,
        value: &Expression,
    ) -> Result<(), String> {
        let fn_name = self
            .current_function
            .unwrap()
            .get_name()
            .to_string_lossy()
            .to_string();
        let alloca = self.create_entry_block_alloca(&fn_name, name, var_type);
        let val = self.visit_expression(value)?;
        self.builder.build_store(alloca, val).unwrap();
        let llvm_type = self.type_to_llvm(var_type);
        self.variables.insert(name.to_string(), (alloca, llvm_type));
        Ok(())
    }

    pub(crate) fn visit_assignment_impl(
        &mut self,
        name: &str,
        value: &Expression,
    ) -> Result<(), String> {
        let (var, _) = *self
            .variables
            .get(name)
            .ok_or_else(|| format!("Variable {} not found", name))?;
        let val = self.visit_expression(value)?;
        self.builder.build_store(var, val).unwrap();
        Ok(())
    }

    pub(crate) fn visit_if_impl(
        &mut self,
        condition: &Expression,
        then_block: &[Statement],
        elif_clauses: &[(Expression, Vec<Statement>)],
        else_block: &Option<Vec<Statement>>,
    ) -> Result<(), String> {
        self.generate_if_statement(condition, then_block, elif_clauses, else_block)
    }

    pub(crate) fn visit_while_impl(
        &mut self,
        condition: &Expression,
        body: &[Statement],
    ) -> Result<(), String> {
        self.generate_while_statement(condition, body)
    }

    pub(crate) fn visit_return_impl(&mut self, expr: Option<&Expression>) -> Result<(), String> {
        if let Some(expr) = expr {
            let val = self.visit_expression(expr)?;
            self.builder.build_return(Some(&val)).unwrap();
        } else {
            self.builder.build_return(None).unwrap();
        }
        Ok(())
    }

    pub(crate) fn visit_pass_impl(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub(crate) fn visit_expr_statement_impl(&mut self, expr: &Expression) -> Result<(), String> {
        self.visit_expression(expr)?;
        Ok(())
    }
}
