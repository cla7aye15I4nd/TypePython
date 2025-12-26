use crate::ast::{BoolOp, Constant, Expr, UnaryOp};
use crate::error::{CompilerError, Result};
use crate::tir::expr::VarRef;
use crate::tir::expr_unresolved::{TirExprKindUnresolved, TirExprUnresolved};
use crate::tir::types_unresolved::TirTypeUnresolved;

use super::body_lowerer::BodyLowerer;

impl<'a> BodyLowerer<'a> {
    pub(crate) fn lower_expr(&mut self, expr: &Expr) -> Result<TirExprUnresolved> {
        match expr {
            Expr::Constant(c) => {
                match c {
                    Constant::Bytes(data) => {
                        // Bytes literals use a special TirExprKindUnresolved
                        let class_id = self.symbols.get_or_create_bytes_class();
                        Ok(TirExprUnresolved::new(
                            TirExprKindUnresolved::Bytes { data: data.clone() },
                            TirTypeUnresolved::Class(class_id),
                        ))
                    }
                    _ => {
                        let ty = match c {
                            Constant::Int(_) => TirTypeUnresolved::Int,
                            Constant::Float(_) => TirTypeUnresolved::Float,
                            Constant::Str(_) => {
                                let class_id = self.symbols.get_or_create_str_class();
                                TirTypeUnresolved::Class(class_id)
                            }
                            Constant::Bool(_) => TirTypeUnresolved::Bool,
                            Constant::Bytes(_) => unreachable!(),
                            Constant::None => TirTypeUnresolved::Void,
                        };
                        Ok(TirExprUnresolved::new(
                            TirExprKindUnresolved::Constant(c.clone()),
                            ty,
                        ))
                    }
                }
            }

            Expr::Name(name) => {
                // Try to resolve as variable
                if let Some((var_ref, ty)) = self.resolve_var(name) {
                    return Ok(TirExprUnresolved::new(
                        TirExprKindUnresolved::Var(var_ref),
                        ty,
                    ));
                }

                // Not a variable - might be a function or class reference
                // These are handled in Call expressions
                Err(CompilerError::UndefinedVariable(name.clone()))
            }

            Expr::BinOp { left, op, right } => {
                use crate::ast::BinOperator;

                let left_expr = self.lower_expr(left)?;
                let right_expr = self.lower_expr(right)?;

                // Check that both operands are numeric
                if !left_expr.ty.is_numeric() {
                    return Err(CompilerError::TypeErrorSimple(format!(
                        "Left operand of binary operator must be numeric, got {:?}",
                        left_expr.ty
                    )));
                }
                if !right_expr.ty.is_numeric() {
                    return Err(CompilerError::TypeErrorSimple(format!(
                        "Right operand of binary operator must be numeric, got {:?}",
                        right_expr.ty
                    )));
                }

                // Determine result type based on operand types and operator
                // - Division (/) always returns Float
                // - If either operand is Float, result is Float
                // - Otherwise, result is Int
                let result_ty = match op {
                    BinOperator::Div => TirTypeUnresolved::Float,
                    _ => {
                        if left_expr.ty == TirTypeUnresolved::Float
                            || right_expr.ty == TirTypeUnresolved::Float
                        {
                            TirTypeUnresolved::Float
                        } else {
                            TirTypeUnresolved::Int
                        }
                    }
                };

                Ok(TirExprUnresolved::new(
                    TirExprKindUnresolved::BinOp {
                        left: Box::new(left_expr),
                        op: *op,
                        right: Box::new(right_expr),
                    },
                    result_ty,
                ))
            }

            Expr::Compare {
                left,
                ops,
                comparators,
            } => {
                let left_expr = self.lower_expr(left)?;

                if ops.len() == 1 && comparators.len() == 1 {
                    // Single comparison
                    let right_expr = self.lower_expr(&comparators[0])?;

                    // Check that operands are compatible for comparison
                    if !left_expr.ty.is_compatible_with(&right_expr.ty) {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "Cannot compare {:?} with {:?}",
                            left_expr.ty, right_expr.ty
                        )));
                    }

                    Ok(TirExprUnresolved::new(
                        TirExprKindUnresolved::Compare {
                            left: Box::new(left_expr),
                            op: ops[0],
                            right: Box::new(right_expr),
                        },
                        TirTypeUnresolved::Bool,
                    ))
                } else {
                    // Chained comparison - desugar to AND of comparisons
                    // a < b < c  =>  (a < b) and (b < c)

                    // Build all comparisons: left op[0] comp[0], comp[0] op[1] comp[1], ...
                    let mut comparisons: Vec<TirExprUnresolved> = Vec::new();
                    let mut current_left = left_expr;

                    for (op, comp) in ops.iter().zip(comparators.iter()) {
                        let right_expr = self.lower_expr(comp)?;

                        // Check that operands are compatible
                        if !current_left.ty.is_compatible_with(&right_expr.ty) {
                            return Err(CompilerError::TypeErrorSimple(format!(
                                "Cannot compare {:?} with {:?}",
                                current_left.ty, right_expr.ty
                            )));
                        }

                        let cmp = TirExprUnresolved::new(
                            TirExprKindUnresolved::Compare {
                                left: Box::new(current_left),
                                op: *op,
                                right: Box::new(right_expr.clone()),
                            },
                            TirTypeUnresolved::Bool,
                        );

                        comparisons.push(cmp);

                        // For next iteration, use the right side as the new left
                        current_left = right_expr;
                    }

                    // Combine all comparisons with BoolOp::And
                    Ok(TirExprUnresolved::new(
                        TirExprKindUnresolved::BoolOp {
                            op: BoolOp::And,
                            values: comparisons,
                        },
                        TirTypeUnresolved::Bool,
                    ))
                }
            }

            Expr::Call { func, args } => self.lower_call(func, args),

            Expr::List { elts } => {
                if elts.is_empty() {
                    // Empty list: create list[TypeVar] with a fresh type variable
                    // The element type will be inferred from usage (e.g., append calls)
                    let elem_type_var = self.constraints.fresh_type_var();

                    // Get or create list class with this type parameter
                    // Convert TirTypeUnresolved to TirType for the symbols API
                    let elem_type_var_tir = elem_type_var.to_tir_type();
                    let list_class_id = self.symbols.get_or_create_list_class(&elem_type_var_tir);

                    // Note: get_or_create_list_class already sets type_params via ClassKey

                    // Create empty list literal
                    Ok(TirExprUnresolved::new(
                        TirExprKindUnresolved::List {
                            elements: vec![],
                            elem_ty: elem_type_var,
                        },
                        TirTypeUnresolved::Class(list_class_id),
                    ))
                } else {
                    // Non-empty list: infer element type from first element
                    let mut elements = Vec::new();
                    let mut elem_ty = TirTypeUnresolved::Int; // Will be overwritten by first element

                    for (i, elt) in elts.iter().enumerate() {
                        let elt_expr = self.lower_expr(elt)?;
                        if i == 0 {
                            elem_ty = elt_expr.ty.clone();
                        } else if !elt_expr.ty.is_compatible_with(&elem_ty) {
                            return Err(CompilerError::TypeErrorSimple(format!(
                                "List element type mismatch at index {}: expected {:?}, got {:?}",
                                i, elem_ty, elt_expr.ty
                            )));
                        }
                        elements.push(elt_expr);
                    }

                    let list_class_id = self
                        .symbols
                        .get_or_create_list_class(&elem_ty.to_tir_type());
                    Ok(TirExprUnresolved::new(
                        TirExprKindUnresolved::List {
                            elements,
                            elem_ty: elem_ty.clone(),
                        },
                        TirTypeUnresolved::Class(list_class_id),
                    ))
                }
            }

            Expr::Subscript { value, index } => {
                let container_expr = self.lower_expr(value)?;
                let index_expr = self.lower_expr(index)?;

                // Look up __getitem__ method and convert to a Call
                call_dunder_method!(
                    self.symbols,
                    &container_expr.ty,
                    "__getitem__",
                    vec![container_expr, index_expr]
                )
            }

            Expr::Attribute { value, attr } => self.lower_attribute(value, attr),

            Expr::BoolOp { op, values } => {
                let mut lowered_values = Vec::new();
                for val in values {
                    let lowered = self.lower_expr(val)?;
                    // Accept both Bool and Int (for truthiness)
                    if !lowered.ty.is_boolean() && !lowered.ty.is_numeric() {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "Boolean operator requires boolean or numeric operand, got {:?}",
                            lowered.ty
                        )));
                    }
                    lowered_values.push(lowered);
                }

                Ok(TirExprUnresolved::new(
                    TirExprKindUnresolved::BoolOp {
                        op: *op,
                        values: lowered_values,
                    },
                    TirTypeUnresolved::Bool,
                ))
            }

            Expr::UnaryOp { op, operand } => {
                let operand_expr = self.lower_expr(operand)?;

                match op {
                    UnaryOp::Not => {
                        // not requires boolean or numeric (truthy)
                        if !operand_expr.ty.is_boolean() && !operand_expr.ty.is_numeric() {
                            return Err(CompilerError::TypeErrorSimple(format!(
                                "'not' requires boolean or numeric operand, got {:?}",
                                operand_expr.ty
                            )));
                        }
                        Ok(TirExprUnresolved::new(
                            TirExprKindUnresolved::UnaryOp {
                                op: *op,
                                operand: Box::new(operand_expr),
                            },
                            TirTypeUnresolved::Bool,
                        ))
                    }
                    UnaryOp::USub => {
                        if !operand_expr.ty.is_numeric() {
                            return Err(CompilerError::TypeErrorSimple(format!(
                                "Unary minus requires numeric operand, got {:?}",
                                operand_expr.ty
                            )));
                        }
                        // Preserve the type of the operand
                        let result_ty = operand_expr.ty.clone();
                        Ok(TirExprUnresolved::new(
                            TirExprKindUnresolved::UnaryOp {
                                op: *op,
                                operand: Box::new(operand_expr),
                            },
                            result_ty,
                        ))
                    }
                }
            }
        }
    }

    fn lower_call(&mut self, func: &Expr, args: &[Expr]) -> Result<TirExprUnresolved> {
        // Handle super().__method__(...) calls
        if let Expr::Attribute { value, attr } = func {
            if let Expr::Call {
                func: super_func,
                args: super_args,
            } = value.as_ref()
            {
                if let Expr::Name(name) = super_func.as_ref() {
                    if name == "super" && super_args.is_empty() {
                        // This is super().method(args)
                        return self.lower_super_method_call(attr, args);
                    }
                }
            }
        }

        // Lower arguments first
        let mut lowered_args = Vec::new();
        for arg in args {
            lowered_args.push(self.lower_expr(arg)?);
        }

        // Handle builtins
        if let Expr::Name(name) = func {
            // Note: print() is handled at statement level in stmt_lowering.rs
            // If print is used as an expression (e.g., x = print("hi")), it will error below

            if name == "len" {
                if lowered_args.len() != 1 {
                    return Err(CompilerError::TypeErrorSimple(
                        "len() takes exactly one argument".to_string(),
                    ));
                }
                // Check if argument is a class with __len__ method
                let receiver = lowered_args.into_iter().next().unwrap();
                return call_dunder_method!(
                    self.symbols,
                    &receiver.ty,
                    "__len__",
                    vec![receiver],
                    TirTypeUnresolved::Int
                );
            }

            // range() builtin - creates a range iterator
            if name == "range" {
                let range_class_id = self.symbols.get_or_create_range_class();
                // range(stop), range(start, stop), or range(start, stop, step)
                if lowered_args.is_empty() || lowered_args.len() > 3 {
                    return Err(CompilerError::TypeErrorSimple(
                        "range() takes 1 to 3 arguments".to_string(),
                    ));
                }
                // All args must be int
                for (i, arg) in lowered_args.iter().enumerate() {
                    if arg.ty != TirTypeUnresolved::Int {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "range() argument {} must be int, got {:?}",
                            i + 1,
                            arg.ty
                        )));
                    }
                }
                return Ok(TirExprUnresolved::new(
                    TirExprKindUnresolved::Construct {
                        class: range_class_id,
                        args: lowered_args,
                    },
                    TirTypeUnresolved::Class(range_class_id),
                ));
            }

            // iter() builtin - calls __iter__ on argument
            if name == "iter" {
                if lowered_args.len() != 1 {
                    return Err(CompilerError::TypeErrorSimple(
                        "iter() takes exactly one argument".to_string(),
                    ));
                }
                let receiver = lowered_args.into_iter().next().unwrap();
                return call_dunder_method!(self.symbols, &receiver.ty, "__iter__", vec![receiver]);
            }

            // next() builtin - calls __next__ on argument
            if name == "next" {
                if lowered_args.len() != 1 {
                    return Err(CompilerError::TypeErrorSimple(
                        "next() takes exactly one argument".to_string(),
                    ));
                }
                let receiver = lowered_args.into_iter().next().unwrap();
                return call_dunder_method!(self.symbols, &receiver.ty, "__next__", vec![receiver]);
            }

            // Check if it's an Exception constructor
            if name == "Exception" {
                let class_id = self.symbols.get_or_create_exception_class();
                // Exception() can take 0 or 1 argument (message)
                if lowered_args.len() > 1 {
                    return Err(CompilerError::TypeErrorSimple(
                        "Exception() takes at most 1 argument (message)".to_string(),
                    ));
                }
                // If there's an argument, it must be a string
                if lowered_args.len() == 1 {
                    let str_class_id = self.symbols.get_or_create_str_class();
                    if lowered_args[0].ty != TirTypeUnresolved::Class(str_class_id) {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "Exception() argument must be a string, got {:?}",
                            lowered_args[0].ty
                        )));
                    }
                }
                return Ok(TirExprUnresolved::new(
                    TirExprKindUnresolved::Construct {
                        class: class_id,
                        args: lowered_args,
                    },
                    TirTypeUnresolved::Class(class_id),
                ));
            }

            // Check if it's a bytearray constructor
            if name == "bytearray" {
                let class_id = self.symbols.get_or_create_bytearray_class();
                // bytearray() can take 0 or 1 argument
                if lowered_args.len() > 1 {
                    return Err(CompilerError::TypeErrorSimple(
                        "bytearray() takes at most 1 argument".to_string(),
                    ));
                }
                // If there's an argument, it must be bytes
                if lowered_args.len() == 1 {
                    match &lowered_args[0].ty {
                        TirTypeUnresolved::Class(class_id)
                            if self.symbols.is_bytes_class(*class_id) => {}
                        _ => {
                            return Err(CompilerError::TypeErrorSimple(format!(
                                "bytearray() argument must be bytes, got {:?}",
                                lowered_args[0].ty
                            )));
                        }
                    }
                }
                return Ok(TirExprUnresolved::new(
                    TirExprKindUnresolved::Construct {
                        class: class_id,
                        args: lowered_args,
                    },
                    TirTypeUnresolved::Class(class_id),
                ));
            }

            // Check if it's a class constructor
            if let Some(&class_id) = self.scope.classes.get(name) {
                // Check if class has an __init__ method
                if let Some(&(_, init_func_id)) = self
                    .symbols
                    .methods
                    .get(&(class_id, "__init__".to_string()))
                {
                    // Get __init__ signature and type check arguments
                    let (param_tys, _) = self.symbols.get_func_signature(init_func_id);
                    if lowered_args.len() != param_tys.len() {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "Constructor for '{}' expects {} arguments, got {}",
                            name,
                            param_tys.len(),
                            lowered_args.len()
                        )));
                    }
                    for (i, (arg, param_ty)) in
                        lowered_args.iter().zip(param_tys.iter()).enumerate()
                    {
                        let param_ty_unresolved = TirTypeUnresolved::from_tir_type(param_ty);
                        if !arg.ty.is_compatible_with(&param_ty_unresolved) {
                            return Err(CompilerError::TypeErrorSimple(format!(
                                "Argument {} to constructor '{}': expected {:?}, got {:?}",
                                i + 1,
                                name,
                                param_ty,
                                arg.ty
                            )));
                        }
                    }
                } else {
                    // No __init__ defined on this class - check if it's an Exception subclass
                    if self.symbols.is_exception_subclass(class_id) {
                        // Exception subclasses can take an optional string message argument
                        if lowered_args.len() > 1 {
                            return Err(CompilerError::TypeErrorSimple(format!(
                                "Exception '{}' takes at most 1 argument (message)",
                                name
                            )));
                        }
                        if lowered_args.len() == 1 {
                            let str_class_id = self.symbols.get_or_create_str_class();
                            if lowered_args[0].ty != TirTypeUnresolved::Class(str_class_id) {
                                return Err(CompilerError::TypeErrorSimple(format!(
                                    "Exception '{}' message argument must be a string, got {:?}",
                                    name, lowered_args[0].ty
                                )));
                            }
                        }
                    } else if !lowered_args.is_empty() {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "Class '{}' has no __init__ method and cannot take arguments",
                            name
                        )));
                    }
                }

                return Ok(TirExprUnresolved::new(
                    TirExprKindUnresolved::Construct {
                        class: class_id,
                        args: lowered_args,
                    },
                    TirTypeUnresolved::Class(class_id),
                ));
            }

            // Check if it's a function
            if let Some(&func_id) = self.scope.functions.get(name) {
                let (param_tys, ret_ty) = self.symbols.get_func_signature(func_id);

                // Type check arguments against parameters
                if lowered_args.len() != param_tys.len() {
                    return Err(CompilerError::TypeErrorSimple(format!(
                        "Function '{}' expects {} arguments, got {}",
                        name,
                        param_tys.len(),
                        lowered_args.len()
                    )));
                }
                for (i, (arg, param_ty)) in lowered_args.iter().zip(param_tys.iter()).enumerate() {
                    let param_ty_unresolved = TirTypeUnresolved::from_tir_type(param_ty);
                    if !arg.ty.is_compatible_with(&param_ty_unresolved) {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "Argument {} to function '{}': expected {:?}, got {:?}",
                            i + 1,
                            name,
                            param_ty,
                            arg.ty
                        )));
                    }
                }

                return Ok(TirExprUnresolved::new(
                    TirExprKindUnresolved::Call {
                        func: func_id,
                        args: lowered_args,
                    },
                    TirTypeUnresolved::from_tir_type(ret_ty),
                ));
            }
        }

        // Handle method calls
        if let Expr::Attribute { value, attr } = func {
            // Check if it's a module.function call
            if let Expr::Name(mod_name) = value.as_ref() {
                if let Some(&mod_id) = self.scope.module_aliases.get(mod_name) {
                    // It's a module reference
                    if let Some(&func_id) = self.symbols.functions.get(&(mod_id, attr.clone())) {
                        let (param_tys, ret_ty) = self.symbols.get_func_signature(func_id);

                        // Type check arguments
                        if lowered_args.len() != param_tys.len() {
                            return Err(CompilerError::TypeErrorSimple(format!(
                                "Function '{}.{}' expects {} arguments, got {}",
                                mod_name,
                                attr,
                                param_tys.len(),
                                lowered_args.len()
                            )));
                        }
                        for (i, (arg, param_ty)) in
                            lowered_args.iter().zip(param_tys.iter()).enumerate()
                        {
                            let param_ty_unresolved = TirTypeUnresolved::from_tir_type(param_ty);
                            if !arg.ty.is_compatible_with(&param_ty_unresolved) {
                                return Err(CompilerError::TypeErrorSimple(format!(
                                    "Argument {} to '{}.{}': expected {:?}, got {:?}",
                                    i + 1,
                                    mod_name,
                                    attr,
                                    param_ty,
                                    arg.ty
                                )));
                            }
                        }

                        return Ok(TirExprUnresolved::new(
                            TirExprKindUnresolved::Call {
                                func: func_id,
                                args: lowered_args,
                            },
                            TirTypeUnresolved::from_tir_type(ret_ty),
                        ));
                    }
                }
            }

            // It's an actual method call
            let receiver = self.lower_expr(value)?;

            // Get the class ID from receiver type
            let receiver_class_id = receiver.ty.class_id();
            if let Some(class_id) = receiver_class_id {
                // Look up the method (including inherited methods)
                if let Some((_method_id, func_id)) = self.symbols.resolve_method(class_id, attr) {
                    let (param_tys, ret_ty) = self.symbols.get_func_signature(func_id);

                    // Type check arguments against parameters
                    if lowered_args.len() != param_tys.len() {
                        return Err(CompilerError::TypeErrorSimple(format!(
                            "Method '{}' expects {} arguments, got {}",
                            attr,
                            param_tys.len(),
                            lowered_args.len()
                        )));
                    }
                    for (i, (arg, param_ty)) in
                        lowered_args.iter().zip(param_tys.iter()).enumerate()
                    {
                        let param_ty_unresolved = TirTypeUnresolved::from_tir_type(param_ty);
                        if !arg.ty.is_compatible_with(&param_ty_unresolved) {
                            return Err(CompilerError::TypeErrorSimple(format!(
                                "Argument {} to method '{}': expected {:?}, got {:?}",
                                i + 1,
                                attr,
                                param_ty,
                                arg.ty
                            )));
                        }
                    }

                    // Create Call with receiver as first argument
                    let mut call_args = vec![receiver.clone()];
                    call_args.extend(lowered_args.clone());

                    // Generate constraints for type inference
                    // For list.append(x), constrain the list's element type to match x's type
                    // Only for generic containers (list[T], set[T], etc.) - not bytearray which has no type params
                    if attr == "append" && call_args.len() == 2 {
                        use crate::tir::lower::constraints::{Constraint, ConstraintOrigin};

                        // Only add ElementType constraint if the class has type parameters
                        // This distinguishes generic containers (list[T]) from non-generic ones (bytearray)
                        if let Some(recv_class_id) = receiver.ty.class_id() {
                            let type_params = self.symbols.get_type_params(recv_class_id);
                            if !type_params.is_empty() {
                                self.constraints.add_constraint(Constraint::ElementType {
                                    container: receiver.ty.clone(),
                                    element: call_args[1].ty.clone(),
                                    origin: ConstraintOrigin::MethodCall {
                                        method_name: "append".to_string(),
                                        line: 0, // TODO: track line numbers from AST
                                    },
                                });
                            }
                        }
                    }

                    return Ok(TirExprUnresolved::new(
                        TirExprKindUnresolved::Call {
                            func: func_id,
                            args: call_args,
                        },
                        TirTypeUnresolved::from_tir_type(ret_ty),
                    ));
                }
            }
        }

        Err(CompilerError::TypeErrorSimple(format!(
            "Cannot lower call to {:?}",
            func
        )))
    }

    fn lower_attribute(&mut self, value: &Expr, attr: &str) -> Result<TirExprUnresolved> {
        // Check if value is a module alias (for module.global access)
        if let Expr::Name(mod_name) = value {
            if let Some(&mod_id) = self.scope.module_aliases.get(mod_name) {
                // It's a module reference - look up the global variable
                if let Some(&global_id) = self.symbols.globals.get(&(mod_id, attr.to_string())) {
                    return Ok(TirExprUnresolved::new(
                        TirExprKindUnresolved::Var(VarRef::Global(mod_id, global_id)),
                        TirTypeUnresolved::Int, // TODO: track global types properly
                    ));
                }
                return Err(CompilerError::TypeErrorSimple(format!(
                    "Module '{}' has no global '{}'",
                    mod_name, attr
                )));
            }
        }

        let receiver = self.lower_expr(value)?;

        // Get field from class
        if let Some(class_id) = receiver.ty.class_id() {
            if let Some(&field_id) = self.symbols.fields.get(&(class_id, attr.to_string())) {
                // Get field type - check inherited_fields first, then own fields
                let class_data = &self.symbols.class_data[class_id.index()];
                let inherited_count = class_data.inherited_fields.len();
                let field_idx = field_id.index();

                let field_ty = if field_idx < inherited_count {
                    // It's an inherited field
                    TirTypeUnresolved::from_tir_type(&class_data.inherited_fields[field_idx].1)
                } else {
                    // It's an own field
                    TirTypeUnresolved::from_tir_type(
                        &class_data.fields[field_idx - inherited_count].1,
                    )
                };

                return Ok(TirExprUnresolved::new(
                    TirExprKindUnresolved::FieldAccess {
                        object: Box::new(receiver),
                        class: class_id,
                        field: field_id,
                    },
                    field_ty,
                ));
            }
        }

        Err(CompilerError::TypeErrorSimple(format!(
            "Cannot access attribute '{}' on {:?}",
            attr, receiver.ty
        )))
    }

    /// Handle super().method(args) calls
    fn lower_super_method_call(
        &mut self,
        method_name: &str,
        args: &[Expr],
    ) -> Result<TirExprUnresolved> {
        // super() can only be used inside a method
        let class_id = self.current_class.ok_or_else(|| {
            CompilerError::TypeErrorSimple("super() can only be used inside a class method".into())
        })?;

        // Get the parent class
        let parent_id = self.symbols.class_data[class_id.index()]
            .parent
            .ok_or_else(|| {
                CompilerError::TypeErrorSimple("super() called but class has no parent".into())
            })?;

        // Look up the method in the parent class
        let (_method_id, func_id) = self
            .symbols
            .resolve_method(parent_id, method_name)
            .ok_or_else(|| {
                CompilerError::TypeErrorSimple(format!(
                    "Parent class has no method '{}'",
                    method_name
                ))
            })?;

        // Lower arguments
        let mut lowered_args = Vec::new();
        for arg in args {
            lowered_args.push(self.lower_expr(arg)?);
        }

        // Get method signature and type check
        let (param_tys, ret_ty) = self.symbols.get_func_signature(func_id);

        if lowered_args.len() != param_tys.len() {
            return Err(CompilerError::TypeErrorSimple(format!(
                "super().{}() expects {} arguments, got {}",
                method_name,
                param_tys.len(),
                lowered_args.len()
            )));
        }

        for (i, (arg, param_ty)) in lowered_args.iter().zip(param_tys.iter()).enumerate() {
            let param_ty_unresolved = TirTypeUnresolved::from_tir_type(param_ty);
            if !arg.ty.is_compatible_with(&param_ty_unresolved) {
                return Err(CompilerError::TypeErrorSimple(format!(
                    "Argument {} to super().{}(): expected {:?}, got {:?}",
                    i + 1,
                    method_name,
                    param_ty,
                    arg.ty
                )));
            }
        }

        // Create Call with self as first argument (self is passed through to parent)
        let self_expr = TirExprUnresolved::new(
            TirExprKindUnresolved::Var(VarRef::SelfRef),
            TirTypeUnresolved::Class(class_id),
        );
        let mut call_args = vec![self_expr];
        call_args.extend(lowered_args);

        Ok(TirExprUnresolved::new(
            TirExprKindUnresolved::Call {
                func: func_id,
                args: call_args,
            },
            TirTypeUnresolved::from_tir_type(ret_ty),
        ))
    }
}
