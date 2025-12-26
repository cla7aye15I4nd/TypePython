use inkwell::AddressSpace;

use crate::tir::stmt::TirStmt;
use crate::tir::TirProgram;

use super::declarations::call_result_to_basic_value;
use super::function_gen::FunctionGenContext;

impl<'ctx, 'a> FunctionGenContext<'ctx, 'a> {
    pub(crate) fn codegen_stmt(&mut self, stmt: &TirStmt, program: &TirProgram) {
        match stmt {
            TirStmt::Let { local, ty: _, init } => {
                let value = self.codegen_expr(init, program);
                let (ptr, _) = self.locals[local.index()];
                self.ctx.builder.build_store(ptr, value).unwrap();
            }

            TirStmt::Assign { target, value } => {
                let val = self.codegen_expr(value, program);
                self.store_to_lvalue(target, val, program);
            }

            TirStmt::AugAssign { target, op, value } => {
                let rhs = self.codegen_expr(value, program);
                let ptr = self.load_var_ptr(target, program);
                let lhs = self
                    .ctx
                    .builder
                    .build_load(self.ctx.context.i64_type(), ptr, "lhs")
                    .unwrap();
                let result = self.codegen_binop(lhs.into_int_value(), *op, rhs.into_int_value());
                self.ctx.builder.build_store(ptr, result).unwrap();
            }

            TirStmt::Expr(expr) => {
                self.codegen_expr(expr, program);
            }

            TirStmt::Return(Some(expr)) => {
                let value = self.codegen_expr(expr, program);
                self.ctx.builder.build_return(Some(&value)).unwrap();
            }

            TirStmt::Return(None) => {
                self.ctx.builder.build_return(None).unwrap();
            }

            TirStmt::If {
                cond,
                then_body,
                else_body,
            } => {
                let cond_val = self.codegen_expr(cond, program);
                // Handle both i1 (bool) and i64 (int) conditions
                let cond_bool = self.convert_to_bool(cond_val);

                let func = self.ctx.current_function.unwrap();
                let then_bb = self.ctx.context.append_basic_block(func, "then");
                let else_bb = self.ctx.context.append_basic_block(func, "else");
                let merge_bb = self.ctx.context.append_basic_block(func, "merge");

                self.ctx
                    .builder
                    .build_conditional_branch(cond_bool, then_bb, else_bb)
                    .unwrap();

                // Then block
                self.ctx.builder.position_at_end(then_bb);
                for s in then_body {
                    self.codegen_stmt(s, program);
                }
                // Check current block (may differ from then_bb after nested statements)
                let then_needs_merge =
                    if let Some(current_block) = self.ctx.builder.get_insert_block() {
                        if current_block.get_terminator().is_none() {
                            self.ctx
                                .builder
                                .build_unconditional_branch(merge_bb)
                                .unwrap();
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                // Else block
                self.ctx.builder.position_at_end(else_bb);
                for s in else_body {
                    self.codegen_stmt(s, program);
                }
                // Check current block (may differ from else_bb after nested statements)
                let else_needs_merge =
                    if let Some(current_block) = self.ctx.builder.get_insert_block() {
                        if current_block.get_terminator().is_none() {
                            self.ctx
                                .builder
                                .build_unconditional_branch(merge_bb)
                                .unwrap();
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                // Only use merge block if at least one branch needs it
                if then_needs_merge || else_needs_merge {
                    self.ctx.builder.position_at_end(merge_bb);
                } else {
                    // Both branches terminated (e.g., both returned), so merge block is unreachable.
                    // Remove it to avoid LLVM validation errors.
                    unsafe { merge_bb.delete().unwrap() };
                }
            }

            TirStmt::While { cond, body } => {
                let func = self.ctx.current_function.unwrap();
                let cond_bb = self.ctx.context.append_basic_block(func, "while.cond");
                let body_bb = self.ctx.context.append_basic_block(func, "while.body");
                let end_bb = self.ctx.context.append_basic_block(func, "while.end");

                self.ctx
                    .builder
                    .build_unconditional_branch(cond_bb)
                    .unwrap();

                // Condition block
                self.ctx.builder.position_at_end(cond_bb);
                let cond_val = self.codegen_expr(cond, program);
                // Handle both i1 (bool) and i64 (int) conditions
                let cond_bool = self.convert_to_bool(cond_val);
                self.ctx
                    .builder
                    .build_conditional_branch(cond_bool, body_bb, end_bb)
                    .unwrap();

                // Body block
                self.ctx.builder.position_at_end(body_bb);
                for s in body {
                    self.codegen_stmt(s, program);
                }
                // Check current block (may differ after nested statements)
                if let Some(current_block) = self.ctx.builder.get_insert_block() {
                    if current_block.get_terminator().is_none() {
                        self.ctx
                            .builder
                            .build_unconditional_branch(cond_bb)
                            .unwrap();
                    }
                }

                self.ctx.builder.position_at_end(end_bb);
            }

            TirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                // Polling-based exception handling:
                // After each statement in try body, check __pyc_has_exception() and branch to handlers if set.
                // This avoids setjmp/longjmp which don't work in freestanding mode.

                let func = self.ctx.current_function.unwrap();
                let i32_type = self.ctx.context.i32_type();

                // Create basic blocks
                let try_bb = self.ctx.context.append_basic_block(func, "try.body");
                let handlers_bb = self.ctx.context.append_basic_block(func, "try.handlers");
                let else_bb = self.ctx.context.append_basic_block(func, "try.else");
                let finally_bb = self.ctx.context.append_basic_block(func, "try.finally");
                let end_bb = self.ctx.context.append_basic_block(func, "try.end");

                // Allocate exception frame on stack (for marking that we have a handler)
                let frame_type = self.ctx.context.i8_type().array_type(64);
                let frame_ptr = self
                    .ctx
                    .builder
                    .build_alloca(frame_type, "exc_frame")
                    .unwrap();

                // Push exception frame
                let push_fn = self
                    .ctx
                    .module
                    .get_function("__pyc_push_exception_frame")
                    .unwrap();
                self.ctx
                    .builder
                    .build_call(push_fn, &[frame_ptr.into()], "")
                    .unwrap();

                // Branch to try body
                self.ctx.builder.build_unconditional_branch(try_bb).unwrap();
                self.ctx.builder.position_at_end(try_bb);

                // Generate try body with polling after each statement
                let has_exc_fn = self.ctx.module.get_function("__pyc_has_exception").unwrap();

                for (i, s) in body.iter().enumerate() {
                    // Execute statement
                    self.codegen_stmt(s, program);

                    // Check if current block was terminated (e.g., by return)
                    let should_poll =
                        if let Some(current_block) = self.ctx.builder.get_insert_block() {
                            current_block.get_terminator().is_none()
                        } else {
                            false
                        };

                    if !should_poll {
                        break; // Block already terminated, no more statements to generate
                    }

                    // Poll for exception
                    let has_exc_call = self
                        .ctx
                        .builder
                        .build_call(has_exc_fn, &[], "has_exc")
                        .unwrap();
                    let has_exc = call_result_to_basic_value(
                        has_exc_call,
                        i32_type.const_int(0, false).into(),
                    )
                    .into_int_value();
                    let has_exc_bool = self
                        .ctx
                        .builder
                        .build_int_compare(
                            inkwell::IntPredicate::NE,
                            has_exc,
                            i32_type.const_int(0, false),
                            "poll",
                        )
                        .unwrap();

                    // Create continuation block
                    let cont_bb = self
                        .ctx
                        .context
                        .append_basic_block(func, &format!("try.cont_{}", i));
                    self.ctx
                        .builder
                        .build_conditional_branch(has_exc_bool, handlers_bb, cont_bb)
                        .unwrap();

                    // Continue in new block
                    self.ctx.builder.position_at_end(cont_bb);
                }

                // If we reach here without exception, go to else block
                if let Some(current_block) = self.ctx.builder.get_insert_block() {
                    if current_block.get_terminator().is_none() {
                        self.ctx
                            .builder
                            .build_unconditional_branch(else_bb)
                            .unwrap();
                    }
                }

                // Handlers block (exception caught)
                self.ctx.builder.position_at_end(handlers_bb);

                // Create unhandled block first
                let unhandled_bb = self.ctx.context.append_basic_block(func, "unhandled");

                // If no handlers, go directly to unhandled (which will re-raise after finally)
                if handlers.is_empty() {
                    self.ctx
                        .builder
                        .build_unconditional_branch(unhandled_bb)
                        .unwrap();
                } else {
                    // Get current exception
                    let get_exc_fn = self.ctx.module.get_function("__pyc_get_exception").unwrap();
                    let exc_call = self.ctx.builder.build_call(get_exc_fn, &[], "exc").unwrap();
                    let default_ptr = self
                        .ctx
                        .context
                        .ptr_type(AddressSpace::default())
                        .const_null()
                        .into();
                    let exc_val = call_result_to_basic_value(exc_call, default_ptr);

                    // Check each handler
                    let mut handler_bbs = Vec::new();
                    for (i, _handler) in handlers.iter().enumerate() {
                        let handler_bb = self
                            .ctx
                            .context
                            .append_basic_block(func, &format!("handler_{}", i));
                        handler_bbs.push(handler_bb);
                    }

                    // For each handler, check if it matches and branch appropriately
                    let mut current_check_bb = handlers_bb;
                    for (i, handler) in handlers.iter().enumerate() {
                        self.ctx.builder.position_at_end(current_check_bb);

                        if let Some(exc_class) = handler.exc_class {
                            // Type-specific handler - check if exception matches
                            let next_check_bb = if i + 1 < handlers.len() {
                                self.ctx
                                    .context
                                    .append_basic_block(func, &format!("check_{}", i + 1))
                            } else {
                                unhandled_bb
                            };

                            // Get the exception class name
                            let class_def = program.class(exc_class);
                            let class_name = class_def
                                .qualified_name
                                .rsplit('.')
                                .next()
                                .unwrap_or(&class_def.qualified_name);

                            // Create a global C string constant for the type name
                            let type_name_ptr = self
                                .ctx
                                .builder
                                .build_global_string_ptr(class_name, "exc_type_name")
                                .unwrap()
                                .as_pointer_value();

                            // Call __pyc_exception_matches(exception, type_name)
                            let matches_fn = self
                                .ctx
                                .module
                                .get_function("__pyc_exception_matches")
                                .unwrap();
                            let matches_call = self
                                .ctx
                                .builder
                                .build_call(
                                    matches_fn,
                                    &[exc_val.into(), type_name_ptr.into()],
                                    "matches",
                                )
                                .unwrap();
                            let default_i32 =
                                self.ctx.context.i32_type().const_int(0, false).into();
                            let matches_val = call_result_to_basic_value(matches_call, default_i32)
                                .into_int_value();

                            // Compare with 0 to get boolean
                            let zero = self.ctx.context.i32_type().const_zero();
                            let cond = self
                                .ctx
                                .builder
                                .build_int_compare(
                                    inkwell::IntPredicate::NE,
                                    matches_val,
                                    zero,
                                    "match_cond",
                                )
                                .unwrap();

                            // Branch: if matches, go to handler, else check next
                            self.ctx
                                .builder
                                .build_conditional_branch(cond, handler_bbs[i], next_check_bb)
                                .unwrap();
                            current_check_bb = next_check_bb;
                        } else {
                            // Bare except: catches all exceptions
                            self.ctx
                                .builder
                                .build_unconditional_branch(handler_bbs[i])
                                .unwrap();
                            break;
                        }
                    }

                    // Generate handler bodies
                    for (i, handler) in handlers.iter().enumerate() {
                        self.ctx.builder.position_at_end(handler_bbs[i]);

                        // Bind exception to local if named
                        if let Some(local_id) = handler.local {
                            let (ptr, _) = self.locals[local_id.index()];
                            self.ctx.builder.build_store(ptr, exc_val).unwrap();
                        }

                        // Clear the original exception at the START of handler
                        // (we're now handling it, any new raise will set a new exception)
                        let clear_fn = self
                            .ctx
                            .module
                            .get_function("__pyc_clear_exception")
                            .unwrap();
                        self.ctx.builder.build_call(clear_fn, &[], "").unwrap();

                        // Execute handler body with polling for new exceptions
                        for (j, s) in handler.body.iter().enumerate() {
                            self.codegen_stmt(s, program);

                            // Check if current block was terminated (e.g., by return)
                            let should_poll =
                                if let Some(current_block) = self.ctx.builder.get_insert_block() {
                                    current_block.get_terminator().is_none()
                                } else {
                                    false
                                };

                            if !should_poll {
                                break;
                            }

                            // Poll for new exception raised in handler
                            let has_exc_call = self
                                .ctx
                                .builder
                                .build_call(has_exc_fn, &[], "has_exc")
                                .unwrap();
                            let has_exc = call_result_to_basic_value(
                                has_exc_call,
                                i32_type.const_int(0, false).into(),
                            )
                            .into_int_value();
                            let has_exc_bool = self
                                .ctx
                                .builder
                                .build_int_compare(
                                    inkwell::IntPredicate::NE,
                                    has_exc,
                                    i32_type.const_int(0, false),
                                    "poll_handler",
                                )
                                .unwrap();

                            // Create continuation block for this handler statement
                            let cont_bb = self
                                .ctx
                                .context
                                .append_basic_block(func, &format!("handler_{}.cont_{}", i, j));
                            // If exception, go to finally (which will re-raise)
                            self.ctx
                                .builder
                                .build_conditional_branch(has_exc_bool, finally_bb, cont_bb)
                                .unwrap();

                            self.ctx.builder.position_at_end(cont_bb);
                        }

                        // Go to finally normally (no exception in handler)
                        if let Some(current_block) = self.ctx.builder.get_insert_block() {
                            if current_block.get_terminator().is_none() {
                                self.ctx
                                    .builder
                                    .build_unconditional_branch(finally_bb)
                                    .unwrap();
                            }
                        }
                    }
                }

                // Unhandled exception - re-raise after finally
                self.ctx.builder.position_at_end(unhandled_bb);
                self.ctx
                    .builder
                    .build_unconditional_branch(finally_bb)
                    .unwrap();

                // Else block (no exception occurred)
                self.ctx.builder.position_at_end(else_bb);
                for s in orelse {
                    self.codegen_stmt(s, program);
                }
                if let Some(current_block) = self.ctx.builder.get_insert_block() {
                    if current_block.get_terminator().is_none() {
                        self.ctx
                            .builder
                            .build_unconditional_branch(finally_bb)
                            .unwrap();
                    }
                }

                // Finally block (always runs)
                self.ctx.builder.position_at_end(finally_bb);

                // Pop exception frame
                let pop_fn = self
                    .ctx
                    .module
                    .get_function("__pyc_pop_exception_frame")
                    .unwrap();
                self.ctx.builder.build_call(pop_fn, &[], "").unwrap();

                // Execute finally body
                for s in finalbody {
                    self.codegen_stmt(s, program);
                }

                // Check if there's a pending exception to re-raise
                let has_exc_call = self
                    .ctx
                    .builder
                    .build_call(has_exc_fn, &[], "has_exc")
                    .unwrap();
                let default_i32_zero = i32_type.const_int(0, false).into();
                let has_exc =
                    call_result_to_basic_value(has_exc_call, default_i32_zero).into_int_value();
                let has_exc_bool = self
                    .ctx
                    .builder
                    .build_int_compare(
                        inkwell::IntPredicate::NE,
                        has_exc,
                        i32_type.const_int(0, false),
                        "has_exc_bool",
                    )
                    .unwrap();

                let reraise_bb = self.ctx.context.append_basic_block(func, "reraise");

                if let Some(current_block) = self.ctx.builder.get_insert_block() {
                    if current_block.get_terminator().is_none() {
                        self.ctx
                            .builder
                            .build_conditional_branch(has_exc_bool, reraise_bb, end_bb)
                            .unwrap();
                    }
                }

                // Re-raise block
                // After reraise, if there's an outer exception frame, __pyc_reraise returns
                // and we continue to end_bb. The outer try block will poll and catch the exception.
                self.ctx.builder.position_at_end(reraise_bb);
                let reraise_fn = self.ctx.module.get_function("__pyc_reraise").unwrap();
                self.ctx.builder.build_call(reraise_fn, &[], "").unwrap();
                // Branch to end_bb - if reraise returned, there's an outer handler that will catch it
                self.ctx.builder.build_unconditional_branch(end_bb).unwrap();

                // End block (continue after try)
                self.ctx.builder.position_at_end(end_bb);
            }

            TirStmt::Raise { exc } => {
                if let Some(exc_expr) = exc {
                    let exc_val = self.codegen_expr(exc_expr, program);
                    let raise_fn = self.ctx.module.get_function("__pyc_raise").unwrap();
                    self.ctx
                        .builder
                        .build_call(raise_fn, &[exc_val.into()], "")
                        .unwrap();
                } else {
                    // Bare raise - re-raise current exception
                    let reraise_fn = self.ctx.module.get_function("__pyc_reraise").unwrap();
                    self.ctx.builder.build_call(reraise_fn, &[], "").unwrap();
                }
                // In polling mode: raise just sets the exception and returns.
                // The try block's polling will detect it and branch to handlers.
                // If not in a try block, __pyc_raise exits the program.
            }
        }
    }
}
