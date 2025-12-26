use std::collections::HashMap;

use crate::tir::decls::TirClass;
use crate::tir::ids::{ClassId, FieldId, FuncId, GlobalId, MethodId, ModuleId};
use crate::tir::types::TirType;

/// Key for looking up classes in the symbol table.
/// Combines qualified name with generic type parameters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ClassKey {
    pub(crate) qualified_name: String,
    pub(crate) type_params: Vec<TirType>,
}

impl ClassKey {
    pub(crate) fn new(qualified_name: impl Into<String>, type_params: Vec<TirType>) -> Self {
        Self {
            qualified_name: qualified_name.into(),
            type_params,
        }
    }

    /// Create a key for a non-generic class
    pub(crate) fn simple(qualified_name: impl Into<String>) -> Self {
        Self {
            qualified_name: qualified_name.into(),
            type_params: vec![],
        }
    }

    /// Create a key for a built-in class
    pub(crate) fn builtin(name: &str) -> Self {
        Self::simple(format!("__builtin__.{}", name))
    }

    /// Create a key for a generic built-in class
    pub(crate) fn builtin_generic(name: &str, type_params: Vec<TirType>) -> Self {
        Self::new(format!("__builtin__.{}", name), type_params)
    }
}

/// Global symbol table built during pass 1
pub(crate) struct GlobalSymbols {
    /// Module name -> ModuleId
    pub(crate) modules: HashMap<String, ModuleId>,

    /// ModuleId -> Module name (reverse lookup)
    pub(crate) module_names: HashMap<ModuleId, String>,

    /// (ModuleId, function name) -> FuncId
    pub(crate) functions: HashMap<(ModuleId, String), FuncId>,

    /// ClassKey -> ClassId
    /// Unified storage for all classes (user-defined and built-in)
    pub(crate) classes: HashMap<ClassKey, ClassId>,

    /// (ClassId, method name) -> (MethodId, FuncId)
    pub(crate) methods: HashMap<(ClassId, String), (MethodId, FuncId)>,

    /// (ClassId, field name) -> FieldId
    pub(crate) fields: HashMap<(ClassId, String), FieldId>,

    /// (ModuleId, global name) -> GlobalId
    pub(crate) globals: HashMap<(ModuleId, String), GlobalId>,

    /// Global variable types: (ModuleId, GlobalId) -> TirType
    pub(crate) global_types: HashMap<(ModuleId, GlobalId), TirType>,

    /// Function signatures: FuncId -> (param types, return type)
    pub(crate) func_signatures: Vec<(Vec<TirType>, TirType)>,

    /// Class data: ClassId -> TirClass (fields and methods)
    pub(crate) class_data: Vec<TirClass>,

    /// Counters for ID allocation
    pub(crate) next_func_id: u32,
    pub(crate) next_class_id: u32,
    pub(crate) next_module_id: u32,

    /// Built-in runtime functions: cache_key -> FuncId
    /// Used for caching to avoid creating duplicate FuncIds for shared functions
    pub(crate) builtin_runtime_funcs: HashMap<String, FuncId>,

    /// Actual runtime function names: FuncId -> C function name
    /// Separates the cache key from the actual C function name to call
    pub(crate) runtime_func_names: HashMap<FuncId, String>,
}

impl GlobalSymbols {
    pub(crate) fn new() -> Self {
        GlobalSymbols {
            modules: HashMap::new(),
            module_names: HashMap::new(),
            functions: HashMap::new(),
            classes: HashMap::new(),
            methods: HashMap::new(),
            fields: HashMap::new(),
            globals: HashMap::new(),
            global_types: HashMap::new(),
            func_signatures: Vec::new(),
            class_data: Vec::new(),
            next_func_id: 0,
            next_class_id: 0,
            next_module_id: 0,
            builtin_runtime_funcs: HashMap::new(),
            runtime_func_names: HashMap::new(),
        }
    }

    pub(crate) fn alloc_module(&mut self, name: &str) -> ModuleId {
        let id = ModuleId(self.next_module_id);
        self.next_module_id += 1;
        self.modules.insert(name.to_string(), id);
        self.module_names.insert(id, name.to_string());
        id
    }

    pub(crate) fn alloc_func(&mut self, params: Vec<TirType>, return_type: TirType) -> FuncId {
        let id = FuncId(self.next_func_id);
        self.next_func_id += 1;
        self.func_signatures.push((params, return_type));
        id
    }

    pub(crate) fn alloc_class(&mut self) -> ClassId {
        let id = ClassId(self.next_class_id);
        self.next_class_id += 1;
        self.class_data.push(TirClass {
            id,
            qualified_name: String::new(),
            parent: None,
            inherited_fields: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            type_params: vec![],
        });
        id
    }

    pub(crate) fn get_func_signature(&self, func_id: FuncId) -> &(Vec<TirType>, TirType) {
        &self.func_signatures[func_id.index()]
    }

    /// Get or create a FuncId for a built-in runtime function
    /// Runtime functions are external C functions that will be linked in
    pub(crate) fn get_or_create_runtime_func(
        &mut self,
        name: &str,
        param_types: Vec<TirType>,
        return_type: TirType,
    ) -> FuncId {
        if let Some(&func_id) = self.builtin_runtime_funcs.get(name) {
            return func_id;
        }

        let func_id = self.alloc_func(param_types, return_type);
        self.builtin_runtime_funcs.insert(name.to_string(), func_id);
        // For shared runtime functions, the cache key is the actual C function name
        self.runtime_func_names.insert(func_id, name.to_string());
        func_id
    }

    /// Look up a class by module ID and class name
    pub(crate) fn lookup_class(&self, mod_id: ModuleId, class_name: &str) -> Option<ClassId> {
        let module_name = self.module_names.get(&mod_id)?;
        let key = ClassKey::simple(format!("{}.{}", module_name, class_name));
        self.classes.get(&key).copied()
    }

    /// Find a class by name across all modules (including builtin classes)
    pub(crate) fn find_class_by_name(&self, class_name: &str) -> Option<ClassId> {
        // First, check for builtin classes by exact name
        let builtin_key = ClassKey::builtin(class_name);
        if let Some(&class_id) = self.classes.get(&builtin_key) {
            return Some(class_id);
        }

        // Then check user-defined classes
        for (key, &class_id) in &self.classes {
            // Skip built-in classes and generic instantiations
            if key.qualified_name.starts_with("__builtin__.") || !key.type_params.is_empty() {
                continue;
            }
            // Check if the class name matches (last component of qualified_name)
            if key.qualified_name.ends_with(&format!(".{}", class_name)) {
                return Some(class_id);
            }
        }
        None
    }

    /// Set the parent class for a class
    pub(crate) fn set_parent(&mut self, class_id: ClassId, parent_id: ClassId) {
        self.class_data[class_id.index()].parent = Some(parent_id);
    }

    /// Collect inherited fields from parent chain (grandparent fields first)
    pub(crate) fn collect_inherited_fields(&self, class_id: ClassId) -> Vec<(String, TirType)> {
        let mut inherited = Vec::new();
        if let Some(parent_id) = self.class_data[class_id.index()].parent {
            // First add grandparent fields (recursively)
            inherited.extend(self.collect_inherited_fields(parent_id));
            // Then add direct parent fields
            inherited.extend(self.class_data[parent_id.index()].fields.clone());
        }
        inherited
    }

    /// Look up a method in a class and its parent chain
    pub(crate) fn resolve_method(
        &self,
        class_id: ClassId,
        method_name: &str,
    ) -> Option<(MethodId, FuncId)> {
        let mut current = Some(class_id);
        while let Some(id) = current {
            if let Some(&result) = self.methods.get(&(id, method_name.to_string())) {
                return Some(result);
            }
            current = self.class_data[id.index()].parent;
        }
        None
    }

    /// Check if a class inherits from Exception (directly or indirectly)
    pub(crate) fn is_exception_subclass(&self, class_id: ClassId) -> bool {
        let mut current = self.class_data[class_id.index()].parent;
        while let Some(id) = current {
            if self.class_data[id.index()].qualified_name == "__builtin__.Exception" {
                return true;
            }
            current = self.class_data[id.index()].parent;
        }
        false
    }

    // ============================================================
    // Type parameter helpers (for generic containers)
    // ============================================================
    //
    // During lowering, type_params use TirTypeUnresolved (can contain TypeVars).
    // After constraint solving and resolution, they become TirType (no TypeVars).

    /// Get the type parameters for a class (unresolved version)
    /// For generic containers like list[int], this returns [int]
    /// For empty containers like [], this may contain TypeVars like [TypeVar(0)]
    pub(crate) fn get_type_params(
        &self,
        class_id: ClassId,
    ) -> Vec<crate::tir::types_unresolved::TirTypeUnresolved> {
        use crate::tir::types_unresolved::TirTypeUnresolved;

        // Convert from TirType to TirTypeUnresolved
        // This is a temporary measure during the transition
        self.class_data[class_id.index()]
            .type_params
            .iter()
            .map(TirTypeUnresolved::from_tir_type)
            .collect()
    }

    /// Apply type variable substitutions to a class's type parameters
    /// This resolves TypeVars to their inferred concrete types after constraint solving
    /// PANICS if any TypeVar remains after substitution (indicates incomplete type inference)
    pub(crate) fn substitute_class_type_params(
        &mut self,
        class_id: ClassId,
        substitutions: &std::collections::HashMap<
            u32,
            crate::tir::types_unresolved::TirTypeUnresolved,
        >,
    ) {
        use crate::tir::types_unresolved::TirTypeUnresolved;

        let type_params = &mut self.class_data[class_id.index()].type_params;
        for param in type_params.iter_mut() {
            // Convert TirType to TirTypeUnresolved, substitute, then convert back
            let unresolved = TirTypeUnresolved::from_tir_type(param);
            let substituted = unresolved.substitute(substitutions);

            // Convert back to TirType (panics if TypeVar remains - indicates incomplete inference)
            *param = substituted.to_tir_type();
        }
    }

    /// Check if all type parameters of a class are fully resolved (contain no TypeVars)
    /// NOTE: Since TirType no longer has TypeVar variant, this always returns true.
    /// The check now happens during resolution from TirTypeUnresolved.
    pub(crate) fn class_type_params_resolved(&self, _class_id: ClassId) -> bool {
        // TirType has no TypeVar variant, so all types in type_params are resolved by definition
        true
    }

    // ============================================================
    // Print-related runtime function helpers
    // ============================================================

    /// Get the FuncId for int.__print__ (prints int without newline)
    pub(crate) fn get_int_print_func(&mut self) -> FuncId {
        self.get_or_create_runtime_func(
            "__pyc___builtin___int___print__",
            vec![TirType::Int],
            TirType::Void,
        )
    }

    /// Get the FuncId for bool.__print__ (prints bool without newline)
    pub(crate) fn get_bool_print_func(&mut self) -> FuncId {
        self.get_or_create_runtime_func(
            "__pyc___builtin___bool___print__",
            vec![TirType::Bool],
            TirType::Void,
        )
    }

    /// Get the FuncId for float.__print__ (prints float without newline)
    pub(crate) fn get_float_print_func(&mut self) -> FuncId {
        self.get_or_create_runtime_func(
            "__pyc___builtin___float___print__",
            vec![TirType::Float],
            TirType::Void,
        )
    }

    /// Get the FuncId for write_string_impl (prints String* without newline)
    pub(crate) fn get_write_string_func(&mut self) -> FuncId {
        let str_class_id = self.get_or_create_str_class();
        self.get_or_create_runtime_func(
            "write_string_impl",
            vec![TirType::Class(str_class_id)],
            TirType::Void,
        )
    }

    /// Get the FuncId for write_space_impl (prints a single space)
    pub(crate) fn get_write_space_func(&mut self) -> FuncId {
        self.get_or_create_runtime_func("write_space_impl", vec![], TirType::Void)
    }

    /// Get the FuncId for write_newline_impl (prints a newline)
    pub(crate) fn get_write_newline_func(&mut self) -> FuncId {
        self.get_or_create_runtime_func("write_newline_impl", vec![], TirType::Void)
    }
}
