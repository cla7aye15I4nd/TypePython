pub mod builtins;
mod visitor;

use crate::ast::visitor::Visitor;
use crate::ast::*;
use crate::types::{CgCtx, PyType, PyValue};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use std::collections::{HashMap, HashSet};

/// Context for tracking loop control flow (break/continue)
pub(crate) struct LoopContext<'ctx> {
    pub(crate) continue_block: BasicBlock<'ctx>,
    pub(crate) break_block: BasicBlock<'ctx>,
}

pub struct CodeGen<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    /// Variables: name -> (pointer, LLVM type for load, Python type)
    pub(crate) variables: HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>, PyType)>,
    pub(crate) current_function: Option<FunctionValue<'ctx>>,
    pub(crate) strings: HashMap<String, u64>,
    pub(crate) module_name: String,
    /// Map of imported symbols: local_name -> real_module_name (for name mangling)
    pub(crate) imported_symbols: HashMap<String, String>,
    /// Map of module_name -> Program for lazy function declaration
    pub(crate) module_data: HashMap<String, Program>,
    /// Stack of loop contexts for break/continue statements
    pub(crate) loop_stack: Vec<LoopContext<'ctx>>,
    /// Set of builtin modules that have been used (for selective linking)
    pub used_builtin_modules: HashSet<String>,
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
            used_builtin_modules: HashSet::new(),
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

        // Scan the module for builtin function usages and update used_builtin_modules
        // This captures usages from type operations that don't go through get_or_declare_builtin_function
        self.scan_for_builtin_usages();

        Ok(())
    }

    /// Scan the module for declared builtin functions and update used_builtin_modules
    fn scan_for_builtin_usages(&mut self) {
        for (_, builtin_fn) in builtins::BUILTIN_TABLE.iter() {
            // Check if this builtin function is declared in the module
            if self.module.get_function(builtin_fn.symbol).is_some() {
                self.used_builtin_modules
                    .insert(builtin_fn.module.to_string());
            }
        }
    }

    /// Evaluate an expression and return its Python value (IR value + type)
    /// This is separate from visit_expression which is part of the Visitor trait
    pub(crate) fn evaluate_expression(
        &mut self,
        expr: &Expression,
    ) -> Result<PyValue<'ctx>, String> {
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
            Expression::Subscript { object, index } => {
                let obj = self.evaluate_expression(object)?;

                // Check if index is a slice expression
                if let Expression::Slice { start, stop, step } = index.as_ref() {
                    match &obj.ty {
                        PyType::Bytes(_) => {
                            // bytes[start:stop:step] returns bytes
                            let len_fn = self.get_or_declare_builtin_function("bytes_len");
                            let len_call = self
                                .builder
                                .build_call(len_fn, &[obj.value.into()], "len")
                                .unwrap();
                            let len = self.extract_int_call_result(len_call)?;

                            if let Some(step_expr) = step {
                                // With step: use bytes_slice_step
                                let step_val = self.evaluate_expression(step_expr)?;

                                // For negative step, default start is len-1, default end is -len-1
                                // For positive step, default start is 0, default end is len
                                // We'll pass special sentinel values and let the C function handle defaults
                                // Actually, let's compute defaults based on step sign at runtime

                                // Get start value (default depends on step sign)
                                let start_val = if let Some(s) = start {
                                    self.evaluate_expression(s)?
                                } else {
                                    // Use a large positive value as sentinel for "use default"
                                    // The C code will interpret based on step sign
                                    // Actually, for simplicity: if step > 0, start=0; if step < 0, start=len-1
                                    // We need runtime branching for this, so let's use INT64_MAX as sentinel
                                    PyValue::int(
                                        self.context
                                            .i64_type()
                                            .const_int(i64::MAX as u64, false)
                                            .into(),
                                    )
                                };

                                // Get stop value (default depends on step sign)
                                let stop_val = if let Some(e) = stop {
                                    self.evaluate_expression(e)?
                                } else {
                                    // Use INT64_MAX as sentinel for "use default"
                                    PyValue::int(
                                        self.context
                                            .i64_type()
                                            .const_int(i64::MAX as u64, false)
                                            .into(),
                                    )
                                };

                                let slice_fn =
                                    self.get_or_declare_builtin_function("bytes_slice_step");
                                let call = self
                                    .builder
                                    .build_call(
                                        slice_fn,
                                        &[
                                            obj.value.into(),
                                            start_val.value.into(),
                                            stop_val.value.into(),
                                            step_val.value.into(),
                                        ],
                                        "bytes_slice_step",
                                    )
                                    .unwrap();
                                self.extract_bytes_call_result(call)
                            } else {
                                // No step: use simpler bytes_slice
                                // Get start value (default 0)
                                let start_val = if let Some(s) = start {
                                    self.evaluate_expression(s)?
                                } else {
                                    PyValue::int(self.context.i64_type().const_zero().into())
                                };

                                // Get stop value (default len)
                                let stop_val = if let Some(e) = stop {
                                    self.evaluate_expression(e)?
                                } else {
                                    len
                                };

                                let slice_fn = self.get_or_declare_builtin_function("bytes_slice");
                                let call = self
                                    .builder
                                    .build_call(
                                        slice_fn,
                                        &[
                                            obj.value.into(),
                                            start_val.value.into(),
                                            stop_val.value.into(),
                                        ],
                                        "bytes_slice",
                                    )
                                    .unwrap();
                                self.extract_bytes_call_result(call)
                            }
                        }
                        _ => Err(format!(
                            "Slice operation not supported for type {:?}",
                            obj.ty
                        )),
                    }
                } else {
                    let idx = self.evaluate_expression(index)?;
                    match &obj.ty {
                        PyType::Bytes(_) => {
                            // bytes[index] returns an int (the byte value at that index)
                            let getitem_fn = self.get_or_declare_builtin_function("bytes_getitem");
                            let call = self
                                .builder
                                .build_call(
                                    getitem_fn,
                                    &[obj.value.into(), idx.value.into()],
                                    "bytes_getitem",
                                )
                                .unwrap();
                            self.extract_int_call_result(call)
                        }
                        _ => Err(format!(
                            "Subscript operation not supported for type {:?}",
                            obj.ty
                        )),
                    }
                }
            }
            Expression::Slice { .. } => {
                Err("Slice expression must be used inside subscript".to_string())
            }
        }
    }

    /// Get or declare a builtin function from the builtin modules
    /// Uses the auto-generated builtin table to look up function signatures
    /// and tracks which builtin modules are used for selective linking
    pub(crate) fn get_or_declare_builtin_function(&mut self, name: &str) -> FunctionValue<'ctx> {
        // Look up the builtin function in the generated table
        let builtin = builtins::BUILTIN_TABLE
            .get(name)
            .unwrap_or_else(|| panic!("Unknown builtin function: {}", name));

        // Track which builtin module this function belongs to
        self.used_builtin_modules.insert(builtin.module.to_string());

        // Check if already declared (use the symbol name, which is the actual C function name)
        if let Some(func) = self.module.get_function(builtin.symbol) {
            return func;
        }

        // Declare the function using the signature from the table
        let fn_type = builtin.to_llvm_fn_type(self.context);
        self.module.add_function(builtin.symbol, fn_type, None)
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
        let cond_bool = self.value_to_bool(&cond_val)?;

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
            .build_conditional_branch(cond_bool, then_bb, else_bb)
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
                let elif_cond_bool = self.value_to_bool(&elif_cond_val)?;

                let elif_then_bb = self.context.append_basic_block(function, "elif_then");
                let next_bb = self.context.append_basic_block(function, "elif_next");

                self.builder
                    .build_conditional_branch(elif_cond_bool, elif_then_bb, next_bb)
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
        let cond_bool = self.value_to_bool(&cond_val)?;
        self.builder
            .build_conditional_branch(cond_bool, body_bb, after_bb)
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
    ) -> Result<PyValue<'ctx>, String> {
        // Handle short-circuit evaluation for logical operators
        if matches!(op, BinaryOp::And | BinaryOp::Or) {
            return self.generate_short_circuit_op(op, left, right);
        }

        let lhs = self.evaluate_expression(left)?;
        let rhs = self.evaluate_expression(right)?;

        // Delegate to the left type's implementation
        // The type handles coercion internally and returns a PyValue with correct type
        let cg = CgCtx::new(self.context, &self.builder, &self.module);
        lhs.binary_op(&cg, op, &rhs)
    }

    /// Generate short-circuit evaluation for `and` and `or` operators
    fn generate_short_circuit_op(
        &mut self,
        op: &BinaryOp,
        left: &Expression,
        right: &Expression,
    ) -> Result<PyValue<'ctx>, String> {
        let current_fn = self.current_function.ok_or("No current function")?;

        // Evaluate left side
        let lhs = self.evaluate_expression(left)?;

        // Convert to boolean for the condition (i1 type)
        let lhs_bool = self.value_to_bool(&lhs)?;

        // Create basic blocks for short-circuit
        let eval_rhs_bb = self.context.append_basic_block(current_fn, "eval_rhs");
        let merge_bb = self.context.append_basic_block(current_fn, "sc_merge");

        // Get current block for PHI
        let entry_bb = self.builder.get_insert_block().unwrap();

        // For `and`: if left is false, short-circuit to merge (result is false)
        // For `or`: if left is true, short-circuit to merge (result is true)
        match op {
            BinaryOp::And => {
                // If lhs is false, skip rhs and return false
                self.builder
                    .build_conditional_branch(lhs_bool, eval_rhs_bb, merge_bb)
                    .unwrap();
            }
            BinaryOp::Or => {
                // If lhs is true, skip rhs and return true
                self.builder
                    .build_conditional_branch(lhs_bool, merge_bb, eval_rhs_bb)
                    .unwrap();
            }
            _ => unreachable!(),
        }

        // Evaluate right side
        self.builder.position_at_end(eval_rhs_bb);
        let rhs = self.evaluate_expression(right)?;
        let rhs_bool = self.value_to_bool(&rhs)?;
        let rhs_bb = self.builder.get_insert_block().unwrap();
        self.builder.build_unconditional_branch(merge_bb).unwrap();

        // Merge block - use PHI to select the result
        self.builder.position_at_end(merge_bb);
        let phi = self
            .builder
            .build_phi(self.context.bool_type(), "sc_result")
            .unwrap();

        match op {
            BinaryOp::And => {
                // If we came from entry (short-circuit), result is false
                // If we came from eval_rhs, result is rhs
                phi.add_incoming(&[
                    (&self.context.bool_type().const_int(0, false), entry_bb),
                    (&rhs_bool, rhs_bb),
                ]);
            }
            BinaryOp::Or => {
                // If we came from entry (short-circuit), result is true
                // If we came from eval_rhs, result is rhs
                phi.add_incoming(&[
                    (&self.context.bool_type().const_int(1, false), entry_bb),
                    (&rhs_bool, rhs_bb),
                ]);
            }
            _ => unreachable!(),
        }

        Ok(PyValue::bool(phi.as_basic_value()))
    }

    /// Convert a PyValue to a boolean (i1)
    fn value_to_bool(
        &mut self,
        val: &PyValue<'ctx>,
    ) -> Result<inkwell::values::IntValue<'ctx>, String> {
        match &val.ty {
            PyType::Bool(_) => {
                // Already a bool
                Ok(val.value.into_int_value())
            }
            PyType::Int(_) => {
                // Non-zero is true
                let int_val = val.value.into_int_value();
                let zero = int_val.get_type().const_zero();
                Ok(self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::NE, int_val, zero, "to_bool")
                    .unwrap())
            }
            PyType::Float(_) => {
                let float_val = val.value.into_float_value();
                let zero = self.context.f64_type().const_zero();
                Ok(self
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::ONE, float_val, zero, "to_bool")
                    .unwrap())
            }
            PyType::None(_) => {
                // None is always falsy
                Ok(self.context.bool_type().const_zero())
            }
            PyType::Bytes(_) => {
                // Bytes is truthy if non-empty (check length > 0)
                let ptr_val = val.value.into_pointer_value();
                let len_fn = self.get_or_declare_builtin_function("bytes_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "bytes_len")
                    .unwrap();
                let len = self
                    .extract_int_call_result(len_call)?
                    .value
                    .into_int_value();
                let zero = len.get_type().const_zero();
                Ok(self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap())
            }
        }
    }

    /// Coerce a value to match the target type
    pub(crate) fn coerce_value_to_type(
        &self,
        val: BasicValueEnum<'ctx>,
        target_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // Check if the value is already the correct type
        match target_type {
            Type::Int => {
                if val.is_int_value() {
                    let int_val = val.into_int_value();
                    if int_val.get_type().get_bit_width() == 64 {
                        return Ok(val);
                    }
                    // Extend bool (i1) to i64
                    return Ok(self
                        .builder
                        .build_int_z_extend(int_val, self.context.i64_type(), "btoi")
                        .unwrap()
                        .into());
                }
                if val.is_float_value() {
                    // Convert float to int (truncate towards zero)
                    let float_val = val.into_float_value();
                    return Ok(self
                        .builder
                        .build_float_to_signed_int(float_val, self.context.i64_type(), "ftoi")
                        .unwrap()
                        .into());
                }
            }
            Type::Float => {
                if val.is_float_value() {
                    return Ok(val);
                }
                if val.is_int_value() {
                    let int_val = val.into_int_value();
                    return Ok(self
                        .builder
                        .build_signed_int_to_float(int_val, self.context.f64_type(), "itof")
                        .unwrap()
                        .into());
                }
            }
            Type::Bool => {
                if val.is_int_value() && val.into_int_value().get_type().get_bit_width() == 1 {
                    return Ok(val);
                }
            }
            _ => {}
        }
        // If no coercion needed or possible, return as-is
        Ok(val)
    }

    pub(crate) fn generate_unary_op(
        &mut self,
        op: &UnaryOp,
        operand: &Expression,
    ) -> Result<PyValue<'ctx>, String> {
        let val = self.evaluate_expression(operand)?;

        // Use the type from PyValue - no inference needed!
        let cg = CgCtx::new(self.context, &self.builder, &self.module);
        let result = val.ty.unary_op(&cg, op, val.value)?;

        // Determine result type
        let result_ty = self.unary_op_result_type(op, &val.ty);
        Ok(PyValue::new(result, result_ty))
    }

    /// Determine the result type of a unary operation
    fn unary_op_result_type(&self, op: &UnaryOp, operand_type: &PyType) -> PyType {
        use crate::types::BoolType;
        match op {
            UnaryOp::Not => PyType::Bool(BoolType),
            _ => operand_type.clone(),
        }
    }

    pub(crate) fn generate_call(
        &mut self,
        name: &str,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        // Handle print() specially - convert to printf calls
        if name == "print" {
            return self.generate_print_call(args);
        }

        // Handle Python built-in math functions
        if let Some(result) = self.try_generate_builtin_math_call(name, args)? {
            return Ok(result);
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
            arg_values.push(self.evaluate_expression(arg)?.value.into());
        }

        let call_site = self
            .builder
            .build_call(function, &arg_values, "call")
            .unwrap();

        // Check if the called function has a non-void return type
        let returns_value = function.get_type().get_return_type().is_some();

        // TODO: We should get the actual return type from function metadata
        // For now, we infer from the LLVM return value (not ideal but maintains compatibility)
        if returns_value {
            use inkwell::values::AnyValue;
            let any_val = call_site.as_any_value_enum();
            match any_val {
                inkwell::values::AnyValueEnum::IntValue(iv) => {
                    let ir_val: BasicValueEnum = iv.into();
                    // Check if it's a bool (i1) or int (i64)
                    if iv.get_type().get_bit_width() == 1 {
                        Ok(PyValue::bool(ir_val))
                    } else {
                        Ok(PyValue::int(ir_val))
                    }
                }
                inkwell::values::AnyValueEnum::FloatValue(fv) => Ok(PyValue::float(fv.into())),
                inkwell::values::AnyValueEnum::PointerValue(pv) => Ok(PyValue::bytes(pv.into())),
                _ => Ok(PyValue::none(
                    self.context
                        .ptr_type(inkwell::AddressSpace::default())
                        .const_null()
                        .into(),
                )),
            }
        } else {
            // Function returns void - return None
            Ok(PyValue::none(
                self.context
                    .ptr_type(inkwell::AddressSpace::default())
                    .const_null()
                    .into(),
            ))
        }
    }

    pub(crate) fn generate_print_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        let print_space = self.get_or_declare_builtin_function("print_space");
        let print_newline = self.get_or_declare_builtin_function("print_newline");

        for (i, arg) in args.iter().enumerate() {
            let val = self.evaluate_expression(arg)?;

            // Use the type from PyValue - no inference needed!
            let print_fn_name = val.ty.print_function_name();
            let print_fn = self.get_or_declare_builtin_function(print_fn_name);
            val.ty.print(&self.builder, print_fn, val.value)?;

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

        // print() returns None
        Ok(PyValue::none(
            self.context
                .ptr_type(inkwell::AddressSpace::default())
                .const_null()
                .into(),
        ))
    }

    /// Helper to extract int result from a call site
    fn extract_int_call_result(
        &self,
        call_site: inkwell::values::CallSiteValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        use inkwell::values::AnyValue;
        match call_site.as_any_value_enum() {
            inkwell::values::AnyValueEnum::IntValue(iv) => Ok(PyValue::int(iv.into())),
            _ => Err("Expected int return value".to_string()),
        }
    }

    /// Helper to extract float result from a call site
    fn extract_float_call_result(
        &self,
        call_site: inkwell::values::CallSiteValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        use inkwell::values::AnyValue;
        match call_site.as_any_value_enum() {
            inkwell::values::AnyValueEnum::FloatValue(fv) => Ok(PyValue::float(fv.into())),
            _ => Err("Expected float return value".to_string()),
        }
    }

    /// Helper to extract bytes (pointer) result from a call site
    fn extract_bytes_call_result(
        &self,
        call_site: inkwell::values::CallSiteValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        use inkwell::values::AnyValue;
        match call_site.as_any_value_enum() {
            inkwell::values::AnyValueEnum::PointerValue(pv) => Ok(PyValue::bytes(pv.into())),
            _ => Err("Expected bytes return value".to_string()),
        }
    }

    /// Generate min/max selection that preserves original value types
    /// Python's min()/max() returns the actual original object, not a coerced value.
    /// For example: max(5, 3.0) returns 5 (int), not 5.0 (float)
    /// is_min: true for min(), false for max()
    fn generate_minmax_select(
        &mut self,
        a: &PyValue<'ctx>,
        b: &PyValue<'ctx>,
        is_min: bool,
    ) -> Result<PyValue<'ctx>, String> {
        use inkwell::FloatPredicate;
        use inkwell::IntPredicate;

        // Check if both operands have the same type (use phi approach)
        // or different types (coerce to float)
        let same_type = std::mem::discriminant(&a.ty) == std::mem::discriminant(&b.ty);

        if same_type {
            // Same type: use select instruction directly on original values
            let condition = match &a.ty {
                PyType::Int(_) => {
                    let pred = if is_min {
                        IntPredicate::SLT
                    } else {
                        IntPredicate::SGT
                    };
                    self.builder
                        .build_int_compare(
                            pred,
                            a.value.into_int_value(),
                            b.value.into_int_value(),
                            "cmp",
                        )
                        .unwrap()
                }
                PyType::Float(_) => {
                    let pred = if is_min {
                        FloatPredicate::OLT
                    } else {
                        FloatPredicate::OGT
                    };
                    self.builder
                        .build_float_compare(
                            pred,
                            a.value.into_float_value(),
                            b.value.into_float_value(),
                            "cmp",
                        )
                        .unwrap()
                }
                PyType::Bool(_) => {
                    // Bools are compared as ints (False=0, True=1)
                    let a_int = self
                        .builder
                        .build_int_z_extend(
                            a.value.into_int_value(),
                            self.context.i64_type(),
                            "btoi",
                        )
                        .unwrap();
                    let b_int = self
                        .builder
                        .build_int_z_extend(
                            b.value.into_int_value(),
                            self.context.i64_type(),
                            "btoi",
                        )
                        .unwrap();
                    let pred = if is_min {
                        IntPredicate::SLT
                    } else {
                        IntPredicate::SGT
                    };
                    self.builder
                        .build_int_compare(pred, a_int, b_int, "cmp")
                        .unwrap()
                }
                _ => return Err("min()/max() arguments must be numbers".to_string()),
            };

            let select_result = self
                .builder
                .build_select(condition, a.value, b.value, "minmax_select")
                .unwrap();

            Ok(PyValue::new(select_result, a.ty.clone()))
        } else {
            // Different types: coerce to float for comparison and result
            // This is necessary because we can't have a single LLVM value with
            // runtime-dependent type. The result will always be float.
            let float_type = self.context.f64_type();

            let a_float = match &a.ty {
                PyType::Float(_) => a.value.into_float_value(),
                PyType::Int(_) => self
                    .builder
                    .build_signed_int_to_float(a.value.into_int_value(), float_type, "itof")
                    .unwrap(),
                PyType::Bool(_) => {
                    let int_val = self
                        .builder
                        .build_int_z_extend(
                            a.value.into_int_value(),
                            self.context.i64_type(),
                            "btoi",
                        )
                        .unwrap();
                    self.builder
                        .build_signed_int_to_float(int_val, float_type, "itof")
                        .unwrap()
                }
                _ => return Err("min()/max() arguments must be numbers".to_string()),
            };

            let b_float = match &b.ty {
                PyType::Float(_) => b.value.into_float_value(),
                PyType::Int(_) => self
                    .builder
                    .build_signed_int_to_float(b.value.into_int_value(), float_type, "itof")
                    .unwrap(),
                PyType::Bool(_) => {
                    let int_val = self
                        .builder
                        .build_int_z_extend(
                            b.value.into_int_value(),
                            self.context.i64_type(),
                            "btoi",
                        )
                        .unwrap();
                    self.builder
                        .build_signed_int_to_float(int_val, float_type, "itof")
                        .unwrap()
                }
                _ => return Err("min()/max() arguments must be numbers".to_string()),
            };

            let pred = if is_min {
                FloatPredicate::OLT
            } else {
                FloatPredicate::OGT
            };
            let condition = self
                .builder
                .build_float_compare(pred, a_float, b_float, "cmp")
                .unwrap();

            let select_result = self
                .builder
                .build_select(condition, a_float, b_float, "minmax_select")
                .unwrap();

            Ok(PyValue::float(select_result))
        }
    }

    /// Try to generate code for Python built-in math functions
    /// Returns Some(result) if the function is a builtin, None if it should be handled as a regular call
    fn try_generate_builtin_math_call(
        &mut self,
        name: &str,
        args: &[Expression],
    ) -> Result<Option<PyValue<'ctx>>, String> {
        match name {
            "abs" => {
                if args.len() != 1 {
                    return Err("abs() takes exactly one argument".to_string());
                }
                let val = self.evaluate_expression(&args[0])?;
                let result = match &val.ty {
                    PyType::Int(_) => {
                        let abs_fn = self.get_or_declare_builtin_function("abs_int");
                        let call = self
                            .builder
                            .build_call(abs_fn, &[val.value.into()], "abs")
                            .unwrap();
                        self.extract_int_call_result(call)?
                    }
                    PyType::Float(_) => {
                        let abs_fn = self.get_or_declare_builtin_function("abs_float");
                        let call = self
                            .builder
                            .build_call(abs_fn, &[val.value.into()], "abs")
                            .unwrap();
                        self.extract_float_call_result(call)?
                    }
                    PyType::Bool(_) => {
                        // abs(bool) -> int (0 or 1)
                        let int_val = self
                            .builder
                            .build_int_z_extend(
                                val.value.into_int_value(),
                                self.context.i64_type(),
                                "btoi",
                            )
                            .unwrap();
                        PyValue::int(int_val.into())
                    }
                    _ => return Err("abs() argument must be a number".to_string()),
                };
                Ok(Some(result))
            }

            "round" => {
                if args.is_empty() || args.len() > 2 {
                    return Err("round() takes 1 or 2 arguments".to_string());
                }
                let val = self.evaluate_expression(&args[0])?;

                if args.len() == 1 {
                    // round(x) - returns int
                    let result = match &val.ty {
                        PyType::Int(_) => val, // Already an int
                        PyType::Float(_) => {
                            let round_fn = self.get_or_declare_builtin_function("round_float");
                            let call = self
                                .builder
                                .build_call(round_fn, &[val.value.into()], "round")
                                .unwrap();
                            self.extract_int_call_result(call)?
                        }
                        PyType::Bool(_) => {
                            let int_val = self
                                .builder
                                .build_int_z_extend(
                                    val.value.into_int_value(),
                                    self.context.i64_type(),
                                    "btoi",
                                )
                                .unwrap();
                            PyValue::int(int_val.into())
                        }
                        _ => return Err("round() argument must be a number".to_string()),
                    };
                    Ok(Some(result))
                } else {
                    // round(x, ndigits) - returns float
                    let ndigits = self.evaluate_expression(&args[1])?;
                    let ndigits_int = match &ndigits.ty {
                        PyType::Int(_) => ndigits.value,
                        PyType::Bool(_) => self
                            .builder
                            .build_int_z_extend(
                                ndigits.value.into_int_value(),
                                self.context.i64_type(),
                                "btoi",
                            )
                            .unwrap()
                            .into(),
                        _ => return Err("round() ndigits must be an integer".to_string()),
                    };

                    let val_float = match &val.ty {
                        PyType::Float(_) => val.value,
                        PyType::Int(_) => self
                            .builder
                            .build_signed_int_to_float(
                                val.value.into_int_value(),
                                self.context.f64_type(),
                                "itof",
                            )
                            .unwrap()
                            .into(),
                        PyType::Bool(_) => {
                            let int_val = self
                                .builder
                                .build_int_z_extend(
                                    val.value.into_int_value(),
                                    self.context.i64_type(),
                                    "btoi",
                                )
                                .unwrap();
                            self.builder
                                .build_signed_int_to_float(int_val, self.context.f64_type(), "itof")
                                .unwrap()
                                .into()
                        }
                        _ => return Err("round() argument must be a number".to_string()),
                    };

                    let round_fn = self.get_or_declare_builtin_function("round_float_ndigits");
                    let call = self
                        .builder
                        .build_call(round_fn, &[val_float.into(), ndigits_int.into()], "round")
                        .unwrap();
                    Ok(Some(self.extract_float_call_result(call)?))
                }
            }

            "min" => {
                if args.len() != 2 {
                    return Err("min() currently supports exactly 2 arguments".to_string());
                }
                let a = self.evaluate_expression(&args[0])?;
                let b = self.evaluate_expression(&args[1])?;

                // Python's min() returns the actual original object, not a coerced value
                // We need to compare values but return the original typed value
                let result = self.generate_minmax_select(&a, &b, true)?;
                Ok(Some(result))
            }

            "max" => {
                if args.len() != 2 {
                    return Err("max() currently supports exactly 2 arguments".to_string());
                }
                let a = self.evaluate_expression(&args[0])?;
                let b = self.evaluate_expression(&args[1])?;

                // Python's max() returns the actual original object, not a coerced value
                // We need to compare values but return the original typed value
                let result = self.generate_minmax_select(&a, &b, false)?;
                Ok(Some(result))
            }

            "pow" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err("pow() takes 2 or 3 arguments".to_string());
                }
                let base = self.evaluate_expression(&args[0])?;
                let exp = self.evaluate_expression(&args[1])?;

                if args.len() == 3 {
                    // pow(base, exp, mod) - modular exponentiation (integers only)
                    let modulo = self.evaluate_expression(&args[2])?;

                    // All arguments must be integers for modular exponentiation
                    let base_int = match &base.ty {
                        PyType::Int(_) => base.value,
                        PyType::Bool(_) => self
                            .builder
                            .build_int_z_extend(
                                base.value.into_int_value(),
                                self.context.i64_type(),
                                "btoi",
                            )
                            .unwrap()
                            .into(),
                        _ => return Err("pow() with modulo requires integer arguments".to_string()),
                    };
                    let exp_int = match &exp.ty {
                        PyType::Int(_) => exp.value,
                        PyType::Bool(_) => self
                            .builder
                            .build_int_z_extend(
                                exp.value.into_int_value(),
                                self.context.i64_type(),
                                "btoi",
                            )
                            .unwrap()
                            .into(),
                        _ => return Err("pow() with modulo requires integer arguments".to_string()),
                    };
                    let mod_int = match &modulo.ty {
                        PyType::Int(_) => modulo.value,
                        PyType::Bool(_) => self
                            .builder
                            .build_int_z_extend(
                                modulo.value.into_int_value(),
                                self.context.i64_type(),
                                "btoi",
                            )
                            .unwrap()
                            .into(),
                        _ => return Err("pow() with modulo requires integer arguments".to_string()),
                    };

                    let pow_mod_fn = self.get_or_declare_builtin_function("pow_int_mod");
                    let call = self
                        .builder
                        .build_call(
                            pow_mod_fn,
                            &[base_int.into(), exp_int.into(), mod_int.into()],
                            "pow",
                        )
                        .unwrap();
                    Ok(Some(self.extract_int_call_result(call)?))
                } else {
                    // pow(base, exp) - use the ** operator through the type system
                    let cg = CgCtx::new(self.context, &self.builder, &self.module);
                    let result = base.binary_op(&cg, &BinaryOp::Pow, &exp)?;
                    Ok(Some(result))
                }
            }

            "len" => {
                if args.len() != 1 {
                    return Err("len() takes exactly one argument".to_string());
                }
                let val = self.evaluate_expression(&args[0])?;
                let result = match &val.ty {
                    PyType::Bytes(_) => {
                        let len_fn = self.get_or_declare_builtin_function("bytes_len");
                        let call = self
                            .builder
                            .build_call(len_fn, &[val.value.into()], "len")
                            .unwrap();
                        self.extract_int_call_result(call)?
                    }
                    _ => return Err("len() argument must be bytes".to_string()),
                };
                Ok(Some(result))
            }

            _ => Ok(None), // Not a builtin function
        }
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
