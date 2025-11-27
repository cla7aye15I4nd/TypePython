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
    let mut name = String::new();
    let mut params = Vec::new();
    let mut return_type = Type::None;
    let mut body = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ID => {
                if name.is_empty() {
                    name = inner.as_str().to_string();
                }
            }
            Rule::param_list => params = build_param_list(inner),
            Rule::type_spec => return_type = build_type(inner),
            Rule::block => body = build_block(inner),
            Rule::DEF | Rule::LPAREN | Rule::RPAREN | Rule::ARROW | Rule::COLON => {}
            _ => panic!("Unexpected rule in function: {:?}", inner.as_rule()),
        }
    }

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
    let mut name = String::new();
    let mut param_type = Type::None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ID => name = inner.as_str().to_string(),
            Rule::type_spec => param_type = build_type(inner),
            Rule::COLON => {}
            _ => panic!("Unexpected rule in parameter: {:?}", inner.as_rule()),
        }
    }

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
    pair.into_inner()
        .filter_map(|p| {
            if p.as_rule() == Rule::indented_statement {
                let mut inner = p.into_inner();
                let _indent = inner.next().unwrap();
                let stmt_pair = inner.next().unwrap();
                Some(build_statement(stmt_pair))
            } else {
                None
            }
        })
        .collect()
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
    let mut name = String::new();
    let mut var_type = Type::None;
    let mut value = Expression::IntLit(0);

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ID => name = inner.as_str().to_string(),
            Rule::type_spec => var_type = build_type(inner),
            Rule::expression => value = build_expression(inner),
            Rule::COLON | Rule::ASSIGN => {}
            _ => panic!("Unexpected rule in var decl: {:?}", inner.as_rule()),
        }
    }

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
    let mut elif_clauses = Vec::new();
    let mut else_block = None;

    let mut iter = pair.into_inner();

    assert_eq!(iter.next().unwrap().as_rule(), Rule::IF);

    // First expression is the if condition
    let expr_pair = iter.next().unwrap();
    assert_eq!(expr_pair.as_rule(), Rule::expression);
    let condition = build_expression(expr_pair);

    assert_eq!(iter.next().unwrap().as_rule(), Rule::COLON);

    let block_pair = iter.next().unwrap();
    assert_eq!(block_pair.as_rule(), Rule::block);
    let then_block = build_block(block_pair);

    // Handle elif and else clauses
    for inner in iter {
        match inner.as_rule() {
            Rule::elif_clause => {
                let mut elif_cond = Expression::BoolLit(true);
                let mut elif_body = Vec::new();

                for elif_inner in inner.into_inner() {
                    match elif_inner.as_rule() {
                        Rule::expression => elif_cond = build_expression(elif_inner),
                        Rule::block => elif_body = build_block(elif_inner),
                        _ => panic!("Unexpected rule in elif clause: {:?}", elif_inner.as_rule()),
                    }
                }
                elif_clauses.push((elif_cond, elif_body));
            }
            Rule::else_clause => {
                for else_inner in inner.into_inner() {
                    if else_inner.as_rule() == Rule::block {
                        else_block = Some(build_block(else_inner));
                    }
                }
            }
            _ => panic!("Unexpected rule in if stmt: {:?}", inner.as_rule()),
        }
    }

    Statement::If {
        condition,
        then_block,
        elif_clauses,
        else_block,
    }
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
    let mut return_value = None;

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::return_inner {
            for ret_inner in inner.into_inner() {
                if ret_inner.as_rule() == Rule::expression {
                    return_value = Some(build_expression(ret_inner));
                }
            }
        }
    }

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
    let mut items: Vec<Expression> = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::AND | Rule::OR => {}
            _ => items.push(build_expression(inner)),
        }
    }

    if items.len() < 2 {
        return items.into_iter().next().unwrap_or(Expression::IntLit(0));
    }

    let mut result = items[0].clone();
    for item in items.into_iter().skip(1) {
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

    let mut result = items[0].1.clone();

    for (op_opt, expr) in items.into_iter().skip(1) {
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
    let mut op = UnaryOp::Neg;
    let mut operand = Expression::IntLit(0);

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::MINUS => op = UnaryOp::Neg,
            Rule::NOT => op = UnaryOp::Not,
            Rule::expression => operand = build_expression(inner),
            Rule::call_expr => operand = build_call_expr(inner),
            _ => panic!("Unexpected rule in unary expr: {:?}", inner.as_rule()),
        }
    }

    Expression::UnaryOp {
        op,
        operand: Box::new(operand),
    }
}

fn build_call_expr(pair: Pair<Rule>) -> Expression {
    let mut name = String::new();
    let mut args = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::atom => {
                // The function name comes from the atom
                if let Expression::Var(var_name) = build_atom(inner) {
                    name = var_name;
                }
            }
            Rule::arg_list => {
                args = inner
                    .into_inner()
                    .filter(|p| p.as_rule() != Rule::COMMA)
                    .map(build_expression)
                    .collect();
            }
            Rule::LPAREN | Rule::RPAREN => {}
            _ => panic!("Unexpected rule in call expr: {:?}", inner.as_rule()),
        }
    }

    Expression::Call { name, args }
}

fn build_atom(pair: Pair<Rule>) -> Expression {
    let mut inner_iter = pair.into_inner();
    match inner_iter.next() {
        Some(inner) => match inner.as_rule() {
            Rule::INT_LIT => Expression::IntLit(inner.as_str().parse().unwrap_or(0)),
            Rule::FLOAT_LIT => Expression::FloatLit(inner.as_str().parse().unwrap_or(0.0)),
            Rule::STR_LIT => {
                let s = inner.as_str();
                Expression::StrLit(s[1..s.len() - 1].to_string())
            }
            Rule::TRUE => Expression::BoolLit(true),
            Rule::FALSE => Expression::BoolLit(false),
            Rule::ID => Expression::Var(inner.as_str().to_string()),
            Rule::expression => build_expression(inner),
            Rule::LPAREN => {
                // Handle parenthesized expressions: (expr)
                // inner_iter already advanced past LPAREN, so next should be the expression
                let expr_pair = inner_iter.next().unwrap();
                build_expression(expr_pair)
            }
            _ => panic!("Unexpected rule in atom: {:?}", inner.as_rule()),
        },
        None => Expression::NoneLit(),
    }
}
