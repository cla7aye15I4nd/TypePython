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
    fn visit_function(&mut self, function: &Function) -> Result<(), Self::Error>;

    /// Called when entering a function (before visiting body)
    fn enter_function(&mut self, function: &Function) -> Result<(), Self::Error>;

    /// Called when exiting a function (after visiting body)
    fn exit_function(&mut self, function: &Function) -> Result<(), Self::Error>;

    /// Visit a class declaration
    fn visit_class(&mut self, class: &Class) -> Result<(), Self::Error>;

    /// Visit a statement
    fn visit_statement(&mut self, statement: &Statement) -> Result<(), Self::Error> {
        match statement {
            Statement::VarDecl {
                name,
                var_type,
                value,
            } => self.visit_var_decl(name, var_type, value),
            Statement::AnnotatedAssignment {
                target,
                var_type,
                value,
            } => self.visit_annotated_assignment(target, var_type, value),
            Statement::Assignment { target, value } => self.visit_assignment(target, value),
            Statement::TupleUnpackAssignment { targets, value } => {
                self.visit_tuple_unpack_assignment(targets, value)
            }
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
            Statement::For {
                targets,
                iter,
                body,
                else_block,
            } => self.visit_for(targets, iter, body, else_block),
            Statement::Return(expr) => self.visit_return(expr.as_ref()),
            Statement::Break => self.visit_break(),
            Statement::Continue => self.visit_continue(),
            Statement::Pass => self.visit_pass(),
            Statement::Delete(target) => self.visit_delete(target),
            Statement::Expr(expr) => self.visit_expr_statement(expr),
            Statement::Try {
                body,
                handlers,
                else_block,
                finally_block,
            } => self.visit_try(body, handlers, else_block, finally_block),
            Statement::Raise { exception, cause } => self.visit_raise(exception, cause),
            Statement::Assert { test, msg } => self.visit_assert(test, msg),
            Statement::Global { names } => self.visit_global(names),
            Statement::Nonlocal { names } => self.visit_nonlocal(names),
        }
    }

    /// Visit variable declaration
    fn visit_var_decl(
        &mut self,
        name: &str,
        var_type: &Type,
        value: &Expression,
    ) -> Result<(), Self::Error>;

    /// Visit annotated assignment (e.g., self.x: int = 5)
    fn visit_annotated_assignment(
        &mut self,
        target: &Expression,
        var_type: &Type,
        value: &Expression,
    ) -> Result<(), Self::Error>;

    /// Visit assignment statement
    fn visit_assignment(
        &mut self,
        target: &Expression,
        value: &Expression,
    ) -> Result<(), Self::Error>;

    /// Visit tuple unpacking assignment (e.g., x, y = expr or self.a, self.b = expr)
    fn visit_tuple_unpack_assignment(
        &mut self,
        targets: &[Expression],
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

    /// Visit for statement (targets can be single var or tuple unpacking)
    fn visit_for(
        &mut self,
        targets: &[String],
        iter: &Expression,
        body: &[Statement],
        else_block: &Option<Vec<Statement>>,
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

    /// Visit try statement
    fn visit_try(
        &mut self,
        body: &[Statement],
        handlers: &[ExceptHandler],
        else_block: &Option<Vec<Statement>>,
        finally_block: &Option<Vec<Statement>>,
    ) -> Result<(), Self::Error>;

    /// Visit raise statement
    fn visit_raise(
        &mut self,
        exception: &Option<Expression>,
        cause: &Option<Expression>,
    ) -> Result<(), Self::Error>;

    /// Visit assert statement
    fn visit_assert(
        &mut self,
        test: &Expression,
        msg: &Option<Expression>,
    ) -> Result<(), Self::Error>;

    /// Visit global statement
    fn visit_global(&mut self, names: &[String]) -> Result<(), Self::Error>;

    /// Visit nonlocal statement
    fn visit_nonlocal(&mut self, names: &[String]) -> Result<(), Self::Error>;
}
