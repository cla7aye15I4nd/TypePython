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
        // Visit imports
        for import in &program.imports {
            self.visit_import(import)?;
        }
        // Visit classes
        for class in &program.classes {
            self.visit_class(class)?;
        }
        // Visit functions
        for function in &program.functions {
            self.visit_function(function)?;
        }
        // Visit statements
        for statement in &program.statements {
            self.visit_statement(statement)?;
        }
        self.finish_program(program)
    }

    /// Called after visiting all nodes in a program
    fn finish_program(&mut self, program: &Program) -> Result<Self::Result, Self::Error>;

    /// Visit an import statement
    fn visit_import(&mut self, import: &Import) -> Result<(), Self::Error>;

    /// Visit a class declaration
    fn visit_class(&mut self, class: &Class) -> Result<(), Self::Error>;

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
            Statement::For {
                var,
                iterable,
                body,
            } => self.visit_for(var, iterable, body),
            Statement::Return(expr) => self.visit_return(expr.as_ref()),
            Statement::Break => self.visit_break(),
            Statement::Continue => self.visit_continue(),
            Statement::Pass => self.visit_pass(),
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
        target: &AssignTarget,
        value: &Expression,
    ) -> Result<(), Self::Error>;

    /// Visit augmented assignment statement
    fn visit_aug_assignment(
        &mut self,
        target: &AssignTarget,
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

    /// Visit for statement
    fn visit_for(
        &mut self,
        var: &str,
        iterable: &Expression,
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

    /// Visit expression statement
    fn visit_expr_statement(&mut self, expr: &Expression) -> Result<(), Self::Error>;

    /// Visit an expression (returns Some value for evaluation)
    fn visit_expression(&mut self, expr: &Expression) -> Result<(), Self::Error> {
        match expr {
            Expression::IntLit(val) => self.visit_int_lit(*val),
            Expression::FloatLit(val) => self.visit_float_lit(*val),
            Expression::StrLit(val) => self.visit_str_lit(val),
            Expression::BytesLit(val) => self.visit_bytes_lit(val),
            Expression::BoolLit(val) => self.visit_bool_lit(*val),
            Expression::NoneLit => self.visit_none_lit(),
            Expression::Var(name) => self.visit_var(name),
            Expression::List(elements) => self.visit_list(elements),
            Expression::Tuple(elements) => self.visit_tuple(elements),
            Expression::Dict(pairs) => self.visit_dict(pairs),
            Expression::Set(elements) => self.visit_set(elements),
            Expression::BinOp { op, left, right } => self.visit_binop(op, left, right),
            Expression::UnaryOp { op, operand } => self.visit_unaryop(op, operand),
            Expression::Call { func, args } => self.visit_call(func, args),
            Expression::Attribute { object, attr } => self.visit_attribute(object, attr),
            Expression::Subscript { object, index } => self.visit_subscript(object, index),
            Expression::Slice { start, stop, step } => self.visit_slice(start, stop, step),
        }
    }

    /// Visit integer literal (default: no-op, used only when visit_expression is called)
    fn visit_int_lit(&mut self, _val: i64) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit float literal (default: no-op, used only when visit_expression is called)
    fn visit_float_lit(&mut self, _val: f64) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit string literal (default: no-op, used only when visit_expression is called)
    fn visit_str_lit(&mut self, _val: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit bytes literal (default: no-op, used only when visit_expression is called)
    fn visit_bytes_lit(&mut self, _val: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit boolean literal (default: no-op, used only when visit_expression is called)
    fn visit_bool_lit(&mut self, _val: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit None literal (default: no-op, used only when visit_expression is called)
    fn visit_none_lit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit variable reference (default: no-op, used only when visit_expression is called)
    fn visit_var(&mut self, _name: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit list literal
    fn visit_list(&mut self, elements: &[Expression]) -> Result<(), Self::Error>;

    /// Visit tuple literal
    fn visit_tuple(&mut self, elements: &[Expression]) -> Result<(), Self::Error>;

    /// Visit dict literal
    fn visit_dict(&mut self, pairs: &[(Expression, Expression)]) -> Result<(), Self::Error>;

    /// Visit set literal
    fn visit_set(&mut self, elements: &[Expression]) -> Result<(), Self::Error>;

    /// Visit binary operation
    fn visit_binop(
        &mut self,
        op: &BinaryOp,
        left: &Expression,
        right: &Expression,
    ) -> Result<(), Self::Error>;

    /// Visit unary operation
    fn visit_unaryop(&mut self, op: &UnaryOp, operand: &Expression) -> Result<(), Self::Error>;

    /// Visit function call
    fn visit_call(&mut self, func: &Expression, args: &[Expression]) -> Result<(), Self::Error>;

    /// Visit attribute access
    fn visit_attribute(&mut self, object: &Expression, attr: &str) -> Result<(), Self::Error>;

    /// Visit subscript operation
    fn visit_subscript(
        &mut self,
        object: &Expression,
        index: &Expression,
    ) -> Result<(), Self::Error>;

    /// Visit slice operation
    fn visit_slice(
        &mut self,
        start: &Option<Box<Expression>>,
        stop: &Option<Box<Expression>>,
        step: &Option<Box<Expression>>,
    ) -> Result<(), Self::Error>;
}
