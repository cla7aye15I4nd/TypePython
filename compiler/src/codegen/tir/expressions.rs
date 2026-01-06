use inkwell::values::BasicValueEnum;

use crate::ast::UnaryOp;
use crate::tir::expr::{TirConstant, TirExpr, TirExprKind};
use crate::tir::{TirProgram, TirType};

use super::declarations::call_result_to_basic_value;
use super::function_gen::FunctionGenContext;

impl<'ctx, 'a> FunctionGenContext<'ctx, 'a> {
    pub(crate) fn codegen_expr(
        &mut self,
        expr: &TirExpr,
        program: &TirProgram,
    ) -> BasicValueEnum<'ctx> {
        match &expr.kind {
            TirExprKind::Constant(c) => self.codegen_constant(c),

            TirExprKind::Var(var_ref) => self.load_var(var_ref, program),

            TirExprKind::BinOp { left, op, right } => {
                use crate::ast::BinOperator;

                // Special case: String concatenation
                if matches!(expr.ty, TirType::Class(_)) && *op == BinOperator::Add {
                    let lhs = self.codegen_expr(left, program);
                    let rhs = self.codegen_expr(right, program);

                    // Call __pyc___builtin___str___add__(lhs, rhs)
                    let func = self
                        .ctx
                        .module
                        .get_function("__pyc___builtin___str___add__")
                        .expect("String __add__ function not declared");

                    let result = self
                        .ctx
                        .builder
                        .build_call(func, &[lhs.into(), rhs.into()], "str_concat")
                        .unwrap();

                    let default = self
                        .ctx
                        .context
                        .ptr_type(inkwell::AddressSpace::default())
                        .const_null()
                        .into();
                    return call_result_to_basic_value(result, default);
                }

                let lhs = self.codegen_expr(left, program);
                let rhs = self.codegen_expr(right, program);

                // Determine if we should use float or int operations
                let is_float_op = matches!(expr.ty, TirType::Float)
                    || matches!(left.ty, TirType::Float)
                    || matches!(right.ty, TirType::Float);

                if is_float_op {
                    // Convert operands to float if necessary
                    let lhs_float = self.convert_to_float(lhs);
                    let rhs_float = self.convert_to_float(rhs);
                    self.codegen_float_binop(lhs_float, *op, rhs_float).into()
                } else {
                    self.codegen_binop(lhs.into_int_value(), *op, rhs.into_int_value())
                        .into()
                }
            }

            TirExprKind::Compare { left, op, right } => {
                use crate::ast::CompareOp;

                // Special case: String comparison
                if matches!(left.ty, TirType::Class(_)) {
                    let lhs = self.codegen_expr(left, program);
                    let rhs = self.codegen_expr(right, program);

                    // Map comparison operator to runtime function
                    let func_name = match op {
                        CompareOp::Eq => "__pyc___builtin___str___eq__",
                        CompareOp::NotEq => "__pyc___builtin___str___ne__",
                        CompareOp::Lt => "__pyc___builtin___str___lt__",
                        CompareOp::LtE => "__pyc___builtin___str___le__",
                        CompareOp::Gt => "__pyc___builtin___str___gt__",
                        CompareOp::GtE => "__pyc___builtin___str___ge__",
                    };

                    let func = self
                        .ctx
                        .module
                        .get_function(func_name)
                        .unwrap_or_else(|| panic!("{} function not declared", func_name));

                    let result = self
                        .ctx
                        .builder
                        .build_call(func, &[lhs.into(), rhs.into()], "str_cmp")
                        .unwrap();

                    let default = self.ctx.context.i8_type().const_int(0, false).into();
                    return call_result_to_basic_value(result, default);
                }

                let lhs = self.codegen_expr(left, program);
                let rhs = self.codegen_expr(right, program);

                // Determine if we should use float or int operations
                let is_float_op =
                    matches!(left.ty, TirType::Float) || matches!(right.ty, TirType::Float);

                if is_float_op {
                    let lhs_float = self.convert_to_float(lhs);
                    let rhs_float = self.convert_to_float(rhs);
                    self.codegen_float_compare(lhs_float, *op, rhs_float).into()
                } else {
                    self.codegen_compare(lhs.into_int_value(), *op, rhs.into_int_value())
                        .into()
                }
            }

            TirExprKind::BoolOp { op, values } => self.codegen_boolop(*op, values, program).into(),

            TirExprKind::UnaryOp { op, operand } => {
                let val = self.codegen_expr(operand, program);
                match op {
                    UnaryOp::Not => {
                        // Convert to bool (i1), negate, then extend back to i8
                        let bool_val = self.convert_to_bool(val);
                        let not_val = self.ctx.builder.build_not(bool_val, "not").unwrap();
                        // Extend i1 back to i8 for bool representation
                        self.ctx
                            .builder
                            .build_int_z_extend(not_val, self.ctx.context.i8_type(), "not_i8")
                            .unwrap()
                            .into()
                    }
                    UnaryOp::USub => {
                        if matches!(operand.ty, TirType::Float) {
                            let float_val = val.into_float_value();
                            self.ctx
                                .builder
                                .build_float_neg(float_val, "fneg")
                                .unwrap()
                                .into()
                        } else {
                            let int_val = val.into_int_value();
                            self.ctx
                                .builder
                                .build_int_neg(int_val, "neg")
                                .unwrap()
                                .into()
                        }
                    }
                }
            }

            TirExprKind::Call { func, args } => {
                let func_def = program.function(*func);

                // Get the LLVM function - either by runtime_name or qualified_name
                let fn_value = if let Some(runtime_name) = &func_def.runtime_name {
                    self.ctx
                        .module
                        .get_function(runtime_name)
                        .unwrap_or_else(|| panic!("Runtime function {} not found", runtime_name))
                } else {
                    self.ctx.functions[&func_def.qualified_name]
                };

                // Get LLVM function parameter types for automatic type conversion
                let fn_type = fn_value.get_type();
                let param_types: Vec<_> = fn_type.get_param_types();

                // Evaluate args with automatic type conversion based on LLVM param types
                let mut call_args = Vec::new();
                for (i, arg) in args.iter().enumerate() {
                    let arg_val = self.codegen_expr(arg, program);

                    // Convert value to match LLVM parameter type if needed
                    let converted = if i < param_types.len() {
                        let expected_type = param_types[i];
                        if expected_type.is_int_type() && arg_val.is_pointer_value() {
                            // LLVM expects i64 but we have a pointer - convert
                            self.value_to_i64(arg_val).into()
                        } else {
                            arg_val
                        }
                    } else {
                        arg_val
                    };
                    call_args.push(converted.into());
                }

                let call = self
                    .ctx
                    .builder
                    .build_call(fn_value, &call_args, "call")
                    .unwrap();
                let default = self.ctx.context.i64_type().const_int(0, false).into();
                let result = call_result_to_basic_value(call, default);

                // Convert result if TIR expects a Class but LLVM returned i64
                if let TirType::Class(_) = &expr.ty {
                    if result.is_int_value() {
                        return self.value_to_pointer(result).into();
                    }
                }

                result
            }

            TirExprKind::Construct { class, args } => {
                let class_def = program.class(*class);

                // Handle bytearray specially
                if class_def.qualified_name == "__builtin__.bytearray" {
                    if let Some(bytearray_new) = self
                        .ctx
                        .module
                        .get_function("__pyc___builtin___bytearray___init__")
                    {
                        let call = self
                            .ctx
                            .builder
                            .build_call(bytearray_new, &[], "bytearray")
                            .unwrap();
                        let default = self
                            .ctx
                            .context
                            .ptr_type(Default::default())
                            .const_null()
                            .into();
                        let ba_ptr = call_result_to_basic_value(call, default);

                        // If there's a bytes argument, copy bytes into bytearray
                        if !args.is_empty() {
                            let bytes_ptr = self.codegen_expr(&args[0], program);

                            // Get length using bytes_len(Bytes*)
                            let bytes_len_fn = self
                                .ctx
                                .module
                                .get_function("__pyc___builtin___bytes___len__")
                                .expect("__pyc___builtin___bytes___len__ not declared");
                            let len_call = self
                                .ctx
                                .builder
                                .build_call(bytes_len_fn, &[bytes_ptr.into()], "len")
                                .unwrap();
                            let i64_type = self.ctx.context.i64_type();
                            let len = call_result_to_basic_value(
                                len_call,
                                i64_type.const_int(0, false).into(),
                            )
                            .into_int_value();

                            // Loop and append each byte using bytes_getitem
                            if let Some(bytearray_append) = self
                                .ctx
                                .module
                                .get_function("__pyc___builtin___bytearray_append")
                            {
                                if let Some(bytes_getitem) = self
                                    .ctx
                                    .module
                                    .get_function("__pyc___builtin___bytes___getitem__")
                                {
                                    let zero = i64_type.const_int(0, false);
                                    let one = i64_type.const_int(1, false);

                                    // Create loop blocks
                                    let current_fn = self.ctx.current_function.unwrap();
                                    let loop_cond = self
                                        .ctx
                                        .context
                                        .append_basic_block(current_fn, "ba_loop_cond");
                                    let loop_body = self
                                        .ctx
                                        .context
                                        .append_basic_block(current_fn, "ba_loop_body");
                                    let loop_end = self
                                        .ctx
                                        .context
                                        .append_basic_block(current_fn, "ba_loop_end");

                                    // Allocate index
                                    let idx_ptr =
                                        self.ctx.builder.build_alloca(i64_type, "idx").unwrap();
                                    self.ctx.builder.build_store(idx_ptr, zero).unwrap();
                                    self.ctx
                                        .builder
                                        .build_unconditional_branch(loop_cond)
                                        .unwrap();

                                    // Loop condition
                                    self.ctx.builder.position_at_end(loop_cond);
                                    let idx = self
                                        .ctx
                                        .builder
                                        .build_load(i64_type, idx_ptr, "idx_val")
                                        .unwrap()
                                        .into_int_value();
                                    let cond = self
                                        .ctx
                                        .builder
                                        .build_int_compare(
                                            inkwell::IntPredicate::SLT,
                                            idx,
                                            len,
                                            "cond",
                                        )
                                        .unwrap();
                                    self.ctx
                                        .builder
                                        .build_conditional_branch(cond, loop_body, loop_end)
                                        .unwrap();

                                    // Loop body
                                    self.ctx.builder.position_at_end(loop_body);
                                    let call = self
                                        .ctx
                                        .builder
                                        .build_call(
                                            bytes_getitem,
                                            &[bytes_ptr.into(), idx.into()],
                                            "byte",
                                        )
                                        .unwrap();
                                    let byte_val = call_result_to_basic_value(
                                        call,
                                        i64_type.const_int(0, false).into(),
                                    );
                                    self.ctx
                                        .builder
                                        .build_call(
                                            bytearray_append,
                                            &[ba_ptr.into(), byte_val.into()],
                                            "",
                                        )
                                        .unwrap();
                                    let next_idx = self
                                        .ctx
                                        .builder
                                        .build_int_add(idx, one, "next_idx")
                                        .unwrap();
                                    self.ctx.builder.build_store(idx_ptr, next_idx).unwrap();
                                    self.ctx
                                        .builder
                                        .build_unconditional_branch(loop_cond)
                                        .unwrap();

                                    // After loop
                                    self.ctx.builder.position_at_end(loop_end);
                                }
                            }
                        }

                        return ba_ptr;
                    } else {
                        return self
                            .ctx
                            .context
                            .ptr_type(Default::default())
                            .const_null()
                            .into();
                    }
                }

                // Note: range() is now handled by TirExprKind::Range

                // Handle list builtin class
                if class_def.qualified_name.starts_with("__builtin__.list") {
                    // List construction is handled via TirExprKind::List
                    return self
                        .ctx
                        .context
                        .ptr_type(Default::default())
                        .const_null()
                        .into();
                }

                // Handle Exception builtin class
                if class_def.qualified_name == "__builtin__.Exception" {
                    if let Some(exc_init) = self
                        .ctx
                        .module
                        .get_function("__pyc___builtin___Exception___init__")
                    {
                        // Get message argument (or use empty string if no args)
                        let msg_val = if !args.is_empty() {
                            self.codegen_expr(&args[0], program)
                        } else {
                            // Create empty string for default message
                            self.create_string_constant("")
                        };

                        let call = self
                            .ctx
                            .builder
                            .build_call(exc_init, &[msg_val.into()], "exception")
                            .unwrap();
                        let default = self
                            .ctx
                            .context
                            .ptr_type(Default::default())
                            .const_null()
                            .into();
                        return call_result_to_basic_value(call, default);
                    } else {
                        return self
                            .ctx
                            .context
                            .ptr_type(Default::default())
                            .const_null()
                            .into();
                    }
                }

                // Handle classes that inherit from Exception
                // Check if this class has Exception as a parent (direct or indirect)
                // and collect parent class names for inheritance matching
                let (is_exception_subclass, parent_names) = {
                    let mut current = class_def.parent;
                    let mut is_exc = false;
                    let mut parents = Vec::new();
                    while let Some(parent_id) = current {
                        let parent_class = program.class(parent_id);
                        // Get the simple class name (last component of qualified_name)
                        let parent_name = parent_class
                            .qualified_name
                            .rsplit('.')
                            .next()
                            .unwrap_or(&parent_class.qualified_name);
                        parents.push(parent_name.to_string());
                        if parent_class.qualified_name == "__builtin__.Exception" {
                            is_exc = true;
                            break;
                        }
                        current = parent_class.parent;
                    }
                    (is_exc, parents.join(","))
                };

                if is_exception_subclass {
                    if let Some(exc_new) = self.ctx.module.get_function("__pyc_exception_new") {
                        // Get the class name (last component of qualified_name)
                        let class_name = class_def
                            .qualified_name
                            .rsplit('.')
                            .next()
                            .unwrap_or(&class_def.qualified_name);
                        let type_name = self.create_string_constant(class_name);

                        // Get message argument (or use empty string if no args)
                        let msg_val = if !args.is_empty() {
                            self.codegen_expr(&args[0], program)
                        } else {
                            self.create_string_constant("")
                        };

                        // Create parent types string for inheritance matching
                        let parent_types = self.create_string_constant(&parent_names);

                        let call = self
                            .ctx
                            .builder
                            .build_call(
                                exc_new,
                                &[type_name.into(), msg_val.into(), parent_types.into()],
                                "exception",
                            )
                            .unwrap();
                        let default = self
                            .ctx
                            .context
                            .ptr_type(Default::default())
                            .const_null()
                            .into();
                        return call_result_to_basic_value(call, default);
                    }
                }

                // Regular class construction
                let class_type = self.ctx.class_types[&class_def.qualified_name];
                let size = class_type.size_of().unwrap();

                // Call class_new runtime function
                if let Some(class_new) = self.ctx.module.get_function("class_new") {
                    let call = self
                        .ctx
                        .builder
                        .build_call(class_new, &[size.into()], "instance")
                        .unwrap();
                    let default = self
                        .ctx
                        .context
                        .ptr_type(Default::default())
                        .const_null()
                        .into();
                    let ptr = call_result_to_basic_value(call, default);

                    // Call __init__ if it exists
                    if let Some(init_func_id) = class_def.get_method("__init__") {
                        let init_func = program.function(init_func_id);
                        // Copy FunctionValue (it's Copy) to end the borrow before codegen_expr
                        if let Some(&init_fn) = self.ctx.functions.get(&init_func.qualified_name) {
                            let mut init_args: Vec<BasicValueEnum<'ctx>> = vec![ptr];
                            for arg in args {
                                init_args.push(self.codegen_expr(arg, program));
                            }
                            self.ctx
                                .builder
                                .build_call(
                                    init_fn,
                                    &init_args.iter().map(|v| (*v).into()).collect::<Vec<_>>(),
                                    "",
                                )
                                .unwrap();
                        }
                    }

                    ptr
                } else {
                    self.ctx
                        .context
                        .ptr_type(Default::default())
                        .const_null()
                        .into()
                }
            }

            TirExprKind::Range { start, stop, step } => {
                // Determine which range function to call based on provided arguments
                let (fn_name, call_args) = match (start, step) {
                    (None, None) => {
                        // range(stop)
                        let stop_val = self.codegen_expr(stop, program);
                        ("__pyc___builtin___range_1", vec![stop_val.into()])
                    }
                    (Some(start_expr), None) => {
                        // range(start, stop)
                        let start_val = self.codegen_expr(start_expr, program);
                        let stop_val = self.codegen_expr(stop, program);
                        (
                            "__pyc___builtin___range_2",
                            vec![start_val.into(), stop_val.into()],
                        )
                    }
                    (Some(start_expr), Some(step_expr)) => {
                        // range(start, stop, step)
                        let start_val = self.codegen_expr(start_expr, program);
                        let stop_val = self.codegen_expr(stop, program);
                        let step_val = self.codegen_expr(step_expr, program);
                        (
                            "__pyc___builtin___range_3",
                            vec![start_val.into(), stop_val.into(), step_val.into()],
                        )
                    }
                    (None, Some(step_expr)) => {
                        // range(stop) with step but no start - use 0 as default start
                        let i64_type = self.ctx.context.i64_type();
                        let start_val = i64_type.const_int(0, false);
                        let stop_val = self.codegen_expr(stop, program);
                        let step_val = self.codegen_expr(step_expr, program);
                        (
                            "__pyc___builtin___range_3",
                            vec![start_val.into(), stop_val.into(), step_val.into()],
                        )
                    }
                };

                let range_fn = self.ctx.module.get_function(fn_name).unwrap();
                let call = self
                    .ctx
                    .builder
                    .build_call(range_fn, &call_args, "range")
                    .unwrap();
                let default = self
                    .ctx
                    .context
                    .ptr_type(Default::default())
                    .const_null()
                    .into();
                call_result_to_basic_value(call, default)
            }

            TirExprKind::FieldAccess {
                object,
                class,
                field,
            } => {
                let obj_val = self.codegen_expr(object, program);
                let class_def = program.class(*class);
                let class_type = self.ctx.class_types[&class_def.qualified_name];

                // Convert to pointer if needed (e.g., from list_get which returns i64)
                let obj_ptr = self.value_to_pointer(obj_val);

                let field_ptr = self
                    .ctx
                    .builder
                    .build_struct_gep(class_type, obj_ptr, field.index() as u32, "field_ptr")
                    .unwrap();

                let field_ty = self.ctx.tir_type_to_llvm(&expr.ty, program);
                self.ctx
                    .builder
                    .build_load(field_ty, field_ptr, "field")
                    .unwrap()
            }

            TirExprKind::List {
                elements,
                elem_ty: _,
            } => {
                // Create a new list
                if let Some(list_new) = self
                    .ctx
                    .module
                    .get_function("__pyc___builtin___list___init__")
                {
                    let call = self.ctx.builder.build_call(list_new, &[], "list").unwrap();
                    let default = self
                        .ctx
                        .context
                        .ptr_type(Default::default())
                        .const_null()
                        .into();
                    let list_ptr = call_result_to_basic_value(call, default);

                    // Append each element
                    if let Some(list_append) = self
                        .ctx
                        .module
                        .get_function("__pyc___builtin___list_append")
                    {
                        for elem in elements {
                            let val = self.codegen_expr(elem, program);
                            // Convert value to i64 for list storage
                            let val_i64 = self.value_to_i64(val);
                            self.ctx
                                .builder
                                .build_call(list_append, &[list_ptr.into(), val_i64.into()], "")
                                .unwrap();
                        }
                    }

                    list_ptr
                } else {
                    self.ctx
                        .context
                        .ptr_type(Default::default())
                        .const_null()
                        .into()
                }
            }

            TirExprKind::Bytes { data } => {
                // Create a static Bytes struct: { i64 len, [N x i8] data }
                // This matches the C Bytes struct layout with flexible array member
                let i64_type = self.ctx.context.i64_type();
                let i8_type = self.ctx.context.i8_type();
                let array_type = i8_type.array_type(data.len() as u32);

                // Create struct type { i64, [N x i8] }
                let bytes_struct_type = self.ctx.context.struct_type(
                    &[i64_type.into(), array_type.into()],
                    false, // not packed
                );

                // Build the constant values
                let len_val = i64_type.const_int(data.len() as u64, false);
                let bytes_values: Vec<_> = data
                    .iter()
                    .map(|&b| i8_type.const_int(b as u64, false))
                    .collect();
                let bytes_array = i8_type.const_array(&bytes_values);

                // Create the struct constant
                let struct_val =
                    bytes_struct_type.const_named_struct(&[len_val.into(), bytes_array.into()]);

                // Create a global constant for this Bytes struct
                let global = self
                    .ctx
                    .module
                    .add_global(bytes_struct_type, None, "bytes_literal");
                global.set_initializer(&struct_val);
                global.set_constant(true);

                // Return pointer to the Bytes struct (Bytes*)
                global.as_pointer_value().into()
            }
        }
    }

    pub(crate) fn codegen_constant(&self, c: &TirConstant) -> BasicValueEnum<'ctx> {
        match c {
            TirConstant::Int(n) => self
                .ctx
                .context
                .i64_type()
                .const_int(*n as u64, true)
                .into(),
            TirConstant::Float(f) => self.ctx.context.f64_type().const_float(*f).into(),
            TirConstant::Str(s) => {
                // Create a static String struct: { i64 len, i32 cp_count, i16 flags, [N+1 x i8] data }
                // This matches the C String struct layout with Unicode support
                // +1 for null terminator for C compatibility
                let i64_type = self.ctx.context.i64_type();
                let i32_type = self.ctx.context.i32_type();
                let i16_type = self.ctx.context.i16_type();
                let i8_type = self.ctx.context.i8_type();
                let str_bytes = s.as_bytes();
                let len = str_bytes.len();

                // Detect if string is ASCII-only
                let is_ascii = str_bytes.iter().all(|&b| b < 128);
                let flags: u16 = if is_ascii { 0x03 } else { 0x02 }; // ASCII_ONLY | VALID_UTF8 or just VALID_UTF8
                let cp_count: i32 = if is_ascii { len as i32 } else { -1 }; // Known for ASCII, unknown for Unicode

                // Array type includes null terminator
                let array_type = i8_type.array_type((len + 1) as u32);

                // Create struct type { i64 len, i32 cp_count, i16 flags, [N+1 x i8] data }
                let string_struct_type = self.ctx.context.struct_type(
                    &[
                        i64_type.into(),
                        i32_type.into(),
                        i16_type.into(),
                        array_type.into(),
                    ],
                    false, // not packed
                );

                // Build the constant values
                let len_val = i64_type.const_int(len as u64, false);
                let cp_count_val = i32_type.const_int(cp_count as u64, true); // signed
                let flags_val = i16_type.const_int(flags as u64, false);
                let mut char_values: Vec<_> = str_bytes
                    .iter()
                    .map(|&b| i8_type.const_int(b as u64, false))
                    .collect();
                char_values.push(i8_type.const_int(0, false)); // null terminator
                let char_array = i8_type.const_array(&char_values);

                // Create the struct constant
                let struct_val = string_struct_type.const_named_struct(&[
                    len_val.into(),
                    cp_count_val.into(),
                    flags_val.into(),
                    char_array.into(),
                ]);

                // Create a global constant for this String struct
                let global = self
                    .ctx
                    .module
                    .add_global(string_struct_type, None, "str_literal");
                global.set_initializer(&struct_val);
                global.set_constant(true);

                // Return pointer to the String struct (String*)
                global.as_pointer_value().into()
            }
            TirConstant::Bool(b) => self
                .ctx
                .context
                .i8_type()
                .const_int(if *b { 1 } else { 0 }, false)
                .into(),
            TirConstant::None => self
                .ctx
                .context
                .ptr_type(Default::default())
                .const_null()
                .into(),
        }
    }

    /// Create a string constant and return a pointer to it
    /// Creates a String struct matching the C layout: { i64 len, i32 cp_count, i16 flags, char[] data }
    fn create_string_constant(&mut self, s: &str) -> inkwell::values::BasicValueEnum<'ctx> {
        let i64_type = self.ctx.context.i64_type();
        let i32_type = self.ctx.context.i32_type();
        let i16_type = self.ctx.context.i16_type();
        let i8_type = self.ctx.context.i8_type();
        let str_bytes = s.as_bytes();
        let len = str_bytes.len();

        // Check if string is ASCII-only
        let is_ascii = str_bytes.iter().all(|&b| b < 128);

        // Array type includes null terminator
        let array_type = i8_type.array_type((len + 1) as u32);

        // Create struct type { i64 len, i32 cp_count, i16 flags, [N+1 x i8] data }
        // This matches the C String struct layout
        let string_struct_type = self.ctx.context.struct_type(
            &[
                i64_type.into(),
                i32_type.into(),
                i16_type.into(),
                array_type.into(),
            ],
            false,
        );

        // Build the constant values
        let len_val = i64_type.const_int(len as u64, false);
        // For ASCII strings, cp_count equals byte length; otherwise -1 (not computed)
        let cp_count_val = if is_ascii {
            i32_type.const_int(len as u64, false)
        } else {
            i32_type.const_int((-1i32) as u64, true)
        };
        // Flags: 0x01 = ASCII_ONLY, 0x02 = VALID_UTF8
        let flags_val = if is_ascii {
            i16_type.const_int(0x03, false) // ASCII_ONLY | VALID_UTF8
        } else {
            i16_type.const_int(0x02, false) // VALID_UTF8 only
        };

        let mut char_values: Vec<_> = str_bytes
            .iter()
            .map(|&b| i8_type.const_int(b as u64, false))
            .collect();
        char_values.push(i8_type.const_int(0, false)); // null terminator
        let char_array = i8_type.const_array(&char_values);

        // Create the struct constant
        let struct_val = string_struct_type.const_named_struct(&[
            len_val.into(),
            cp_count_val.into(),
            flags_val.into(),
            char_array.into(),
        ]);

        // Create a global constant for this String struct
        let global = self
            .ctx
            .module
            .add_global(string_struct_type, None, "str_literal");
        global.set_initializer(&struct_val);
        global.set_constant(true);

        // Return pointer to the String struct (String*)
        global.as_pointer_value().into()
    }
}
