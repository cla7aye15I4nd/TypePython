/// Visitor pattern implementation for LLVM code generation
///
/// This module is organized into separate files for better maintainability:
/// - expressions.rs: Expression visitor methods (literals, variables, operators, calls)
/// - statements.rs: Statement visitor methods (var decl, assignment, if, while, return, etc.)
/// - program.rs: Program and function visitor methods
mod expressions;
mod program;
mod statements;

use super::CodeGen;
use crate::ast::visitor::Visitor;
use crate::ast::*;
use inkwell::values::BasicValueEnum;

// Implement the Visitor trait for CodeGen by delegating to the _impl methods
impl<'ctx> Visitor for CodeGen<'ctx> {
    type Result = BasicValueEnum<'ctx>;
    type Error = String;

    // Program-level methods
    fn visit_program(&mut self, program: &Program) -> Result<Self::Result, Self::Error> {
        self.visit_program_impl(program)
    }

    fn finish_program(&mut self, program: &Program) -> Result<Self::Result, Self::Error> {
        self.finish_program_impl(program)
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

    fn visit_import(&mut self, _import: &Import) -> Result<(), Self::Error> {
        // Imports are handled at module level, no code gen needed
        Ok(())
    }

    fn visit_class(&mut self, _class: &Class) -> Result<(), Self::Error> {
        todo!("Classes")
    }

    fn visit_assignment(
        &mut self,
        target: &AssignTarget,
        value: &Expression,
    ) -> Result<(), Self::Error> {
        self.visit_assignment_impl(target, value)
    }

    fn visit_aug_assignment(
        &mut self,
        target: &AssignTarget,
        op: &AugAssignOp,
        value: &Expression,
    ) -> Result<(), Self::Error> {
        self.visit_aug_assignment_impl(target, op, value)
    }

    fn visit_for(
        &mut self,
        var: &str,
        iterable: &Expression,
        body: &[Statement],
    ) -> Result<(), Self::Error> {
        self.visit_for_impl(var, iterable, body)
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

    fn visit_return(&mut self, expr: Option<&Expression>) -> Result<(), Self::Error> {
        self.visit_return_impl(expr)
    }

    fn visit_pass(&mut self) -> Result<(), Self::Error> {
        self.visit_pass_impl()
    }

    fn visit_expr_statement(&mut self, expr: &Expression) -> Result<(), Self::Error> {
        self.visit_expr_statement_impl(expr)
    }

    // Expression methods
    fn visit_int_lit(&mut self, val: i64) -> Result<(), Self::Error> {
        self.visit_int_lit_impl(val)?;
        Ok(())
    }

    fn visit_float_lit(&mut self, val: f64) -> Result<(), Self::Error> {
        self.visit_float_lit_impl(val)?;
        Ok(())
    }

    fn visit_str_lit(&mut self, val: &str) -> Result<(), Self::Error> {
        self.visit_str_lit_impl(val)?;
        Ok(())
    }

    fn visit_bytes_lit(&mut self, val: &str) -> Result<(), Self::Error> {
        self.visit_bytes_lit_impl(val)?;
        Ok(())
    }

    fn visit_bool_lit(&mut self, val: bool) -> Result<(), Self::Error> {
        self.visit_bool_lit_impl(val)?;
        Ok(())
    }

    fn visit_none_lit(&mut self) -> Result<(), Self::Error> {
        self.visit_none_lit_impl()?;
        Ok(())
    }

    fn visit_var(&mut self, name: &str) -> Result<(), Self::Error> {
        self.visit_var_impl(name)?;
        Ok(())
    }

    fn visit_list(&mut self, _elements: &[Expression]) -> Result<(), Self::Error> {
        todo!("List literals")
    }

    fn visit_tuple(&mut self, _elements: &[Expression]) -> Result<(), Self::Error> {
        todo!("Tuple literals")
    }

    fn visit_dict(&mut self, _pairs: &[(Expression, Expression)]) -> Result<(), Self::Error> {
        todo!("Dict literals")
    }

    fn visit_set(&mut self, _elements: &[Expression]) -> Result<(), Self::Error> {
        todo!("Set literals")
    }

    fn visit_binop(
        &mut self,
        op: &BinaryOp,
        left: &Expression,
        right: &Expression,
    ) -> Result<(), Self::Error> {
        self.visit_binop_impl(op, left, right)
    }

    fn visit_unaryop(&mut self, op: &UnaryOp, operand: &Expression) -> Result<(), Self::Error> {
        self.visit_unaryop_impl(op, operand)
    }

    fn visit_call(&mut self, func: &Expression, args: &[Expression]) -> Result<(), Self::Error> {
        self.visit_call_impl(func, args)
    }

    fn visit_attribute(&mut self, _object: &Expression, _attr: &str) -> Result<(), Self::Error> {
        todo!("Attribute access")
    }

    fn visit_subscript(
        &mut self,
        _object: &Expression,
        _index: &Expression,
    ) -> Result<(), Self::Error> {
        todo!("Subscript operation")
    }

    fn visit_slice(
        &mut self,
        _start: &Option<Box<Expression>>,
        _stop: &Option<Box<Expression>>,
        _step: &Option<Box<Expression>>,
    ) -> Result<(), Self::Error> {
        todo!("Slice operation")
    }
}
