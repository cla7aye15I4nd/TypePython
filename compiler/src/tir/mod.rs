//! Typed Intermediate Representation (TIR)
//!
//! TIR is the representation used after type checking and before code generation.
//! All symbols are resolved to numeric IDs and all types are embedded in expressions.
//! This allows code generation to be infallible (no Result types needed).
//!
//! ## Two-Layer Architecture
//!
//! TIR uses a two-layer architecture to provide compile-time guarantees about type resolution:
//!
//! - **Unresolved TIR** (internal): Used during lowering and type inference. Contains TypeVar
//!   variants representing types that haven't been fully inferred yet. These types are defined
//!   in the *_unresolved modules and are internal to the lowering process.
//!
//! - **Resolved TIR** (public): Used after constraint solving and during code generation. The
//!   TirType enum in this representation does not have a TypeVar variant, providing a
//!   compile-time guarantee that all types are fully resolved. Codegen only accepts this
//!   representation, making it impossible for unresolved types to reach code generation.

pub mod decls;
pub mod decls_unresolved;
pub mod expr;
pub mod expr_unresolved;
pub mod ids;
pub mod lower;
pub mod program;
pub mod program_unresolved;
pub mod resolve;
pub mod stmt;
pub mod stmt_unresolved;
pub mod types;
pub mod types_unresolved;

pub use decls::{TirClass, TirFunction};
pub use expr::{TirConstant, TirExpr, TirExprKind, VarRef};
pub use ids::{ClassId, FieldId, FuncId, GlobalId, LocalId, MethodId, ModuleId};
pub use lower::lower_to_tir;
pub use program::{TirModule, TirProgram};
pub use stmt::{TirLValue, TirStmt};
pub use types::TirType;
