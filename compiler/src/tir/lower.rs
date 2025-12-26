//! AST to TIR lowering
//!
//! Lowers the AST to TIR after type checking. This is a multi-pass process:
//! 1. Collect all definitions and assign numeric IDs
//! 2. Build per-module import resolution
//! 3. Lower function/method bodies with resolved references

#[macro_use]
mod utils;

mod body_lowerer;
mod builtins;
mod constraints;
mod expr_lowering;
mod passes;
mod scope;
mod stmt_lowering;
mod symbols;

use body_lowerer::BodyLowerer;
use passes::{BodyLoweringPass, DefinitionCollector, ScopeBuilder};
use std::collections::HashMap;
use symbols::{ClassKey, GlobalSymbols};

// Re-export GlobalSymbols for use by the resolve module
pub(crate) use symbols::GlobalSymbols as GlobalSymbolsInternal;

use crate::ast::{Expr, Module, ModuleName, Stmt};
use crate::error::Result;

use super::decls::{TirClass, TirFunction};
use super::expr::VarRef;
use super::ids::{FuncId, GlobalId};
use super::program::{TirGlobal, TirModule, TirProgram};
use super::stmt_unresolved::TirStmtUnresolved;
use super::types::TirType;
use super::types_unresolved::TirTypeUnresolved;

pub fn lower_to_tir(
    modules: HashMap<ModuleName, Module>,
    entry_name: ModuleName,
) -> Result<TirProgram> {
    let mut symbols = GlobalSymbols::new();
    let mut module_order: Vec<ModuleName> = modules.keys().cloned().collect();
    module_order.sort_by(|a, b| a.0.cmp(&b.0));

    // Pre-create builtin classes that can be used as base classes
    // This ensures Exception is available when user classes inherit from it
    symbols.get_or_create_exception_class();

    // Collect all definitions (types, functions, methods, fields, globals)
    let mut collector = DefinitionCollector::new(&mut symbols);
    collector.run(&modules, &module_order)?;

    // Build per-module scopes
    let scope_builder = ScopeBuilder::new(&symbols);
    let module_scopes = scope_builder.build_all(&modules, &module_order);

    // Lower all function/method bodies
    let mut body_pass = BodyLoweringPass::new(&mut symbols, &module_scopes);
    let (mut tir_functions, mut tir_classes) = body_pass.run(&modules, &module_order)?;

    let mut tir_modules: Vec<TirModule> = Vec::new();

    // Build modules with init bodies
    for ast_mod_id in &module_order {
        let module = &modules[ast_mod_id];
        let mod_id = symbols.modules[&ast_mod_id.0];
        let scope = &module_scopes[&mod_id];

        let mut globals_unresolved: Vec<(GlobalId, String, TirTypeUnresolved)> = Vec::new();
        let mut functions = Vec::new();
        let mut classes = Vec::new();
        let mut init_body_unresolved: Vec<TirStmtUnresolved> = Vec::new();

        // Module init has void return type
        let mut lowerer = BodyLowerer::new(&mut symbols, scope, None, TirTypeUnresolved::Void);

        for stmt in &module.body {
            match stmt {
                Stmt::FunctionDef { name, .. } => {
                    if let Some(&func_id) = lowerer.symbols.functions.get(&(mod_id, name.clone())) {
                        functions.push(func_id);
                    }
                }
                Stmt::ClassDef { name, .. } => {
                    let key = ClassKey::simple(format!("{}.{}", ast_mod_id.0, name));
                    if let Some(&class_id) = lowerer.symbols.classes.get(&key) {
                        classes.push(class_id);
                    }
                }
                Stmt::Assign {
                    target: Expr::Name(name),
                    value,
                    type_annotation,
                } => {
                    if let Some(&global_id) = lowerer.symbols.globals.get(&(mod_id, name.clone())) {
                        let ty = type_annotation
                            .as_ref()
                            .map(|ann| lowerer.convert_annotation(ann))
                            .unwrap_or(TirTypeUnresolved::Int);
                        globals_unresolved.push((global_id, name.clone(), ty));

                        // Add init statement (unresolved)
                        let value_expr = lowerer.lower_expr(value)?;
                        init_body_unresolved.push(TirStmtUnresolved::Assign {
                            target: super::stmt_unresolved::TirLValueUnresolved::Var(
                                VarRef::Global(mod_id, global_id),
                            ),
                            value: value_expr,
                        });
                    }
                }
                _ => {
                    // Other statements at module level go into init
                    // This includes Stmt::Expr which needs to go through lower_stmt
                    // for proper print() expansion
                    init_body_unresolved.extend(lowerer.lower_stmt(stmt)?);
                }
            }
        }

        // Extract data from lowerer before dropping it
        let constraints = lowerer.constraints.constraints.clone();
        let init_locals_unresolved = lowerer.locals.clone();
        drop(lowerer); // Explicitly drop to release mutable borrow on symbols

        // Solve type constraints for module init
        let mut solver = constraints::ConstraintSolver::new(&symbols);
        solver.solve(&constraints)?;
        let substitutions = solver.get_substitutions().clone();

        // Resolve globals
        let globals: Vec<TirGlobal> = globals_unresolved
            .into_iter()
            .map(|(id, name, ty)| {
                let resolved_ty = super::resolve::resolve_type(&ty, &substitutions, &mut symbols)?;
                Ok(TirGlobal {
                    id,
                    name,
                    ty: resolved_ty,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        // Resolve the init body
        let init_body =
            super::resolve::resolve_body(init_body_unresolved, &substitutions, &mut symbols)?;

        // Resolve init locals
        let init_locals: Vec<(String, TirType)> = init_locals_unresolved
            .into_iter()
            .map(|(name, ty)| {
                let resolved_ty = super::resolve::resolve_type(&ty, &substitutions, &mut symbols)?;
                Ok((name, resolved_ty))
            })
            .collect::<Result<Vec<_>>>()?;

        tir_modules.push(TirModule {
            id: mod_id,
            name: ast_mod_id.0.clone(),
            globals,
            functions,
            classes,
            init_body,
            init_locals,
        });
    }

    // Sort modules by ID
    tir_modules.sort_by_key(|m| m.id.0);

    // Build TirClass for all built-in classes
    for (key, &class_id) in &symbols.classes {
        if key.qualified_name.starts_with("__builtin__.") {
            let class_data = &symbols.class_data[class_id.index()];
            let methods = class_data.methods.clone();
            tir_classes.push(TirClass {
                id: class_id,
                qualified_name: class_data.qualified_name.clone(),
                parent: None, // Builtin classes don't have parents
                inherited_fields: vec![],
                fields: vec![],
                methods,
                type_params: key.type_params.clone(),
            });
        }
    }

    // Re-sort classes by ID to include builtin classes
    tir_classes.sort_by_key(|c| c.id.0);

    // Resize tir_functions vector to accommodate all allocated FuncIds
    // (some were allocated after initial vector creation for builtin methods)
    tir_functions.resize_with(symbols.next_func_id as usize, || TirFunction {
        id: FuncId(0),
        name: String::new(),
        qualified_name: String::new(),
        params: Vec::new(),
        return_type: TirType::Void,
        locals: Vec::new(),
        body: Vec::new(),
        class: None,
        runtime_name: None,
    });

    // Fill in TirFunction entries for all runtime functions at their correct indices
    // These are stub functions that indicate external runtime functions
    for (cache_key, func_id) in &symbols.builtin_runtime_funcs {
        let (params, return_type) = &symbols.func_signatures[func_id.index()];
        // Convert param types to (name, type) tuples with placeholder names
        let params_with_names: Vec<(String, TirType)> = params
            .iter()
            .enumerate()
            .map(|(i, ty)| (format!("arg{}", i), ty.clone()))
            .collect();

        // Get actual C function name from runtime_func_names (may differ from cache_key for type-specific methods)
        let actual_runtime_name = symbols
            .runtime_func_names
            .get(func_id)
            .cloned()
            .unwrap_or_else(|| cache_key.clone());

        // Replace the placeholder function at this index
        tir_functions[func_id.index()] = TirFunction {
            id: *func_id,
            name: actual_runtime_name.clone(),
            qualified_name: actual_runtime_name.clone(),
            params: params_with_names,
            return_type: return_type.clone(),
            locals: vec![],
            body: vec![],
            class: None,
            runtime_name: Some(actual_runtime_name),
        };
    }

    let entry_mod_id = symbols.modules[&entry_name.0];

    Ok(TirProgram {
        functions: tir_functions,
        classes: tir_classes,
        modules: tir_modules,
        entry: entry_mod_id,
    })
}
