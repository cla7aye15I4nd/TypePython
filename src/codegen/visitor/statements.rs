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

        // Coerce the value to match the declared type if needed
        let coerced_val = self.coerce_value_to_type(val, var_type)?;

        self.builder.build_store(alloca, coerced_val).unwrap();
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
                        BinaryOp::FloorDiv => {
                            // Call floordiv_int for Python-style floor division
                            use inkwell::values::AnyValue;
                            let floordiv_fn = self.get_or_declare_builtin_function("floordiv_int");
                            let call_site = self
                                .builder
                                .build_call(
                                    floordiv_fn,
                                    &[lhs_int.into(), rhs_int.into()],
                                    "floordivtmp",
                                )
                                .unwrap();
                            let any_val = call_site.as_any_value_enum();
                            if let inkwell::values::AnyValueEnum::IntValue(iv) = any_val {
                                iv
                            } else {
                                return Err("floordiv_int did not return an int value".to_string());
                            }
                        }
                        BinaryOp::Mod => {
                            // Call mod_int for Python-style modulo
                            use inkwell::values::AnyValue;
                            let mod_fn = self.get_or_declare_builtin_function("mod_int");
                            let call_site = self
                                .builder
                                .build_call(mod_fn, &[lhs_int.into(), rhs_int.into()], "modtmp")
                                .unwrap();
                            let any_val = call_site.as_any_value_enum();
                            if let inkwell::values::AnyValueEnum::IntValue(iv) = any_val {
                                iv
                            } else {
                                return Err("mod_int did not return an int value".to_string());
                            }
                        }
                        BinaryOp::Pow => {
                            // Call pow_int builtin
                            use inkwell::values::AnyValue;
                            let pow_fn = self.get_or_declare_builtin_function("pow_int");
                            let call_site = self
                                .builder
                                .build_call(pow_fn, &[lhs_int.into(), rhs_int.into()], "powtmp")
                                .unwrap();
                            let any_val = call_site.as_any_value_enum();
                            if let inkwell::values::AnyValueEnum::IntValue(iv) = any_val {
                                iv
                            } else {
                                return Err("pow_int did not return an int value".to_string());
                            }
                        }
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
                                "Unsupported augmented assignment operator for int: {:?}",
                                op
                            ))
                        }
                    };
                    int_result.into()
                } else if current.is_float_value() && rhs.is_float_value() {
                    let lhs_float = current.into_float_value();
                    let rhs_float = rhs.into_float_value();

                    let float_result = match bin_op {
                        BinaryOp::Add => self
                            .builder
                            .build_float_add(lhs_float, rhs_float, "faddtmp")
                            .unwrap(),
                        BinaryOp::Sub => self
                            .builder
                            .build_float_sub(lhs_float, rhs_float, "fsubtmp")
                            .unwrap(),
                        BinaryOp::Mul => self
                            .builder
                            .build_float_mul(lhs_float, rhs_float, "fmultmp")
                            .unwrap(),
                        BinaryOp::Div => self
                            .builder
                            .build_float_div(lhs_float, rhs_float, "fdivtmp")
                            .unwrap(),
                        BinaryOp::FloorDiv => {
                            // Float floor division: divide then floor
                            use inkwell::values::AnyValue;
                            let div_result = self
                                .builder
                                .build_float_div(lhs_float, rhs_float, "fdivtmp")
                                .unwrap();
                            let floor_fn = self.get_or_declare_builtin_function("floor_float");
                            let call_site = self
                                .builder
                                .build_call(floor_fn, &[div_result.into()], "floortmp")
                                .unwrap();
                            let any_val = call_site.as_any_value_enum();
                            if let inkwell::values::AnyValueEnum::FloatValue(fv) = any_val {
                                fv
                            } else {
                                return Err("floor_float did not return a float value".to_string());
                            }
                        }
                        BinaryOp::Mod => {
                            // Call mod_float for Python-style float modulo
                            use inkwell::values::AnyValue;
                            let fmod_fn = self.get_or_declare_builtin_function("mod_float");
                            let call_site = self
                                .builder
                                .build_call(
                                    fmod_fn,
                                    &[lhs_float.into(), rhs_float.into()],
                                    "fmodtmp",
                                )
                                .unwrap();
                            let any_val = call_site.as_any_value_enum();
                            if let inkwell::values::AnyValueEnum::FloatValue(fv) = any_val {
                                fv
                            } else {
                                return Err("mod_float did not return a float value".to_string());
                            }
                        }
                        BinaryOp::Pow => {
                            // Call pow_float builtin for floats
                            use inkwell::values::AnyValue;
                            let pow_fn = self.get_or_declare_builtin_function("pow_float");
                            let call_site = self
                                .builder
                                .build_call(
                                    pow_fn,
                                    &[lhs_float.into(), rhs_float.into()],
                                    "fpowtmp",
                                )
                                .unwrap();
                            let any_val = call_site.as_any_value_enum();
                            if let inkwell::values::AnyValueEnum::FloatValue(fv) = any_val {
                                fv
                            } else {
                                return Err("pow_float did not return a float value".to_string());
                            }
                        }
                        _ => {
                            return Err(format!(
                                "Unsupported augmented assignment operator for float: {:?}",
                                op
                            ))
                        }
                    };
                    float_result.into()
                } else {
                    return Err(
                        "Augmented assignment requires matching types (both int or both float)"
                            .to_string(),
                    );
                };

                // Store result
                self.builder.build_store(var, result).unwrap();
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
