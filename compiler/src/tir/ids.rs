//! Numeric ID types for TIR
//!
//! All references in TIR use numeric IDs instead of strings.
//! This enables O(1) lookups via direct array indexing.

/// Unique identifier for a function within the program.
/// Functions are stored in a flat Vec indexed by FuncId.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(pub u32);

impl FuncId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Unique identifier for a class within the program.
/// Classes are stored in a flat Vec indexed by ClassId.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClassId(pub u32);

impl ClassId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Unique identifier for a module within the program.
/// Modules are stored in a flat Vec indexed by ModuleId.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleId(pub u32);

impl ModuleId {
    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Identifier for a local variable within a function scope.
/// Each function has its own local variable table indexed by LocalId.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(pub u32);

impl LocalId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Identifier for a global variable within a module.
/// Each module has its own global variable table indexed by GlobalId.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalId(pub u32);

impl GlobalId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Identifier for a field within a class.
/// Fields are ordered and indexed by FieldId within each class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldId(pub u32);

impl FieldId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Identifier for a method within a class.
/// Methods reference functions via FuncId.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MethodId(pub u32);

impl MethodId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}
