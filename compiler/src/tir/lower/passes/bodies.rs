//! Body Lowering
//!
//! Lowers function and method bodies from AST to TIR.
//! Handles type constraint solving and resolution.

use std::collections::HashMap;

use crate::ast::{Arg, ClassBodyItem, Module, ModuleName, Stmt};
use crate::error::{ErrorCollector, Result};
use crate::tir::decls::{TirClass, TirFunction};
use crate::tir::ids::{ClassId, FuncId, ModuleId};
use crate::tir::stmt::TirStmt;
use crate::tir::stmt_unresolved::TirStmtUnresolved;
use crate::tir::types::TirType;
use crate::tir::types_unresolved::TirTypeUnresolved;

use super::super::body_lowerer::BodyLowerer;
use super::super::constraints;
use super::super::scope::ModuleScope;
use super::super::symbols::{ClassKey, GlobalSymbols};
use super::convert_annotation_simple;

/// Orchestrates body lowering for all functions and methods.
pub struct BodyLoweringPass<'a> {
    symbols: &'a mut GlobalSymbols,
    module_scopes: &'a HashMap<ModuleId, ModuleScope>,
}

impl<'a> BodyLoweringPass<'a> {
    pub fn new(
        symbols: &'a mut GlobalSymbols,
        module_scopes: &'a HashMap<ModuleId, ModuleScope>,
    ) -> Self {
        Self {
            symbols,
            module_scopes,
        }
    }

    /// Lower all function and method bodies.
    /// Returns the TIR functions and classes.
    /// Collects all errors instead of stopping at the first one.
    pub fn run(
        &mut self,
        modules: &HashMap<ModuleName, Module>,
        module_order: &[ModuleName],
    ) -> Result<(Vec<TirFunction>, Vec<TirClass>)> {
        let mut errors = ErrorCollector::new();

        // Initialize function vector with placeholders
        let mut tir_functions: Vec<TirFunction> = vec![
            TirFunction {
                id: FuncId(0),
                name: String::new(),
                qualified_name: String::new(),
                params: Vec::new(),
                return_type: TirType::Void,
                locals: Vec::new(),
                body: Vec::new(),
                class: None,
                runtime_name: None,
            };
            self.symbols.next_func_id as usize
        ];

        // Build TirClass entries
        let mut tir_classes = self.build_classes(modules, module_order);

        // Lower function and method bodies, collecting all errors
        for ast_mod_id in module_order {
            let module = &modules[ast_mod_id];
            let mod_id = self.symbols.modules[&ast_mod_id.0];
            let scope = &self.module_scopes[&mod_id];

            self.lower_module_functions(
                module,
                ast_mod_id,
                mod_id,
                scope,
                &mut tir_functions,
                &mut errors,
            );

            self.lower_module_methods(
                module,
                ast_mod_id,
                mod_id,
                scope,
                &mut tir_functions,
                &mut errors,
            );
        }

        // Sort classes by ID
        tir_classes.sort_by_key(|c| c.id.0);

        errors.into_result_with((tir_functions, tir_classes))
    }

    /// Build TirClass entries from AST.
    fn build_classes(
        &self,
        modules: &HashMap<ModuleName, Module>,
        module_order: &[ModuleName],
    ) -> Vec<TirClass> {
        let mut tir_classes = Vec::new();

        for ast_mod_id in module_order {
            let module = &modules[ast_mod_id];

            for stmt in &module.body {
                if let Stmt::ClassDef { name, .. } = stmt {
                    let key = ClassKey::simple(format!("{}.{}", ast_mod_id.0, name));
                    let class_id = self.symbols.classes[&key];

                    let class_data = &self.symbols.class_data[class_id.index()];
                    tir_classes.push(TirClass {
                        id: class_id,
                        qualified_name: format!("{}.{}", ast_mod_id.0, name),
                        parent: class_data.parent,
                        inherited_fields: class_data.inherited_fields.clone(),
                        fields: class_data.fields.clone(),
                        methods: class_data.methods.clone(),
                        type_params: vec![],
                    });
                }
            }
        }

        tir_classes
    }

    /// Lower all function bodies in a module.
    fn lower_module_functions(
        &mut self,
        module: &Module,
        ast_mod_id: &ModuleName,
        mod_id: ModuleId,
        scope: &ModuleScope,
        tir_functions: &mut [TirFunction],
        errors: &mut ErrorCollector,
    ) {
        for stmt in &module.body {
            if let Stmt::FunctionDef {
                name,
                args,
                return_type,
                body,
            } = stmt
            {
                let func_id = self.symbols.functions[&(mod_id, name.clone())];
                let qualified_name = format!("{}.{}", ast_mod_id.0, name);

                match self.lower_function_body(
                    name,
                    &qualified_name,
                    args,
                    return_type.as_ref(),
                    body,
                    func_id,
                    mod_id,
                    scope,
                    None,
                ) {
                    Ok(tir_func) => tir_functions[func_id.index()] = tir_func,
                    Err(e) => errors.push(e),
                }
            }
        }
    }

    /// Lower all method bodies in a module.
    fn lower_module_methods(
        &mut self,
        module: &Module,
        ast_mod_id: &ModuleName,
        mod_id: ModuleId,
        scope: &ModuleScope,
        tir_functions: &mut [TirFunction],
        errors: &mut ErrorCollector,
    ) {
        for stmt in &module.body {
            if let Stmt::ClassDef { name, body, .. } = stmt {
                let key = ClassKey::simple(format!("{}.{}", ast_mod_id.0, name));
                let class_id = self.symbols.classes[&key];

                for item in body {
                    if let ClassBodyItem::MethodDef {
                        name: method_name,
                        args,
                        return_type,
                        body: method_body,
                    } = item
                    {
                        let (_, func_id) = self.symbols.methods[&(class_id, method_name.clone())];
                        let qualified_name = format!("{}.{}.{}", ast_mod_id.0, name, method_name);

                        // Skip self parameter for methods
                        let method_args: Vec<_> = args.iter().skip(1).cloned().collect();

                        match self.lower_function_body(
                            method_name,
                            &qualified_name,
                            &method_args,
                            return_type.as_ref(),
                            method_body,
                            func_id,
                            mod_id,
                            scope,
                            Some(class_id),
                        ) {
                            Ok(tir_func) => tir_functions[func_id.index()] = tir_func,
                            Err(e) => errors.push(e),
                        }
                    }
                }
            }
        }
    }

    /// Lower a single function or method body.
    #[allow(clippy::too_many_arguments)]
    fn lower_function_body(
        &mut self,
        name: &str,
        qualified_name: &str,
        args: &[Arg],
        return_type: Option<&crate::ast::TypeAnnotation>,
        body: &[Stmt],
        func_id: FuncId,
        mod_id: ModuleId,
        scope: &ModuleScope,
        class_id: Option<ClassId>,
    ) -> Result<TirFunction> {
        // Compute return type
        let ret_ty = return_type
            .map(|ann| convert_annotation_simple(ann, self.symbols, mod_id))
            .unwrap_or(TirType::Void);
        let ret_ty_unresolved = TirTypeUnresolved::from_tir_type(&ret_ty);

        // Create body lowerer
        let mut lowerer = BodyLowerer::new(self.symbols, scope, class_id, ret_ty_unresolved);

        // Add parameters
        for arg in args {
            if let Some(annot) = &arg.annotation {
                let ty = lowerer.convert_annotation(annot);
                lowerer.add_param(&arg.name, ty);
            }
        }

        // Lower body statements
        let mut tir_body_unresolved: Vec<TirStmtUnresolved> = Vec::new();
        for stmt in body {
            tir_body_unresolved.extend(lowerer.lower_stmt(stmt)?);
        }

        // Extract data before dropping lowerer
        let constraints = lowerer.constraints.constraints.clone();
        let locals_unresolved = lowerer.locals.clone();
        let param_types_unresolved = lowerer.param_types.clone();
        drop(lowerer);

        // Solve type constraints
        let mut solver = constraints::ConstraintSolver::new(self.symbols);
        solver.solve(&constraints)?;
        let substitutions = solver.get_substitutions().clone();

        // Resolve body
        let tir_body =
            crate::tir::resolve::resolve_body(tir_body_unresolved, &substitutions, self.symbols)?;

        // Resolve parameter types
        let resolved_params: Vec<(String, TirType)> = param_types_unresolved
            .iter()
            .enumerate()
            .map(|(i, ty)| {
                let resolved_ty =
                    crate::tir::resolve::resolve_type(ty, &substitutions, self.symbols)?;
                Ok((args[i].name.clone(), resolved_ty))
            })
            .collect::<Result<Vec<_>>>()?;

        // Resolve return type
        let resolved_ret_ty = crate::tir::resolve::resolve_type(
            &TirTypeUnresolved::from_tir_type(&ret_ty),
            &substitutions,
            self.symbols,
        )?;

        // Resolve locals
        let resolved_locals: Vec<(String, TirType)> = locals_unresolved
            .into_iter()
            .map(|(local_name, ty)| {
                let resolved_ty =
                    crate::tir::resolve::resolve_type(&ty, &substitutions, self.symbols)?;
                Ok((local_name, resolved_ty))
            })
            .collect::<Result<Vec<_>>>()?;

        let tir_func = TirFunction {
            id: func_id,
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            params: resolved_params,
            return_type: resolved_ret_ty,
            locals: resolved_locals,
            body: tir_body,
            class: class_id,
            runtime_name: None,
        };

        // Validate return paths
        validate_return_paths(&tir_func)?;

        Ok(tir_func)
    }
}

/// Check if a statement list always terminates with a return statement.
fn always_returns(stmts: &[TirStmt]) -> bool {
    for (i, stmt) in stmts.iter().enumerate() {
        match stmt {
            TirStmt::Return(_) => return true,
            TirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                if always_returns(then_body) && always_returns(else_body) && i == stmts.len() - 1 {
                    return true;
                }
            }
            TirStmt::While { body, .. } => {
                let _ = body;
            }
            _ => {}
        }
    }
    false
}

/// Validate that a function with a non-void return type always returns on all paths.
fn validate_return_paths(func: &TirFunction) -> Result<()> {
    use crate::error::CompilerError;

    if func.return_type == TirType::Void {
        return Ok(());
    }

    if !always_returns(&func.body) {
        return Err(CompilerError::TypeErrorSimple(format!(
            "Function '{}' with return type {:?} does not return a value on all code paths",
            func.name, func.return_type
        )));
    }

    Ok(())
}
