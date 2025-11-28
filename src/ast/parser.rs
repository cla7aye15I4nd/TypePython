use crate::ast::*;
use crate::Rule;
use pest::iterators::{Pair, Pairs};

pub fn build_program(mut pairs: Pairs<Rule>) -> Program {
    let mut imports = Vec::new();
    let mut functions = Vec::new();
    let mut classes = Vec::new();
    let mut statements = Vec::new();

    // The program rule wraps everything, so we need to get its inner pairs
    if let Some(program_pair) = pairs.next() {
        if program_pair.as_rule() == Rule::program {
            for pair in program_pair.into_inner() {
                match pair.as_rule() {
                    Rule::import_stmt => imports.push(build_import(pair)),
                    Rule::func_decl => functions.push(build_function(pair)),
                    Rule::class_decl => classes.push(build_class(pair)),
                    Rule::statement => statements.push(build_statement(pair)),
                    Rule::EOI | Rule::NEWLINE => {}
                    _ => panic!("Unexpected rule in program: {:?}", pair.as_rule()),
                }
            }
        }
    }

    Program {
        imports,
        functions,
        classes,
        statements,
    }
}

fn build_import(pair: Pair<Rule>) -> Import {
    let mut inner = pair.into_inner();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::IMPORT);

    let module_path_pair = inner.next().unwrap();
    assert_eq!(module_path_pair.as_rule(), Rule::module_path);

    let module_path = build_module_path(module_path_pair);

    assert_eq!(inner.next(), None);

    Import { module_path }
}

fn build_module_path(pair: Pair<Rule>) -> Vec<String> {
    pair.into_inner()
        .filter(|p| p.as_rule() != Rule::DOT)
        .map(|id| {
            assert_eq!(id.as_rule(), Rule::ID);
            id.as_str().to_string()
        })
        .collect()
}

fn build_class(pair: Pair<Rule>) -> Class {
    let mut inner = pair.into_inner();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::CLASS);

    let name_pair = inner.next().unwrap();
    assert_eq!(name_pair.as_rule(), Rule::ID);
    let name = name_pair.as_str().to_string();

    let next_pair = inner.next().unwrap();
    let base_class = if next_pair.as_rule() == Rule::class_inheritance {
        let base_name = next_pair
            .into_inner()
            .find(|p| p.as_rule() == Rule::ID)
            .map(|id| id.as_str().to_string());
        inner.next(); // skip COLON
        base_name
    } else {
        assert_eq!(next_pair.as_rule(), Rule::COLON);
        None
    };

    let class_body_pair = inner.next().unwrap();
    assert_eq!(class_body_pair.as_rule(), Rule::class_body);
    let members = build_class_body(class_body_pair);

    assert_eq!(inner.next(), None);

    Class {
        name,
        base_class,
        members,
    }
}

fn build_class_body(pair: Pair<Rule>) -> Vec<ClassMember> {
    let mut members = Vec::new();

    for member_pair in pair.into_inner() {
        assert_eq!(member_pair.as_rule(), Rule::class_member);
        let inner = member_pair.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::method_decl => members.push(ClassMember::Method(build_method(inner))),
            Rule::var_decl_stmt => {
                if let Statement::VarDecl {
                    name,
                    var_type,
                    value,
                } = build_var_decl(inner.into_inner().next().unwrap())
                {
                    members.push(ClassMember::Field {
                        name,
                        field_type: var_type,
                        value,
                    });
                }
            }
            Rule::pass_stmt => {} // Skip pass statements in class body
            _ => panic!("Unexpected rule in class member: {:?}", inner.as_rule()),
        }
    }

    members
}

fn build_method(pair: Pair<Rule>) -> Function {
    let mut inner = pair.into_inner();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::DEF);

    let id_stmt = inner.next().unwrap();
    assert_eq!(id_stmt.as_rule(), Rule::ID);
    let name = id_stmt.as_str().to_string();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::LPAREN);

    let param_or_rparen_pair = inner.next().unwrap();
    let params = if param_or_rparen_pair.as_rule() == Rule::method_param_list {
        assert_eq!(inner.next().unwrap().as_rule(), Rule::RPAREN);
        build_method_param_list(param_or_rparen_pair)
    } else {
        assert_eq!(param_or_rparen_pair.as_rule(), Rule::RPAREN);
        vec![]
    };

    // Optional return type
    let next_pair = inner.next().unwrap();
    let return_type = if next_pair.as_rule() == Rule::method_return_type {
        let type_pair = next_pair.into_inner().nth(1).unwrap(); // Skip ARROW
        assert_eq!(type_pair.as_rule(), Rule::type_spec);
        let ret_type = build_type(type_pair);
        inner.next(); // skip COLON
        ret_type
    } else {
        assert_eq!(next_pair.as_rule(), Rule::COLON);
        Type::None
    };

    let block_pair = inner.next().unwrap();
    assert_eq!(block_pair.as_rule(), Rule::block);
    let body = build_block(block_pair);

    assert_eq!(inner.next(), None);

    Function {
        name,
        params,
        return_type,
        body,
    }
}

fn build_method_param_list(pair: Pair<Rule>) -> Vec<Parameter> {
    pair.into_inner()
        .filter(|p| p.as_rule() != Rule::COMMA)
        .map(build_method_parameter)
        .collect()
}

fn build_method_parameter(pair: Pair<Rule>) -> Parameter {
    let mut inner = pair.into_inner();

    let id_pair = inner.next().unwrap();
    assert_eq!(id_pair.as_rule(), Rule::ID);
    let name = id_pair.as_str().to_string();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::COLON);

    let type_pair = inner.next().unwrap();
    assert_eq!(type_pair.as_rule(), Rule::type_spec);
    let param_type = build_type(type_pair);

    assert_eq!(inner.next(), None);

    Parameter { name, param_type }
}

fn build_function(pair: Pair<Rule>) -> Function {
    let mut inner = pair.into_inner();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::DEF);

    let id_stmt = inner.next().unwrap();
    assert_eq!(id_stmt.as_rule(), Rule::ID);
    let name = id_stmt.as_str().to_string();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::LPAREN);

    let param_or_rparen_pair = inner.next().unwrap();
    let params = if param_or_rparen_pair.as_rule() == Rule::param_list {
        assert_eq!(inner.next().unwrap().as_rule(), Rule::RPAREN);
        build_param_list(param_or_rparen_pair)
    } else {
        assert_eq!(param_or_rparen_pair.as_rule(), Rule::RPAREN);
        vec![]
    };

    assert_eq!(inner.next().unwrap().as_rule(), Rule::ARROW);

    let return_pair = inner.next().unwrap();
    assert_eq!(return_pair.as_rule(), Rule::type_spec);
    let return_type = build_type(return_pair);

    assert_eq!(inner.next().unwrap().as_rule(), Rule::COLON);

    let block_pair = inner.next().unwrap();
    assert_eq!(block_pair.as_rule(), Rule::block);
    let body = build_block(block_pair);

    assert_eq!(inner.next(), None);

    Function {
        name,
        params,
        return_type,
        body,
    }
}

fn build_param_list(pair: Pair<Rule>) -> Vec<Parameter> {
    pair.into_inner()
        .filter(|p| p.as_rule() != Rule::COMMA)
        .map(build_parameter)
        .collect()
}

fn build_parameter(pair: Pair<Rule>) -> Parameter {
    let mut inner = pair.into_inner();

    let id_pair = inner.next().unwrap();
    assert_eq!(id_pair.as_rule(), Rule::ID);
    let name = id_pair.as_str().to_string();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::COLON);

    let type_pair = inner.next().unwrap();
    assert_eq!(type_pair.as_rule(), Rule::type_spec);
    let param_type = build_type(type_pair);

    assert_eq!(inner.next(), None);

    Parameter { name, param_type }
}

fn build_type(pair: Pair<Rule>) -> Type {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::INT_TYPE => Type::Int,
        Rule::FLOAT_TYPE => Type::Float,
        Rule::BOOL_TYPE => Type::Bool,
        Rule::STR_TYPE => Type::Str,
        Rule::BYTES_TYPE => Type::Bytes,
        Rule::NONE_TYPE => Type::None,
        _ => panic!("Unexpected rule in type spec: {:?}", inner.as_rule()),
    }
}

fn build_block(pair: Pair<Rule>) -> Vec<Statement> {
    let mut stmts = Vec::new();

    for stmt_pair in pair.into_inner() {
        assert_eq!(stmt_pair.as_rule(), Rule::statement);
        stmts.push(build_statement(stmt_pair));
    }

    stmts
}

fn build_statement(pair: Pair<Rule>) -> Statement {
    if let Some(inner) = pair.into_inner().next() {
        match inner.as_rule() {
            Rule::var_decl_stmt => build_var_decl(inner.into_inner().next().unwrap()),
            Rule::assignment_stmt => build_assignment(inner.into_inner().next().unwrap()),
            Rule::aug_assignment_stmt => build_aug_assignment(inner.into_inner().next().unwrap()),
            Rule::if_stmt => build_if_stmt(inner),
            Rule::while_stmt => build_while_stmt(inner),
            Rule::for_stmt => build_for_stmt(inner),
            Rule::return_stmt => build_return_stmt(inner),
            Rule::break_stmt => Statement::Break,
            Rule::continue_stmt => Statement::Continue,
            Rule::pass_stmt => Statement::Pass,
            Rule::expr_stmt => {
                Statement::Expr(build_expression(inner.into_inner().next().unwrap()))
            }
            _ => panic!("Unexpected rule in statement: {:?}", inner.as_rule()),
        }
    } else {
        Statement::Pass
    }
}

fn build_var_decl(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();

    let target_pair = inner.next().unwrap();
    assert_eq!(target_pair.as_rule(), Rule::target);

    // For var_decl, target must be a simple ID
    let name = {
        let mut target_inner = target_pair.into_inner();
        if let Some(id_pair) = target_inner.next() {
            // Could be subscript_expr, attribute_expr, or ID
            match id_pair.as_rule() {
                Rule::ID => id_pair.as_str().to_string(),
                Rule::subscript_expr | Rule::attribute_expr => {
                    panic!("Variable declaration target must be a simple identifier, not a subscript or attribute");
                }
                _ => panic!(
                    "Unexpected rule in var_decl target: {:?}",
                    id_pair.as_rule()
                ),
            }
        } else {
            panic!("Expected ID in var_decl target");
        }
    };

    assert_eq!(inner.next().unwrap().as_rule(), Rule::COLON);

    let type_pair = inner.next().unwrap();
    assert_eq!(type_pair.as_rule(), Rule::type_spec);
    let var_type = build_type(type_pair);

    assert_eq!(inner.next().unwrap().as_rule(), Rule::ASSIGN);

    let expr_pair = inner.next().unwrap();
    assert_eq!(expr_pair.as_rule(), Rule::expression);
    let value = build_expression(expr_pair);

    assert_eq!(inner.next(), None);

    Statement::VarDecl {
        name,
        var_type,
        value,
    }
}

fn build_assignment(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();

    let target_pair = inner.next().unwrap();
    assert_eq!(target_pair.as_rule(), Rule::target);
    let target = build_assign_target(target_pair);

    assert_eq!(inner.next().unwrap().as_rule(), Rule::ASSIGN);

    let expr_pair = inner.next().unwrap();
    assert_eq!(expr_pair.as_rule(), Rule::expression);
    let value = build_expression(expr_pair);

    assert_eq!(inner.next(), None);

    Statement::Assignment { target, value }
}

fn build_aug_assignment(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();

    let target_pair = inner.next().unwrap();
    assert_eq!(target_pair.as_rule(), Rule::target);
    let target = build_assign_target(target_pair);

    let aug_op_pair = inner.next().unwrap();
    assert_eq!(aug_op_pair.as_rule(), Rule::aug_op);
    let op = build_aug_op(aug_op_pair);

    let expr_pair = inner.next().unwrap();
    assert_eq!(expr_pair.as_rule(), Rule::expression);
    let value = build_expression(expr_pair);

    assert_eq!(inner.next(), None);

    Statement::AugAssignment { target, op, value }
}

fn build_assign_target(pair: Pair<Rule>) -> AssignTarget {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::ID => AssignTarget::Var(inner.as_str().to_string()),
        Rule::subscript_expr | Rule::attribute_expr | Rule::postfix_expr => {
            // These are expressions, need to convert to AssignTarget
            build_assign_target_from_expr(build_expression(inner))
        }
        _ => panic!("Unexpected rule in assign target: {:?}", inner.as_rule()),
    }
}

fn build_assign_target_from_expr(expr: Expression) -> AssignTarget {
    match expr {
        Expression::Var(name) => AssignTarget::Var(name),
        Expression::Attribute { object, attr } => AssignTarget::Attribute { object, attr },
        Expression::Subscript { object, index } => AssignTarget::Subscript { object, index },
        _ => panic!("Invalid assignment target: {:?}", expr),
    }
}

fn build_aug_op(pair: Pair<Rule>) -> AugAssignOp {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::PLUS_ASSIGN => AugAssignOp::Add,
        Rule::MINUS_ASSIGN => AugAssignOp::Sub,
        Rule::STAR_ASSIGN => AugAssignOp::Mul,
        Rule::SLASH_ASSIGN => AugAssignOp::Div,
        Rule::DSLASH_ASSIGN => AugAssignOp::FloorDiv,
        Rule::PERCENT_ASSIGN => AugAssignOp::Mod,
        Rule::POW_ASSIGN => AugAssignOp::Pow,
        Rule::BITOR_ASSIGN => AugAssignOp::BitOr,
        Rule::BITXOR_ASSIGN => AugAssignOp::BitXor,
        Rule::BITAND_ASSIGN => AugAssignOp::BitAnd,
        Rule::LSHIFT_ASSIGN => AugAssignOp::LShift,
        Rule::RSHIFT_ASSIGN => AugAssignOp::RShift,
        _ => panic!(
            "Unexpected augmented assignment operator: {:?}",
            inner.as_rule()
        ),
    }
}

fn build_for_stmt(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::FOR);

    let var_pair = inner.next().unwrap();
    assert_eq!(var_pair.as_rule(), Rule::ID);
    let var = var_pair.as_str().to_string();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::IN);

    let iterable_pair = inner.next().unwrap();
    assert_eq!(iterable_pair.as_rule(), Rule::expression);
    let iterable = build_expression(iterable_pair);

    assert_eq!(inner.next().unwrap().as_rule(), Rule::COLON);

    let block_pair = inner.next().unwrap();
    assert_eq!(block_pair.as_rule(), Rule::block);
    let body = build_block(block_pair);

    assert_eq!(inner.next(), None);

    Statement::For {
        var,
        iterable,
        body,
    }
}

fn build_if_stmt(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::IF);

    let expr_pair = inner.next().unwrap();
    assert_eq!(expr_pair.as_rule(), Rule::expression);
    let condition = build_expression(expr_pair);

    assert_eq!(inner.next().unwrap().as_rule(), Rule::COLON);

    let block_pair = inner.next().unwrap();
    assert_eq!(block_pair.as_rule(), Rule::block);
    let then_block = build_block(block_pair);

    let mut elif_clauses = Vec::new();
    let mut else_block = None;

    for clause in inner {
        match clause.as_rule() {
            Rule::elif_clause => {
                elif_clauses.push(build_elif_clause(clause));
            }
            Rule::else_clause => {
                else_block = Some(build_else_clause(clause));
            }
            _ => panic!("Unexpected rule in if stmt: {:?}", clause.as_rule()),
        }
    }

    Statement::If {
        condition,
        then_block,
        elif_clauses,
        else_block,
    }
}

fn build_elif_clause(pair: Pair<Rule>) -> (Expression, Vec<Statement>) {
    let mut inner = pair.into_inner();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::ELIF);

    let expr_pair = inner.next().unwrap();
    assert_eq!(expr_pair.as_rule(), Rule::expression);
    let condition = build_expression(expr_pair);

    assert_eq!(inner.next().unwrap().as_rule(), Rule::COLON);

    let block_pair = inner.next().unwrap();
    assert_eq!(block_pair.as_rule(), Rule::block);
    let body = build_block(block_pair);

    assert_eq!(inner.next(), None);

    (condition, body)
}

fn build_else_clause(pair: Pair<Rule>) -> Vec<Statement> {
    let mut inner = pair.into_inner();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::ELSE);
    assert_eq!(inner.next().unwrap().as_rule(), Rule::COLON);

    let block_pair = inner.next().unwrap();
    assert_eq!(block_pair.as_rule(), Rule::block);
    let body = build_block(block_pair);

    assert_eq!(inner.next(), None);

    body
}

fn build_while_stmt(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::WHILE);

    let cond_stmt = inner.next().unwrap();
    assert_eq!(cond_stmt.as_rule(), Rule::expression);
    let condition = build_expression(cond_stmt);

    assert_eq!(inner.next().unwrap().as_rule(), Rule::COLON);

    let block_pair = inner.next().unwrap();
    assert_eq!(block_pair.as_rule(), Rule::block);
    let body = build_block(block_pair);

    assert_eq!(inner.next(), None);

    Statement::While { condition, body }
}

fn build_return_stmt(pair: Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();

    let return_inner_pair = inner.next().unwrap();
    assert_eq!(return_inner_pair.as_rule(), Rule::return_inner);

    assert_eq!(inner.next(), None);

    let mut return_inner = return_inner_pair.into_inner();
    assert_eq!(return_inner.next().unwrap().as_rule(), Rule::RETURN);

    let return_value = return_inner.next().map(|expr_pair| {
        assert_eq!(expr_pair.as_rule(), Rule::expression);
        build_expression(expr_pair)
    });

    assert_eq!(return_inner.next(), None);

    Statement::Return(return_value)
}

fn build_expression(pair: Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::expression => {
            let inner = pair.into_inner().next().unwrap();
            build_expression(inner)
        }
        Rule::logic_or_expr => build_binary_chain(pair, BinaryOp::Or),
        Rule::logic_and_expr => build_binary_chain(pair, BinaryOp::And),
        Rule::logic_not_expr => build_logic_not_expr(pair),
        Rule::comparison_expr => build_comparison_expr(pair),
        Rule::bitwise_or_expr => build_binary_chain(pair, BinaryOp::BitOr),
        Rule::bitwise_xor_expr => build_binary_chain(pair, BinaryOp::BitXor),
        Rule::bitwise_and_expr => build_binary_chain(pair, BinaryOp::BitAnd),
        Rule::shift_expr => build_shift_expr(pair),
        Rule::arith_expr => build_arith_expr(pair),
        Rule::term_expr => build_term_expr(pair),
        Rule::factor_expr => build_factor_expr(pair),
        Rule::power_expr => build_power_expr(pair),
        Rule::postfix_expr => build_postfix_expr(pair),
        Rule::primary => {
            let inner = pair.into_inner().next().unwrap();
            build_expression(inner)
        }
        Rule::atom => build_atom(pair),
        _ => panic!("Unexpected rule in expression: {:?}", pair.as_rule()),
    }
}

fn build_logic_not_expr(pair: Pair<Rule>) -> Expression {
    let inner_items: Vec<_> = pair.into_inner().collect();

    // Count NOT operators at the beginning
    let not_count = inner_items
        .iter()
        .take_while(|p| p.as_rule() == Rule::NOT)
        .count();

    // The last item is the comparison_expr
    let mut expr = build_expression(inner_items.last().unwrap().clone());

    // Apply NOT operations from right to left
    for _ in 0..not_count {
        expr = Expression::UnaryOp {
            op: UnaryOp::Not,
            operand: Box::new(expr),
        };
    }

    expr
}

fn build_comparison_expr(pair: Pair<Rule>) -> Expression {
    let mut items = Vec::new();
    let mut current_op = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::comp_op => {
                current_op = Some(build_comp_op(inner));
            }
            _ => items.push((current_op.clone(), build_expression(inner))),
        }
    }

    build_binop_chain(items)
}

fn build_comp_op(pair: Pair<Rule>) -> BinaryOp {
    let inner: Vec<_> = pair.into_inner().collect();

    if inner.is_empty() {
        panic!("Empty comp_op");
    }

    // Handle multi-token operators like "is not", "not in"
    if inner.len() == 2 {
        match (inner[0].as_rule(), inner[1].as_rule()) {
            (Rule::IS, Rule::NOT) => BinaryOp::IsNot,
            (Rule::NOT, Rule::IN) => BinaryOp::NotIn,
            _ => panic!(
                "Unexpected comp_op combination: {:?} {:?}",
                inner[0].as_rule(),
                inner[1].as_rule()
            ),
        }
    } else {
        match inner[0].as_rule() {
            Rule::EQ => BinaryOp::Eq,
            Rule::NE => BinaryOp::Ne,
            Rule::LE => BinaryOp::Le,
            Rule::GE => BinaryOp::Ge,
            Rule::LT => BinaryOp::Lt,
            Rule::GT => BinaryOp::Gt,
            Rule::IN => BinaryOp::In,
            Rule::IS => BinaryOp::Is,
            _ => panic!("Unexpected comp_op: {:?}", inner[0].as_rule()),
        }
    }
}

fn build_shift_expr(pair: Pair<Rule>) -> Expression {
    let mut items = Vec::new();
    let mut current_op = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::LSHIFT => current_op = Some(BinaryOp::LShift),
            Rule::RSHIFT => current_op = Some(BinaryOp::RShift),
            _ => items.push((current_op.clone(), build_expression(inner))),
        }
    }

    build_binop_chain(items)
}

fn build_arith_expr(pair: Pair<Rule>) -> Expression {
    let mut items = Vec::new();
    let mut current_op = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::PLUS => current_op = Some(BinaryOp::Add),
            Rule::MINUS => current_op = Some(BinaryOp::Sub),
            _ => items.push((current_op.clone(), build_expression(inner))),
        }
    }

    build_binop_chain(items)
}

fn build_term_expr(pair: Pair<Rule>) -> Expression {
    let mut items = Vec::new();
    let mut current_op = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::STAR => current_op = Some(BinaryOp::Mul),
            Rule::SLASH => current_op = Some(BinaryOp::Div),
            Rule::DSLASH => current_op = Some(BinaryOp::FloorDiv),
            Rule::PERCENT => current_op = Some(BinaryOp::Mod),
            _ => items.push((current_op.clone(), build_expression(inner))),
        }
    }

    build_binop_chain(items)
}

fn build_factor_expr(pair: Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();

    let first_pair = inner.next().unwrap();

    // Check if this is a unary operator or just a power_expr
    match first_pair.as_rule() {
        Rule::unary_op => {
            let op = build_unary_op(first_pair);
            let operand_pair = inner.next().unwrap();
            let operand = build_expression(operand_pair);

            assert_eq!(inner.next(), None);

            Expression::UnaryOp {
                op,
                operand: Box::new(operand),
            }
        }
        Rule::power_expr => {
            assert_eq!(inner.next(), None);
            build_expression(first_pair)
        }
        _ => panic!("Unexpected rule in factor expr: {:?}", first_pair.as_rule()),
    }
}

fn build_unary_op(pair: Pair<Rule>) -> UnaryOp {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::PLUS => UnaryOp::Pos,
        Rule::MINUS => UnaryOp::Neg,
        Rule::BITNOT => UnaryOp::BitNot,
        _ => panic!("Unexpected unary operator: {:?}", inner.as_rule()),
    }
}

fn build_power_expr(pair: Pair<Rule>) -> Expression {
    let items: Vec<_> = pair.into_inner().collect();

    if items.len() == 1 {
        // No power operator
        return build_expression(items[0].clone());
    }

    // Power is right-associative: 2**3**2 = 2**(3**2)
    // items[0] is postfix_expr, items[1] is POW, items[2] is factor_expr
    let base = build_expression(items[0].clone());
    // Skip POW token at items[1]
    let exponent = build_expression(items[2].clone());

    Expression::BinOp {
        op: BinaryOp::Pow,
        left: Box::new(base),
        right: Box::new(exponent),
    }
}

fn build_postfix_expr(pair: Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();

    let atom_pair = inner.next().unwrap();
    assert_eq!(atom_pair.as_rule(), Rule::atom);
    let mut expr = build_atom(atom_pair);

    // Process postfix operations
    for trailer in inner {
        assert_eq!(trailer.as_rule(), Rule::postfix_trailer);
        expr = build_postfix_trailer(expr, trailer);
    }

    expr
}

fn build_postfix_trailer(base: Expression, pair: Pair<Rule>) -> Expression {
    let items: Vec<_> = pair.into_inner().collect();

    if items.is_empty() {
        return base;
    }

    // Determine the type of postfix operation
    match items[0].as_rule() {
        Rule::DOT => {
            // Attribute access: base.attr
            let attr_pair = items.iter().find(|p| p.as_rule() == Rule::ID).unwrap();
            Expression::Attribute {
                object: Box::new(base),
                attr: attr_pair.as_str().to_string(),
            }
        }
        Rule::LBRACKET => {
            // Subscript: base[index] or base[slice]
            let index_or_slice = items
                .iter()
                .find(|p| p.as_rule() == Rule::expression || p.as_rule() == Rule::slice_expr)
                .unwrap();

            if index_or_slice.as_rule() == Rule::slice_expr {
                let slice = build_slice_expr(index_or_slice.clone());
                Expression::Subscript {
                    object: Box::new(base),
                    index: Box::new(slice),
                }
            } else {
                Expression::Subscript {
                    object: Box::new(base),
                    index: Box::new(build_expression(index_or_slice.clone())),
                }
            }
        }
        Rule::LPAREN => {
            // Function call: base(args)
            let args = items
                .iter()
                .find(|p| p.as_rule() == Rule::arg_list)
                .map(|arg_list| build_arg_list(arg_list.clone()))
                .unwrap_or_else(Vec::new);

            Expression::Call {
                func: Box::new(base),
                args,
            }
        }
        _ => panic!("Unexpected postfix trailer: {:?}", items[0].as_rule()),
    }
}

fn build_slice_expr(pair: Pair<Rule>) -> Expression {
    let items: Vec<_> = pair.into_inner().collect();

    let mut parts = [None, None, None]; // start, stop, step
    let mut part_idx = 0;

    for item in items {
        if item.as_rule() == Rule::COLON {
            part_idx += 1;
        } else if item.as_rule() == Rule::expression {
            parts[part_idx] = Some(Box::new(build_expression(item)));
        }
    }

    Expression::Slice {
        start: parts[0].clone(),
        stop: parts[1].clone(),
        step: parts[2].clone(),
    }
}

fn build_binary_chain(pair: Pair<Rule>, op: BinaryOp) -> Expression {
    let items: Vec<Expression> = pair
        .into_inner()
        .filter(|inner| {
            !matches!(
                inner.as_rule(),
                Rule::AND | Rule::OR | Rule::BITOR | Rule::BITXOR | Rule::BITAND
            )
        })
        .map(build_expression)
        .collect();

    if items.len() < 2 {
        return items.into_iter().next().unwrap_or(Expression::IntLit(0));
    }

    let mut iter = items.into_iter();
    let mut result = iter.next().unwrap();

    for item in iter {
        result = Expression::BinOp {
            op: op.clone(),
            left: Box::new(result),
            right: Box::new(item),
        };
    }

    result
}

fn build_binop_chain(items: Vec<(Option<BinaryOp>, Expression)>) -> Expression {
    if items.is_empty() {
        return Expression::IntLit(0);
    }

    let mut iter = items.into_iter();
    let mut result = iter.next().unwrap().1;

    for (op_opt, expr) in iter {
        if let Some(op) = op_opt {
            result = Expression::BinOp {
                op,
                left: Box::new(result),
                right: Box::new(expr),
            };
        }
    }

    result
}

fn build_arg_list(pair: Pair<Rule>) -> Vec<Expression> {
    pair.into_inner()
        .filter(|p| p.as_rule() != Rule::COMMA)
        .map(build_expression)
        .collect()
}

fn build_atom(pair: Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();

    match inner.next() {
        Some(first) => match first.as_rule() {
            Rule::NONE => {
                assert_eq!(inner.next(), None);
                Expression::NoneLit
            }
            Rule::TRUE => {
                assert_eq!(inner.next(), None);
                Expression::BoolLit(true)
            }
            Rule::FALSE => {
                assert_eq!(inner.next(), None);
                Expression::BoolLit(false)
            }
            Rule::INT_LIT => {
                assert_eq!(inner.next(), None);
                Expression::IntLit(first.as_str().parse().unwrap_or(0))
            }
            Rule::BIN_LIT => {
                assert_eq!(inner.next(), None);
                // Parse binary literal like 0b1010 (skip "0b" prefix)
                let s = first.as_str();
                Expression::IntLit(i64::from_str_radix(&s[2..], 2).unwrap_or(0))
            }
            Rule::OCT_LIT => {
                assert_eq!(inner.next(), None);
                // Parse octal literal like 0o755 (skip "0o" prefix)
                let s = first.as_str();
                Expression::IntLit(i64::from_str_radix(&s[2..], 8).unwrap_or(0))
            }
            Rule::HEX_LIT => {
                assert_eq!(inner.next(), None);
                // Parse hex literal like 0xFF (skip "0x" prefix)
                let s = first.as_str();
                Expression::IntLit(i64::from_str_radix(&s[2..], 16).unwrap_or(0))
            }
            Rule::FLOAT_LIT => {
                assert_eq!(inner.next(), None);
                Expression::FloatLit(first.as_str().parse().unwrap_or(0.0))
            }
            Rule::STR_LIT => {
                assert_eq!(inner.next(), None);
                let s = first.as_str();
                // Strip quotes and process escape sequences
                let content = &s[1..s.len() - 1];
                Expression::StrLit(process_escape_sequences(content))
            }
            Rule::BYTES_LIT => {
                assert_eq!(inner.next(), None);
                let s = first.as_str();
                // Strip b"..." to get content (skip 'b' and quotes)
                let content = &s[2..s.len() - 1];
                Expression::BytesLit(process_escape_sequences(content))
            }
            Rule::list_literal => {
                assert_eq!(inner.next(), None);
                build_list_literal(first)
            }
            Rule::tuple_literal => {
                assert_eq!(inner.next(), None);
                build_tuple_literal(first)
            }
            Rule::dict_literal => {
                assert_eq!(inner.next(), None);
                build_dict_literal(first)
            }
            Rule::set_literal => {
                assert_eq!(inner.next(), None);
                build_set_literal(first)
            }
            Rule::ID => {
                assert_eq!(inner.next(), None);
                Expression::Var(first.as_str().to_string())
            }
            Rule::expression => {
                assert_eq!(inner.next(), None);
                build_expression(first)
            }
            Rule::LPAREN => {
                let expr_pair = inner.next().unwrap();
                assert_eq!(expr_pair.as_rule(), Rule::expression);
                let expr = build_expression(expr_pair);
                assert_eq!(inner.next().unwrap().as_rule(), Rule::RPAREN);
                assert_eq!(inner.next(), None);
                expr
            }
            _ => panic!("Unexpected rule in atom: {:?}", first.as_rule()),
        },
        None => Expression::NoneLit,
    }
}

fn build_list_literal(pair: Pair<Rule>) -> Expression {
    let elements: Vec<Expression> = pair
        .into_inner()
        .filter(|p| !matches!(p.as_rule(), Rule::LBRACKET | Rule::RBRACKET | Rule::COMMA))
        .map(build_expression)
        .collect();

    Expression::List(elements)
}

fn build_tuple_literal(pair: Pair<Rule>) -> Expression {
    let elements: Vec<Expression> = pair
        .into_inner()
        .filter(|p| !matches!(p.as_rule(), Rule::LPAREN | Rule::RPAREN | Rule::COMMA))
        .map(build_expression)
        .collect();

    Expression::Tuple(elements)
}

fn build_dict_literal(pair: Pair<Rule>) -> Expression {
    let pairs: Vec<(Expression, Expression)> = pair
        .into_inner()
        .filter(|p| p.as_rule() == Rule::dict_pair)
        .map(|dict_pair| {
            let mut items = dict_pair.into_inner();
            let key = build_expression(items.next().unwrap());
            // Skip COLON
            let value = build_expression(items.next().unwrap());
            (key, value)
        })
        .collect();

    Expression::Dict(pairs)
}

fn build_set_literal(pair: Pair<Rule>) -> Expression {
    let elements: Vec<Expression> = pair
        .into_inner()
        .filter(|p| !matches!(p.as_rule(), Rule::LBRACE | Rule::RBRACE | Rule::COMMA))
        .map(build_expression)
        .collect();

    Expression::Set(elements)
}

/// Process escape sequences in a string literal
fn process_escape_sequences(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            // Process escape sequence
            if let Some(next_ch) = chars.next() {
                match next_ch {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    '\'' => result.push('\''),
                    _ => {
                        // Unknown escape sequence, keep as-is
                        result.push('\\');
                        result.push(next_ch);
                    }
                }
            } else {
                // Backslash at end of string
                result.push('\\');
            }
        } else {
            result.push(ch);
        }
    }

    result
}
