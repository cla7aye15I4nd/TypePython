use std::collections::HashMap;

use crate::ast;
use crate::tir::expr::VarRef;
use crate::tir::ids::{ClassId, LocalId};
use crate::tir::types_unresolved::TirTypeUnresolved;

use super::constraints::ConstraintSet;
use super::scope::ModuleScope;
use super::symbols::GlobalSymbols;

/// Context for lowering a function body
pub(crate) struct BodyLowerer<'a> {
    pub(crate) symbols: &'a mut GlobalSymbols,
    pub(crate) scope: &'a ModuleScope,

    /// Current class (if in a method)
    pub(crate) current_class: Option<ClassId>,

    /// Expected return type for the current function
    pub(crate) return_type: TirTypeUnresolved,

    /// Local variables: (name, type)
    pub(crate) locals: Vec<(String, TirTypeUnresolved)>,

    /// Local variable name -> LocalId
    pub(crate) local_names: HashMap<String, LocalId>,

    /// Parameter name -> index
    pub(crate) params: HashMap<String, u32>,

    /// Parameter types (for type lookup)
    pub(crate) param_types: Vec<TirTypeUnresolved>,

    /// Scope stack for shadowing
    pub(crate) scopes: Vec<HashMap<String, LocalId>>,

    /// Counter for local ID allocation
    pub(crate) next_local_id: u32,

    /// Type constraints collected during lowering (for type inference)
    pub(crate) constraints: ConstraintSet,
}

impl<'a> BodyLowerer<'a> {
    pub(crate) fn new(
        symbols: &'a mut GlobalSymbols,
        scope: &'a ModuleScope,
        current_class: Option<ClassId>,
        return_type: TirTypeUnresolved,
    ) -> Self {
        BodyLowerer {
            symbols,
            scope,
            current_class,
            return_type,
            locals: Vec::new(),
            local_names: HashMap::new(),
            params: HashMap::new(),
            param_types: Vec::new(),
            scopes: vec![HashMap::new()],
            next_local_id: 0,
            constraints: ConstraintSet::new(),
        }
    }

    pub(crate) fn add_param(&mut self, name: &str, ty: TirTypeUnresolved) {
        let index = self.params.len() as u32;
        self.params.insert(name.to_string(), index);
        self.param_types.push(ty);
    }

    pub(crate) fn alloc_local(&mut self, name: &str, ty: TirTypeUnresolved) -> LocalId {
        let id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push((name.to_string(), ty));
        self.local_names.insert(name.to_string(), id);
        // Also add to current scope for shadowing
        self.scopes.last_mut().unwrap().insert(name.to_string(), id);
        id
    }

    pub(crate) fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub(crate) fn exit_scope(&mut self) {
        let scope = self.scopes.pop().unwrap();
        // Remove locals from this scope from local_names
        for (name, _) in scope {
            self.local_names.remove(&name);
        }
    }

    pub(crate) fn resolve_var(&self, name: &str) -> Option<(VarRef, TirTypeUnresolved)> {
        // Check if it's 'self'
        if name == "self" {
            if let Some(class_id) = self.current_class {
                return Some((VarRef::SelfRef, TirTypeUnresolved::Class(class_id)));
            }
        }

        // Check locals (current scope lookup)
        if let Some(&local_id) = self.local_names.get(name) {
            let ty = self.locals[local_id.index()].1.clone();
            return Some((VarRef::Local(local_id), ty));
        }

        // Check parameters
        if let Some(&index) = self.params.get(name) {
            let ty = self.param_types[index as usize].clone();
            return Some((VarRef::Param(index), ty));
        }

        // Check module globals
        if let Some(&(mod_id, global_id)) = self.scope.globals.get(name) {
            // Look up the global's type from symbols (and convert to unresolved)
            let ty = self
                .symbols
                .global_types
                .get(&(mod_id, global_id))
                .map(TirTypeUnresolved::from_tir_type)
                .unwrap_or(TirTypeUnresolved::Int);
            return Some((VarRef::Global(mod_id, global_id), ty));
        }

        None
    }

    pub(crate) fn convert_annotation(&mut self, annot: &ast::TypeAnnotation) -> TirTypeUnresolved {
        match annot {
            ast::TypeAnnotation::Int => TirTypeUnresolved::Int,
            ast::TypeAnnotation::Float => TirTypeUnresolved::Float,
            ast::TypeAnnotation::Str => {
                let class_id = self.symbols.get_or_create_str_class();
                TirTypeUnresolved::Class(class_id)
            }
            ast::TypeAnnotation::Bool => TirTypeUnresolved::Bool,
            ast::TypeAnnotation::Bytes => {
                let class_id = self.symbols.get_or_create_bytes_class();
                TirTypeUnresolved::Class(class_id)
            }
            ast::TypeAnnotation::ByteArray => {
                let class_id = self.symbols.get_or_create_bytearray_class();
                TirTypeUnresolved::Class(class_id)
            }
            ast::TypeAnnotation::List(inner) => {
                let elem_ty = self.convert_annotation(inner);
                let class_id = self
                    .symbols
                    .get_or_create_list_class(&elem_ty.to_tir_type());
                TirTypeUnresolved::Class(class_id)
            }
            ast::TypeAnnotation::ClassName(name) => {
                // Look up class in scope
                if let Some(&class_id) = self.scope.classes.get(name) {
                    TirTypeUnresolved::Class(class_id)
                } else {
                    // Try to find in all modules
                    if let Some(class_id) = self.symbols.find_class_by_name(name) {
                        return TirTypeUnresolved::Class(class_id);
                    }
                    panic!("Class '{}' not found during lowering", name)
                }
            }
        }
    }
}
