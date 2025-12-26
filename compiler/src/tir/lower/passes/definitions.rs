//! Definition Collection
//!
//! Collects all definitions (classes, functions, globals) and assigns numeric IDs.
//! This runs in four phases:
//! 1. `register_types` - Allocate module and class IDs
//! 2. `resolve_inheritance` - Link parent classes
//! 3. `collect_signatures` - Gather function/method signatures and fields
//! 4. `finalize_field_layout` - Compute field indices with inheritance

use std::collections::HashMap;

use crate::ast::{self, ClassBodyItem, Constant, Expr, Module, ModuleName, Stmt};
use crate::error::{CompilerError, ErrorCollector, Result};
use crate::tir::ids::{ClassId, FieldId, GlobalId, MethodId, ModuleId};
use crate::tir::types::TirType;

use super::super::symbols::{ClassKey, GlobalSymbols};

/// Collects all definitions from AST modules and registers them in the symbol table.
pub struct DefinitionCollector<'a> {
    pub symbols: &'a mut GlobalSymbols,
    /// Maps class IDs to their base class names (if any)
    class_bases: HashMap<ClassId, Option<String>>,
}

impl<'a> DefinitionCollector<'a> {
    pub fn new(symbols: &'a mut GlobalSymbols) -> Self {
        Self {
            symbols,
            class_bases: HashMap::new(),
        }
    }

    /// Run all definition collection phases.
    pub fn run(
        &mut self,
        modules: &HashMap<ModuleName, Module>,
        module_order: &[ModuleName],
    ) -> Result<()> {
        self.register_types(modules, module_order);
        self.resolve_inheritance()?;
        self.collect_signatures(modules, module_order);
        self.finalize_field_layout(modules, module_order);
        Ok(())
    }

    /// Phase 1: Register all module and class IDs.
    /// After this phase, all type names are known but not yet linked.
    fn register_types(
        &mut self,
        modules: &HashMap<ModuleName, Module>,
        module_order: &[ModuleName],
    ) {
        for ast_mod_id in module_order {
            let module = &modules[ast_mod_id];
            let _mod_id = self.symbols.alloc_module(&ast_mod_id.0);

            for stmt in &module.body {
                if let Stmt::ClassDef { name, base, .. } = stmt {
                    let class_id = self.symbols.alloc_class();
                    let key = ClassKey::simple(format!("{}.{}", ast_mod_id.0, name));
                    self.symbols.classes.insert(key, class_id);
                    self.class_bases.insert(class_id, base.clone());
                }
            }
        }
    }

    /// Phase 2: Resolve parent class references.
    /// Links each class to its parent in the symbol table.
    /// Collects all inheritance errors instead of stopping at the first one.
    fn resolve_inheritance(&mut self) -> Result<()> {
        let mut errors = ErrorCollector::new();

        for (&class_id, base_name_opt) in &self.class_bases.clone() {
            if let Some(base_name) = base_name_opt {
                match self.symbols.find_class_by_name(base_name) {
                    Some(parent_id) => {
                        self.symbols.set_parent(class_id, parent_id);
                    }
                    None => {
                        errors.push(CompilerError::TypeErrorSimple(format!(
                            "Undefined base class: '{}'",
                            base_name
                        )));
                    }
                }
            }
        }

        errors.into_result()
    }

    /// Phase 3: Collect all function signatures, method signatures, fields, and globals.
    fn collect_signatures(
        &mut self,
        modules: &HashMap<ModuleName, Module>,
        module_order: &[ModuleName],
    ) {
        for ast_mod_id in module_order {
            let module = &modules[ast_mod_id];
            let mod_id = self.symbols.modules[&ast_mod_id.0];

            self.collect_functions(module, mod_id);
            self.collect_class_members(module, ast_mod_id, mod_id);
            self.collect_globals(module, mod_id);
        }
    }

    fn collect_functions(&mut self, module: &Module, mod_id: ModuleId) {
        for stmt in &module.body {
            if let Stmt::FunctionDef {
                name,
                args,
                return_type,
                ..
            } = stmt
            {
                let params: Vec<TirType> = args
                    .iter()
                    .filter_map(|arg| arg.annotation.as_ref())
                    .map(|ann| convert_annotation_simple(ann, self.symbols, mod_id))
                    .collect();
                let ret_ty = return_type
                    .as_ref()
                    .map(|ann| convert_annotation_simple(ann, self.symbols, mod_id))
                    .unwrap_or(TirType::Void);
                let func_id = self.symbols.alloc_func(params, ret_ty);
                self.symbols
                    .functions
                    .insert((mod_id, name.clone()), func_id);
            }
        }
    }

    fn collect_class_members(
        &mut self,
        module: &Module,
        ast_mod_id: &ModuleName,
        mod_id: ModuleId,
    ) {
        for stmt in &module.body {
            if let Stmt::ClassDef { name, body, .. } = stmt {
                let key = ClassKey::simple(format!("{}.{}", ast_mod_id.0, name));
                let class_id = self.symbols.classes[&key];

                // Collect own fields
                for item in body {
                    if let ClassBodyItem::FieldDef {
                        name: field_name,
                        annotation,
                    } = item
                    {
                        let field_ty = convert_annotation_simple(annotation, self.symbols, mod_id);
                        self.symbols.class_data[class_id.index()]
                            .fields
                            .push((field_name.clone(), field_ty));
                    }
                }

                // Collect methods
                self.collect_methods(body, class_id, mod_id);
            }
        }
    }

    fn collect_methods(&mut self, body: &[ClassBodyItem], class_id: ClassId, mod_id: ModuleId) {
        let mut method_idx = 0u32;
        for item in body {
            if let ClassBodyItem::MethodDef {
                name: method_name,
                args,
                return_type,
                ..
            } = item
            {
                // Skip 'self' parameter
                let params: Vec<TirType> = args
                    .iter()
                    .skip(1)
                    .filter_map(|arg| arg.annotation.as_ref())
                    .map(|ann| convert_annotation_simple(ann, self.symbols, mod_id))
                    .collect();
                let ret_ty = return_type
                    .as_ref()
                    .map(|ann| convert_annotation_simple(ann, self.symbols, mod_id))
                    .unwrap_or(TirType::Void);

                let func_id = self.symbols.alloc_func(params, ret_ty);
                let method_id = MethodId(method_idx);
                method_idx += 1;

                self.symbols
                    .methods
                    .insert((class_id, method_name.clone()), (method_id, func_id));
                self.symbols.class_data[class_id.index()]
                    .methods
                    .push((method_name.clone(), func_id));
            }
        }
    }

    fn collect_globals(&mut self, module: &Module, mod_id: ModuleId) {
        let mut global_idx = 0u32;
        for stmt in &module.body {
            if let Stmt::Assign {
                target: Expr::Name(name),
                type_annotation,
                value,
            } = stmt
            {
                let global_id = GlobalId(global_idx);
                global_idx += 1;
                self.symbols
                    .globals
                    .insert((mod_id, name.clone()), global_id);

                let ty = if let Some(annot) = type_annotation {
                    convert_annotation_simple(annot, self.symbols, mod_id)
                } else {
                    // Infer type from constant value
                    match value {
                        Expr::Constant(Constant::Int(_)) => TirType::Int,
                        Expr::Constant(Constant::Str(_)) => {
                            let class_id = self.symbols.get_or_create_str_class();
                            TirType::Class(class_id)
                        }
                        Expr::Constant(Constant::Bool(_)) => TirType::Bool,
                        Expr::Constant(Constant::Bytes(_)) => {
                            let class_id = self.symbols.get_or_create_bytes_class();
                            TirType::Class(class_id)
                        }
                        _ => TirType::Int,
                    }
                };
                self.symbols.global_types.insert((mod_id, global_id), ty);
            }
        }
    }

    /// Phase 4: Compute final field layout including inherited fields.
    /// Assigns field IDs with proper offsets accounting for inheritance.
    fn finalize_field_layout(
        &mut self,
        modules: &HashMap<ModuleName, Module>,
        module_order: &[ModuleName],
    ) {
        // Process classes with parents first
        for &class_id in self.class_bases.keys() {
            let inherited = self.symbols.collect_inherited_fields(class_id);
            self.symbols.class_data[class_id.index()].inherited_fields = inherited;

            // Register inherited fields at indices 0..N
            let inherited_count = self.symbols.class_data[class_id.index()]
                .inherited_fields
                .len();
            for (idx, (field_name, _)) in self.symbols.class_data[class_id.index()]
                .inherited_fields
                .clone()
                .iter()
                .enumerate()
            {
                let field_id = FieldId(idx as u32);
                self.symbols
                    .fields
                    .insert((class_id, field_name.clone()), field_id);
            }

            // Register own fields at indices N..N+M
            for (idx, (field_name, _)) in self.symbols.class_data[class_id.index()]
                .fields
                .clone()
                .iter()
                .enumerate()
            {
                let field_id = FieldId((inherited_count + idx) as u32);
                self.symbols
                    .fields
                    .insert((class_id, field_name.clone()), field_id);
            }
        }

        // Register fields for classes without parents
        for ast_mod_id in module_order {
            let module = &modules[ast_mod_id];
            for stmt in &module.body {
                if let Stmt::ClassDef { name, .. } = stmt {
                    let key = ClassKey::simple(format!("{}.{}", ast_mod_id.0, name));
                    let class_id = self.symbols.classes[&key];

                    if self.symbols.class_data[class_id.index()].parent.is_some() {
                        continue;
                    }

                    for (idx, (field_name, _)) in self.symbols.class_data[class_id.index()]
                        .fields
                        .clone()
                        .iter()
                        .enumerate()
                    {
                        let field_id = FieldId(idx as u32);
                        self.symbols
                            .fields
                            .insert((class_id, field_name.clone()), field_id);
                    }
                }
            }
        }
    }
}

/// Convert a type annotation to TirType without full scope resolution.
/// Used during definition collection when scopes aren't built yet.
pub fn convert_annotation_simple(
    annot: &ast::TypeAnnotation,
    symbols: &mut GlobalSymbols,
    current_mod: ModuleId,
) -> TirType {
    match annot {
        ast::TypeAnnotation::Int => TirType::Int,
        ast::TypeAnnotation::Float => TirType::Float,
        ast::TypeAnnotation::Str => {
            let class_id = symbols.get_or_create_str_class();
            TirType::Class(class_id)
        }
        ast::TypeAnnotation::Bool => TirType::Bool,
        ast::TypeAnnotation::Bytes => {
            let class_id = symbols.get_or_create_bytes_class();
            TirType::Class(class_id)
        }
        ast::TypeAnnotation::ByteArray => {
            let class_id = symbols.get_or_create_bytearray_class();
            TirType::Class(class_id)
        }
        ast::TypeAnnotation::List(inner) => {
            let elem_ty = convert_annotation_simple(inner, symbols, current_mod);
            let class_id = symbols.get_or_create_list_class(&elem_ty);
            TirType::Class(class_id)
        }
        ast::TypeAnnotation::ClassName(name) => {
            // First try current module, then global lookup
            if let Some(class_id) = symbols.lookup_class(current_mod, name) {
                return TirType::Class(class_id);
            }
            if let Some(class_id) = symbols.find_class_by_name(name) {
                return TirType::Class(class_id);
            }
            // Forward reference placeholder
            TirType::Class(ClassId(u32::MAX))
        }
    }
}
