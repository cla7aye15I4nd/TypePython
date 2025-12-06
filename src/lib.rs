// Public modules
pub mod ast;
pub mod cli;
pub mod codegen;
pub mod module;
pub mod pipeline;

// Re-export types from codegen for backwards compatibility
pub use codegen::types;
