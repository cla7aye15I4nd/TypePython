//! TIR function and class declarations
//!
//! Functions and classes are stored in flat vectors and referenced by ID.

use super::ids::{ClassId, FuncId};
use super::stmt::TirStmt;
use super::types::TirType;

/// A typed function definition
#[derive(Debug, Clone)]
pub struct TirFunction {
    /// Function ID for self-reference
    pub id: FuncId,

    /// Original name (for debugging/LLVM naming)
    pub name: String,

    /// Qualified name (module.function) for LLVM symbol
    pub qualified_name: String,

    /// Parameters: (name, type)
    pub params: Vec<(String, TirType)>,

    /// Return type
    pub return_type: TirType,

    /// Local variables (not including parameters)
    /// Indexed by LocalId
    pub locals: Vec<(String, TirType)>,

    /// Function body
    pub body: Vec<TirStmt>,

    /// If this is a method, which class it belongs to
    pub class: Option<ClassId>,

    /// If Some, this function is an external runtime function (no body to codegen)
    /// The value is the runtime function name to call (e.g., "list_len", "bytearray_append")
    pub runtime_name: Option<String>,
}

/// A typed class definition
#[derive(Debug, Clone)]
pub struct TirClass {
    /// Class ID for self-reference
    pub id: ClassId,

    /// Qualified name (module.class) for LLVM symbol
    pub qualified_name: String,

    /// Parent class (for single inheritance)
    pub parent: Option<ClassId>,

    /// Fields inherited from parent chain (in order: grandparent, parent, ...)
    /// These come first in struct layout
    pub inherited_fields: Vec<(String, TirType)>,

    /// Own fields in declaration order: (name, type)
    /// Indexed by FieldId (offset by inherited_fields.len())
    pub fields: Vec<(String, TirType)>,

    /// Methods: (name, FuncId)
    /// The FuncId points to the function in TirProgram.functions
    pub methods: Vec<(String, FuncId)>,

    /// Generic type parameters (e.g., element type for list[int])
    pub type_params: Vec<TirType>,
}

impl TirClass {
    /// Look up a method by name
    pub fn get_method(&self, name: &str) -> Option<FuncId> {
        self.methods
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, id)| *id)
    }

    /// Get all fields in struct layout order (inherited first, then own)
    pub fn all_fields(&self) -> impl Iterator<Item = &(String, TirType)> {
        self.inherited_fields.iter().chain(self.fields.iter())
    }
}
