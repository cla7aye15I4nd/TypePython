use crate::ast::{Constant, Expr, Stmt, UnaryOp};
use crate::error::{CompilerError, Result};
use crate::tir::expr::VarRef;
use crate::tir::expr_unresolved::{TirExprKindUnresolved, TirExprUnresolved};
use crate::tir::stmt_unresolved::{
    TirExceptHandlerUnresolved, TirLValueUnresolved, TirStmtUnresolved,
};
use crate::tir::types_unresolved::TirTypeUnresolved;

use super::body_lowerer::BodyLowerer;

impl<'a> BodyLowerer<'a> {
    pub(crate) fn lower_stmt(&mut self, stmt: &Stmt) -> Result<Vec<TirStmtUnresolved>> {
        match stmt {
            Stmt::Assign {
                target,
                value,
                type_annotation,
            } => {
                let value_expr = self.lower_expr(value)?;

                match target {
                    Expr::Name(name) => {
                        // Check if this is a new variable or existing
                        if let Some((var_ref, var_ty)) = self.resolve_var(name) {
                            // Existing variable - check type compatibility
                            if !value_expr.ty.is_compatible_with(&var_ty) {
                                return Err(CompilerError::TypeErrorSimple(format!(
                                    "Cannot assign {:?} to variable '{}' of type {:?}",
                                    value_expr.ty, name, var_ty
                                )));
                            }
                            Ok(vec![TirStmtUnresolved::Assign {
                                target: TirLValueUnresolved::Var(var_ref),
                                value: value_expr,
                            }])
                        } else {
                            // New variable - create Let
                            let ty = if let Some(annot) = type_annotation {
                                let declared_ty = self.convert_annotation(annot);
                                // Check that value type matches declared type
                                if !value_expr.ty.is_compatible_with(&declared_ty) {
                                    return Err(CompilerError::TypeErrorSimple(format!(
                                        "Cannot assign {:?} to variable of type {:?}",
                                        value_expr.ty, declared_ty
                                    )));
                                }
                                declared_ty
                            } else {
                                value_expr.ty.clone()
                            };
                            let local_id = self.alloc_local(name, ty.clone());
                            Ok(vec![TirStmtUnresolved::Let {
                                local: local_id,
                                ty,
                                init: value_expr,
                            }])
                        }
                    }

                    Expr::Attribute {
                        value: obj,
                        attr: field,
                    } => {
                        let obj_expr = self.lower_expr(obj)?;
                        if let Some(class_id) = obj_expr.ty.class_id() {
                            if let Some(&field_id) =
                                self.symbols.fields.get(&(class_id, field.clone()))
                            {
                                // Get field type - check inherited_fields first, then own fields
                                let class_data = &self.symbols.class_data[class_id.index()];
                                let inherited_count = class_data.inherited_fields.len();
                                let field_idx = field_id.index();

                                let field_ty = if field_idx < inherited_count {
                                    // It's an inherited field
                                    TirTypeUnresolved::from_tir_type(
                                        &class_data.inherited_fields[field_idx].1,
                                    )
                                } else {
                                    // It's an own field
                                    TirTypeUnresolved::from_tir_type(
                                        &class_data.fields[field_idx - inherited_count].1,
                                    )
                                };

                                // Check compatibility
                                if !value_expr.ty.is_compatible_with(&field_ty) {
                                    return Err(CompilerError::TypeErrorSimple(format!(
                                        "Cannot assign {:?} to field '{}' of type {:?}",
                                        value_expr.ty, field, field_ty
                                    )));
                                }

                                return Ok(vec![TirStmtUnresolved::Assign {
                                    target: TirLValueUnresolved::Field {
                                        object: Box::new(obj_expr),
                                        class: class_id,
                                        field: field_id,
                                    },
                                    value: value_expr,
                                }]);
                            }
                        }
                        Err(CompilerError::TypeErrorSimple(format!(
                            "Cannot assign to field {}",
                            field
                        )))
                    }

                    Expr::Subscript {
                        value: container,
                        index,
                    } => {
                        let container_expr = self.lower_expr(container)?;
                        let index_expr = self.lower_expr(index)?;

                        // Look up __setitem__ method and convert to a Call expression statement
                        let setitem_call = call_dunder_method!(
                            self.symbols,
                            &container_expr.ty,
                            "__setitem__",
                            vec![container_expr, index_expr, value_expr]
                        )?;
                        Ok(vec![TirStmtUnresolved::Expr(setitem_call)])
                    }

                    _ => Err(CompilerError::UnsupportedFeature(format!(
                        "Unsupported assignment target: {:?}",
                        target
                    ))),
                }
            }

            Stmt::AugAssign { target, op, value } => {
                let value_expr = self.lower_expr(value)?;
                if let Some((var_ref, var_ty)) = self.resolve_var(target) {
                    // Check that both target and value are numeric for augmented assignment
                    if !var_ty.is_numeric() {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "Augmented assignment target '{}' must be numeric, got {:?}",
                            target, var_ty
                        )));
                    }
                    if !value_expr.ty.is_numeric() {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "Augmented assignment value must be numeric, got {:?}",
                            value_expr.ty
                        )));
                    }
                    Ok(vec![TirStmtUnresolved::AugAssign {
                        target: var_ref,
                        op: *op,
                        value: value_expr,
                    }])
                } else {
                    Err(CompilerError::UndefinedVariable(target.clone()))
                }
            }

            Stmt::Expr { value } => {
                // Check if this is a print() call - expand at statement level
                if let Expr::Call { func, args } = value {
                    if let Expr::Name(name) = &**func {
                        if name == "print" {
                            return self.expand_print_stmt(args);
                        }
                    }
                }
                let expr = self.lower_expr(value)?;
                Ok(vec![TirStmtUnresolved::Expr(expr)])
            }

            Stmt::Return { value } => {
                let expr = value.as_ref().map(|v| self.lower_expr(v)).transpose()?;

                // Check return type compatibility
                match (&expr, &self.return_type) {
                    (Some(e), TirTypeUnresolved::Void) => {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "Cannot return {:?} from function with void return type",
                            e.ty
                        )));
                    }
                    (None, ty) if *ty != TirTypeUnresolved::Void => {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "Missing return value, expected {:?}",
                            ty
                        )));
                    }
                    (Some(e), expected_ty) if !e.ty.is_compatible_with(expected_ty) => {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "Return type mismatch: expected {:?}, got {:?}",
                            expected_ty, e.ty
                        )));
                    }
                    _ => {}
                }

                Ok(vec![TirStmtUnresolved::Return(expr)])
            }

            Stmt::If { test, body, orelse } => {
                let cond = self.lower_expr(test)?;

                self.enter_scope();
                let mut then_body = Vec::new();
                for stmt in body {
                    then_body.extend(self.lower_stmt(stmt)?);
                }
                self.exit_scope();

                self.enter_scope();
                let mut else_body = Vec::new();
                for stmt in orelse {
                    else_body.extend(self.lower_stmt(stmt)?);
                }
                self.exit_scope();

                Ok(vec![TirStmtUnresolved::If {
                    cond,
                    then_body,
                    else_body,
                }])
            }

            Stmt::While { test, body } => {
                let cond = self.lower_expr(test)?;

                self.enter_scope();
                let mut loop_body = Vec::new();
                for stmt in body {
                    loop_body.extend(self.lower_stmt(stmt)?);
                }
                self.exit_scope();

                Ok(vec![TirStmtUnresolved::While {
                    cond,
                    body: loop_body,
                }])
            }

            Stmt::For { target, iter, body } => {
                // Desugar for loop:
                //   for target in iter:
                //       <body>
                // becomes:
                //   _iter = iter.__iter__()
                //   _done = False
                //   try:
                //       while not _done:
                //           try:
                //               target = _iter.__next__()
                //               <body>
                //           except StopIteration:
                //               _done = True
                //   finally:
                //       _iter.__dealloc__()

                let mut result = Vec::new();

                // Lower the iterable expression
                let iterable_expr = self.lower_expr(iter)?;

                // Call __iter__ on the iterable
                let iter_call = call_dunder_method!(
                    self.symbols,
                    &iterable_expr.ty,
                    "__iter__",
                    vec![iterable_expr]
                )?;
                let iter_ty = iter_call.ty.clone();

                // Create unique names for temporaries using local counter
                let iter_name = format!("_for_iter_{}", self.next_local_id);
                let done_name = format!("_for_done_{}", self.next_local_id);

                // Allocate _iter local and initialize it
                let iter_local_id = self.alloc_local(&iter_name, iter_ty.clone());
                result.push(TirStmtUnresolved::Let {
                    local: iter_local_id,
                    ty: iter_ty.clone(),
                    init: iter_call,
                });

                // Allocate _done local and initialize to False
                let done_local_id = self.alloc_local(&done_name, TirTypeUnresolved::Bool);
                result.push(TirStmtUnresolved::Let {
                    local: done_local_id,
                    ty: TirTypeUnresolved::Bool,
                    init: TirExprUnresolved::new(
                        TirExprKindUnresolved::Constant(Constant::Bool(false)),
                        TirTypeUnresolved::Bool,
                    ),
                });

                // Get StopIteration class for exception handling
                let stop_iteration_class = self.symbols.get_or_create_stop_iteration_class();

                // Build the while loop body with try/except
                self.enter_scope();

                // Call __next__ on the iterator
                let iter_var = TirExprUnresolved::new(
                    TirExprKindUnresolved::Var(VarRef::Local(iter_local_id)),
                    iter_ty.clone(),
                );
                let next_call = call_dunder_method!(
                    self.symbols,
                    &iter_ty,
                    "__next__",
                    vec![iter_var.clone()]
                )?;
                let elem_ty = next_call.ty.clone();

                // Allocate the loop target variable
                let target_local_id = self.alloc_local(target, elem_ty.clone());

                // Build the try body: target = _iter.__next__() followed by loop body
                let mut try_body = vec![TirStmtUnresolved::Let {
                    local: target_local_id,
                    ty: elem_ty,
                    init: next_call,
                }];

                // Lower the actual for loop body
                for stmt in body {
                    try_body.extend(self.lower_stmt(stmt)?);
                }

                // Build the except handler: _done = True
                let except_handler = TirExceptHandlerUnresolved {
                    exc_class: Some(stop_iteration_class),
                    local: None,
                    body: vec![TirStmtUnresolved::Assign {
                        target: TirLValueUnresolved::Var(VarRef::Local(done_local_id)),
                        value: TirExprUnresolved::new(
                            TirExprKindUnresolved::Constant(Constant::Bool(true)),
                            TirTypeUnresolved::Bool,
                        ),
                    }],
                };

                // Build the inner try statement for __next__ call
                let inner_try_stmt = TirStmtUnresolved::Try {
                    body: try_body,
                    handlers: vec![except_handler],
                    orelse: vec![],
                    finalbody: vec![],
                };

                self.exit_scope();

                // Build while condition: not _done
                let done_var = TirExprUnresolved::new(
                    TirExprKindUnresolved::Var(VarRef::Local(done_local_id)),
                    TirTypeUnresolved::Bool,
                );
                let while_cond = TirExprUnresolved::new(
                    TirExprKindUnresolved::UnaryOp {
                        op: UnaryOp::Not,
                        operand: Box::new(done_var),
                    },
                    TirTypeUnresolved::Bool,
                );

                // Build the while loop
                let while_stmt = TirStmtUnresolved::While {
                    cond: while_cond,
                    body: vec![inner_try_stmt],
                };

                // Build the finally block to deallocate the iterator
                let dealloc_call =
                    call_dunder_method!(self.symbols, &iter_ty, "__dealloc__", vec![iter_var])?;
                let finally_body = vec![TirStmtUnresolved::Expr(dealloc_call)];

                // Wrap the while loop in try-finally to ensure iterator cleanup
                result.push(TirStmtUnresolved::Try {
                    body: vec![while_stmt],
                    handlers: vec![],
                    orelse: vec![],
                    finalbody: finally_body,
                });

                Ok(result)
            }

            Stmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                // Lower try body
                self.enter_scope();
                let mut tir_body = Vec::new();
                for stmt in body {
                    tir_body.extend(self.lower_stmt(stmt)?);
                }
                self.exit_scope();

                // Lower exception handlers
                let mut tir_handlers = Vec::new();
                for handler in handlers {
                    self.enter_scope();

                    // Resolve exception class if specified
                    let exc_class = if let Some(type_name) = &handler.exc_type {
                        Some(self.resolve_exception_class(type_name)?)
                    } else {
                        None
                    };

                    // Allocate local for exception variable if named
                    let local = if let Some(name) = &handler.name {
                        let exc_class_id = self.symbols.get_or_create_exception_class();
                        Some(self.alloc_local(name, TirTypeUnresolved::Class(exc_class_id)))
                    } else {
                        None
                    };

                    // Lower handler body
                    let mut handler_body = Vec::new();
                    for stmt in &handler.body {
                        handler_body.extend(self.lower_stmt(stmt)?);
                    }

                    self.exit_scope();

                    tir_handlers.push(TirExceptHandlerUnresolved {
                        exc_class,
                        local,
                        body: handler_body,
                    });
                }

                // Lower else clause
                self.enter_scope();
                let mut tir_orelse = Vec::new();
                for stmt in orelse {
                    tir_orelse.extend(self.lower_stmt(stmt)?);
                }
                self.exit_scope();

                // Lower finally clause
                self.enter_scope();
                let mut tir_finalbody = Vec::new();
                for stmt in finalbody {
                    tir_finalbody.extend(self.lower_stmt(stmt)?);
                }
                self.exit_scope();

                Ok(vec![TirStmtUnresolved::Try {
                    body: tir_body,
                    handlers: tir_handlers,
                    orelse: tir_orelse,
                    finalbody: tir_finalbody,
                }])
            }

            Stmt::Raise { exc } => {
                let tir_exc = exc.as_ref().map(|e| self.lower_expr(e)).transpose()?;
                Ok(vec![TirStmtUnresolved::Raise { exc: tir_exc }])
            }

            // Skip function and class definitions - they're handled at module level
            Stmt::FunctionDef { .. } | Stmt::ClassDef { .. } => Ok(vec![]),
        }
    }

    /// Expand print(args...) into multiple TIR statements
    ///
    /// print(x, y, z) becomes:
    ///   call(int_print, x)     or call(write_string_impl, x.__str__())
    ///   call(write_space_impl)
    ///   call(int_print, y)     or call(write_string_impl, y.__str__())
    ///   call(write_space_impl)
    ///   call(int_print, z)     or call(write_string_impl, z.__str__())
    ///   call(write_newline_impl)
    fn expand_print_stmt(&mut self, args: &[Expr]) -> Result<Vec<TirStmtUnresolved>> {
        let mut stmts = Vec::new();

        // Get FuncIds for print helpers
        let int_print_func = self.symbols.get_int_print_func();
        let float_print_func = self.symbols.get_float_print_func();
        let bool_print_func = self.symbols.get_bool_print_func();
        let write_string_func = self.symbols.get_write_string_func();
        let write_space_func = self.symbols.get_write_space_func();
        let write_newline_func = self.symbols.get_write_newline_func();

        let str_class_id = self.symbols.get_or_create_str_class();
        let str_type = TirTypeUnresolved::Class(str_class_id);

        for (i, arg) in args.iter().enumerate() {
            // Add space separator between arguments
            if i > 0 {
                stmts.push(TirStmtUnresolved::Expr(TirExprUnresolved::new(
                    TirExprKindUnresolved::Call {
                        func: write_space_func,
                        args: vec![],
                    },
                    TirTypeUnresolved::Void,
                )));
            }

            // Lower the argument
            let lowered_arg = self.lower_expr(arg)?;

            // Generate the appropriate print call based on type
            let print_stmt = match &lowered_arg.ty {
                TirTypeUnresolved::Int => TirStmtUnresolved::Expr(TirExprUnresolved::new(
                    TirExprKindUnresolved::Call {
                        func: int_print_func,
                        args: vec![lowered_arg],
                    },
                    TirTypeUnresolved::Void,
                )),
                TirTypeUnresolved::Float => TirStmtUnresolved::Expr(TirExprUnresolved::new(
                    TirExprKindUnresolved::Call {
                        func: float_print_func,
                        args: vec![lowered_arg],
                    },
                    TirTypeUnresolved::Void,
                )),
                TirTypeUnresolved::Bool => TirStmtUnresolved::Expr(TirExprUnresolved::new(
                    TirExprKindUnresolved::Call {
                        func: bool_print_func,
                        args: vec![lowered_arg],
                    },
                    TirTypeUnresolved::Void,
                )),
                TirTypeUnresolved::Class(class_id) => {
                    // For classes, call __str__ or __repr__ to convert to String*
                    let str_expr = if let Some((_method_id, func_id)) =
                        self.symbols.resolve_method(*class_id, "__str__")
                    {
                        TirExprUnresolved::new(
                            TirExprKindUnresolved::Call {
                                func: func_id,
                                args: vec![lowered_arg],
                            },
                            str_type.clone(),
                        )
                    } else if let Some((_method_id, func_id)) =
                        self.symbols.resolve_method(*class_id, "__repr__")
                    {
                        TirExprUnresolved::new(
                            TirExprKindUnresolved::Call {
                                func: func_id,
                                args: vec![lowered_arg],
                            },
                            str_type.clone(),
                        )
                    } else {
                        // No __str__ or __repr__ - create default string
                        let class_name = &self.symbols.class_data[class_id.index()].qualified_name;
                        let simple_name = class_name.rsplit('.').next().unwrap_or(class_name);
                        let default_str = format!("<{} object>", simple_name);
                        TirExprUnresolved::new(
                            TirExprKindUnresolved::Constant(Constant::Str(default_str)),
                            str_type.clone(),
                        )
                    };

                    TirStmtUnresolved::Expr(TirExprUnresolved::new(
                        TirExprKindUnresolved::Call {
                            func: write_string_func,
                            args: vec![str_expr],
                        },
                        TirTypeUnresolved::Void,
                    ))
                }
                TirTypeUnresolved::TypeVar(_) => {
                    // TypeVars should only appear inside Class type_params, never as top-level types
                    // If this is hit, it's a compiler bug in the type inference system
                    unreachable!("Bare TypeVar in expression type - should be wrapped in Class")
                }
                TirTypeUnresolved::Void => {
                    // Shouldn't happen, but handle gracefully
                    continue;
                }
            };

            stmts.push(print_stmt);
        }

        // Add final newline
        stmts.push(TirStmtUnresolved::Expr(TirExprUnresolved::new(
            TirExprKindUnresolved::Call {
                func: write_newline_func,
                args: vec![],
            },
            TirTypeUnresolved::Void,
        )));

        Ok(stmts)
    }

    /// Resolve an exception class name to a ClassId
    fn resolve_exception_class(&mut self, name: &str) -> Result<crate::tir::ids::ClassId> {
        // Check for base Exception class first
        if name == "Exception" {
            return Ok(self.symbols.get_or_create_exception_class());
        }

        // Check if there's a user-defined class with this name in scope
        if let Some(&class_id) = self.scope.classes.get(name) {
            // Verify it's actually an exception subclass
            if self.symbols.is_exception_subclass(class_id) {
                return Ok(class_id);
            }
        }

        // Try to find the class by name in the global symbol table
        if let Some(class_id) = self.symbols.find_class_by_name(name) {
            if self.symbols.is_exception_subclass(class_id) {
                return Ok(class_id);
            }
        }

        // Fall back to base Exception class if not found
        // This allows catching any exception type
        Ok(self.symbols.get_or_create_exception_class())
    }
}
