mod visitor;

use crate::ast::visitor::Visitor;
use crate::ast::*;
use crate::types::{infer_type_from_value, TypeCodeGen};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
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

        // Infer types from values
        let lhs_type = infer_type_from_value(&lhs)?;
        let rhs_type = infer_type_from_value(&rhs)?;

        // Determine the common type for the operation
        let (op_type, lhs_coerced, rhs_coerced) =
            self.coerce_operands_for_binary_op(&lhs_type, &rhs_type, lhs, rhs)?;

        // Delegate to the type-specific implementation
        op_type.binary_op(
            self.context,
            &self.builder,
            &self.module,
            op,
            lhs_coerced,
            rhs_coerced,
        )
    }

    /// Coerce operands to a common type for binary operations
    fn coerce_operands_for_binary_op(
        &self,
        lhs_type: &TypeCodeGen,
        rhs_type: &TypeCodeGen,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<(TypeCodeGen, BasicValueEnum<'ctx>, BasicValueEnum<'ctx>), String> {
        use crate::types::{BoolType, BytesType, FloatType, IntType};

        match (lhs_type, rhs_type) {
            // Same types - no coercion needed
            (TypeCodeGen::Int(_), TypeCodeGen::Int(_)) => Ok((TypeCodeGen::Int(IntType), lhs, rhs)),
            (TypeCodeGen::Float(_), TypeCodeGen::Float(_)) => {
                Ok((TypeCodeGen::Float(FloatType), lhs, rhs))
            }
            (TypeCodeGen::Bool(_), TypeCodeGen::Bool(_)) => {
                Ok((TypeCodeGen::Bool(BoolType), lhs, rhs))
            }
            (TypeCodeGen::Bytes(_), TypeCodeGen::Bytes(_)) => {
                Ok((TypeCodeGen::Bytes(BytesType), lhs, rhs))
            }

            // Int/Float coercion - promote to Float
            (TypeCodeGen::Int(_), TypeCodeGen::Float(_)) => {
                let lhs_float = self
                    .builder
                    .build_signed_int_to_float(
                        lhs.into_int_value(),
                        self.context.f64_type(),
                        "itof",
                    )
                    .unwrap();
                Ok((TypeCodeGen::Float(FloatType), lhs_float.into(), rhs))
            }
            (TypeCodeGen::Float(_), TypeCodeGen::Int(_)) => {
                let rhs_float = self
                    .builder
                    .build_signed_int_to_float(
                        rhs.into_int_value(),
                        self.context.f64_type(),
                        "itof",
                    )
                    .unwrap();
                Ok((TypeCodeGen::Float(FloatType), lhs, rhs_float.into()))
            }

            // Bool/Int coercion - promote Bool to Int
            (TypeCodeGen::Bool(_), TypeCodeGen::Int(_)) => {
                let lhs_int = self
                    .builder
                    .build_int_z_extend(lhs.into_int_value(), self.context.i64_type(), "btoi")
                    .unwrap();
                Ok((TypeCodeGen::Int(IntType), lhs_int.into(), rhs))
            }
            (TypeCodeGen::Int(_), TypeCodeGen::Bool(_)) => {
                let rhs_int = self
                    .builder
                    .build_int_z_extend(rhs.into_int_value(), self.context.i64_type(), "btoi")
                    .unwrap();
                Ok((TypeCodeGen::Int(IntType), lhs, rhs_int.into()))
            }

            // Bool/Float coercion - promote Bool to Float
            (TypeCodeGen::Bool(_), TypeCodeGen::Float(_)) => {
                let lhs_int = self
                    .builder
                    .build_int_z_extend(lhs.into_int_value(), self.context.i64_type(), "btoi")
                    .unwrap();
                let lhs_float = self
                    .builder
                    .build_signed_int_to_float(lhs_int, self.context.f64_type(), "itof")
                    .unwrap();
                Ok((TypeCodeGen::Float(FloatType), lhs_float.into(), rhs))
            }
            (TypeCodeGen::Float(_), TypeCodeGen::Bool(_)) => {
                let rhs_int = self
                    .builder
                    .build_int_z_extend(rhs.into_int_value(), self.context.i64_type(), "btoi")
                    .unwrap();
                let rhs_float = self
                    .builder
                    .build_signed_int_to_float(rhs_int, self.context.f64_type(), "itof")
                    .unwrap();
                Ok((TypeCodeGen::Float(FloatType), lhs, rhs_float.into()))
            }

            // None type
            (TypeCodeGen::None(_), TypeCodeGen::None(_)) => {
                Ok((TypeCodeGen::None(crate::types::NoneType), lhs, rhs))
            }

            _ => Err(format!(
                "Incompatible types for binary operation: {:?} and {:?}",
                std::mem::discriminant(lhs_type),
                std::mem::discriminant(rhs_type)
            )),
        }
    }

    pub(crate) fn generate_unary_op(
        &mut self,
        op: &UnaryOp,
        operand: &Expression,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let val = self.evaluate_expression(operand)?;

        // Infer type from value and delegate to type-specific implementation
        let val_type = infer_type_from_value(&val)?;
        val_type.unary_op(self.context, &self.builder, op, val)
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
        let print_space = self.get_or_declare_builtin_function("tpy_print_space");
        let print_newline = self.get_or_declare_builtin_function("tpy_print_newline");

        for (i, arg) in args.iter().enumerate() {
            let val = self.evaluate_expression(arg)?;

            // Infer type and delegate to type-specific print
            let val_type = infer_type_from_value(&val)?;
            let print_fn_name = val_type.print_function_name();
            let print_fn = self.get_or_declare_builtin_function(print_fn_name);
            val_type.print(&self.builder, print_fn, val)?;

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
