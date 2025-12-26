//! Scope Building
//!
//! Builds per-module scopes with local definitions and import resolution.
//! After this phase, each module has a scope mapping local names to their definitions.

use std::collections::HashMap;

use crate::ast::{Expr, ImportAlias, ImportKind, Module, ModuleName, Stmt};
use crate::tir::ids::ModuleId;

use super::super::scope::ModuleScope;
use super::super::symbols::{ClassKey, GlobalSymbols};

/// Builds module scopes from AST modules and symbol tables.
pub struct ScopeBuilder<'a> {
    symbols: &'a GlobalSymbols,
}

impl<'a> ScopeBuilder<'a> {
    pub fn new(symbols: &'a GlobalSymbols) -> Self {
        Self { symbols }
    }

    /// Build scopes for all modules.
    /// Returns a map from module ID to its scope.
    pub fn build_all(
        &self,
        modules: &HashMap<ModuleName, Module>,
        module_order: &[ModuleName],
    ) -> HashMap<ModuleId, ModuleScope> {
        let mut module_scopes = HashMap::new();

        for ast_mod_id in module_order {
            let module = &modules[ast_mod_id];
            let mod_id = self.symbols.modules[&ast_mod_id.0];
            let scope = self.build_module_scope(module, ast_mod_id, mod_id);
            module_scopes.insert(mod_id, scope);
        }

        module_scopes
    }

    /// Build a scope for a single module.
    fn build_module_scope(
        &self,
        module: &Module,
        ast_mod_id: &ModuleName,
        mod_id: ModuleId,
    ) -> ModuleScope {
        let mut scope = ModuleScope::new(mod_id);

        // Add local definitions
        self.add_local_definitions(&mut scope, module, ast_mod_id, mod_id);

        // Resolve imports
        self.resolve_imports(&mut scope, module);

        scope
    }

    /// Add local function, class, and global definitions to the scope.
    fn add_local_definitions(
        &self,
        scope: &mut ModuleScope,
        module: &Module,
        ast_mod_id: &ModuleName,
        mod_id: ModuleId,
    ) {
        for stmt in &module.body {
            match stmt {
                Stmt::FunctionDef { name, .. } => {
                    if let Some(&func_id) = self.symbols.functions.get(&(mod_id, name.clone())) {
                        scope.functions.insert(name.clone(), func_id);
                    }
                }
                Stmt::ClassDef { name, .. } => {
                    let key = ClassKey::simple(format!("{}.{}", ast_mod_id.0, name));
                    if let Some(&class_id) = self.symbols.classes.get(&key) {
                        scope.classes.insert(name.clone(), class_id);
                    }
                }
                Stmt::Assign {
                    target: Expr::Name(name),
                    ..
                } => {
                    if let Some(&global_id) = self.symbols.globals.get(&(mod_id, name.clone())) {
                        scope.globals.insert(name.clone(), (mod_id, global_id));
                    }
                }
                _ => {}
            }
        }
    }

    /// Resolve import statements and add imported symbols to scope.
    fn resolve_imports(&self, scope: &mut ModuleScope, module: &Module) {
        for import in &module.imports {
            let Some(&imported_mod_id) = self.symbols.modules.get(&import.module_id.0) else {
                continue;
            };

            match &import.kind {
                ImportKind::Module { alias } => {
                    self.resolve_module_import(scope, imported_mod_id, alias, &import.module_id.0);
                }
                ImportKind::Names(names) => {
                    self.resolve_named_imports(scope, imported_mod_id, names, &import.module_id.0);
                }
                ImportKind::Star => {
                    self.resolve_star_import(scope, imported_mod_id, &import.module_id.0);
                }
            }
        }
    }

    /// Handle `import module` or `import module as alias`.
    fn resolve_module_import(
        &self,
        scope: &mut ModuleScope,
        imported_mod_id: ModuleId,
        alias: &Option<String>,
        module_name: &str,
    ) {
        let local_name = alias.clone().unwrap_or_else(|| module_name.to_string());
        scope.module_aliases.insert(local_name, imported_mod_id);
    }

    /// Handle `from module import name1, name2 as alias`.
    fn resolve_named_imports(
        &self,
        scope: &mut ModuleScope,
        imported_mod_id: ModuleId,
        names: &[ImportAlias],
        module_name: &str,
    ) {
        for name_alias in names {
            let local_name = name_alias.alias.as_ref().unwrap_or(&name_alias.name);

            // Try function first
            if let Some(&func_id) = self
                .symbols
                .functions
                .get(&(imported_mod_id, name_alias.name.clone()))
            {
                scope.functions.insert(local_name.clone(), func_id);
                continue;
            }

            // Try class
            let class_key = ClassKey::simple(format!("{}.{}", module_name, name_alias.name));
            if let Some(&class_id) = self.symbols.classes.get(&class_key) {
                scope.classes.insert(local_name.clone(), class_id);
                continue;
            }

            // Try global
            if let Some(&global_id) = self
                .symbols
                .globals
                .get(&(imported_mod_id, name_alias.name.clone()))
            {
                scope
                    .globals
                    .insert(local_name.clone(), (imported_mod_id, global_id));
            }
        }
    }

    /// Handle `from module import *`.
    fn resolve_star_import(
        &self,
        scope: &mut ModuleScope,
        imported_mod_id: ModuleId,
        module_name: &str,
    ) {
        // Import all public functions
        for (&(m, ref name), &func_id) in &self.symbols.functions {
            if m == imported_mod_id && !name.starts_with('_') {
                scope.functions.insert(name.clone(), func_id);
            }
        }

        // Import all public classes
        let prefix = format!("{}.", module_name);
        for (key, &class_id) in &self.symbols.classes {
            if key.qualified_name.starts_with(&prefix) && key.type_params.is_empty() {
                let class_name = &key.qualified_name[prefix.len()..];
                if !class_name.starts_with('_') {
                    scope.classes.insert(class_name.to_string(), class_id);
                }
            }
        }

        // Import all public globals
        for (&(m, ref name), &global_id) in &self.symbols.globals {
            if m == imported_mod_id && !name.starts_with('_') {
                scope
                    .globals
                    .insert(name.clone(), (imported_mod_id, global_id));
            }
        }
    }
}
