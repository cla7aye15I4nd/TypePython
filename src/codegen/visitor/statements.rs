/// Statement visitor implementation for code generation
use super::super::CodeGen;
use crate::ast::*;
use crate::types::{CgCtx, PyValue};

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
        let val = self.evaluate_expression(value)?;

        // Coerce the value to match the declared type if needed
        let coerced_val = self.coerce_value_to_type(val.value, var_type)?;

        self.builder.build_store(alloca, coerced_val).unwrap();
        let llvm_type = self.type_to_llvm(var_type);
        // Create an addressable PyValue for the variable
        let var = PyValue::from_ast_type_addressable(var_type, coerced_val, alloca, llvm_type)?;
        self.variables.insert(name.to_string(), var);
        Ok(())
    }

    pub(crate) fn visit_assignment_impl(
        &mut self,
        target: &AssignTarget,
        value: &Expression,
    ) -> Result<(), String> {
        match target {
            AssignTarget::Var(name) => {
                let var = self
                    .variables
                    .get(name)
                    .ok_or_else(|| format!("Variable {} not found", name))?
                    .clone();
                let val = self.evaluate_expression(value)?;
                var.store_value(&self.builder, &val)?;
                Ok(())
            }
            AssignTarget::Attribute { .. } => {
                todo!("Attribute assignment")
            }
            AssignTarget::Subscript { .. } => {
                todo!("Subscript assignment")
            }
        }
    }

    pub(crate) fn visit_aug_assignment_impl(
        &mut self,
        target: &AssignTarget,
        op: &AugAssignOp,
        value: &Expression,
    ) -> Result<(), String> {
        match target {
            AssignTarget::Var(name) => {
                let var = self
                    .variables
                    .get(name)
                    .ok_or_else(|| format!("Variable {} not found", name))?
                    .clone();

                // Load current value from the addressable PyValue
                let current = var.load(&self.builder, name);

                // Evaluate RHS
                let rhs = self.evaluate_expression(value)?;

                // Convert augmented op to binary op
                let bin_op = match op {
                    AugAssignOp::Add => BinaryOp::Add,
                    AugAssignOp::Sub => BinaryOp::Sub,
                    AugAssignOp::Mul => BinaryOp::Mul,
                    AugAssignOp::Div => BinaryOp::Div,
                    AugAssignOp::FloorDiv => BinaryOp::FloorDiv,
                    AugAssignOp::Mod => BinaryOp::Mod,
                    AugAssignOp::Pow => BinaryOp::Pow,
                    AugAssignOp::BitOr => BinaryOp::BitOr,
                    AugAssignOp::BitXor => BinaryOp::BitXor,
                    AugAssignOp::BitAnd => BinaryOp::BitAnd,
                    AugAssignOp::LShift => BinaryOp::LShift,
                    AugAssignOp::RShift => BinaryOp::RShift,
                };

                // Delegate to the left type's implementation
                let cg = CgCtx::new(self.context, &self.builder, &self.module);
                let result = current.binary_op(&cg, &bin_op, &rhs)?;

                // Store result to the addressable variable
                var.store_value(&self.builder, &result)?;
                Ok(())
            }
            AssignTarget::Attribute { .. } => {
                todo!("Augmented attribute assignment")
            }
            AssignTarget::Subscript { .. } => {
                todo!("Augmented subscript assignment")
            }
        }
    }

    pub(crate) fn visit_for_impl(
        &mut self,
        _var: &str,
        _iterable: &Expression,
        _body: &[Statement],
    ) -> Result<(), String> {
        todo!("For loops")
    }

    pub(crate) fn visit_break_impl(&mut self) -> Result<(), String> {
        if let Some(loop_ctx) = self.loop_stack.last() {
            self.builder
                .build_unconditional_branch(loop_ctx.break_block)
                .unwrap();
            Ok(())
        } else {
            Err("Break statement outside of loop".to_string())
        }
    }

    pub(crate) fn visit_continue_impl(&mut self) -> Result<(), String> {
        if let Some(loop_ctx) = self.loop_stack.last() {
            self.builder
                .build_unconditional_branch(loop_ctx.continue_block)
                .unwrap();
            Ok(())
        } else {
            Err("Continue statement outside of loop".to_string())
        }
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
            let val = self.evaluate_expression(expr)?;
            self.builder.build_return(Some(&val.value)).unwrap();
        } else {
            self.builder.build_return(None).unwrap();
        }
        Ok(())
    }

    pub(crate) fn visit_pass_impl(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub(crate) fn visit_expr_statement_impl(&mut self, expr: &Expression) -> Result<(), String> {
        self.evaluate_expression(expr)?;
        Ok(())
    }
}
