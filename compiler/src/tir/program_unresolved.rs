//! TIR unresolved program structure
//!
//! This module defines the program structure used during lowering and type inference.
//! Unlike the final TirProgram, these structures can contain TirTypeUnresolved with TypeVar variants.
//!
//! After constraint solving, these are converted to fully resolved TirProgram via the resolve module.

use super::decls::TirClass;
use super::decls_unresolved::TirFunctionUnresolved;
use super::ids::{ClassId, FuncId, GlobalId, ModuleId};
use super::stmt_unresolved::TirStmtUnresolved;
use super::types_unresolved::TirTypeUnresolved;

/// Global variable definition (unresolved version)
#[derive(Debug, Clone)]
pub struct TirGlobalUnresolved {
    /// Global variable ID within the module
    pub id: GlobalId,

    /// Variable name
    pub name: String,

    /// Variable type (may contain TypeVar)
    pub ty: TirTypeUnresolved,
}

/// A typed module (unresolved version)
/// Types may contain TypeVar that will be resolved during constraint solving.
#[derive(Debug, Clone)]
pub struct TirModuleUnresolved {
    /// Module ID
    pub id: ModuleId,

    /// Module name (e.g., "mypackage.submodule")
    pub name: String,

    /// Global variables defined in this module (may have unresolved types)
    pub globals: Vec<TirGlobalUnresolved>,

    /// Functions defined in this module (IDs into TirProgramUnresolved.functions)
    pub functions: Vec<FuncId>,

    /// Classes defined in this module (IDs into TirProgramUnresolved.classes)
    pub classes: Vec<ClassId>,

    /// Module-level initialization code (non-function statements)
    /// May contain unresolved types
    pub init_body: Vec<TirStmtUnresolved>,

    /// Local variables used in init_body (e.g., for loop temporaries)
    /// Types may contain TypeVar
    pub init_locals: Vec<(String, TirTypeUnresolved)>,
}

/// The complete typed program - all modules combined (unresolved version).
/// This is the intermediate representation after lowering but before constraint solving.
/// After constraint solving and resolution, it becomes TirProgram for code generation.
#[derive(Debug)]
pub struct TirProgramUnresolved {
    /// All functions across all modules (indexed by FuncId)
    /// Functions may have unresolved types
    pub functions: Vec<TirFunctionUnresolved>,

    /// All classes across all modules (indexed by ClassId)
    /// Note: TirClass is the same in both resolved and unresolved representations
    /// since type_params are handled separately during constraint solving
    pub classes: Vec<TirClass>,

    /// All modules (indexed by ModuleId)
    /// Modules may have unresolved types
    pub modules: Vec<TirModuleUnresolved>,

    /// Entry module ID
    pub entry_module: ModuleId,
}
