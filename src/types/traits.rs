//! Type traits for Python type operations
//!
//! This module defines traits that Python types can implement for various operations.
//! Each trait represents a single capability, following the single-responsibility principle.

use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FunctionValue};

use super::value::{CgCtx, PyValue};

// ============================================================================
// Print Operations
// ============================================================================

/// Trait for types that can be printed
pub trait Printable<'ctx> {
    /// Generate a print call for this value
    fn print(&self, builder: &Builder<'ctx>, print_fn: FunctionValue<'ctx>) -> Result<(), String>;

    /// Get the print function name for this type
    fn print_function_name(&self) -> &'static str;
}

// ============================================================================
// Arithmetic Operations (Binary)
// ============================================================================

/// Trait for types that support addition
pub trait Addable<'ctx> {
    fn add<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support subtraction
pub trait Subtractable<'ctx> {
    fn sub<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support multiplication
pub trait Multipliable<'ctx> {
    fn mul<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support true division (/)
pub trait Divisible<'ctx> {
    fn div<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support floor division (//)
pub trait FloorDivisible<'ctx> {
    fn floordiv<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support modulo (%)
pub trait Modulo<'ctx> {
    fn modulo<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support power (**)
pub trait Powable<'ctx> {
    fn pow<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String>;
}

// ============================================================================
// Bitwise Operations
// ============================================================================

/// Trait for types that support bitwise AND (&)
pub trait BitAndable<'ctx> {
    fn bitand<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support bitwise OR (|)
pub trait BitOrable<'ctx> {
    fn bitor<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>)
        -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support bitwise XOR (^)
pub trait BitXorable<'ctx> {
    fn bitxor<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support left shift (<<)
pub trait LeftShiftable<'ctx> {
    fn lshift<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support right shift (>>)
pub trait RightShiftable<'ctx> {
    fn rshift<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String>;
}

// ============================================================================
// Comparison Operations
// ============================================================================

/// Trait for types that support equality comparison (==, !=)
pub trait EqualityComparable<'ctx> {
    fn eq<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String>;

    fn ne<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support ordering comparison (<, <=, >, >=)
pub trait OrderComparable<'ctx> {
    fn lt<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String>;

    fn le<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String>;

    fn gt<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String>;

    fn ge<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support identity comparison (is, is not)
pub trait IdentityComparable<'ctx> {
    fn is<'a>(&self, cg: &CgCtx<'a, 'ctx>, rhs: &PyValue<'ctx>) -> Result<PyValue<'ctx>, String>;

    fn is_not<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String>;
}

// ============================================================================
// Logical Operations
// ============================================================================

/// Trait for types that support logical AND
pub trait LogicalAndable<'ctx> {
    fn logical_and<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support logical OR
pub trait LogicalOrable<'ctx> {
    fn logical_or<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String>;
}

// ============================================================================
// Membership Operations
// ============================================================================

/// Trait for types that support membership testing (in, not in)
pub trait MembershipTestable<'ctx> {
    fn contains<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        item: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String>;
}

// ============================================================================
// Unary Operations
// ============================================================================

/// Trait for types that support negation (-)
pub trait Negatable<'ctx> {
    fn neg<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String>;
}

/// Trait for types that support unary plus (+)
pub trait UnaryPlusable<'ctx> {
    fn pos<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String>;
}

/// Trait for types that support logical not (not)
pub trait LogicalNegatable<'ctx> {
    fn logical_not<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String>;
}

/// Trait for types that support bitwise not (~)
pub trait BitNegatable<'ctx> {
    fn bitnot<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String>;
}

// ============================================================================
// Type Conversion
// ============================================================================

/// Trait for types that can be converted to bool (truthiness)
pub trait ToBool<'ctx> {
    fn to_bool<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String>;
}

// ============================================================================
// Sequence Operations
// ============================================================================

/// Trait for sequence types (bytes, lists, etc.)
pub trait Sequence<'ctx> {
    /// Get the length of this sequence
    fn len<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String>;

    /// Get an item at the given index
    fn getitem<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        index: BasicValueEnum<'ctx>,
    ) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support concatenation (bytes, strings, lists)
pub trait Concatenatable<'ctx> {
    fn concat<'a>(&self, cg: &CgCtx<'a, 'ctx>, other: &Self) -> Result<PyValue<'ctx>, String>;
}

/// Trait for types that support repetition (bytes * int, list * int)
pub trait Repeatable<'ctx> {
    fn repeat<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        count: BasicValueEnum<'ctx>,
    ) -> Result<PyValue<'ctx>, String>;
}

// ============================================================================
// Composite Traits (for convenience)
// ============================================================================

/// Full arithmetic support (add, sub, mul, div, floordiv, mod, pow)
pub trait Arithmetic<'ctx>:
    Addable<'ctx>
    + Subtractable<'ctx>
    + Multipliable<'ctx>
    + Divisible<'ctx>
    + FloorDivisible<'ctx>
    + Modulo<'ctx>
    + Powable<'ctx>
{
}

/// Full bitwise support (and, or, xor, lshift, rshift)
pub trait Bitwise<'ctx>:
    BitAndable<'ctx> + BitOrable<'ctx> + BitXorable<'ctx> + LeftShiftable<'ctx> + RightShiftable<'ctx>
{
}

/// Full comparison support (eq, ne, lt, le, gt, ge, is, is_not)
pub trait Comparable<'ctx>:
    EqualityComparable<'ctx> + OrderComparable<'ctx> + IdentityComparable<'ctx>
{
}

/// Full unary support (neg, pos, not, bitnot)
pub trait UnaryOps<'ctx>:
    Negatable<'ctx> + UnaryPlusable<'ctx> + LogicalNegatable<'ctx> + BitNegatable<'ctx>
{
}
