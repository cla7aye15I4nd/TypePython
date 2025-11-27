use crate::ast::*;
use crate::Rule;
use pest::iterators::{Pair, Pairs};

pub fn build_program(mut pairs: Pairs<Rule>) -> Program {
    let mut functions = Vec::new();
    let mut statements = Vec::new();

    // The program rule wraps everything, so we need to get its inner pairs
    if let Some(program_pair) = pairs.next() {
        if program_pair.as_rule() == Rule::program {
            for pair in program_pair.into_inner() {
                match pair.as_rule() {
                    Rule::func_decl => functions.push(build_function(pair)),
                    Rule::statement => statements.push(build_statement(pair)),
                    Rule::EOI | Rule::NEWLINE => {}
                    _ => panic!("Unexpected rule in program: {:?}", pair.as_rule()),
                }
            }
        }
    }

    Program {
        functions,
        statements,
    }
}

fn build_function(pair: Pair<Rule>) -> Function {
    let mut inner = pair.into_inner();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::DEF);

    let id_stmt = inner.next().unwrap();
    assert_eq!(id_stmt.as_rule(), Rule::ID);
    let name = id_stmt.as_str().to_string();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::LPAREN);

    let param_pair = inner.next().unwrap();
    assert_eq!(param_pair.as_rule(), Rule::param_list);
    let params = build_param_list(param_pair);

    assert_eq!(inner.next().unwrap().as_rule(), Rule::RPAREN);
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
            Rule::if_stmt => build_if_stmt(inner),
            Rule::while_stmt => build_while_stmt(inner),
            Rule::return_stmt => build_return_stmt(inner),
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

    let id_pair = inner.next().unwrap();
    assert_eq!(id_pair.as_rule(), Rule::ID);
    let name = id_pair.as_str().to_string();

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

    let id = inner.next().unwrap();
    assert_eq!(id.as_rule(), Rule::ID);
    let name = id.as_str().to_string();

    assert_eq!(inner.next().unwrap().as_rule(), Rule::ASSIGN);

    let expr_pair = inner.next().unwrap();
    assert_eq!(expr_pair.as_rule(), Rule::expression);
    let value = build_expression(expr_pair);

    assert_eq!(inner.next(), None);

    Statement::Assignment { name, value }
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
        Rule::equality_expr => build_equality_expr(pair),
        Rule::comparison_expr => build_comparison_expr(pair),
        Rule::term_expr => build_term_expr(pair),
        Rule::factor_expr => build_factor_expr(pair),
        Rule::unary_expr => build_unary_expr(pair),
        Rule::call_expr => build_call_expr(pair),
        Rule::atom => build_atom(pair),
        _ => panic!("Unexpected rule in expression: {:?}", pair.as_rule()),
    }
}

fn build_binary_chain(pair: Pair<Rule>, op: BinaryOp) -> Expression {
    let items: Vec<Expression> = pair
        .into_inner()
        .filter(|inner| !matches!(inner.as_rule(), Rule::AND | Rule::OR))
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

fn build_equality_expr(pair: Pair<Rule>) -> Expression {
    let mut items = Vec::new();
    let mut current_op = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::EQ => current_op = Some(BinaryOp::Eq),
            Rule::NE => current_op = Some(BinaryOp::Ne),
            _ => items.push((current_op.clone(), build_expression(inner))),
        }
    }

    build_binop_chain(items)
}

fn build_comparison_expr(pair: Pair<Rule>) -> Expression {
    let mut items = Vec::new();
    let mut current_op = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::LT => current_op = Some(BinaryOp::Lt),
            Rule::LE => current_op = Some(BinaryOp::Le),
            Rule::GT => current_op = Some(BinaryOp::Gt),
            Rule::GE => current_op = Some(BinaryOp::Ge),
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
            Rule::PLUS => current_op = Some(BinaryOp::Add),
            Rule::MINUS => current_op = Some(BinaryOp::Sub),
            _ => items.push((current_op.clone(), build_expression(inner))),
        }
    }

    build_binop_chain(items)
}

fn build_factor_expr(pair: Pair<Rule>) -> Expression {
    let mut items = Vec::new();
    let mut current_op = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::STAR => current_op = Some(BinaryOp::Mul),
            Rule::SLASH => current_op = Some(BinaryOp::Div),
            Rule::PERCENT => current_op = Some(BinaryOp::Mod),
            _ => items.push((current_op.clone(), build_expression(inner))),
        }
    }

    build_binop_chain(items)
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

fn build_unary_expr(pair: Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();

    let op_pair = inner.next().unwrap();
    let op = match op_pair.as_rule() {
        Rule::MINUS => UnaryOp::Neg,
        Rule::NOT => UnaryOp::Not,
        _ => panic!("Unexpected unary operator: {:?}", op_pair.as_rule()),
    };

    let operand_pair = inner.next().unwrap();
    let operand = match operand_pair.as_rule() {
        Rule::expression => build_expression(operand_pair),
        Rule::call_expr => build_call_expr(operand_pair),
        _ => panic!(
            "Unexpected rule in unary expr: {:?}",
            operand_pair.as_rule()
        ),
    };

    assert_eq!(inner.next(), None);

    Expression::UnaryOp {
        op,
        operand: Box::new(operand),
    }
}

fn build_call_expr(pair: Pair<Rule>) -> Expression {
    let mut inner = pair.into_inner();

    let atom_pair = inner.next().unwrap();
    assert_eq!(atom_pair.as_rule(), Rule::atom);
    let name = match build_atom(atom_pair) {
        Expression::Var(var_name) => var_name,
        _ => panic!("Expected identifier in function call"),
    };

    assert_eq!(inner.next().unwrap().as_rule(), Rule::LPAREN);

    let args = match inner.next() {
        Some(pair) if pair.as_rule() == Rule::arg_list => {
            let args = build_arg_list(pair);
            assert_eq!(inner.next().unwrap().as_rule(), Rule::RPAREN);
            args
        }
        Some(pair) if pair.as_rule() == Rule::RPAREN => Vec::new(),
        other => panic!("Unexpected in call_expr: {:?}", other.map(|p| p.as_rule())),
    };

    assert_eq!(inner.next(), None);

    Expression::Call { name, args }
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
            Rule::INT_LIT => {
                assert_eq!(inner.next(), None);
                Expression::IntLit(first.as_str().parse().unwrap_or(0))
            }
            Rule::FLOAT_LIT => {
                assert_eq!(inner.next(), None);
                Expression::FloatLit(first.as_str().parse().unwrap_or(0.0))
            }
            Rule::STR_LIT => {
                assert_eq!(inner.next(), None);
                let s = first.as_str();
                Expression::StrLit(s[1..s.len() - 1].to_string())
            }
            Rule::TRUE => {
                assert_eq!(inner.next(), None);
                Expression::BoolLit(true)
            }
            Rule::FALSE => {
                assert_eq!(inner.next(), None);
                Expression::BoolLit(false)
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
        None => Expression::NoneLit(),
    }
}
