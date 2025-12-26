//! Lowering passes for AST to TIR conversion
//!
//! The lowering process consists of:
//! - Definition collection: Register types and collect all signatures
//! - Scope building: Build per-module scopes with import resolution
//! - Body lowering: Lower function and method bodies to TIR

mod bodies;
mod definitions;
mod scopes;

pub use bodies::BodyLoweringPass;
pub use definitions::{convert_annotation_simple, DefinitionCollector};
pub use scopes::ScopeBuilder;
