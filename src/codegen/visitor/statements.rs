/// Statement visitor implementation for code generation
use super::super::CodeGen;
use crate::ast::*;
use crate::types::{PyType, PyValue};

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
        let coerced_val = self.coerce_value_to_type(val.value(), var_type)?;

        self.cg.builder.build_store(alloca, coerced_val).unwrap();
        let var = PyValue::from_ast_type(var_type, coerced_val, Some(alloca))?;
        self.variables.insert(name.to_string(), var);
        Ok(())
    }

    pub(crate) fn visit_assignment_impl(
        &mut self,
        target: &Expression,
        value: &Expression,
    ) -> Result<(), String> {
        match target {
            Expression::Var(name) => {
                // First evaluate the value to get its type
                let val = self.evaluate_expression(value)?;

                // Check if variable exists
                if let Some(var) = self.variables.get(name).cloned() {
                    // Variable exists, store to it
                    var.store_value(&self.cg.builder, &val)?;
                } else {
                    // Variable doesn't exist, create it with inferred type
                    let llvm_type = self.pytype_to_llvm(&val.ty);
                    let alloca = self.create_entry_block_alloca_with_type(name, llvm_type);
                    self.cg.builder.build_store(alloca, val.value()).unwrap();
                    let var = PyValue::new(val.value(), val.ty.clone(), Some(alloca));
                    self.variables.insert(name.to_string(), var);
                }
                Ok(())
            }
            Expression::Subscript { object, index } => {
                let obj = self.evaluate_expression(object)?;
                let idx = self.evaluate_expression(index)?;
                let val = self.evaluate_expression(value)?;

                match &obj.ty {
                    PyType::List(_) => {
                        self.list_setitem(obj.value(), idx.value(), val.value())?;
                        Ok(())
                    }
                    PyType::Dict(_, _) => {
                        self.dict_setitem(obj.value(), idx.value(), val.value())?;
                        Ok(())
                    }
                    _ => panic!("Subscript assignment not supported for type {:?}", obj.ty),
                }
            }
            _ => panic!("Invalid assignment target: {:?}", target),
        }
    }

    pub(crate) fn visit_aug_assignment_impl(
        &mut self,
        target: &Expression,
        op: &AugAssignOp,
        value: &Expression,
    ) -> Result<(), String> {
        match target {
            Expression::Var(name) => {
                let var = self
                    .variables
                    .get(name)
                    .ok_or_else(|| format!("Variable {} not found", name))?
                    .clone();

                // Load current value from the addressable PyValue
                let current = var.load(&self.cg.builder, name);

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
                let result = current.binary_op(&self.cg, &bin_op, &rhs)?;

                // Store result to the addressable variable
                var.store_value(&self.cg.builder, &result)?;
                Ok(())
            }
            _ => panic!("Augmented assignment only supported for variables"),
        }
    }

    pub(crate) fn visit_break_impl(&mut self) -> Result<(), String> {
        if let Some(loop_ctx) = self.loop_stack.last() {
            self.cg
                .builder
                .build_unconditional_branch(loop_ctx.break_block)
                .unwrap();
            Ok(())
        } else {
            Err("Break statement outside of loop".to_string())
        }
    }

    pub(crate) fn visit_continue_impl(&mut self) -> Result<(), String> {
        if let Some(loop_ctx) = self.loop_stack.last() {
            self.cg
                .builder
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
            self.cg.builder.build_return(Some(&val.value())).unwrap();
        } else {
            self.cg.builder.build_return(None).unwrap();
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

    pub(crate) fn visit_delete_impl(&mut self, target: &Expression) -> Result<(), String> {
        match target {
            Expression::Var(_name) => {
                // del variable - not commonly used, could unset the variable
                // For now, we don't support deleting simple variables
                Err("del on variables is not supported".to_string())
            }
            Expression::Subscript { object, index } => {
                let obj = self.evaluate_expression(object)?;
                let idx = self.evaluate_expression(index)?;

                match &obj.ty {
                    PyType::List(_) => {
                        self.list_delitem(obj.value(), idx.value())?;
                        Ok(())
                    }
                    PyType::Dict(_, _) => {
                        self.dict_delitem(obj.value(), idx.value())?;
                        Ok(())
                    }
                    _ => panic!("del not supported for type {:?}", obj.ty),
                }
            }
            _ => panic!("del not supported for target: {:?}", target),
        }
    }
}
