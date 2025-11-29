pub mod builtin;
pub mod builtins;
pub mod types;
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
use std::collections::{HashMap, HashSet};
use types::{CgCtx, PyType, PyValue};

/// Context for tracking loop control flow (break/continue)
pub(crate) struct LoopContext<'ctx> {
    pub(crate) continue_block: BasicBlock<'ctx>,
    pub(crate) break_block: BasicBlock<'ctx>,
}

pub struct CodeGen<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    /// Local variables: name -> PyValue (addressable values with ptr, llvm_type, and Python type)
    pub(crate) variables: HashMap<String, PyValue<'ctx>>,
    /// Global variables: imported modules and functions as PyValue
    pub(crate) global_variables: HashMap<String, PyValue<'ctx>>,
    /// Programs for all modules (for lazy function declaration)
    pub(crate) current_function: Option<FunctionValue<'ctx>>,
    pub(crate) strings: HashMap<String, u64>,
    pub(crate) module_name: String,
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
            global_variables: HashMap::new(),
            current_function: None,
            strings: HashMap::new(),
            module_name: module_name.to_string(),
            loop_stack: Vec::new(),
            used_builtin_modules: HashSet::new(),
        }
    }

    /// Set global variables (imported modules) for this codegen instance
    pub fn set_global_variables(&mut self, globals: HashMap<String, PyValue<'ctx>>) {
        self.global_variables = globals;
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

    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    pub fn generate(&mut self, program: &Program) -> Result<(), String> {
        // Use the visitor pattern to generate code
        self.visit_program(program)?;

        // Scan the module for builtin function usages and update used_builtin_modules
        // This captures usages from type operations that don't go through get_or_declare_c_builtin
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
                // Evaluate the function expression first
                let func_val = self.evaluate_expression(func)?;

                // Check if it's a macro (builtin with special expansion)
                if func_val.ty == PyType::Macro {
                    return self.expand_macro(&func_val, args);
                }

                // Regular function call
                let function_info = func_val.get_function()?;
                let function = function_info.function;
                let return_type = function_info.return_type;

                // Prepend bound args (for method calls), then add explicit args
                let mut arg_values: Vec<inkwell::values::BasicMetadataValueEnum> = function_info
                    .bound_args
                    .iter()
                    .map(|v| (*v).into())
                    .collect();
                for arg in args {
                    arg_values.push(self.evaluate_expression(arg)?.value().into());
                }

                let call_site = self
                    .builder
                    .build_call(function, &arg_values, "call")
                    .unwrap();

                // Use the return type from func_value metadata
                match &return_type {
                    PyType::None => Ok(PyValue::none(
                        self.context
                            .ptr_type(inkwell::AddressSpace::default())
                            .const_null()
                            .into(),
                    )),
                    PyType::Int => Ok(self.extract_int_call_result(call_site)),
                    PyType::Float => self.extract_float_call_result(call_site),
                    PyType::Bool => self.extract_bool_call_result(call_site),
                    PyType::Bytes => Ok(self.extract_bytes_call_result(call_site)),
                    PyType::List(_) | PyType::Dict(_, _) | PyType::Set(_) => {
                        // Container types return pointers
                        let ptr_val = self.extract_ptr_call_result(call_site);
                        Ok(PyValue::new(ptr_val.value(), return_type.clone(), None))
                    }
                    _ => Err(format!("Unsupported return type: {:?}", return_type)),
                }
            }
            Expression::List(elements) => self.visit_list_lit_impl(elements),
            Expression::Tuple(_) => {
                todo!("Tuple literals")
            }
            Expression::Dict(pairs) => self.visit_dict_lit_impl(pairs),
            Expression::Set(elements) => self.visit_set_lit_impl(elements),
            Expression::Attribute { object, attr } => {
                let obj = self.evaluate_expression(object)?;
                match &obj.ty {
                    PyType::Module => {
                        // Get member from module
                        let member = obj.get_member(attr)?;

                        // If it's a function, ensure it's declared in our module
                        if let PyType::Function = &member.ty {
                            let func_info = member.get_function()?;
                            // Declare the function in our module using PyType info
                            let function = func_info.declare_in_module(self.context, &self.module);
                            // Return a new FunctionInfo with the correct function reference
                            Ok(PyValue::function(types::FunctionInfo {
                                mangled_name: func_info.mangled_name,
                                function,
                                param_types: func_info.param_types,
                                return_type: func_info.return_type,
                                bound_args: func_info.bound_args,
                            }))
                        } else {
                            Ok(member)
                        }
                    }
                    PyType::Bytes => {
                        // Look up bytes method
                        self.get_bytes_method(&obj, attr)
                    }
                    PyType::List(elem_type) => {
                        // Look up list method
                        self.get_list_method(obj.value(), attr, elem_type)
                    }
                    PyType::Dict(key_type, val_type) => {
                        // Look up dict method
                        self.get_dict_method(obj.value(), attr, key_type, val_type)
                    }
                    PyType::Set(elem_type) => {
                        // Look up set method
                        self.get_set_method(obj.value(), attr, elem_type)
                    }
                    _ => Err(format!(
                        "Attribute access not supported for type {:?}",
                        obj.ty
                    )),
                }
            }
            Expression::Subscript { object, index } => {
                let obj = self.evaluate_expression(object)?;

                // Check if index is a slice expression
                if let Expression::Slice { start, stop, step } = index.as_ref() {
                    match &obj.ty {
                        PyType::Bytes => {
                            let obj_val = obj.value();
                            // bytes[start:stop:step] returns bytes
                            let len = self.bytes_len(obj_val)?;

                            if let Some(step_expr) = step {
                                // With step: use bytes_slice_step
                                let step_val = self.evaluate_expression(step_expr)?;

                                // Get start value (INT64_MAX as sentinel for "use default")
                                let start_val = if let Some(s) = start {
                                    self.evaluate_expression(s)?
                                } else {
                                    PyValue::int(
                                        self.context
                                            .i64_type()
                                            .const_int(i64::MAX as u64, false)
                                            .into(),
                                    )
                                };

                                // Get stop value (INT64_MAX as sentinel for "use default")
                                let stop_val = if let Some(e) = stop {
                                    self.evaluate_expression(e)?
                                } else {
                                    PyValue::int(
                                        self.context
                                            .i64_type()
                                            .const_int(i64::MAX as u64, false)
                                            .into(),
                                    )
                                };

                                self.bytes_slice_step(
                                    obj_val,
                                    start_val.value(),
                                    stop_val.value(),
                                    step_val.value(),
                                )
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

                                self.bytes_slice(obj_val, start_val.value(), stop_val.value())
                            }
                        }
                        PyType::List(elem_type) => {
                            let obj_val = obj.value();
                            let elem_type = elem_type.as_ref().clone();
                            // list[start:stop:step] returns list
                            let len = self.list_len(obj_val)?;

                            if let Some(step_expr) = step {
                                // With step: use list_slice_step
                                let step_val = self.evaluate_expression(step_expr)?;

                                // Get start value (INT64_MAX as sentinel for "use default")
                                let start_val = if let Some(s) = start {
                                    self.evaluate_expression(s)?
                                } else {
                                    PyValue::int(
                                        self.context
                                            .i64_type()
                                            .const_int(i64::MAX as u64, false)
                                            .into(),
                                    )
                                };

                                // Get stop value (INT64_MAX as sentinel for "use default")
                                let stop_val = if let Some(e) = stop {
                                    self.evaluate_expression(e)?
                                } else {
                                    PyValue::int(
                                        self.context
                                            .i64_type()
                                            .const_int(i64::MAX as u64, false)
                                            .into(),
                                    )
                                };

                                self.list_slice_step(
                                    obj_val,
                                    start_val.value(),
                                    stop_val.value(),
                                    step_val.value(),
                                    &elem_type,
                                )
                            } else {
                                // No step: use simpler list_slice
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

                                self.list_slice(
                                    obj_val,
                                    start_val.value(),
                                    stop_val.value(),
                                    &elem_type,
                                )
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
                        PyType::Bytes => {
                            // bytes[index] returns an int (the byte value at that index)
                            self.bytes_getitem(obj.value(), idx.value())
                        }
                        PyType::List(elem_type) => {
                            // list[index] returns the element at that index
                            self.list_getitem(obj.value(), idx.value(), elem_type.as_ref())
                        }
                        PyType::Dict(_, val_type) => {
                            // dict[key] returns the value at that key
                            self.dict_getitem(obj.value(), idx.value(), val_type.as_ref())
                        }
                        PyType::Set(_) => {
                            Err("Set does not support subscript operation".to_string())
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
            PyType::Bool => {
                // Already a bool
                Ok(val.value().into_int_value())
            }
            PyType::Int => {
                // Non-zero is true
                let int_val = val.value().into_int_value();
                let zero = int_val.get_type().const_zero();
                Ok(self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::NE, int_val, zero, "to_bool")
                    .unwrap())
            }
            PyType::Float => {
                let float_val = val.value().into_float_value();
                let zero = self.context.f64_type().const_zero();
                Ok(self
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::ONE, float_val, zero, "to_bool")
                    .unwrap())
            }
            PyType::None => {
                // None is always falsy
                Ok(self.context.bool_type().const_zero())
            }
            PyType::Bytes => {
                // Bytes is truthy if non-empty (check length > 0)
                let ptr_val = val.value().into_pointer_value();
                let len_fn = self.get_or_declare_c_builtin("bytes_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "bytes_len")
                    .unwrap();
                let len = self
                    .extract_int_call_result(len_call)
                    .value()
                    .into_int_value();
                let zero = len.get_type().const_zero();
                Ok(self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap())
            }
            PyType::Function | PyType::Module | PyType::Macro => {
                // Functions, modules, and macros are always truthy
                Ok(self.context.bool_type().const_int(1, false))
            }
            PyType::List(_) => {
                // List is truthy if non-empty (check length > 0)
                let ptr_val = val.value().into_pointer_value();
                let len_fn = self.get_or_declare_c_builtin("list_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "list_len")
                    .unwrap();
                let len = self
                    .extract_int_call_result(len_call)
                    .value()
                    .into_int_value();
                let zero = len.get_type().const_zero();
                Ok(self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap())
            }
            PyType::Dict(_, _) => {
                // Dict is truthy if non-empty (check length > 0)
                let ptr_val = val.value().into_pointer_value();
                let len_fn = self.get_or_declare_c_builtin("dict_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "dict_len")
                    .unwrap();
                let len = self
                    .extract_int_call_result(len_call)
                    .value()
                    .into_int_value();
                let zero = len.get_type().const_zero();
                Ok(self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap())
            }
            PyType::Set(_) => {
                // Set is truthy if non-empty (check length > 0)
                let ptr_val = val.value().into_pointer_value();
                let len_fn = self.get_or_declare_c_builtin("set_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "set_len")
                    .unwrap();
                let len = self
                    .extract_int_call_result(len_call)
                    .value()
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
        let result = val.unary_op(&cg, op)?;

        // Determine result type (Not always returns Bool, others preserve type)
        match op {
            UnaryOp::Not => Ok(PyValue::bool(result)),
            _ => match &val.ty {
                PyType::Int => Ok(PyValue::int(result)),
                PyType::Float => Ok(PyValue::float(result)),
                PyType::Bool => Ok(PyValue::bool(result)),
                PyType::Bytes => Ok(PyValue::bytes(result)),
                PyType::None => Ok(PyValue::none(result)),
                PyType::List(_) | PyType::Dict(_, _) | PyType::Set(_) => {
                    Err("Unary operations not supported on container types".to_string())
                }
                PyType::Function | PyType::Module | PyType::Macro => Err(
                    "Unary operations not supported on functions, modules or macros".to_string(),
                ),
            },
        }
    }

    /// Helper to extract int result from a call site
    /// This function is infallible because the type system guarantees the call returns an int
    fn extract_int_call_result(
        &self,
        call_site: inkwell::values::CallSiteValue<'ctx>,
    ) -> PyValue<'ctx> {
        use inkwell::values::AnyValue;
        let iv = call_site.as_any_value_enum().into_int_value();
        PyValue::int(iv.into())
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

    /// Helper to extract bool result from a call site
    /// Handles both i1 (native bool) and int64_t (C int) return types
    fn extract_bool_call_result(
        &self,
        call_site: inkwell::values::CallSiteValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        use inkwell::values::AnyValue;
        match call_site.as_any_value_enum() {
            inkwell::values::AnyValueEnum::IntValue(iv) => {
                if iv.get_type().get_bit_width() == 1 {
                    Ok(PyValue::bool(iv.into()))
                } else {
                    // Convert int64_t to bool by comparing with zero
                    let zero = iv.get_type().const_zero();
                    let bool_val = self
                        .builder
                        .build_int_compare(inkwell::IntPredicate::NE, iv, zero, "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
            }
            _ => Err("Expected bool return value".to_string()),
        }
    }

    /// Helper to extract bytes (pointer) result from a call site
    /// This function is infallible because the type system guarantees the call returns a pointer
    fn extract_bytes_call_result(
        &self,
        call_site: inkwell::values::CallSiteValue<'ctx>,
    ) -> PyValue<'ctx> {
        use inkwell::values::AnyValue;
        let pv = call_site.as_any_value_enum().into_pointer_value();
        PyValue::bytes(pv.into())
    }

    /// Helper to extract pointer result from a call site (for container types)
    /// This function is infallible because the type system guarantees the call returns a pointer
    pub(crate) fn extract_ptr_call_result(
        &self,
        call_site: inkwell::values::CallSiteValue<'ctx>,
    ) -> PyValue<'ctx> {
        use inkwell::values::AnyValue;
        let pv = call_site.as_any_value_enum().into_pointer_value();
        // Return as int type - caller will wrap with proper container type
        PyValue::int(pv.into())
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
            // Container types are all pointers to heap-allocated structs
            Type::List(_) => self
                .context
                .ptr_type(inkwell::AddressSpace::default())
                .into(),
            Type::Dict(_, _) => self
                .context
                .ptr_type(inkwell::AddressSpace::default())
                .into(),
            Type::Set(_) => self
                .context
                .ptr_type(inkwell::AddressSpace::default())
                .into(),
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
