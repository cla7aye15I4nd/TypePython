mod visitor;

use crate::ast::visitor::Visitor;
use crate::ast::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::{FloatPredicate, IntPredicate};
use std::collections::HashMap;

pub struct CodeGen<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    pub(crate) variables: HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>,
    pub(crate) current_function: Option<FunctionValue<'ctx>>,
    pub(crate) strings: HashMap<String, u64>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        CodeGen {
            context,
            module,
            builder,
            variables: HashMap::new(),
            current_function: None,
            strings: HashMap::new(),
        }
    }

    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    pub fn generate(&mut self, program: &Program) -> Result<(), String> {
        // Use the visitor pattern to generate code
        self.visit_program(program)?;
        Ok(())
    }

    pub(crate) fn declare_printf(&mut self) {
        let i32_type = self.context.i32_type();
        let str_type = self.context.ptr_type(inkwell::AddressSpace::default());

        // printf signature: i32 @printf(i8*, ...)
        let printf_type = i32_type.fn_type(&[str_type.into()], true); // true = variadic
        self.module.add_function("printf", printf_type, None);
    }

    pub(crate) fn declare_function(
        &mut self,
        func: &Function,
    ) -> Result<FunctionValue<'ctx>, String> {
        let param_types: Vec<BasicMetadataTypeEnum> = func
            .params
            .iter()
            .map(|p| self.type_to_llvm(&p.param_type).into())
            .collect();

        let fn_type = match func.return_type {
            Type::None => self.context.void_type().fn_type(&param_types, false),
            _ => {
                let return_type = self.type_to_llvm(&func.return_type);
                return_type.fn_type(&param_types, false)
            }
        };

        let function = self.module.add_function(&func.name, fn_type, None);

        // Set parameter names
        for (i, param) in func.params.iter().enumerate() {
            function
                .get_nth_param(i as u32)
                .unwrap()
                .set_name(&param.name);
        }

        Ok(function)
    }

    pub(crate) fn generate_main_function(
        &mut self,
        statements: &[Statement],
    ) -> Result<(), String> {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);

        self.current_function = Some(function);

        let entry_bb = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_bb);

        // Clear variables for new function scope
        self.variables.clear();

        // Generate all statements
        for stmt in statements {
            self.visit_statement(stmt)?;
        }

        // Return 0 if not already terminated
        if !self.is_block_terminated() {
            let zero = i32_type.const_int(0, false);
            self.builder.build_return(Some(&zero)).unwrap();
        }

        Ok(())
    }

    pub(crate) fn generate_if_statement(
        &mut self,
        condition: &Expression,
        then_block: &[Statement],
        elif_clauses: &[(Expression, Vec<Statement>)],
        else_block: &Option<Vec<Statement>>,
    ) -> Result<(), String> {
        let cond_val = self.visit_expression(condition)?;
        let cond_int = cond_val.into_int_value();

        let function = self.current_function.unwrap();
        let then_bb = self.context.append_basic_block(function, "then");
        let merge_bb = self.context.append_basic_block(function, "ifcont");

        // Handle elif and else
        let else_bb = if !elif_clauses.is_empty() || else_block.is_some() {
            self.context.append_basic_block(function, "else")
        } else {
            merge_bb
        };

        self.builder
            .build_conditional_branch(cond_int, then_bb, else_bb)
            .unwrap();

        // Generate then block
        self.builder.position_at_end(then_bb);
        for stmt in then_block {
            self.visit_statement(stmt)?;
        }
        if !self.is_block_terminated() {
            self.builder.build_unconditional_branch(merge_bb).unwrap();
        }

        // Generate elif/else chains
        if !elif_clauses.is_empty() || else_block.is_some() {
            self.builder.position_at_end(else_bb);

            // Process elif clauses
            for (elif_cond, elif_body) in elif_clauses {
                let elif_cond_val = self.visit_expression(elif_cond)?;
                let elif_cond_int = elif_cond_val.into_int_value();

                let elif_then_bb = self.context.append_basic_block(function, "elif_then");
                let next_bb = self.context.append_basic_block(function, "elif_next");

                self.builder
                    .build_conditional_branch(elif_cond_int, elif_then_bb, next_bb)
                    .unwrap();

                self.builder.position_at_end(elif_then_bb);
                for stmt in elif_body {
                    self.visit_statement(stmt)?;
                }
                if !self.is_block_terminated() {
                    self.builder.build_unconditional_branch(merge_bb).unwrap();
                }

                self.builder.position_at_end(next_bb);
            }

            // Process else block
            if let Some(else_stmts) = else_block {
                for stmt in else_stmts {
                    self.visit_statement(stmt)?;
                }
            }

            if !self.is_block_terminated() {
                self.builder.build_unconditional_branch(merge_bb).unwrap();
            }
        }

        self.builder.position_at_end(merge_bb);
        Ok(())
    }

    pub(crate) fn generate_while_statement(
        &mut self,
        condition: &Expression,
        body: &[Statement],
    ) -> Result<(), String> {
        let function = self.current_function.unwrap();
        let cond_bb = self.context.append_basic_block(function, "while_cond");
        let body_bb = self.context.append_basic_block(function, "while_body");
        let after_bb = self.context.append_basic_block(function, "while_after");

        self.builder.build_unconditional_branch(cond_bb).unwrap();

        // Condition block
        self.builder.position_at_end(cond_bb);
        let cond_val = self.visit_expression(condition)?;
        let cond_int = cond_val.into_int_value();
        self.builder
            .build_conditional_branch(cond_int, body_bb, after_bb)
            .unwrap();

        // Body block
        self.builder.position_at_end(body_bb);
        for stmt in body {
            self.visit_statement(stmt)?;
        }
        if !self.is_block_terminated() {
            self.builder.build_unconditional_branch(cond_bb).unwrap();
        }

        // After block
        self.builder.position_at_end(after_bb);
        Ok(())
    }

    pub(crate) fn generate_binary_op(
        &mut self,
        op: &BinaryOp,
        left: &Expression,
        right: &Expression,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let lhs = self.visit_expression(left)?;
        let rhs = self.visit_expression(right)?;

        // Determine if we're working with floats or ints
        let is_float = lhs.is_float_value() || rhs.is_float_value();

        if is_float {
            let lhs_float = if lhs.is_float_value() {
                lhs.into_float_value()
            } else {
                self.builder
                    .build_signed_int_to_float(
                        lhs.into_int_value(),
                        self.context.f64_type(),
                        "itof",
                    )
                    .unwrap()
            };

            let rhs_float = if rhs.is_float_value() {
                rhs.into_float_value()
            } else {
                self.builder
                    .build_signed_int_to_float(
                        rhs.into_int_value(),
                        self.context.f64_type(),
                        "itof",
                    )
                    .unwrap()
            };

            let result = match op {
                BinaryOp::Add => self
                    .builder
                    .build_float_add(lhs_float, rhs_float, "fadd")
                    .unwrap(),
                BinaryOp::Sub => self
                    .builder
                    .build_float_sub(lhs_float, rhs_float, "fsub")
                    .unwrap(),
                BinaryOp::Mul => self
                    .builder
                    .build_float_mul(lhs_float, rhs_float, "fmul")
                    .unwrap(),
                BinaryOp::Div => self
                    .builder
                    .build_float_div(lhs_float, rhs_float, "fdiv")
                    .unwrap(),
                BinaryOp::Mod => self
                    .builder
                    .build_float_rem(lhs_float, rhs_float, "frem")
                    .unwrap(),
                BinaryOp::Eq => {
                    let cmp = self
                        .builder
                        .build_float_compare(FloatPredicate::OEQ, lhs_float, rhs_float, "feq")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::Ne => {
                    let cmp = self
                        .builder
                        .build_float_compare(FloatPredicate::ONE, lhs_float, rhs_float, "fne")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::Lt => {
                    let cmp = self
                        .builder
                        .build_float_compare(FloatPredicate::OLT, lhs_float, rhs_float, "flt")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::Le => {
                    let cmp = self
                        .builder
                        .build_float_compare(FloatPredicate::OLE, lhs_float, rhs_float, "fle")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::Gt => {
                    let cmp = self
                        .builder
                        .build_float_compare(FloatPredicate::OGT, lhs_float, rhs_float, "fgt")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::Ge => {
                    let cmp = self
                        .builder
                        .build_float_compare(FloatPredicate::OGE, lhs_float, rhs_float, "fge")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::And | BinaryOp::Or => {
                    return Err("Logical operators not supported on floats".to_string());
                }
            };
            Ok(result.into())
        } else {
            let lhs_int = lhs.into_int_value();
            let rhs_int = rhs.into_int_value();

            let result = match op {
                BinaryOp::Add => self.builder.build_int_add(lhs_int, rhs_int, "add").unwrap(),
                BinaryOp::Sub => self.builder.build_int_sub(lhs_int, rhs_int, "sub").unwrap(),
                BinaryOp::Mul => self.builder.build_int_mul(lhs_int, rhs_int, "mul").unwrap(),
                BinaryOp::Div => self
                    .builder
                    .build_int_signed_div(lhs_int, rhs_int, "div")
                    .unwrap(),
                BinaryOp::Mod => self
                    .builder
                    .build_int_signed_rem(lhs_int, rhs_int, "mod")
                    .unwrap(),
                BinaryOp::Eq => {
                    let cmp = self
                        .builder
                        .build_int_compare(IntPredicate::EQ, lhs_int, rhs_int, "eq")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::Ne => {
                    let cmp = self
                        .builder
                        .build_int_compare(IntPredicate::NE, lhs_int, rhs_int, "ne")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::Lt => {
                    let cmp = self
                        .builder
                        .build_int_compare(IntPredicate::SLT, lhs_int, rhs_int, "lt")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::Le => {
                    let cmp = self
                        .builder
                        .build_int_compare(IntPredicate::SLE, lhs_int, rhs_int, "le")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::Gt => {
                    let cmp = self
                        .builder
                        .build_int_compare(IntPredicate::SGT, lhs_int, rhs_int, "gt")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::Ge => {
                    let cmp = self
                        .builder
                        .build_int_compare(IntPredicate::SGE, lhs_int, rhs_int, "ge")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::And => self.builder.build_and(lhs_int, rhs_int, "and").unwrap(),
                BinaryOp::Or => self.builder.build_or(lhs_int, rhs_int, "or").unwrap(),
            };
            Ok(result.into())
        }
    }

    pub(crate) fn generate_unary_op(
        &mut self,
        op: &UnaryOp,
        operand: &Expression,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let val = self.visit_expression(operand)?;

        match op {
            UnaryOp::Neg => {
                if val.is_float_value() {
                    Ok(self
                        .builder
                        .build_float_neg(val.into_float_value(), "fneg")
                        .unwrap()
                        .into())
                } else {
                    Ok(self
                        .builder
                        .build_int_neg(val.into_int_value(), "neg")
                        .unwrap()
                        .into())
                }
            }
            UnaryOp::Not => {
                let int_val = val.into_int_value();
                Ok(self.builder.build_not(int_val, "not").unwrap().into())
            }
        }
    }

    pub(crate) fn generate_call(
        &mut self,
        name: &str,
        args: &[Expression],
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // Handle print() specially - convert to printf calls
        if name == "print" {
            return self.generate_print_call(args);
        }

        let function = self
            .module
            .get_function(name)
            .ok_or_else(|| format!("Function {} not found", name))?;

        let mut arg_values: Vec<inkwell::values::BasicMetadataValueEnum> = Vec::new();
        for arg in args {
            arg_values.push(self.visit_expression(arg)?.into());
        }

        let call_site = self
            .builder
            .build_call(function, &arg_values, "call")
            .unwrap();

        // Check if the called function has a non-void return type
        let returns_value = function.get_type().get_return_type().is_some();

        if returns_value {
            // The call returns a value - convert through AnyValue
            use inkwell::values::AnyValue;
            let any_val = call_site.as_any_value_enum();
            match any_val {
                inkwell::values::AnyValueEnum::IntValue(iv) => Ok(iv.into()),
                inkwell::values::AnyValueEnum::FloatValue(fv) => Ok(fv.into()),
                inkwell::values::AnyValueEnum::PointerValue(pv) => Ok(pv.into()),
                inkwell::values::AnyValueEnum::ArrayValue(av) => Ok(av.into()),
                inkwell::values::AnyValueEnum::StructValue(sv) => Ok(sv.into()),
                inkwell::values::AnyValueEnum::VectorValue(vv) => Ok(vv.into()),
                _ => Ok(self.context.i32_type().const_zero().into()),
            }
        } else {
            // Function returns void
            Ok(self.context.i32_type().const_zero().into())
        }
    }

    pub(crate) fn generate_print_call(
        &mut self,
        args: &[Expression],
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let printf = self.module.get_function("printf").unwrap();

        for (i, arg) in args.iter().enumerate() {
            let val = self.visit_expression(arg)?;
            let suffix = if i < args.len() - 1 { " " } else { "\n" };

            if val.is_int_value() {
                let int_val = val.into_int_value();
                if int_val.get_type().get_bit_width() == 1 {
                    // Boolean - print "True" or "False" like Python
                    let function = self.current_function.unwrap();
                    let true_bb = self.context.append_basic_block(function, "print_true");
                    let false_bb = self.context.append_basic_block(function, "print_false");
                    let cont_bb = self.context.append_basic_block(function, "print_cont");

                    self.builder
                        .build_conditional_branch(int_val, true_bb, false_bb)
                        .unwrap();

                    // Print "True"
                    self.builder.position_at_end(true_bb);
                    let true_str = format!("True{}", suffix);
                    let true_const = self
                        .builder
                        .build_global_string_ptr(&true_str, "true_str")
                        .unwrap();
                    self.builder
                        .build_call(printf, &[true_const.as_pointer_value().into()], "printf")
                        .unwrap();
                    self.builder.build_unconditional_branch(cont_bb).unwrap();

                    // Print "False"
                    self.builder.position_at_end(false_bb);
                    let false_str = format!("False{}", suffix);
                    let false_const = self
                        .builder
                        .build_global_string_ptr(&false_str, "false_str")
                        .unwrap();
                    self.builder
                        .build_call(printf, &[false_const.as_pointer_value().into()], "printf")
                        .unwrap();
                    self.builder.build_unconditional_branch(cont_bb).unwrap();

                    self.builder.position_at_end(cont_bb);
                } else {
                    // Integer - use %lld
                    let format_str = format!("%lld{}", suffix);
                    let format_const = self
                        .builder
                        .build_global_string_ptr(&format_str, "fmt_int")
                        .unwrap();
                    self.builder
                        .build_call(
                            printf,
                            &[format_const.as_pointer_value().into(), val.into()],
                            "printf",
                        )
                        .unwrap();
                }
            } else if val.is_float_value() {
                // Float - check if it's a whole number for Python-like printing
                let float_val = val.into_float_value();
                let function = self.current_function.unwrap();

                // Convert float -> int -> float to truncate decimal part
                let as_int = self
                    .builder
                    .build_float_to_signed_int(float_val, self.context.i64_type(), "fptosi")
                    .unwrap();
                let truncated = self
                    .builder
                    .build_signed_int_to_float(as_int, self.context.f64_type(), "sitofp")
                    .unwrap();

                // Check if val == truncated (i.e., it's a whole number)
                let is_whole = self
                    .builder
                    .build_float_compare(FloatPredicate::OEQ, float_val, truncated, "is_whole")
                    .unwrap();

                let whole_bb = self
                    .context
                    .append_basic_block(function, "print_whole_float");
                let frac_bb = self
                    .context
                    .append_basic_block(function, "print_frac_float");
                let cont_bb = self
                    .context
                    .append_basic_block(function, "print_float_cont");

                self.builder
                    .build_conditional_branch(is_whole, whole_bb, frac_bb)
                    .unwrap();

                // Print whole number float like "1.0"
                self.builder.position_at_end(whole_bb);
                let whole_fmt = format!("%.1f{}", suffix);
                let whole_const = self
                    .builder
                    .build_global_string_ptr(&whole_fmt, "fmt_whole_float")
                    .unwrap();
                self.builder
                    .build_call(
                        printf,
                        &[whole_const.as_pointer_value().into(), float_val.into()],
                        "printf",
                    )
                    .unwrap();
                self.builder.build_unconditional_branch(cont_bb).unwrap();

                // Print fractional float with full precision
                self.builder.position_at_end(frac_bb);
                let frac_fmt = format!("%.15g{}", suffix);
                let frac_const = self
                    .builder
                    .build_global_string_ptr(&frac_fmt, "fmt_frac_float")
                    .unwrap();
                self.builder
                    .build_call(
                        printf,
                        &[frac_const.as_pointer_value().into(), float_val.into()],
                        "printf",
                    )
                    .unwrap();
                self.builder.build_unconditional_branch(cont_bb).unwrap();

                self.builder.position_at_end(cont_bb);
            } else if val.is_pointer_value() {
                // String
                let format_str = format!("%s{}", suffix);
                let format_const = self
                    .builder
                    .build_global_string_ptr(&format_str, "fmt_str")
                    .unwrap();
                self.builder
                    .build_call(
                        printf,
                        &[format_const.as_pointer_value().into(), val.into()],
                        "printf",
                    )
                    .unwrap();
            } else {
                return Err("print() only supports int, float, bool, and string types".to_string());
            }
        }

        Ok(self.context.i32_type().const_zero().into())
    }
    pub(crate) fn type_to_llvm(&self, ty: &Type) -> BasicTypeEnum<'ctx> {
        match ty {
            Type::Int => self.context.i64_type().into(),
            Type::Float => self.context.f64_type().into(),
            Type::Bool => self.context.bool_type().into(),
            Type::Str => self
                .context
                .ptr_type(inkwell::AddressSpace::default())
                .into(),
            Type::None => self.context.i32_type().into(),
        }
    }

    pub(crate) fn create_entry_block_alloca(
        &self,
        _fn_name: &str,
        var_name: &str,
        var_type: &Type,
    ) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();

        let entry = self
            .current_function
            .unwrap()
            .get_first_basic_block()
            .unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        let llvm_type = self.type_to_llvm(var_type);
        builder.build_alloca(llvm_type, var_name).unwrap()
    }

    pub(crate) fn is_block_terminated(&self) -> bool {
        if let Some(bb) = self.builder.get_insert_block() {
            if let Some(_terminator) = bb.get_terminator() {
                return true;
            }
        }
        false
    }
}
