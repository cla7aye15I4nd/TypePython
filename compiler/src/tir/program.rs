//! TIR program structure
//!
//! The program contains all modules, functions, and classes indexed by their IDs.

use super::decls::{TirClass, TirFunction};
use super::ids::{ClassId, FuncId, GlobalId, ModuleId};
use super::stmt::TirStmt;
use super::types::TirType;

/// Global variable definition
#[derive(Debug, Clone)]
pub struct TirGlobal {
    /// Global variable ID within the module
    pub id: GlobalId,

    /// Variable name
    pub name: String,

    /// Variable type
    pub ty: TirType,
}

/// A typed module
#[derive(Debug, Clone)]
pub struct TirModule {
    /// Module ID
    pub id: ModuleId,

    /// Module name (e.g., "mypackage.submodule")
    pub name: String,

    /// Global variables defined in this module
    pub globals: Vec<TirGlobal>,

    /// Functions defined in this module (IDs into TirProgram.functions)
    pub functions: Vec<FuncId>,

    /// Classes defined in this module (IDs into TirProgram.classes)
    pub classes: Vec<ClassId>,

    /// Module-level initialization code (non-function statements)
    pub init_body: Vec<TirStmt>,

    /// Local variables used in init_body (e.g., for loop temporaries)
    pub init_locals: Vec<(String, TirType)>,
}

/// The complete typed program - all modules combined.
/// This is the input to code generation.
#[derive(Debug)]
pub struct TirProgram {
    /// All functions across all modules (indexed by FuncId)
    pub functions: Vec<TirFunction>,

    /// All classes across all modules (indexed by ClassId)
    pub classes: Vec<TirClass>,

    /// All modules (indexed by ModuleId)
    pub modules: Vec<TirModule>,

    /// Entry module ID
    pub entry: ModuleId,
}

impl TirProgram {
    /// Get a function by ID
    pub fn function(&self, id: FuncId) -> &TirFunction {
        &self.functions[id.index()]
    }

    /// Get a class by ID
    pub fn class(&self, id: ClassId) -> &TirClass {
        &self.classes[id.index()]
    }

    /// Get a module by ID
    pub fn module(&self, id: ModuleId) -> &TirModule {
        &self.modules[id.index()]
    }
}
