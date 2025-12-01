/// Visitor pattern implementation for LLVM code generation
///
/// This module is organized into separate files for better maintainability:
/// - expressions.rs: Expression visitor methods (literals, variables, operators, calls)
/// - statements.rs: Statement visitor methods (var decl, assignment, if, while, return, etc.)
/// - program.rs: Program and function visitor methods
mod expressions;
mod generator;
mod program;
mod statements;

use super::CodeGen;
use crate::ast::visitor::Visitor;
use crate::ast::*;
pub use generator::is_generator_function;
use inkwell::values::BasicValueEnum;

// Implement the Visitor trait for CodeGen by delegating to the _impl methods
impl<'ctx> Visitor for CodeGen<'ctx> {
    type Result = BasicValueEnum<'ctx>;
    type Error = String;

    // Program-level methods
    fn visit_program(&mut self, program: &Program) -> Result<Self::Result, Self::Error> {
        self.visit_program_impl(program)
    }

    // Override visit_function to handle generators
    fn visit_function(&mut self, function: &Function) -> Result<(), Self::Error> {
        if is_generator_function(function) {
            // Generate as a generator (coroutine-based)
            self.generate_generator_function(function)
        } else {
            // Normal function - use default visitor behavior
            self.enter_function(function)?;
            for statement in &function.body {
                self.visit_statement(statement)?;
            }
            self.exit_function(function)
        }
    }

    // Function-level methods
    fn enter_function(&mut self, func: &Function) -> Result<(), Self::Error> {
        self.enter_function_impl(func)
    }

    fn exit_function(&mut self, func: &Function) -> Result<(), Self::Error> {
        self.exit_function_impl(func)
    }

    // Statement methods
    fn visit_var_decl(
        &mut self,
        name: &str,
        var_type: &Type,
        value: &Expression,
    ) -> Result<(), Self::Error> {
        self.visit_var_decl_impl(name, var_type, value)
    }

    fn visit_annotated_assignment(
        &mut self,
        target: &Expression,
        _var_type: &Type,
        value: &Expression,
    ) -> Result<(), Self::Error> {
        // Annotated assignment is treated as a regular assignment at runtime
        // The type annotation is used only for static analysis and field type inference
        self.visit_assignment_impl(target, value)
    }

    fn visit_assignment(
        &mut self,
        target: &Expression,
        value: &Expression,
    ) -> Result<(), Self::Error> {
        self.visit_assignment_impl(target, value)
    }

    fn visit_tuple_unpack_assignment(
        &mut self,
        targets: &[Expression],
        value: &Expression,
    ) -> Result<(), Self::Error> {
        self.visit_tuple_unpack_assignment_impl(targets, value)
    }

    fn visit_aug_assignment(
        &mut self,
        target: &Expression,
        op: &AugAssignOp,
        value: &Expression,
    ) -> Result<(), Self::Error> {
        self.visit_aug_assignment_impl(target, op, value)
    }

    fn visit_break(&mut self) -> Result<(), Self::Error> {
        self.visit_break_impl()
    }

    fn visit_continue(&mut self) -> Result<(), Self::Error> {
        self.visit_continue_impl()
    }

    fn visit_if(
        &mut self,
        condition: &Expression,
        then_block: &[Statement],
        elif_clauses: &[(Expression, Vec<Statement>)],
        else_block: &Option<Vec<Statement>>,
    ) -> Result<(), Self::Error> {
        self.visit_if_impl(condition, then_block, elif_clauses, else_block)
    }

    fn visit_while(
        &mut self,
        condition: &Expression,
        body: &[Statement],
    ) -> Result<(), Self::Error> {
        self.visit_while_impl(condition, body)
    }

    fn visit_for(
        &mut self,
        targets: &[String],
        iter: &Expression,
        body: &[Statement],
        else_block: &Option<Vec<Statement>>,
    ) -> Result<(), Self::Error> {
        self.visit_for_impl(targets, iter, body, else_block)
    }

    fn visit_return(&mut self, expr: Option<&Expression>) -> Result<(), Self::Error> {
        self.visit_return_impl(expr)
    }

    fn visit_pass(&mut self) -> Result<(), Self::Error> {
        self.visit_pass_impl()
    }

    fn visit_delete(&mut self, target: &Expression) -> Result<(), Self::Error> {
        self.visit_delete_impl(target)
    }

    fn visit_expr_statement(&mut self, expr: &Expression) -> Result<(), Self::Error> {
        self.visit_expr_statement_impl(expr)
    }

    fn visit_import(&mut self, _import: &Import) -> Result<(), Self::Error> {
        // Imports are handled at program level, no action needed here
        Ok(())
    }

    fn visit_class(&mut self, class: &Class) -> Result<(), Self::Error> {
        self.visit_class_impl(class)
    }

    fn visit_try(
        &mut self,
        body: &[Statement],
        handlers: &[ExceptHandler],
        else_block: &Option<Vec<Statement>>,
        finally_block: &Option<Vec<Statement>>,
    ) -> Result<(), Self::Error> {
        self.visit_try_impl(body, handlers, else_block, finally_block)
    }

    fn visit_raise(
        &mut self,
        exception: &Option<Expression>,
        cause: &Option<Expression>,
    ) -> Result<(), Self::Error> {
        self.visit_raise_impl(exception, cause)
    }

    fn visit_assert(
        &mut self,
        test: &Expression,
        msg: &Option<Expression>,
    ) -> Result<(), Self::Error> {
        self.visit_assert_impl(test, msg)
    }

    fn visit_global(&mut self, names: &[String]) -> Result<(), Self::Error> {
        self.visit_global_impl(names)
    }

    fn visit_nonlocal(&mut self, names: &[String]) -> Result<(), Self::Error> {
        self.visit_nonlocal_impl(names)
    }
}
