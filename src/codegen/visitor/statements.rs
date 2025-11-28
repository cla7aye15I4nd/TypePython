/// Statement visitor implementation for code generation
use super::super::CodeGen;
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
        let val = self.evaluate_expression(value)?;
        self.builder.build_store(alloca, val).unwrap();
        let llvm_type = self.type_to_llvm(var_type);
        self.variables.insert(name.to_string(), (alloca, llvm_type));
        Ok(())
    }

    pub(crate) fn visit_assignment_impl(
        &mut self,
        target: &AssignTarget,
        value: &Expression,
    ) -> Result<(), String> {
        match target {
            AssignTarget::Var(name) => {
                let (var, _) = *self
                    .variables
                    .get(name)
                    .ok_or_else(|| format!("Variable {} not found", name))?;
                let val = self.evaluate_expression(value)?;
                self.builder.build_store(var, val).unwrap();
                Ok(())
            }
            AssignTarget::Attribute { .. } => {
                Err("Attribute assignment is not yet supported".to_string())
            }
            AssignTarget::Subscript { .. } => {
                Err("Subscript assignment is not yet supported".to_string())
            }
        }
    }

    pub(crate) fn visit_aug_assignment_impl(
        &mut self,
        target: &AssignTarget,
        op: &AugAssignOp,
        value: &Expression,
    ) -> Result<(), String> {
        // For now, only support simple variable targets
        match target {
            AssignTarget::Var(name) => {
                let (var, load_type) = *self
                    .variables
                    .get(name)
                    .ok_or_else(|| format!("Variable {} not found", name))?;

                // Load current value
                let current = self.builder.build_load(load_type, var, name).unwrap();

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

                // Generate binary operation
                // We already have the loaded values, so we need to do the operation directly
                // For now, only support integer operations
                use inkwell::values::BasicValueEnum;
                let result: BasicValueEnum = if current.is_int_value() && rhs.is_int_value() {
                    let lhs_int = current.into_int_value();
                    let rhs_int = rhs.into_int_value();

                    let int_result = match bin_op {
                        BinaryOp::Add => self
                            .builder
                            .build_int_add(lhs_int, rhs_int, "addtmp")
                            .unwrap(),
                        BinaryOp::Sub => self
                            .builder
                            .build_int_sub(lhs_int, rhs_int, "subtmp")
                            .unwrap(),
                        BinaryOp::Mul => self
                            .builder
                            .build_int_mul(lhs_int, rhs_int, "multmp")
                            .unwrap(),
                        BinaryOp::Div => self
                            .builder
                            .build_int_signed_div(lhs_int, rhs_int, "divtmp")
                            .unwrap(),
                        BinaryOp::FloorDiv => self
                            .builder
                            .build_int_signed_div(lhs_int, rhs_int, "floordivtmp")
                            .unwrap(),
                        BinaryOp::Mod => self
                            .builder
                            .build_int_signed_rem(lhs_int, rhs_int, "modtmp")
                            .unwrap(),
                        BinaryOp::BitOr => {
                            self.builder.build_or(lhs_int, rhs_int, "ortmp").unwrap()
                        }
                        BinaryOp::BitXor => {
                            self.builder.build_xor(lhs_int, rhs_int, "xortmp").unwrap()
                        }
                        BinaryOp::BitAnd => {
                            self.builder.build_and(lhs_int, rhs_int, "andtmp").unwrap()
                        }
                        BinaryOp::LShift => self
                            .builder
                            .build_left_shift(lhs_int, rhs_int, "lshifttmp")
                            .unwrap(),
                        BinaryOp::RShift => self
                            .builder
                            .build_right_shift(lhs_int, rhs_int, true, "rshifttmp")
                            .unwrap(),
                        _ => {
                            return Err(format!(
                                "Unsupported augmented assignment operator: {:?}",
                                op
                            ))
                        }
                    };
                    int_result.into()
                } else {
                    return Err(
                        "Augmented assignment only supports integer types for now".to_string()
                    );
                };

                // Store result
                self.builder.build_store(var, result).unwrap();
                Ok(())
            }
            AssignTarget::Attribute { .. } => {
                Err("Augmented attribute assignment is not yet supported".to_string())
            }
            AssignTarget::Subscript { .. } => {
                Err("Augmented subscript assignment is not yet supported".to_string())
            }
        }
    }

    pub(crate) fn visit_for_impl(
        &mut self,
        _var: &str,
        _iterable: &Expression,
        _body: &[Statement],
    ) -> Result<(), String> {
        Err("For loops are not yet supported in code generation".to_string())
    }

    pub(crate) fn visit_break_impl(&mut self) -> Result<(), String> {
        Err("Break statements are not yet supported in code generation".to_string())
    }

    pub(crate) fn visit_continue_impl(&mut self) -> Result<(), String> {
        Err("Continue statements are not yet supported in code generation".to_string())
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
        self.evaluate_expression(expr)?;
        Ok(())
    }
}
