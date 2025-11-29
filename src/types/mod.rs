//! Type system for TypePython
//!
//! This module provides Python types with code generation support.
//! Each type is implemented in its own file with its codegen operations.

pub mod bool;
pub mod bytes;
pub mod float;
pub mod int;
pub mod none;
pub mod traits;
pub mod value;

pub use value::{CgCtx, PyType, PyValue};

// Re-export type wrappers
pub use bool::PyBool;
pub use bytes::PyBytes;
pub use float::PyFloat;
pub use int::PyInt;
pub use none::PyNone;

// Re-export traits
pub use traits::{
    // Arithmetic
    Addable,
    // Composite
    Arithmetic,
    // Bitwise
    BitAndable,
    BitNegatable,
    BitOrable,
    BitXorable,
    Bitwise,
    Comparable,
    // Sequence
    Concatenatable,
    Divisible,
    // Comparison
    EqualityComparable,
    FloorDivisible,
    IdentityComparable,
    LeftShiftable,
    // Logical
    LogicalAndable,
    LogicalNegatable,
    LogicalOrable,
    // Membership
    MembershipTestable,
    Modulo,
    Multipliable,
    // Unary
    Negatable,
    OrderComparable,
    Powable,
    // Other
    Printable,
    Repeatable,
    RightShiftable,
    Sequence,
    Subtractable,
    ToBool,
    UnaryOps,
    UnaryPlusable,
};
