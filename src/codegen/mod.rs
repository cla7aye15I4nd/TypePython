pub mod builtin;
pub mod builtins;
pub mod types;
mod visitor;

use crate::ast::visitor::Visitor;
use crate::ast::*;
use inkwell::basic_block::BasicBlock;
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
    pub(crate) cg: CgCtx<'ctx>,
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
        let cg = CgCtx::new(context, module_name);

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
        cg.module
            .set_data_layout(&target_machine.get_target_data().get_data_layout());
        cg.module.set_triple(&target_triple);

        CodeGen {
            cg,
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
        &self.cg.module
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
            if self.cg.module.get_function(builtin_fn.symbol).is_some() {
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
                if func_val.ty() == PyType::Macro {
                    return self.expand_macro(&func_val, args);
                }

                // Regular function call
                let function_info = func_val.get_function();
                // Declare function in module if needed
                let function = function_info.get_or_declare(self.cg.ctx, &self.cg.module);
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
                    .cg
                    .builder
                    .build_call(function, &arg_values, "call")
                    .unwrap();

                // Use the return type from func_value metadata
                match &return_type {
                    PyType::None => Ok(PyValue::none(self.cg.ctx.i32_type().const_zero())),
                    PyType::Int => Ok(self.extract_int_call_result(call_site)),
                    PyType::Float => Ok(self.extract_float_call_result(call_site)),
                    PyType::Bool => Ok(self.extract_bool_call_result(call_site)),
                    PyType::Bytes => Ok(self.extract_bytes_call_result(call_site)),
                    PyType::Str => Ok(self.extract_str_call_result(call_site)),
                    PyType::List(_) | PyType::Dict(_, _) | PyType::Set(_) => {
                        // Container types return pointers
                        let ptr_val = self.extract_ptr_call_result(call_site);
                        Ok(PyValue::new(ptr_val.value(), return_type.clone(), None))
                    }
                    _ => Err(format!("Unsupported return type: {:?}", return_type)),
                }
            }
            Expression::List(elements) => self.visit_list_lit_impl(elements),
            Expression::Tuple(_) => Err("Tuple literals not yet implemented".to_string()),
            Expression::Dict(pairs) => self.visit_dict_lit_impl(pairs),
            Expression::Set(elements) => self.visit_set_lit_impl(elements),
            Expression::Attribute { object, attr } => {
                let obj = self.evaluate_expression(object)?;
                // Use unified get_member for all types (modules, bytes, str, list, dict, set)
                obj.get_member(attr)
            }
            Expression::Subscript { object, index } => {
                let obj = self.evaluate_expression(object)?;

                // Check if index is a slice expression
                if let Expression::Slice { start, stop, step } = index.as_ref() {
                    match &obj.ty() {
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
                                        self.cg.ctx.i64_type().const_int(i64::MAX as u64, false),
                                    )
                                };

                                // Get stop value (INT64_MAX as sentinel for "use default")
                                let stop_val = if let Some(e) = stop {
                                    self.evaluate_expression(e)?
                                } else {
                                    PyValue::int(
                                        self.cg.ctx.i64_type().const_int(i64::MAX as u64, false),
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
                                    PyValue::int(self.cg.ctx.i64_type().const_zero())
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
                                        self.cg.ctx.i64_type().const_int(i64::MAX as u64, false),
                                    )
                                };

                                // Get stop value (INT64_MAX as sentinel for "use default")
                                let stop_val = if let Some(e) = stop {
                                    self.evaluate_expression(e)?
                                } else {
                                    PyValue::int(
                                        self.cg.ctx.i64_type().const_int(i64::MAX as u64, false),
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
                                    PyValue::int(self.cg.ctx.i64_type().const_zero())
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
                            obj.ty()
                        )),
                    }
                } else {
                    let idx = self.evaluate_expression(index)?;
                    match &obj.ty() {
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
                            obj.ty()
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
            .map(|p| {
                let py_type = PyType::from_ast_type(&p.param_type).unwrap_or(PyType::None);
                py_type.to_llvm(self.cg.ctx).into()
            })
            .collect();

        let fn_type = match func.return_type {
            Type::None => self.cg.ctx.void_type().fn_type(&param_types, false),
            _ => {
                let return_py_type =
                    PyType::from_ast_type(&func.return_type).unwrap_or(PyType::None);
                let return_type = return_py_type.to_llvm(self.cg.ctx);
                return_type.fn_type(&param_types, false)
            }
        };

        // Mangle function name with current module name
        let mangled_name = self.mangle_function_name(&self.module_name, &func.name);
        let function = self.cg.module.add_function(&mangled_name, fn_type, None);

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
        let i32_type = self.cg.ctx.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.cg.module.add_function("main", fn_type, None);

        self.current_function = Some(function);

        let entry_bb = self.cg.ctx.append_basic_block(function, "entry");
        self.cg.builder.position_at_end(entry_bb);

        // Clear variables for new function scope
        self.variables.clear();

        // Generate all statements
        for stmt in statements {
            self.visit_statement(stmt)?;
        }

        // Return 0 if not already terminated
        if !self.is_block_terminated() {
            let zero = i32_type.const_int(0, false);
            self.cg.builder.build_return(Some(&zero)).unwrap();
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
        let cond_bool = self.cg.value_to_bool(&cond_val);

        let function = self.current_function.unwrap();
        let then_bb = self.cg.ctx.append_basic_block(function, "then");
        let merge_bb = self.cg.ctx.append_basic_block(function, "ifcont");

        // Handle elif and else
        let else_bb = if !elif_clauses.is_empty() || else_block.is_some() {
            self.cg.ctx.append_basic_block(function, "else")
        } else {
            merge_bb
        };

        self.cg
            .builder
            .build_conditional_branch(cond_bool, then_bb, else_bb)
            .unwrap();

        // Generate then block
        self.cg.builder.position_at_end(then_bb);
        for stmt in then_block {
            self.visit_statement(stmt)?;
        }
        if !self.is_block_terminated() {
            self.cg
                .builder
                .build_unconditional_branch(merge_bb)
                .unwrap();
        }

        // Generate elif/else chains
        if !elif_clauses.is_empty() || else_block.is_some() {
            self.cg.builder.position_at_end(else_bb);

            // Process elif clauses
            for (elif_cond, elif_body) in elif_clauses {
                let elif_cond_val = self.evaluate_expression(elif_cond)?;
                let elif_cond_bool = self.cg.value_to_bool(&elif_cond_val);

                let elif_then_bb = self.cg.ctx.append_basic_block(function, "elif_then");
                let next_bb = self.cg.ctx.append_basic_block(function, "elif_next");

                self.cg
                    .builder
                    .build_conditional_branch(elif_cond_bool, elif_then_bb, next_bb)
                    .unwrap();

                self.cg.builder.position_at_end(elif_then_bb);
                for stmt in elif_body {
                    self.visit_statement(stmt)?;
                }
                if !self.is_block_terminated() {
                    self.cg
                        .builder
                        .build_unconditional_branch(merge_bb)
                        .unwrap();
                }

                self.cg.builder.position_at_end(next_bb);
            }

            // Process else block
            if let Some(else_stmts) = else_block {
                for stmt in else_stmts {
                    self.visit_statement(stmt)?;
                }
            }

            if !self.is_block_terminated() {
                self.cg
                    .builder
                    .build_unconditional_branch(merge_bb)
                    .unwrap();
            }
        }

        self.cg.builder.position_at_end(merge_bb);
        Ok(())
    }

    pub(crate) fn generate_while_statement(
        &mut self,
        condition: &Expression,
        body: &[Statement],
    ) -> Result<(), String> {
        let function = self.current_function.unwrap();
        let cond_bb = self.cg.ctx.append_basic_block(function, "while_cond");
        let body_bb = self.cg.ctx.append_basic_block(function, "while_body");
        let after_bb = self.cg.ctx.append_basic_block(function, "while_after");

        self.cg.builder.build_unconditional_branch(cond_bb).unwrap();

        // Condition block
        self.cg.builder.position_at_end(cond_bb);
        let cond_val = self.evaluate_expression(condition)?;
        let cond_bool = self.cg.value_to_bool(&cond_val);
        self.cg
            .builder
            .build_conditional_branch(cond_bool, body_bb, after_bb)
            .unwrap();

        // Body block
        self.cg.builder.position_at_end(body_bb);

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
            self.cg.builder.build_unconditional_branch(cond_bb).unwrap();
        }

        // After block
        self.cg.builder.position_at_end(after_bb);
        Ok(())
    }

    pub(crate) fn generate_binary_op(
        &mut self,
        op: &BinaryOp,
        left: &Expression,
        right: &Expression,
    ) -> Result<PyValue<'ctx>, String> {
        let lhs = self.evaluate_expression(left)?;
        let rhs = self.evaluate_expression(right)?;

        // Delegate to the left type's implementation
        // The type handles coercion internally and returns a PyValue with correct type
        lhs.binary_op(&self.cg, op, &rhs)
    }

    /// Pass through value unchanged - TypePython does not support implicit type coercion
    pub(crate) fn coerce_value_to_type(
        &self,
        val: BasicValueEnum<'ctx>,
        _target_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // TypePython requires explicit type matching; no implicit coercion is performed
        Ok(val)
    }

    pub(crate) fn generate_unary_op(
        &mut self,
        op: &UnaryOp,
        operand: &Expression,
    ) -> Result<PyValue<'ctx>, String> {
        let val = self.evaluate_expression(operand)?;
        // Unary ops now return PyValue with correct type
        val.unary_op(&self.cg, op)
    }

    /// Helper to extract int result from a call site
    /// This function is infallible because the type system guarantees the call returns an int
    fn extract_int_call_result(
        &self,
        call_site: inkwell::values::CallSiteValue<'ctx>,
    ) -> PyValue<'ctx> {
        use inkwell::values::AnyValue;
        let iv = call_site.as_any_value_enum().into_int_value();
        PyValue::int(iv)
    }

    /// Helper to extract float result from a call site
    /// This function is infallible because the type system guarantees the call returns a float
    fn extract_float_call_result(
        &self,
        call_site: inkwell::values::CallSiteValue<'ctx>,
    ) -> PyValue<'ctx> {
        use inkwell::values::AnyValue;
        let fv = call_site.as_any_value_enum().into_float_value();
        PyValue::float(fv)
    }

    /// Helper to extract bool result from a call site
    /// Handles both i1 (native bool) and int64_t (C int) return types
    /// This function is infallible because the type system guarantees the call returns a bool/int
    fn extract_bool_call_result(
        &self,
        call_site: inkwell::values::CallSiteValue<'ctx>,
    ) -> PyValue<'ctx> {
        use inkwell::values::AnyValue;
        let iv = call_site.as_any_value_enum().into_int_value();
        if iv.get_type().get_bit_width() == 1 {
            PyValue::bool(iv)
        } else {
            // Convert int64_t to bool by comparing with zero
            let zero = iv.get_type().const_zero();
            let bool_val = self
                .cg
                .builder
                .build_int_compare(inkwell::IntPredicate::NE, iv, zero, "to_bool")
                .unwrap();
            PyValue::bool(bool_val)
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
        PyValue::bytes(pv)
    }

    /// Helper to extract str (pointer) result from a call site
    /// This function is infallible because the type system guarantees the call returns a pointer
    fn extract_str_call_result(
        &self,
        call_site: inkwell::values::CallSiteValue<'ctx>,
    ) -> PyValue<'ctx> {
        use inkwell::values::AnyValue;
        let pv = call_site.as_any_value_enum().into_pointer_value();
        PyValue::new_str(pv)
    }

    /// Helper to extract pointer result from a call site (for container types)
    /// This function is infallible because the type system guarantees the call returns a pointer
    pub(crate) fn extract_ptr_call_result(
        &self,
        call_site: inkwell::values::CallSiteValue<'ctx>,
    ) -> PyValue<'ctx> {
        use inkwell::values::AnyValue;
        let pv = call_site.as_any_value_enum().into_pointer_value();
        // Return as str type since it holds pointer - caller will wrap with proper container type
        PyValue::new_str(pv)
    }

    pub(crate) fn create_entry_block_alloca(
        &self,
        _fn_name: &str,
        var_name: &str,
        var_type: &PyType,
    ) -> PointerValue<'ctx> {
        let llvm_type = var_type.to_llvm(self.cg.ctx);
        self.create_entry_block_alloca_with_type(var_name, llvm_type)
    }

    pub(crate) fn create_entry_block_alloca_with_type(
        &self,
        var_name: &str,
        llvm_type: BasicTypeEnum<'ctx>,
    ) -> PointerValue<'ctx> {
        let builder = self.cg.ctx.create_builder();

        let entry = self
            .current_function
            .unwrap()
            .get_first_basic_block()
            .unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(llvm_type, var_name).unwrap()
    }

    pub(crate) fn is_block_terminated(&self) -> bool {
        if let Some(bb) = self.cg.builder.get_insert_block() {
            if let Some(_terminator) = bb.get_terminator() {
                return true;
            }
        }
        false
    }
}
