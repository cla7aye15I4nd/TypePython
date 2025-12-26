//! Conversion from TirUnresolved* to Tir* after constraint solving
//!
//! This module converts the unresolved TIR (with TypeVars) to the
//! resolved TIR (without TypeVars) by applying type substitutions.
//!
//! The critical function is `resolve_type()`, which is the single validation
//! point that ensures all TypeVars are resolved before conversion. If any
//! TypeVar remains unresolved, it returns an error with a clear message.
//!
//! This architectural design provides compile-time guarantees: after this
//! conversion, the resulting TirType enum has no TypeVar variant, making it
//! impossible for unresolved types to reach codegen.

use crate::ast::Constant;
use crate::error::{CompilerError, Result};
use crate::tir::expr::{TirConstant, TirExpr, TirExprKind};
use crate::tir::expr_unresolved::{TirExprKindUnresolved, TirExprUnresolved};
use crate::tir::lower::GlobalSymbolsInternal as GlobalSymbols;
use crate::tir::stmt::{TirExceptHandler, TirLValue, TirStmt};
use crate::tir::stmt_unresolved::{
    TirExceptHandlerUnresolved, TirLValueUnresolved, TirStmtUnresolved,
};
use crate::tir::types::TirType;
use crate::tir::types_unresolved::TirTypeUnresolved;
use std::collections::HashMap;

/// Resolve a function body (vector of statements)
pub(crate) fn resolve_body(
    body: Vec<TirStmtUnresolved>,
    substitutions: &HashMap<u32, TirTypeUnresolved>,
    symbols: &mut GlobalSymbols,
) -> Result<Vec<TirStmt>> {
    body.into_iter()
        .map(|stmt| resolve_stmt(stmt, substitutions, symbols))
        .collect()
}

/// Resolve a single statement
fn resolve_stmt(
    stmt: TirStmtUnresolved,
    substitutions: &HashMap<u32, TirTypeUnresolved>,
    symbols: &mut GlobalSymbols,
) -> Result<TirStmt> {
    match stmt {
        TirStmtUnresolved::Let { local, ty, init } => Ok(TirStmt::Let {
            local,
            ty: resolve_type(&ty, substitutions, symbols)?,
            init: resolve_expr(init, substitutions, symbols)?,
        }),
        TirStmtUnresolved::Assign { target, value } => Ok(TirStmt::Assign {
            target: resolve_lvalue(target, substitutions, symbols)?,
            value: resolve_expr(value, substitutions, symbols)?,
        }),
        TirStmtUnresolved::AugAssign { target, op, value } => Ok(TirStmt::AugAssign {
            target,
            op,
            value: resolve_expr(value, substitutions, symbols)?,
        }),
        TirStmtUnresolved::Expr(expr) => {
            Ok(TirStmt::Expr(resolve_expr(expr, substitutions, symbols)?))
        }
        TirStmtUnresolved::Return(expr) => Ok(TirStmt::Return(
            expr.map(|e| resolve_expr(e, substitutions, symbols))
                .transpose()?,
        )),
        TirStmtUnresolved::If {
            cond,
            then_body,
            else_body,
        } => Ok(TirStmt::If {
            cond: resolve_expr(cond, substitutions, symbols)?,
            then_body: resolve_body(then_body, substitutions, symbols)?,
            else_body: resolve_body(else_body, substitutions, symbols)?,
        }),
        TirStmtUnresolved::While { cond, body } => Ok(TirStmt::While {
            cond: resolve_expr(cond, substitutions, symbols)?,
            body: resolve_body(body, substitutions, symbols)?,
        }),
        TirStmtUnresolved::Try {
            body,
            handlers,
            orelse,
            finalbody,
        } => {
            let resolved_handlers = handlers
                .into_iter()
                .map(|handler| resolve_except_handler(handler, substitutions, symbols))
                .collect::<Result<Vec<_>>>()?;

            Ok(TirStmt::Try {
                body: resolve_body(body, substitutions, symbols)?,
                handlers: resolved_handlers,
                orelse: resolve_body(orelse, substitutions, symbols)?,
                finalbody: resolve_body(finalbody, substitutions, symbols)?,
            })
        }
        TirStmtUnresolved::Raise { exc } => Ok(TirStmt::Raise {
            exc: exc
                .map(|e| resolve_expr(e, substitutions, symbols))
                .transpose()?,
        }),
    }
}

/// Resolve an except handler
fn resolve_except_handler(
    handler: TirExceptHandlerUnresolved,
    substitutions: &HashMap<u32, TirTypeUnresolved>,
    symbols: &mut GlobalSymbols,
) -> Result<TirExceptHandler> {
    Ok(TirExceptHandler {
        exc_class: handler.exc_class,
        local: handler.local,
        body: resolve_body(handler.body, substitutions, symbols)?,
    })
}

/// Resolve an lvalue (assignment target)
fn resolve_lvalue(
    lvalue: TirLValueUnresolved,
    substitutions: &HashMap<u32, TirTypeUnresolved>,
    symbols: &mut GlobalSymbols,
) -> Result<TirLValue> {
    match lvalue {
        TirLValueUnresolved::Var(var_ref) => Ok(TirLValue::Var(var_ref)),
        TirLValueUnresolved::Field {
            object,
            class,
            field,
        } => Ok(TirLValue::Field {
            object: Box::new(resolve_expr(*object, substitutions, symbols)?),
            class,
            field,
        }),
    }
}

/// Resolve an expression
fn resolve_expr(
    expr: TirExprUnresolved,
    substitutions: &HashMap<u32, TirTypeUnresolved>,
    symbols: &mut GlobalSymbols,
) -> Result<TirExpr> {
    let resolved_ty = resolve_type(&expr.ty, substitutions, symbols)?;

    let resolved_kind = match expr.kind {
        TirExprKindUnresolved::Constant(c) => {
            // Convert AST Constant to TIR-specific TirConstant
            let tir_const = match c {
                Constant::Int(n) => TirConstant::Int(n),
                Constant::Float(f) => TirConstant::Float(f),
                Constant::Str(s) => TirConstant::Str(s),
                Constant::Bool(b) => TirConstant::Bool(b),
                Constant::None => TirConstant::None,
                // Bytes should have been converted to TirExprKindUnresolved::Bytes during lowering
                Constant::Bytes(_) => {
                    return Err(CompilerError::CodegenError(
                        "Bytes constant should use TirExprKindUnresolved::Bytes".to_string(),
                    ))
                }
            };
            TirExprKind::Constant(tir_const)
        }
        TirExprKindUnresolved::Var(v) => TirExprKind::Var(v),
        TirExprKindUnresolved::BinOp { left, op, right } => TirExprKind::BinOp {
            left: Box::new(resolve_expr(*left, substitutions, symbols)?),
            op,
            right: Box::new(resolve_expr(*right, substitutions, symbols)?),
        },
        TirExprKindUnresolved::Compare { left, op, right } => TirExprKind::Compare {
            left: Box::new(resolve_expr(*left, substitutions, symbols)?),
            op,
            right: Box::new(resolve_expr(*right, substitutions, symbols)?),
        },
        TirExprKindUnresolved::BoolOp { op, values } => {
            let resolved_values = values
                .into_iter()
                .map(|v| resolve_expr(v, substitutions, symbols))
                .collect::<Result<Vec<_>>>()?;
            TirExprKind::BoolOp {
                op,
                values: resolved_values,
            }
        }
        TirExprKindUnresolved::UnaryOp { op, operand } => TirExprKind::UnaryOp {
            op,
            operand: Box::new(resolve_expr(*operand, substitutions, symbols)?),
        },
        TirExprKindUnresolved::Call { func, args } => {
            let resolved_args = args
                .into_iter()
                .map(|arg| resolve_expr(arg, substitutions, symbols))
                .collect::<Result<Vec<_>>>()?;
            TirExprKind::Call {
                func,
                args: resolved_args,
            }
        }
        TirExprKindUnresolved::Construct { class, args } => {
            // Check if this is a range() construction
            let class_data = &symbols.class_data[class.index()];
            if class_data.qualified_name == "__builtin__.range" {
                // Convert to TirExprKind::Range with explicit start/stop/step
                let resolved_args: Vec<TirExpr> = args
                    .into_iter()
                    .map(|arg| resolve_expr(arg, substitutions, symbols))
                    .collect::<Result<Vec<_>>>()?;

                match resolved_args.len() {
                    1 => TirExprKind::Range {
                        start: None,
                        stop: Box::new(resolved_args.into_iter().next().unwrap()),
                        step: None,
                    },
                    2 => {
                        let mut iter = resolved_args.into_iter();
                        TirExprKind::Range {
                            start: Some(Box::new(iter.next().unwrap())),
                            stop: Box::new(iter.next().unwrap()),
                            step: None,
                        }
                    }
                    3 => {
                        let mut iter = resolved_args.into_iter();
                        TirExprKind::Range {
                            start: Some(Box::new(iter.next().unwrap())),
                            stop: Box::new(iter.next().unwrap()),
                            step: Some(Box::new(iter.next().unwrap())),
                        }
                    }
                    n => {
                        return Err(CompilerError::CodegenError(format!(
                            "range() takes 1 to 3 arguments, got {}",
                            n
                        )))
                    }
                }
            } else {
                // Regular class construction
                let resolved_args = args
                    .into_iter()
                    .map(|arg| resolve_expr(arg, substitutions, symbols))
                    .collect::<Result<Vec<_>>>()?;
                TirExprKind::Construct {
                    class,
                    args: resolved_args,
                }
            }
        }
        TirExprKindUnresolved::FieldAccess {
            object,
            class,
            field,
        } => TirExprKind::FieldAccess {
            object: Box::new(resolve_expr(*object, substitutions, symbols)?),
            class,
            field,
        },
        TirExprKindUnresolved::List { elements, elem_ty } => {
            let resolved_elements = elements
                .into_iter()
                .map(|elem| resolve_expr(elem, substitutions, symbols))
                .collect::<Result<Vec<_>>>()?;
            TirExprKind::List {
                elements: resolved_elements,
                elem_ty: resolve_type(&elem_ty, substitutions, symbols)?,
            }
        }
        TirExprKindUnresolved::Bytes { data } => TirExprKind::Bytes { data },
    };

    Ok(TirExpr::new(resolved_kind, resolved_ty))
}

/// Convert TirTypeUnresolved to TirType
///
/// CRITICAL: This is where we enforce that all TypeVars are resolved.
/// This is the single validation point in the entire conversion process.
///
/// If any TypeVar remains after applying substitutions, this function returns
/// an error, preventing unresolved types from reaching the resolved TIR.
pub(crate) fn resolve_type(
    ty: &TirTypeUnresolved,
    substitutions: &HashMap<u32, TirTypeUnresolved>,
    symbols: &mut GlobalSymbols,
) -> Result<TirType> {
    // Apply substitutions first
    let substituted = ty.substitute(substitutions);

    match substituted {
        TirTypeUnresolved::Int => Ok(TirType::Int),
        TirTypeUnresolved::Float => Ok(TirType::Float),
        TirTypeUnresolved::Bool => Ok(TirType::Bool),
        TirTypeUnresolved::Void => Ok(TirType::Void),
        TirTypeUnresolved::Class(class_id) => {
            // For classes, we need to substitute type_params in ClassData
            // and validate that all type parameters are resolved
            symbols.substitute_class_type_params(class_id, substitutions);

            // Validate all type_params are resolved (no TypeVars remain)
            if !symbols.class_type_params_resolved(class_id) {
                return Err(CompilerError::TypeInferenceError(format!(
                    "Class {:?} still has unresolved type parameters after constraint solving. \
                     This typically means there is insufficient type information in the code. \
                     For example, empty containers like [] need to be used with operations \
                     that reveal their element type (like append, indexing, or type annotations).",
                    class_id
                )));
            }

            Ok(TirType::Class(class_id))
        }
        TirTypeUnresolved::TypeVar(id) => {
            // This is the critical error: TypeVar not resolved
            // This means constraint solving couldn't infer the type
            Err(CompilerError::TypeInferenceError(format!(
                "Type variable {} could not be resolved - insufficient type information. \
                 This typically happens with unused variables or empty containers that are \
                 never used in a way that reveals their type. Consider adding type annotations \
                 or using the variable in a typed context.",
                id
            )))
        }
    }
}
