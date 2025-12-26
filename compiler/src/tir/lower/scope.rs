use std::collections::HashMap;

use crate::tir::ids::{ClassId, FuncId, GlobalId, ModuleId};

/// Per-module scope for name resolution
pub(crate) struct ModuleScope {
    /// Local function names -> FuncId
    pub(crate) functions: HashMap<String, FuncId>,

    /// Local class names -> ClassId
    pub(crate) classes: HashMap<String, ClassId>,

    /// Local global variable names -> (ModuleId, GlobalId)
    pub(crate) globals: HashMap<String, (ModuleId, GlobalId)>,

    /// Module alias names -> ModuleId (for `import x` or `import x as y`)
    pub(crate) module_aliases: HashMap<String, ModuleId>,
}

impl ModuleScope {
    pub(crate) fn new(_mod_id: ModuleId) -> Self {
        ModuleScope {
            functions: HashMap::new(),
            classes: HashMap::new(),
            globals: HashMap::new(),
            module_aliases: HashMap::new(),
        }
    }
}
