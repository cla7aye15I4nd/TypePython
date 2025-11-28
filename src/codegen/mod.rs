mod visitor;

use crate::ast::visitor::Visitor;
use crate::ast::*;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::{FloatPredicate, IntPredicate};
use std::collections::HashMap;

/// Context for tracking loop control flow (break/continue)
pub(crate) struct LoopContext<'ctx> {
    pub(crate) continue_block: BasicBlock<'ctx>,
    pub(crate) break_block: BasicBlock<'ctx>,
}

pub struct CodeGen<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    pub(crate) variables: HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>,
    pub(crate) current_function: Option<FunctionValue<'ctx>>,
    pub(crate) strings: HashMap<String, u64>,
    pub(crate) module_name: String,
    /// Map of imported symbols: local_name -> real_module_name (for name mangling)
    pub(crate) imported_symbols: HashMap<String, String>,
    /// Map of module_name -> Program for lazy function declaration
    pub(crate) module_data: HashMap<String, Program>,
    /// Stack of loop contexts for break/continue statements
    pub(crate) loop_stack: Vec<LoopContext<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        // Initialize target for the native platform
        Target::initialize_native(&InitializationConfig::default())
            .expect("Failed to initialize native target");

        // Get the native target and create target machine
        let target_triple = inkwell::targets::TargetMachine::get_default_triple();
        let target =
            Target::from_triple(&target_triple).expect("Failed to create target from triple");
        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                inkwell::OptimizationLevel::Default,
                inkwell::targets::RelocMode::PIC,
                inkwell::targets::CodeModel::Default,
            )
            .expect("Failed to create target machine");

        // Set the data layout and target triple for the module
        module.set_data_layout(&target_machine.get_target_data().get_data_layout());
        module.set_triple(&target_triple);

        CodeGen {
            context,
            module,
            builder,
            variables: HashMap::new(),
            current_function: None,
            strings: HashMap::new(),
            module_name: module_name.to_string(),
            imported_symbols: HashMap::new(),
            module_data: HashMap::new(),
            loop_stack: Vec::new(),
        }
    }

    pub fn set_imported_symbols(&mut self, imported_symbols: HashMap<String, String>) {
        self.imported_symbols = imported_symbols;
    }

    /// Mangle function name with module name
    /// Format: {module_name}_{function_name}
    /// Replaces special characters in module name (., <, >) with underscores
    fn mangle_function_name(&self, module_name: &str, function_name: &str) -> String {
        let clean_module = module_name
            .replace(".", "_")
            .replace("<", "")
            .replace(">", "");
        format!("{}_{}", clean_module, function_name)
    }

    pub fn set_module_data(&mut self, module_data: HashMap<String, Program>) {
        self.module_data = module_data;
    }

    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    pub fn generate(&mut self, program: &Program) -> Result<(), String> {
        // Use the visitor pattern to generate code
        self.visit_program(program)?;
        Ok(())
    }

    /// Evaluate an expression and return its LLVM value
    /// This is separate from visit_expression which is part of the Visitor trait
    pub(crate) fn evaluate_expression(
        &mut self,
        expr: &Expression,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            Expression::IntLit(val) => self.visit_int_lit_impl(*val),
            Expression::FloatLit(val) => self.visit_float_lit_impl(*val),
            Expression::StrLit(val) => self.visit_str_lit_impl(val),
            Expression::BytesLit(val) => self.visit_bytes_lit_impl(val),
            Expression::BoolLit(val) => self.visit_bool_lit_impl(*val),
            Expression::NoneLit => self.visit_none_lit_impl(),
            Expression::Var(name) => self.visit_var_impl(name),
            Expression::BinOp { op, left, right } => self.generate_binary_op(op, left, right),
            Expression::UnaryOp { op, operand } => self.generate_unary_op(op, operand),
            Expression::Call { func, args } => {
                match func.as_ref() {
                    // Simple function call: function_name()
                    Expression::Var(name) => self.generate_call(name, args),
                    // Qualified call: module.function()
                    Expression::Attribute { object, attr } => {
                        if let Expression::Var(module_name) = object.as_ref() {
                            let qualified_name = format!("{}.{}", module_name, attr);
                            self.generate_call(&qualified_name, args)
                        } else {
                            Err("Only simple module.function() calls are supported".to_string())
                        }
                    }
                    _ => Err(
                        "Only simple function calls and module.function() calls are supported"
                            .to_string(),
                    ),
                }
            }
            Expression::List(_) => {
                todo!("List literals")
            }
            Expression::Tuple(_) => {
                todo!("Tuple literals")
            }
            Expression::Dict(_) => {
                todo!("Dict literals")
            }
            Expression::Set(_) => {
                todo!("Set literals")
            }
            Expression::Attribute { .. } => {
                todo!("Attribute access")
            }
            Expression::Subscript { .. } => {
                todo!("Subscript operation")
            }
            Expression::Slice { .. } => {
                todo!("Slice operation")
            }
        }
    }

    /// Get or declare a builtin function from builtin.c
    /// These functions are always available and linked from the builtin C module
    pub(crate) fn get_or_declare_builtin_function(&mut self, name: &str) -> FunctionValue<'ctx> {
        // Check if already declared
        if let Some(func) = self.module.get_function(name) {
            return func;
        }

        // Declare the builtin function based on its name
        let i64_type = self.context.i64_type();
        let f64_type = self.context.f64_type();
        let str_type = self.context.ptr_type(inkwell::AddressSpace::default());
        let void_type = self.context.void_type();
        let bool_type = self.context.bool_type();

        match name {
            "tpy_print_int" => {
                let fn_type = void_type.fn_type(&[i64_type.into()], false);
                self.module.add_function(name, fn_type, None)
            }
            "tpy_print_float" => {
                let fn_type = void_type.fn_type(&[f64_type.into()], false);
                self.module.add_function(name, fn_type, None)
            }
            "tpy_print_bool" => {
                let fn_type = void_type.fn_type(&[bool_type.into()], false);
                self.module.add_function(name, fn_type, None)
            }
            "tpy_print_str" => {
                let fn_type = void_type.fn_type(&[str_type.into()], false);
                self.module.add_function(name, fn_type, None)
            }
            "tpy_print_space" => {
                let fn_type = void_type.fn_type(&[], false);
                self.module.add_function(name, fn_type, None)
            }
            "tpy_print_newline" => {
                let fn_type = void_type.fn_type(&[], false);
                self.module.add_function(name, fn_type, None)
            }
            "tpy_pow" => {
                let fn_type = f64_type.fn_type(&[f64_type.into(), f64_type.into()], false);
                self.module.add_function(name, fn_type, None)
            }
            "tpy_pow_int" => {
                let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);
                self.module.add_function(name, fn_type, None)
            }
            "tpy_floor" => {
                let fn_type = f64_type.fn_type(&[f64_type.into()], false);
                self.module.add_function(name, fn_type, None)
            }
            "tpy_strcat" => {
                let str_type = self.context.ptr_type(inkwell::AddressSpace::default());
                let fn_type = str_type.fn_type(&[str_type.into(), str_type.into()], false);
                self.module.add_function(name, fn_type, None)
            }
            "tpy_strcmp" => {
                let str_type = self.context.ptr_type(inkwell::AddressSpace::default());
                let fn_type = i64_type.fn_type(&[str_type.into(), str_type.into()], false);
                self.module.add_function(name, fn_type, None)
            }
            _ => panic!("Unknown builtin function: {}", name),
        }
    }

    /// Lazily declare an external function when needed
    /// Looks up the function in module_data and declares it if not already declared
    fn get_or_declare_external_function(
        &mut self,
        module_name: &str,
        function_name: &str,
    ) -> Result<FunctionValue<'ctx>, String> {
        let mangled_name = self.mangle_function_name(module_name, function_name);

        // If already declared, return it
        if let Some(func) = self.module.get_function(&mangled_name) {
            return Ok(func);
        }

        // Look up the function definition in module_data
        let program = self
            .module_data
            .get(module_name)
            .ok_or_else(|| format!("Module '{}' not found in module_data", module_name))?;

        let func_def = program
            .functions
            .iter()
            .find(|f| f.name == function_name)
            .ok_or_else(|| {
                format!(
                    "Function '{}' not found in module '{}'",
                    function_name, module_name
                )
            })?;

        // Declare the function
        let param_types: Vec<BasicMetadataTypeEnum> = func_def
            .params
            .iter()
            .map(|p| self.type_to_llvm(&p.param_type).into())
            .collect();

        let fn_type = match func_def.return_type {
            Type::None => self.context.void_type().fn_type(&param_types, false),
            _ => {
                let return_type = self.type_to_llvm(&func_def.return_type);
                return_type.fn_type(&param_types, false)
            }
        };

        let function = self.module.add_function(&mangled_name, fn_type, None);

        // Set parameter names
        for (i, param) in func_def.params.iter().enumerate() {
            function
                .get_nth_param(i as u32)
                .unwrap()
                .set_name(&param.name);
        }

        Ok(function)
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

        // Mangle function name with current module name
        let mangled_name = self.mangle_function_name(&self.module_name, &func.name);
        let function = self.module.add_function(&mangled_name, fn_type, None);

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
        let cond_val = self.evaluate_expression(condition)?;
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
                let elif_cond_val = self.evaluate_expression(elif_cond)?;
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
        let cond_val = self.evaluate_expression(condition)?;
        let cond_int = cond_val.into_int_value();
        self.builder
            .build_conditional_branch(cond_int, body_bb, after_bb)
            .unwrap();

        // Body block
        self.builder.position_at_end(body_bb);

        // Push loop context for break/continue support
        self.loop_stack.push(LoopContext {
            continue_block: cond_bb,
            break_block: after_bb,
        });

        for stmt in body {
            self.visit_statement(stmt)?;
        }

        // Pop loop context
        self.loop_stack.pop();

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
        let lhs = self.evaluate_expression(left)?;
        let rhs = self.evaluate_expression(right)?;

        // Handle bytes operations (C-style null-terminated byte sequences)
        if lhs.is_pointer_value() && rhs.is_pointer_value() {
            match op {
                BinaryOp::Add => {
                    // Bytes concatenation - call tpy_strcat builtin
                    let lhs_ptr = lhs.into_pointer_value();
                    let rhs_ptr = rhs.into_pointer_value();
                    let strcat_fn = self.get_or_declare_builtin_function("tpy_strcat");
                    let call_site = self
                        .builder
                        .build_call(strcat_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytescat")
                        .unwrap();
                    use inkwell::values::AnyValue;
                    let any_val = call_site.as_any_value_enum();
                    if let inkwell::values::AnyValueEnum::PointerValue(pv) = any_val {
                        return Ok(pv.into());
                    } else {
                        return Err("tpy_strcat did not return a pointer value".to_string());
                    }
                }
                BinaryOp::Eq => {
                    // Bytes equality - call tpy_strcmp builtin
                    let lhs_ptr = lhs.into_pointer_value();
                    let rhs_ptr = rhs.into_pointer_value();
                    let strcmp_fn = self.get_or_declare_builtin_function("tpy_strcmp");
                    let call_site = self
                        .builder
                        .build_call(strcmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytescmp")
                        .unwrap();
                    use inkwell::values::AnyValue;
                    let any_val = call_site.as_any_value_enum();
                    if let inkwell::values::AnyValueEnum::IntValue(iv) = any_val {
                        // Convert i64 to i1 (boolean) by truncating
                        let bool_val = self
                            .builder
                            .build_int_truncate(iv, self.context.bool_type(), "to_bool")
                            .unwrap();
                        return Ok(bool_val.into());
                    } else {
                        return Err("tpy_strcmp did not return an int value".to_string());
                    }
                }
                BinaryOp::Ne => {
                    // Bytes inequality - call strcmp and negate
                    let lhs_ptr = lhs.into_pointer_value();
                    let rhs_ptr = rhs.into_pointer_value();
                    let strcmp_fn = self.get_or_declare_builtin_function("tpy_strcmp");
                    let call_site = self
                        .builder
                        .build_call(strcmp_fn, &[lhs_ptr.into(), rhs_ptr.into()], "bytescmp")
                        .unwrap();
                    use inkwell::values::AnyValue;
                    let any_val = call_site.as_any_value_enum();
                    if let inkwell::values::AnyValueEnum::IntValue(iv) = any_val {
                        // Convert i64 to i1, then negate
                        let bool_val = self
                            .builder
                            .build_int_truncate(iv, self.context.bool_type(), "to_bool")
                            .unwrap();
                        let negated = self.builder.build_not(bool_val, "ne").unwrap();
                        return Ok(negated.into());
                    } else {
                        return Err("tpy_strcmp did not return an int value".to_string());
                    }
                }
                _ => {
                    return Err(format!("Operator {:?} not supported for bytes type", op));
                }
            }
        }

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
                BinaryOp::Pow => {
                    // Call tpy_pow builtin for float power
                    let pow_fn = self.get_or_declare_builtin_function("tpy_pow");
                    let call_site = self
                        .builder
                        .build_call(pow_fn, &[lhs_float.into(), rhs_float.into()], "fpow")
                        .unwrap();
                    use inkwell::values::AnyValue;
                    let any_val = call_site.as_any_value_enum();
                    if let inkwell::values::AnyValueEnum::FloatValue(fv) = any_val {
                        return Ok(fv.into());
                    } else {
                        return Err("tpy_pow did not return a float value".to_string());
                    }
                }
                BinaryOp::FloorDiv => {
                    // Floor division for floats: divide then floor
                    let div_result = self
                        .builder
                        .build_float_div(lhs_float, rhs_float, "fdiv")
                        .unwrap();
                    // Call floor function from math library
                    let floor_fn = self.get_or_declare_builtin_function("tpy_floor");
                    let call_site = self
                        .builder
                        .build_call(floor_fn, &[div_result.into()], "floor")
                        .unwrap();
                    use inkwell::values::AnyValue;
                    let any_val = call_site.as_any_value_enum();
                    if let inkwell::values::AnyValueEnum::FloatValue(fv) = any_val {
                        return Ok(fv.into());
                    } else {
                        return Err("tpy_floor did not return a float value".to_string());
                    }
                }
                BinaryOp::Is => {
                    // Identity comparison for floats - compare bit patterns
                    let cmp = self
                        .builder
                        .build_float_compare(FloatPredicate::OEQ, lhs_float, rhs_float, "is")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::IsNot => {
                    // Inverse identity comparison
                    let cmp = self
                        .builder
                        .build_float_compare(FloatPredicate::ONE, lhs_float, rhs_float, "isnot")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::BitOr
                | BinaryOp::BitXor
                | BinaryOp::BitAnd
                | BinaryOp::LShift
                | BinaryOp::RShift => {
                    return Err(format!("Bitwise operator {:?} not supported on floats", op));
                }
                BinaryOp::In | BinaryOp::NotIn => {
                    return Err(format!(
                        "Membership operator {:?} requires container support",
                        op
                    ));
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
                BinaryOp::FloorDiv => self
                    .builder
                    .build_int_signed_div(lhs_int, rhs_int, "floordiv")
                    .unwrap(),
                BinaryOp::BitOr => self.builder.build_or(lhs_int, rhs_int, "bitor").unwrap(),
                BinaryOp::BitXor => self.builder.build_xor(lhs_int, rhs_int, "bitxor").unwrap(),
                BinaryOp::BitAnd => self.builder.build_and(lhs_int, rhs_int, "bitand").unwrap(),
                BinaryOp::LShift => self
                    .builder
                    .build_left_shift(lhs_int, rhs_int, "lshift")
                    .unwrap(),
                BinaryOp::RShift => self
                    .builder
                    .build_right_shift(lhs_int, rhs_int, true, "rshift")
                    .unwrap(),
                BinaryOp::Pow => {
                    // Call tpy_pow_int builtin for integer power
                    let pow_fn = self.get_or_declare_builtin_function("tpy_pow_int");
                    let call_site = self
                        .builder
                        .build_call(pow_fn, &[lhs_int.into(), rhs_int.into()], "ipow")
                        .unwrap();
                    use inkwell::values::AnyValue;
                    let any_val = call_site.as_any_value_enum();
                    if let inkwell::values::AnyValueEnum::IntValue(iv) = any_val {
                        return Ok(iv.into());
                    } else {
                        return Err("tpy_pow_int did not return an int value".to_string());
                    }
                }
                BinaryOp::Is => {
                    // Identity comparison for integers - compare values
                    let cmp = self
                        .builder
                        .build_int_compare(IntPredicate::EQ, lhs_int, rhs_int, "is")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::IsNot => {
                    // Inverse identity comparison for integers
                    let cmp = self
                        .builder
                        .build_int_compare(IntPredicate::NE, lhs_int, rhs_int, "isnot")
                        .unwrap();
                    return Ok(cmp.into());
                }
                BinaryOp::In | BinaryOp::NotIn => {
                    return Err(format!(
                        "Membership operator {:?} requires container support",
                        op
                    ));
                }
            };
            Ok(result.into())
        }
    }

    pub(crate) fn generate_unary_op(
        &mut self,
        op: &UnaryOp,
        operand: &Expression,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let val = self.evaluate_expression(operand)?;

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
            UnaryOp::Pos => {
                // Unary plus is a no-op
                Ok(val)
            }
            UnaryOp::BitNot => {
                let int_val = val.into_int_value();
                Ok(self.builder.build_not(int_val, "bitnot").unwrap().into())
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

        // Determine the module and function name, then lazily declare if needed
        let function = if name.contains('.') {
            // Qualified call: module.function
            let parts: Vec<&str> = name.split('.').collect();
            let module_local_name = parts[0];
            let function_name = parts[1];

            // Look up the real module name from imported symbols
            let real_module_name = self
                .imported_symbols
                .get(module_local_name)
                .ok_or_else(|| format!("Module {} not found in imports", module_local_name))?
                .clone();

            // Lazily declare the external function
            self.get_or_declare_external_function(&real_module_name, function_name)?
        } else {
            // Unqualified call: function - use current module name
            let mangled_name = self.mangle_function_name(&self.module_name, name);
            self.module
                .get_function(&mangled_name)
                .ok_or_else(|| format!("Function {} (mangled: {}) not found", name, mangled_name))?
        };

        let mut arg_values: Vec<inkwell::values::BasicMetadataValueEnum> = Vec::new();
        for arg in args {
            arg_values.push(self.evaluate_expression(arg)?.into());
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
        // Get or declare runtime print functions from builtin.c
        let print_int = self.get_or_declare_builtin_function("tpy_print_int");
        let print_float = self.get_or_declare_builtin_function("tpy_print_float");
        let print_bool = self.get_or_declare_builtin_function("tpy_print_bool");
        let print_str = self.get_or_declare_builtin_function("tpy_print_str");
        let print_space = self.get_or_declare_builtin_function("tpy_print_space");
        let print_newline = self.get_or_declare_builtin_function("tpy_print_newline");

        for (i, arg) in args.iter().enumerate() {
            let val = self.evaluate_expression(arg)?;

            if val.is_int_value() {
                let int_val = val.into_int_value();
                if int_val.get_type().get_bit_width() == 1 {
                    // Boolean - use tpy_print_bool
                    self.builder
                        .build_call(print_bool, &[int_val.into()], "print_bool")
                        .unwrap();
                } else {
                    // Integer - use tpy_print_int
                    self.builder
                        .build_call(print_int, &[int_val.into()], "print_int")
                        .unwrap();
                }
            } else if val.is_float_value() {
                // Float - use tpy_print_float
                let float_val = val.into_float_value();
                self.builder
                    .build_call(print_float, &[float_val.into()], "print_float")
                    .unwrap();
            } else if val.is_pointer_value() {
                // Bytes - use tpy_print_str
                let ptr_val = val.into_pointer_value();
                self.builder
                    .build_call(print_str, &[ptr_val.into()], "print_bytes")
                    .unwrap();
            } else {
                return Err("print() only supports int, float, bool, and bytes types".to_string());
            }

            // Print space between arguments (but not after the last one)
            if i < args.len() - 1 {
                self.builder
                    .build_call(print_space, &[], "print_space")
                    .unwrap();
            }
        }

        // Print newline at the end
        self.builder
            .build_call(print_newline, &[], "print_newline")
            .unwrap();

        Ok(self.context.i32_type().const_zero().into())
    }
    pub(crate) fn type_to_llvm(&self, ty: &Type) -> BasicTypeEnum<'ctx> {
        match ty {
            Type::Int => self.context.i64_type().into(),
            Type::Float => self.context.f64_type().into(),
            Type::Bool => self.context.bool_type().into(),
            Type::Str => todo!("str type (use bytes instead)"),
            Type::Bytes => self
                .context
                .ptr_type(inkwell::AddressSpace::default())
                .into(),
            Type::None => self.context.i32_type().into(),
            Type::List(_) => todo!("List type"),
            Type::Dict(_, _) => todo!("Dict type"),
            Type::Set(_) => todo!("Set type"),
            Type::Tuple(_) => todo!("Tuple type"),
            Type::Custom(_) => todo!("Custom type"),
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
