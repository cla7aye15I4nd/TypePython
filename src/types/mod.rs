//! Type system for TypePython
//!
//! This module provides Python types with code generation support.
//! All type operations are consolidated in the PyValue struct.

pub mod value;

pub use value::{CgCtx, PyType, PyValue};
