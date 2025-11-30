/// Visitor pattern for traversing the AST
use super::*;

/// The Visitor trait defines methods for visiting each AST node type.
/// Implement this trait to perform operations on the AST such as code generation,
/// analysis, transformation, etc.
pub trait Visitor {
    type Result;
    type Error;

    /// Visit a program node
    fn visit_program(&mut self, program: &Program) -> Result<Self::Result, Self::Error>;

    /// Visit an import statement
    fn visit_import(&mut self, import: &Import) -> Result<(), Self::Error>;

    /// Visit a function declaration
    fn visit_function(&mut self, function: &Function) -> Result<(), Self::Error> {
        self.enter_function(function)?;
        for statement in &function.body {
            self.visit_statement(statement)?;
        }
        self.exit_function(function)
    }

    /// Called when entering a function (before visiting body)
    fn enter_function(&mut self, function: &Function) -> Result<(), Self::Error>;

    /// Called when exiting a function (after visiting body)
    fn exit_function(&mut self, function: &Function) -> Result<(), Self::Error>;

    /// Visit a statement
    fn visit_statement(&mut self, statement: &Statement) -> Result<(), Self::Error> {
        match statement {
            Statement::VarDecl {
                name,
                var_type,
                value,
            } => self.visit_var_decl(name, var_type, value),
            Statement::Assignment { target, value } => self.visit_assignment(target, value),
            Statement::AugAssignment { target, op, value } => {
                self.visit_aug_assignment(target, op, value)
            }
            Statement::If {
                condition,
                then_block,
                elif_clauses,
                else_block,
            } => self.visit_if(condition, then_block, elif_clauses, else_block),
            Statement::While { condition, body } => self.visit_while(condition, body),
            Statement::Return(expr) => self.visit_return(expr.as_ref()),
            Statement::Break => self.visit_break(),
            Statement::Continue => self.visit_continue(),
            Statement::Pass => self.visit_pass(),
            Statement::Delete(target) => self.visit_delete(target),
            Statement::Expr(expr) => self.visit_expr_statement(expr),
        }
    }

    /// Visit variable declaration
    fn visit_var_decl(
        &mut self,
        name: &str,
        var_type: &Type,
        value: &Expression,
    ) -> Result<(), Self::Error>;

    /// Visit assignment statement
    fn visit_assignment(
        &mut self,
        target: &Expression,
        value: &Expression,
    ) -> Result<(), Self::Error>;

    /// Visit augmented assignment statement
    fn visit_aug_assignment(
        &mut self,
        target: &Expression,
        op: &AugAssignOp,
        value: &Expression,
    ) -> Result<(), Self::Error>;

    /// Visit if statement
    fn visit_if(
        &mut self,
        condition: &Expression,
        then_block: &[Statement],
        elif_clauses: &[(Expression, Vec<Statement>)],
        else_block: &Option<Vec<Statement>>,
    ) -> Result<(), Self::Error>;

    /// Visit while statement
    fn visit_while(
        &mut self,
        condition: &Expression,
        body: &[Statement],
    ) -> Result<(), Self::Error>;

    /// Visit return statement
    fn visit_return(&mut self, expr: Option<&Expression>) -> Result<(), Self::Error>;

    /// Visit break statement
    fn visit_break(&mut self) -> Result<(), Self::Error>;

    /// Visit continue statement
    fn visit_continue(&mut self) -> Result<(), Self::Error>;

    /// Visit pass statement
    fn visit_pass(&mut self) -> Result<(), Self::Error>;

    /// Visit delete statement
    fn visit_delete(&mut self, target: &Expression) -> Result<(), Self::Error>;

    /// Visit expression statement
    fn visit_expr_statement(&mut self, expr: &Expression) -> Result<(), Self::Error>;
}
