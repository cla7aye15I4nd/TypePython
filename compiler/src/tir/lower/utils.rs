/// Macro to call a dunder method on a class type.
///
/// This macro matches on a TirTypeUnresolved, looks up a method (like `__len__`, `__getitem__`, `__setitem__`)
/// on a class, checks argument types, and generates a Call expression if found.
///
/// # Usage
///
/// With inferred return type from function signature:
/// ```ignore
/// call_dunder_method!(self.symbols, &expr.ty, "__getitem__", vec![expr, index])
/// ```
///
/// With explicit return type:
/// ```ignore
/// call_dunder_method!(self.symbols, &expr.ty, "__len__", vec![expr], TirTypeUnresolved::Int)
/// ```
macro_rules! call_dunder_method {
    // Variant that uses the function signature's return type
    ($symbols:expr, $ty:expr, $method_name:expr, $args:expr) => {{
        use crate::tir::expr_unresolved::{TirExprKindUnresolved, TirExprUnresolved};
        use crate::tir::types_unresolved::TirTypeUnresolved;

        match $ty {
            TirTypeUnresolved::Class(class_id) => {
                // Use resolve_method to support inheritance
                if let Some((_method_id, func_id)) =
                    $symbols.resolve_method(*class_id, $method_name)
                {
                    let (param_tys, return_ty) = $symbols.get_func_signature(func_id);
                    let args_vec: Vec<TirExprUnresolved> = $args;

                    // Check argument count (args includes self, params does not)
                    if args_vec.len() - 1 != param_tys.len() {
                        Err(CompilerError::TypeErrorSimple(format!(
                            "{} expects {} arguments, got {}",
                            $method_name,
                            param_tys.len(),
                            args_vec.len() - 1
                        )))
                    } else {
                        // Check argument types (skip self which is args[0])
                        let mut type_error: Option<String> = None;
                        for (i, (arg, param_ty)) in
                            args_vec.iter().skip(1).zip(param_tys.iter()).enumerate()
                        {
                            let param_ty_unresolved = TirTypeUnresolved::from_tir_type(param_ty);
                            if !arg.ty.is_compatible_with(&param_ty_unresolved) {
                                type_error = Some(format!(
                                    "argument {} to {}: expected {:?}, got {:?}",
                                    i + 1,
                                    $method_name,
                                    param_ty,
                                    arg.ty
                                ));
                                break;
                            }
                        }

                        if let Some(err) = type_error {
                            Err(CompilerError::TypeErrorSimple(err))
                        } else {
                            Ok(TirExprUnresolved::new(
                                TirExprKindUnresolved::Call {
                                    func: func_id,
                                    args: args_vec,
                                },
                                TirTypeUnresolved::from_tir_type(return_ty),
                            ))
                        }
                    }
                } else {
                    Err(CompilerError::TypeErrorSimple(format!(
                        "object of type {:?} has no {}",
                        $ty, $method_name
                    )))
                }
            }
            _ => Err(CompilerError::TypeErrorSimple(format!(
                "type {:?} does not support {}",
                $ty, $method_name
            ))),
        }
    }};

    // Variant with explicit return type (for cases like __len__ which always returns int)
    ($symbols:expr, $ty:expr, $method_name:expr, $args:expr, $return_ty:expr) => {{
        use crate::tir::expr_unresolved::{TirExprKindUnresolved, TirExprUnresolved};
        use crate::tir::types_unresolved::TirTypeUnresolved;

        match $ty {
            TirTypeUnresolved::Class(class_id) => {
                // Use resolve_method to support inheritance
                if let Some((_method_id, func_id)) =
                    $symbols.resolve_method(*class_id, $method_name)
                {
                    let (param_tys, _) = $symbols.get_func_signature(func_id);
                    let args_vec: Vec<TirExprUnresolved> = $args;

                    // Check argument count (args includes self, params does not)
                    if args_vec.len() - 1 != param_tys.len() {
                        Err(CompilerError::TypeErrorSimple(format!(
                            "{} expects {} arguments, got {}",
                            $method_name,
                            param_tys.len(),
                            args_vec.len() - 1
                        )))
                    } else {
                        // Check argument types (skip self which is args[0])
                        let mut type_error: Option<String> = None;
                        for (i, (arg, param_ty)) in
                            args_vec.iter().skip(1).zip(param_tys.iter()).enumerate()
                        {
                            let param_ty_unresolved = TirTypeUnresolved::from_tir_type(param_ty);
                            if !arg.ty.is_compatible_with(&param_ty_unresolved) {
                                type_error = Some(format!(
                                    "argument {} to {}: expected {:?}, got {:?}",
                                    i + 1,
                                    $method_name,
                                    param_ty,
                                    arg.ty
                                ));
                                break;
                            }
                        }

                        if let Some(err) = type_error {
                            Err(CompilerError::TypeErrorSimple(err))
                        } else {
                            Ok(TirExprUnresolved::new(
                                TirExprKindUnresolved::Call {
                                    func: func_id,
                                    args: args_vec,
                                },
                                $return_ty,
                            ))
                        }
                    }
                } else {
                    Err(CompilerError::TypeErrorSimple(format!(
                        "object of type {:?} has no {}",
                        $ty, $method_name
                    )))
                }
            }
            _ => Err(CompilerError::TypeErrorSimple(format!(
                "type {:?} does not support {}",
                $ty, $method_name
            ))),
        }
    }};
}
