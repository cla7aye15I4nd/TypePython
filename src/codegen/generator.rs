/// Generator support using LLVM coroutines
///
/// This module provides:
/// - Detection of generator functions (functions containing yield)
/// - Transformation of generator functions to LLVM coroutines
/// - Runtime support for generator iteration
use crate::ast::{Expression, Function, Statement};
use inkwell::context::Context;
use inkwell::intrinsics::Intrinsic;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::FunctionValue;

/// Check if a function is a generator (contains yield expression)
pub fn is_generator_function(func: &Function) -> bool {
    func.body.iter().any(contains_yield)
}

/// Check if a statement contains a yield expression
fn contains_yield(stmt: &Statement) -> bool {
    match stmt {
        Statement::Expr(expr) => expr_contains_yield(expr),
        Statement::Return(Some(expr)) => expr_contains_yield(expr),
        Statement::Return(None) => false,
        Statement::VarDecl { value, .. } => expr_contains_yield(value),
        Statement::Assignment { value, target } => {
            expr_contains_yield(value) || expr_contains_yield(target)
        }
        Statement::AugAssignment { value, target, .. } => {
            expr_contains_yield(value) || expr_contains_yield(target)
        }
        Statement::If {
            condition,
            then_block,
            elif_clauses,
            else_block,
        } => {
            expr_contains_yield(condition)
                || then_block.iter().any(contains_yield)
                || elif_clauses
                    .iter()
                    .any(|(c, b)| expr_contains_yield(c) || b.iter().any(contains_yield))
                || else_block
                    .as_ref()
                    .map(|b| b.iter().any(contains_yield))
                    .unwrap_or(false)
        }
        Statement::While { condition, body } => {
            expr_contains_yield(condition) || body.iter().any(contains_yield)
        }
        Statement::For { iter, body, .. } => {
            expr_contains_yield(iter) || body.iter().any(contains_yield)
        }
        Statement::Try {
            body,
            handlers,
            else_block,
            finally_block,
        } => {
            body.iter().any(contains_yield)
                || handlers.iter().any(|h| h.body.iter().any(contains_yield))
                || else_block
                    .as_ref()
                    .map(|b| b.iter().any(contains_yield))
                    .unwrap_or(false)
                || finally_block
                    .as_ref()
                    .map(|b| b.iter().any(contains_yield))
                    .unwrap_or(false)
        }
        Statement::Raise { exception, cause } => {
            exception.as_ref().map(expr_contains_yield).unwrap_or(false)
                || cause.as_ref().map(expr_contains_yield).unwrap_or(false)
        }
        Statement::Assert { test, msg } => {
            expr_contains_yield(test) || msg.as_ref().map(expr_contains_yield).unwrap_or(false)
        }
        Statement::Delete(expr) => expr_contains_yield(expr),
        Statement::Break | Statement::Continue | Statement::Pass => false,
        Statement::Global { .. } | Statement::Nonlocal { .. } => false,
    }
}

/// Check if an expression contains a yield
fn expr_contains_yield(expr: &Expression) -> bool {
    match expr {
        Expression::Yield { .. } => true,
        Expression::BinOp { left, right, .. } => {
            expr_contains_yield(left) || expr_contains_yield(right)
        }
        Expression::UnaryOp { operand, .. } => expr_contains_yield(operand),
        Expression::Call { func, args } => {
            expr_contains_yield(func) || args.iter().any(expr_contains_yield)
        }
        Expression::Attribute { object, .. } => expr_contains_yield(object),
        Expression::Subscript { object, index } => {
            expr_contains_yield(object) || expr_contains_yield(index)
        }
        Expression::List(elems) | Expression::Tuple(elems) | Expression::Set(elems) => {
            elems.iter().any(expr_contains_yield)
        }
        Expression::Dict(pairs) => pairs
            .iter()
            .any(|(k, v)| expr_contains_yield(k) || expr_contains_yield(v)),
        Expression::Slice { start, stop, step } => {
            start
                .as_ref()
                .map(|e| expr_contains_yield(e))
                .unwrap_or(false)
                || stop
                    .as_ref()
                    .map(|e| expr_contains_yield(e))
                    .unwrap_or(false)
                || step
                    .as_ref()
                    .map(|e| expr_contains_yield(e))
                    .unwrap_or(false)
        }
        Expression::IntLit(_)
        | Expression::FloatLit(_)
        | Expression::StrLit(_)
        | Expression::BytesLit(_)
        | Expression::BoolLit(_)
        | Expression::NoneLit
        | Expression::Var(_) => false,
    }
}

/// Count yield points in a function body for state machine generation
pub fn count_yield_points(func: &Function) -> usize {
    func.body.iter().map(count_yields_in_stmt).sum()
}

fn count_yields_in_stmt(stmt: &Statement) -> usize {
    match stmt {
        Statement::Expr(expr) => count_yields_in_expr(expr),
        Statement::Return(Some(expr)) => count_yields_in_expr(expr),
        Statement::Return(None) => 0,
        Statement::VarDecl { value, .. } => count_yields_in_expr(value),
        Statement::Assignment { value, target } => {
            count_yields_in_expr(value) + count_yields_in_expr(target)
        }
        Statement::AugAssignment { value, target, .. } => {
            count_yields_in_expr(value) + count_yields_in_expr(target)
        }
        Statement::If {
            condition,
            then_block,
            elif_clauses,
            else_block,
        } => {
            count_yields_in_expr(condition)
                + then_block.iter().map(count_yields_in_stmt).sum::<usize>()
                + elif_clauses
                    .iter()
                    .map(|(c, b)| {
                        count_yields_in_expr(c) + b.iter().map(count_yields_in_stmt).sum::<usize>()
                    })
                    .sum::<usize>()
                + else_block
                    .as_ref()
                    .map(|b| b.iter().map(count_yields_in_stmt).sum())
                    .unwrap_or(0)
        }
        Statement::While { condition, body } => {
            count_yields_in_expr(condition) + body.iter().map(count_yields_in_stmt).sum::<usize>()
        }
        Statement::For { iter, body, .. } => {
            count_yields_in_expr(iter) + body.iter().map(count_yields_in_stmt).sum::<usize>()
        }
        Statement::Try {
            body,
            handlers,
            else_block,
            finally_block,
        } => {
            body.iter().map(count_yields_in_stmt).sum::<usize>()
                + handlers
                    .iter()
                    .map(|h| h.body.iter().map(count_yields_in_stmt).sum::<usize>())
                    .sum::<usize>()
                + else_block
                    .as_ref()
                    .map(|b| b.iter().map(count_yields_in_stmt).sum())
                    .unwrap_or(0)
                + finally_block
                    .as_ref()
                    .map(|b| b.iter().map(count_yields_in_stmt).sum())
                    .unwrap_or(0)
        }
        Statement::Raise { exception, cause } => {
            exception.as_ref().map(count_yields_in_expr).unwrap_or(0)
                + cause.as_ref().map(count_yields_in_expr).unwrap_or(0)
        }
        Statement::Assert { test, msg } => {
            count_yields_in_expr(test) + msg.as_ref().map(count_yields_in_expr).unwrap_or(0)
        }
        Statement::Delete(expr) => count_yields_in_expr(expr),
        Statement::Break | Statement::Continue | Statement::Pass => 0,
        Statement::Global { .. } | Statement::Nonlocal { .. } => 0,
    }
}

fn count_yields_in_expr(expr: &Expression) -> usize {
    match expr {
        Expression::Yield { value, .. } => {
            1 + value.as_ref().map(|e| count_yields_in_expr(e)).unwrap_or(0)
        }
        Expression::BinOp { left, right, .. } => {
            count_yields_in_expr(left) + count_yields_in_expr(right)
        }
        Expression::UnaryOp { operand, .. } => count_yields_in_expr(operand),
        Expression::Call { func, args } => {
            count_yields_in_expr(func) + args.iter().map(count_yields_in_expr).sum::<usize>()
        }
        Expression::Attribute { object, .. } => count_yields_in_expr(object),
        Expression::Subscript { object, index } => {
            count_yields_in_expr(object) + count_yields_in_expr(index)
        }
        Expression::List(elems) | Expression::Tuple(elems) | Expression::Set(elems) => {
            elems.iter().map(count_yields_in_expr).sum()
        }
        Expression::Dict(pairs) => pairs
            .iter()
            .map(|(k, v)| count_yields_in_expr(k) + count_yields_in_expr(v))
            .sum(),
        Expression::Slice { start, stop, step } => {
            start.as_ref().map(|e| count_yields_in_expr(e)).unwrap_or(0)
                + stop.as_ref().map(|e| count_yields_in_expr(e)).unwrap_or(0)
                + step.as_ref().map(|e| count_yields_in_expr(e)).unwrap_or(0)
        }
        _ => 0,
    }
}

/// LLVM Coroutine intrinsics helper
pub struct CoroutineIntrinsics<'ctx> {
    pub coro_id: FunctionValue<'ctx>,
    pub coro_size_i64: FunctionValue<'ctx>,
    pub coro_begin: FunctionValue<'ctx>,
    pub coro_suspend: FunctionValue<'ctx>,
    pub coro_end: FunctionValue<'ctx>,
    pub coro_free: FunctionValue<'ctx>,
    pub coro_resume: FunctionValue<'ctx>,
    pub coro_destroy: FunctionValue<'ctx>,
    pub coro_done: FunctionValue<'ctx>,
    pub coro_promise: FunctionValue<'ctx>,
    pub coro_alloc: FunctionValue<'ctx>,
}

impl<'ctx> CoroutineIntrinsics<'ctx> {
    /// Get or declare all coroutine intrinsics needed for generator implementation
    pub fn new(ctx: &'ctx Context, module: &Module<'ctx>) -> Self {
        let ptr_type = ctx.ptr_type(inkwell::AddressSpace::default());
        let i1_type = ctx.bool_type();
        let i8_type = ctx.i8_type();
        let i32_type = ctx.i32_type();
        let i64_type = ctx.i64_type();

        // @llvm.coro.id(i32 <align>, ptr <promise>, ptr <coroaddr>, ptr <fnaddrs>)
        let coro_id = get_intrinsic_decl(ctx, module, "llvm.coro.id", &[]);

        // @llvm.coro.size.i64() -> i64
        let coro_size_i64 =
            get_intrinsic_decl(ctx, module, "llvm.coro.size.i64", &[i64_type.into()]);

        // @llvm.coro.begin(token <id>, ptr <mem>) -> ptr
        let coro_begin = get_intrinsic_decl(ctx, module, "llvm.coro.begin", &[]);

        // @llvm.coro.suspend(token <save>, i1 <final>) -> i8
        let coro_suspend = get_intrinsic_decl(ctx, module, "llvm.coro.suspend", &[]);

        // @llvm.coro.end(ptr <handle>, i1 <unwind>, token <save>) -> i1
        let coro_end = get_intrinsic_decl(ctx, module, "llvm.coro.end", &[]);

        // @llvm.coro.free(token <id>, ptr <frame>) -> ptr
        let coro_free = get_intrinsic_decl(ctx, module, "llvm.coro.free", &[]);

        // @llvm.coro.resume(ptr <handle>)
        let coro_resume = get_intrinsic_decl(ctx, module, "llvm.coro.resume", &[]);

        // @llvm.coro.destroy(ptr <handle>)
        let coro_destroy = get_intrinsic_decl(ctx, module, "llvm.coro.destroy", &[]);

        // @llvm.coro.done(ptr <handle>) -> i1
        let coro_done = get_intrinsic_decl(ctx, module, "llvm.coro.done", &[]);

        // @llvm.coro.promise(ptr <handle>, i32 <align>, i1 <from>) -> ptr
        let coro_promise = get_intrinsic_decl(ctx, module, "llvm.coro.promise", &[]);

        // @llvm.coro.alloc(token <id>) -> i1
        let coro_alloc = get_intrinsic_decl(ctx, module, "llvm.coro.alloc", &[]);

        // Suppress unused variable warnings
        let _ = (ptr_type, i1_type, i8_type, i32_type, i64_type);

        CoroutineIntrinsics {
            coro_id,
            coro_size_i64,
            coro_begin,
            coro_suspend,
            coro_end,
            coro_free,
            coro_resume,
            coro_destroy,
            coro_done,
            coro_promise,
            coro_alloc,
        }
    }
}

/// Get or declare an LLVM intrinsic
fn get_intrinsic_decl<'ctx>(
    _ctx: &'ctx Context,
    module: &Module<'ctx>,
    name: &str,
    param_types: &[BasicTypeEnum<'ctx>],
) -> FunctionValue<'ctx> {
    // Check if already declared
    if let Some(f) = module.get_function(name) {
        return f;
    }

    // Find the intrinsic
    let intrinsic =
        Intrinsic::find(name).unwrap_or_else(|| panic!("LLVM intrinsic {} not found", name));

    // Get declaration
    intrinsic
        .get_declaration(module, param_types)
        .unwrap_or_else(|| panic!("Failed to get declaration for {}", name))
}

/// Generator state constants
pub const GEN_STATE_CREATED: i64 = 0;
pub const GEN_STATE_SUSPENDED: i64 = 1;
pub const GEN_STATE_RUNNING: i64 = 2;
pub const GEN_STATE_CLOSED: i64 = 3;
