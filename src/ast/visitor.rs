/// Visitor pattern for traversing the AST
use super::*;

/// The Visitor trait defines methods for visiting each AST node type.
/// Implement this trait to perform operations on the AST such as code generation,
/// analysis, transformation, etc.
pub trait Visitor {
    type Result;
    type Error;

    /// Visit a program node
    fn visit_program(&mut self, program: &Program) -> Result<Self::Result, Self::Error> {
        // Default implementation visits all functions and statements
        for function in &program.functions {
            self.visit_function(function)?;
        }
        for statement in &program.statements {
            self.visit_statement(statement)?;
        }
        self.finish_program(program)
    }

    /// Called after visiting all functions and statements in a program
    fn finish_program(&mut self, program: &Program) -> Result<Self::Result, Self::Error>;

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
            Statement::Assignment { name, value } => self.visit_assignment(name, value),
            Statement::If {
                condition,
                then_block,
                elif_clauses,
                else_block,
            } => self.visit_if(condition, then_block, elif_clauses, else_block),
            Statement::While { condition, body } => self.visit_while(condition, body),
            Statement::Return(expr) => self.visit_return(expr.as_ref()),
            Statement::Pass => self.visit_pass(),
            Statement::Expr(expr) => self.visit_expr_statement(expr),
        }
    }

    /// Visit a variable declaration
    fn visit_var_decl(
        &mut self,
        name: &str,
        var_type: &Type,
        value: &Expression,
    ) -> Result<(), Self::Error>;

    /// Visit an assignment statement
    fn visit_assignment(&mut self, name: &str, value: &Expression) -> Result<(), Self::Error>;

    /// Visit an if statement
    fn visit_if(
        &mut self,
        condition: &Expression,
        then_block: &[Statement],
        elif_clauses: &[(Expression, Vec<Statement>)],
        else_block: &Option<Vec<Statement>>,
    ) -> Result<(), Self::Error>;

    /// Visit a while statement
    fn visit_while(
        &mut self,
        condition: &Expression,
        body: &[Statement],
    ) -> Result<(), Self::Error>;

    /// Visit a return statement
    fn visit_return(&mut self, expr: Option<&Expression>) -> Result<(), Self::Error>;

    /// Visit a pass statement
    fn visit_pass(&mut self) -> Result<(), Self::Error>;

    /// Visit an expression statement
    fn visit_expr_statement(&mut self, expr: &Expression) -> Result<(), Self::Error>;

    /// Visit an expression and return a value
    fn visit_expression(&mut self, expr: &Expression) -> Result<Self::Result, Self::Error> {
        match expr {
            Expression::IntLit(val) => self.visit_int_lit(*val),
            Expression::FloatLit(val) => self.visit_float_lit(*val),
            Expression::StrLit(val) => self.visit_str_lit(val),
            Expression::BoolLit(val) => self.visit_bool_lit(*val),
            Expression::NoneLit() => self.visit_none_lit(),
            Expression::Var(name) => self.visit_var(name),
            Expression::BinOp { op, left, right } => self.visit_binary_op(op, left, right),
            Expression::UnaryOp { op, operand } => self.visit_unary_op(op, operand),
            Expression::Call { name, args } => self.visit_call(name, args),
        }
    }

    /// Visit an integer literal
    fn visit_int_lit(&mut self, val: i64) -> Result<Self::Result, Self::Error>;

    /// Visit a float literal
    fn visit_float_lit(&mut self, val: f64) -> Result<Self::Result, Self::Error>;

    /// Visit a string literal
    fn visit_str_lit(&mut self, val: &str) -> Result<Self::Result, Self::Error>;

    /// Visit a boolean literal
    fn visit_bool_lit(&mut self, val: bool) -> Result<Self::Result, Self::Error>;

    /// Visit a None literal
    fn visit_none_lit(&mut self) -> Result<Self::Result, Self::Error>;

    /// Visit a variable reference
    fn visit_var(&mut self, name: &str) -> Result<Self::Result, Self::Error>;

    /// Visit a binary operation
    fn visit_binary_op(
        &mut self,
        op: &BinaryOp,
        left: &Expression,
        right: &Expression,
    ) -> Result<Self::Result, Self::Error>;

    /// Visit a unary operation
    fn visit_unary_op(
        &mut self,
        op: &UnaryOp,
        operand: &Expression,
    ) -> Result<Self::Result, Self::Error>;

    /// Visit a function call
    fn visit_call(&mut self, name: &str, args: &[Expression]) -> Result<Self::Result, Self::Error>;
}
