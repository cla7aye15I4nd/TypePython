//! Type constraint generation and solving for type inference
//!
//! This module implements a bidirectional type inference system using constraint
//! generation and unification. Empty containers like `[]`, `{}`, and `set()` are
//! represented as Class(ClassId) where ClassData.type_params contains TypeVars,
//! which are then unified based on usage context.
//!
//! ## Architecture
//!
//! 1. **Pass 2.5**: Constraint Generation
//!    - Lower expressions, creating TypeVars for unknown types
//!    - Generate constraints from operations (append, indexing, etc.)
//!
//! 2. **Pass 2.75**: Constraint Solving
//!    - Unify constraints to find type variable substitutions
//!    - Detect infinite types via occurs-check
//!
//! 3. **Pass 3**: Apply substitutions and complete lowering

use crate::error::{CompilerError, Result};
use crate::tir::types_unresolved::TirTypeUnresolved;
use std::collections::HashMap;
use std::fmt;

use super::symbols::GlobalSymbols;

/// Origin of a type constraint - used for error reporting
#[derive(Debug, Clone)]
pub enum ConstraintOrigin {
    /// Method call on container
    MethodCall { method_name: String, line: usize },
}

impl fmt::Display for ConstraintOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstraintOrigin::MethodCall { method_name, line } => {
                write!(f, "method call '{}' at line {}", method_name, line)
            }
        }
    }
}

/// Type constraint that must be satisfied during type inference
#[derive(Debug, Clone)]
pub enum Constraint {
    /// Container element constraint: container has element type T
    /// Generated from operations like: list.append(x), list[0] = x
    ElementType {
        container: TirTypeUnresolved,
        element: TirTypeUnresolved,
        origin: ConstraintOrigin,
    },
}

/// Collection of type constraints generated during lowering
pub struct ConstraintSet {
    /// All constraints collected so far
    pub constraints: Vec<Constraint>,

    /// Next type variable ID to allocate
    next_type_var: u32,
}

impl ConstraintSet {
    /// Create a new empty constraint set
    pub fn new() -> Self {
        ConstraintSet {
            constraints: Vec::new(),
            next_type_var: 0,
        }
    }

    /// Generate a fresh type variable with a unique ID
    /// Used for empty containers and other unresolved types
    pub fn fresh_type_var(&mut self) -> TirTypeUnresolved {
        let id = self.next_type_var;
        self.next_type_var += 1;
        TirTypeUnresolved::TypeVar(id)
    }

    /// Add a constraint to the set
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }
}

/// Solver for type constraints using unification
pub struct ConstraintSolver<'a> {
    /// Substitution map: TypeVar ID -> resolved type
    /// This grows as constraints are solved
    substitutions: HashMap<u32, TirTypeUnresolved>,

    /// Reference to global symbols for accessing ClassData.type_params
    symbols: &'a GlobalSymbols,
}

impl<'a> ConstraintSolver<'a> {
    /// Create a new constraint solver
    pub fn new(symbols: &'a GlobalSymbols) -> Self {
        ConstraintSolver {
            substitutions: HashMap::new(),
            symbols,
        }
    }

    /// Solve all constraints, producing a substitution map
    /// Returns an error if constraints are unsatisfiable
    pub fn solve(&mut self, constraints: &[Constraint]) -> Result<()> {
        for constraint in constraints {
            match constraint {
                Constraint::ElementType {
                    container,
                    element,
                    origin,
                } => {
                    self.unify_element_type(container, element, origin)?;
                }
            }
        }
        Ok(())
    }

    /// Unify two types - make them equal by finding appropriate type variable bindings
    fn unify(
        &mut self,
        t1: &TirTypeUnresolved,
        t2: &TirTypeUnresolved,
        origin: &ConstraintOrigin,
    ) -> Result<()> {
        // Apply existing substitutions first to get most resolved types
        let t1 = t1.substitute(&self.substitutions);
        let t2 = t2.substitute(&self.substitutions);

        match (&t1, &t2) {
            // Same concrete types - success, nothing to do
            (TirTypeUnresolved::Int, TirTypeUnresolved::Int) => Ok(()),
            (TirTypeUnresolved::Bool, TirTypeUnresolved::Bool) => Ok(()),
            (TirTypeUnresolved::Void, TirTypeUnresolved::Void) => Ok(()),
            (TirTypeUnresolved::Class(c1), TirTypeUnresolved::Class(c2)) if c1 == c2 => {
                // Same class - check if type parameters match
                let type_params1 = self.symbols.get_type_params(*c1);
                let type_params2 = self.symbols.get_type_params(*c2);

                if type_params1.len() != type_params2.len() {
                    return Err(CompilerError::TypeInferenceError(format!(
                        "Type parameter count mismatch for class {:?} (at {})",
                        c1, origin
                    )));
                }

                // Unify each type parameter pair recursively
                for (t1, t2) in type_params1.iter().zip(type_params2.iter()) {
                    self.unify(t1, t2, origin)?;
                }

                Ok(())
            }

            // TypeVar unification - bind the type variable
            (TirTypeUnresolved::TypeVar(id), t) | (t, TirTypeUnresolved::TypeVar(id)) => {
                // If both are the same type variable, nothing to do
                if let TirTypeUnresolved::TypeVar(id2) = t {
                    if id == id2 {
                        return Ok(());
                    }
                }

                // Occurs check: prevent infinite types like T = list[T]
                if t.contains_type_var(*id) {
                    return Err(CompilerError::TypeInferenceError(format!(
                        "Infinite type detected: type variable {} occurs in {:?} (at {})",
                        id, t, origin
                    )));
                }

                // Bind the type variable to the concrete type
                self.substitutions.insert(*id, t.clone());
                Ok(())
            }

            // TODO: When unifying Class(id1) with Class(id2), compare type_params recursively
            // This happens when we have: x = [] (Class with TypeVar) and y: list[int] (Class)
            // Requires access to ClassData.type_params - implement in task #4

            // Unification failure - incompatible types
            _ => Err(CompilerError::TypeInferenceError(format!(
                "Cannot unify {:?} with {:?} (at {})",
                t1, t2, origin
            ))),
        }
    }

    /// Unify element type constraint: container[elem_ty]
    /// For lists and sets where we need to constrain the element type
    fn unify_element_type(
        &mut self,
        container: &TirTypeUnresolved,
        element: &TirTypeUnresolved,
        origin: &ConstraintOrigin,
    ) -> Result<()> {
        let container = container.substitute(&self.substitutions);
        let _element = element.substitute(&self.substitutions);

        match container {
            TirTypeUnresolved::Class(class_id) => {
                // For list[T] or set[T], type_params[0] should be the element type
                let type_params = self.symbols.get_type_params(class_id);

                if type_params.is_empty() {
                    return Err(CompilerError::TypeInferenceError(format!(
                        "ElementType constraint on class without type parameters: {:?} (at {})",
                        class_id, origin
                    )));
                }

                // Unify the first type parameter (element type) with the element
                self.unify(&type_params[0], &_element, origin)
            }
            _ => Err(CompilerError::TypeInferenceError(format!(
                "ElementType constraint on non-container type {:?} (at {})",
                container, origin
            ))),
        }
    }

    /// Get the final substitutions after solving
    pub fn get_substitutions(&self) -> &HashMap<u32, TirTypeUnresolved> {
        &self.substitutions
    }
}

// Note: No Default impl since ConstraintSolver requires a reference to GlobalSymbols

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_type_var() {
        let mut constraints = ConstraintSet::new();
        let t1 = constraints.fresh_type_var();
        let t2 = constraints.fresh_type_var();

        // Should generate unique IDs
        match (t1, t2) {
            (TirTypeUnresolved::TypeVar(id1), TirTypeUnresolved::TypeVar(id2)) => {
                assert_ne!(id1, id2);
                assert_eq!(id1, 0);
                assert_eq!(id2, 1);
            }
            _ => panic!("fresh_type_var should return TypeVar"),
        }
    }

    #[test]
    fn test_unify_concrete_types() {
        let symbols = GlobalSymbols::new();
        let mut solver = ConstraintSolver::new(&symbols);
        let origin = ConstraintOrigin::MethodCall {
            method_name: "test".to_string(),
            line: 1,
        };

        // Same types should unify
        assert!(solver
            .unify(&TirTypeUnresolved::Int, &TirTypeUnresolved::Int, &origin)
            .is_ok());
        assert!(solver
            .unify(&TirTypeUnresolved::Bool, &TirTypeUnresolved::Bool, &origin)
            .is_ok());

        // Different types should fail
        assert!(solver
            .unify(&TirTypeUnresolved::Int, &TirTypeUnresolved::Bool, &origin)
            .is_err());
    }

    #[test]
    fn test_unify_type_var() {
        let symbols = GlobalSymbols::new();
        let mut solver = ConstraintSolver::new(&symbols);
        let origin = ConstraintOrigin::MethodCall {
            method_name: "test".to_string(),
            line: 1,
        };

        let t_var = TirTypeUnresolved::TypeVar(0);
        let t_int = TirTypeUnresolved::Int;

        // Unifying TypeVar with Int should bind the TypeVar
        solver.unify(&t_var, &t_int, &origin).unwrap();

        // Check substitution was recorded
        assert_eq!(solver.substitutions.get(&0), Some(&TirTypeUnresolved::Int));
    }

    #[test]
    fn test_occurs_check() {
        // TODO: Rewrite this test once we have Class(ClassId) with type_params
        // For now, test basic occurs check with TypeVar
        let symbols = GlobalSymbols::new();
        let mut solver = ConstraintSolver::new(&symbols);
        let origin = ConstraintOrigin::MethodCall {
            method_name: "test".to_string(),
            line: 1,
        };

        let t_var = TirTypeUnresolved::TypeVar(0);
        // For now, we can't create a list[T0] without Generic*
        // This test will be updated when ClassData.type_params is implemented

        // Test that same TypeVar unifies with itself
        assert!(solver.unify(&t_var, &t_var, &origin).is_ok());
    }

    #[test]
    fn test_substitute() {
        let mut substitutions = HashMap::new();
        substitutions.insert(0, TirTypeUnresolved::Int);
        substitutions.insert(1, TirTypeUnresolved::Bool);

        let t_var0 = TirTypeUnresolved::TypeVar(0);
        let t_var1 = TirTypeUnresolved::TypeVar(1);

        // Substitute TypeVar(0) -> Int, TypeVar(1) -> Bool
        assert_eq!(t_var0.substitute(&substitutions), TirTypeUnresolved::Int);
        assert_eq!(t_var1.substitute(&substitutions), TirTypeUnresolved::Bool);

        // TODO: Add tests for substituting within ClassData.type_params
        // once we have access to ClassData
    }
}
