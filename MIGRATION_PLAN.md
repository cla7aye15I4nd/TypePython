# Migration Plan: Use Python AST Directly

This document outlines the plan to replace the custom AST with Python's `ast` module structures directly.

## Overview

**Goal**: Eliminate custom AST entirely and use Python's AST node types in Rust.

**Current State**:
- Custom AST in `src/ast/mod.rs` (~295 lines)
- Pest grammar `src/grammar.pest` (551 lines)
- AST builder `src/ast/parser.rs` (2061 lines)
- Preprocessor `src/preprocessor/` (~150 lines)
- Visitor trait `src/ast/visitor.rs`
- CodeGen visitor using custom AST

**Target State**:
- Python AST types in `src/ast/mod.rs` (mirrors Python's ast module)
- JSON serializer `scripts/ast_to_json.py` (~100 lines)
- JSON deserializer in Rust using serde
- CodeGen working directly with Python AST nodes

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     NEW ARCHITECTURE                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  .py file                                                    │
│      │                                                       │
│      ▼                                                       │
│  ┌────────────────────────────────────┐                     │
│  │  Python: ast.parse() + json.dumps  │                     │
│  │  (scripts/ast_to_json.py)          │                     │
│  └────────────────────────────────────┘                     │
│      │                                                       │
│      │ JSON (Python AST structure)                          │
│      ▼                                                       │
│  ┌────────────────────────────────────┐                     │
│  │  Rust: serde_json deserialize      │                     │
│  │  → Python AST types (src/ast/)     │                     │
│  └────────────────────────────────────┘                     │
│      │                                                       │
│      │ Python AST nodes                                      │
│      ▼                                                       │
│  ┌────────────────────────────────────┐                     │
│  │  CodeGen (visitor over Python AST) │                     │
│  │  → LLVM IR                         │                     │
│  └────────────────────────────────────┘                     │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Python AST Node Types

Reference: https://docs.python.org/3/library/ast.html

### Module (Top-level)
```python
Module(body=[stmt*], type_ignores=[])
```

### Statements (stmt)
```python
FunctionDef(name, args, body, decorator_list, returns, type_comment)
AsyncFunctionDef(name, args, body, decorator_list, returns, type_comment)
ClassDef(name, bases, keywords, body, decorator_list)
Return(value?)
Delete(targets)
Assign(targets, value, type_comment)
AugAssign(target, op, value)
AnnAssign(target, annotation, value?, simple)
For(target, iter, body, orelse, type_comment)
AsyncFor(target, iter, body, orelse, type_comment)
While(test, body, orelse)
If(test, body, orelse)
With(items, body, type_comment)
AsyncWith(items, body, type_comment)
Match(subject, cases)
Raise(exc?, cause?)
Try(body, handlers, orelse, finalbody)
TryStar(body, handlers, orelse, finalbody)
Assert(test, msg?)
Import(names)
ImportFrom(module?, names, level)
Global(names)
Nonlocal(names)
Expr(value)
Pass
Break
Continue
```

### Expressions (expr)
```python
BoolOp(op, values)
NamedExpr(target, value)  # walrus operator :=
BinOp(left, op, right)
UnaryOp(op, operand)
Lambda(args, body)
IfExp(test, body, orelse)
Dict(keys, values)
Set(elts)
ListComp(elt, generators)
SetComp(elt, generators)
DictComp(key, value, generators)
GeneratorExp(elt, generators)
Await(value)
Yield(value?)
YieldFrom(value)
Compare(left, ops, comparators)
Call(func, args, keywords)
FormattedValue(value, conversion, format_spec)
JoinedStr(values)  # f-strings
Constant(value, kind)
Attribute(value, attr, ctx)
Subscript(value, slice, ctx)
Starred(value, ctx)
Name(id, ctx)
List(elts, ctx)
Tuple(elts, ctx)
Slice(lower?, upper?, step?)
```

### Operators
```python
# boolop
And | Or

# operator (binary)
Add | Sub | Mult | MatMult | Div | Mod | Pow | LShift | RShift | BitOr | BitXor | BitAnd | FloorDiv

# unaryop
Invert | Not | UAdd | USub

# cmpop
Eq | NotEq | Lt | LtE | Gt | GtE | Is | IsNot | In | NotIn
```

### Other
```python
comprehension(target, iter, ifs, is_async)
ExceptHandler(type?, name?, body)
arguments(posonlyargs, args, vararg, kwonlyargs, kw_defaults, kwarg, defaults)
arg(arg, annotation, type_comment)
keyword(arg?, value)
alias(name, asname?)
withitem(context_expr, optional_vars?)
match_case(pattern, guard?, body)
```

---

## Phase 1: Create Python AST JSON Serializer

### File: `scripts/ast_to_json.py`

```python
#!/usr/bin/env python3
"""
Serialize Python AST to JSON for TypePython compiler.
Output matches Python's ast module structure exactly.
"""

import ast
import json
import sys
from typing import Any


def node_to_dict(node: ast.AST) -> dict[str, Any]:
    """Convert an AST node to a dictionary."""
    result = {"_type": node.__class__.__name__}

    for field, value in ast.iter_fields(node):
        result[field] = serialize_value(value)

    # Include location info
    if hasattr(node, "lineno"):
        result["lineno"] = node.lineno
    if hasattr(node, "col_offset"):
        result["col_offset"] = node.col_offset
    if hasattr(node, "end_lineno"):
        result["end_lineno"] = node.end_lineno
    if hasattr(node, "end_col_offset"):
        result["end_col_offset"] = node.end_col_offset

    return result


def serialize_value(value: Any) -> Any:
    """Serialize a field value."""
    if isinstance(value, ast.AST):
        return node_to_dict(value)
    elif isinstance(value, list):
        return [serialize_value(v) for v in value]
    elif isinstance(value, bytes):
        # Bytes literals need special handling
        return {"_bytes": list(value)}
    elif value is None or isinstance(value, (str, int, float, bool)):
        return value
    else:
        # For complex numbers, etc.
        return repr(value)


def parse_file(filepath: str) -> dict[str, Any]:
    """Parse a Python file and return JSON-serializable AST."""
    with open(filepath, "r") as f:
        source = f.read()

    tree = ast.parse(source, filename=filepath)
    return node_to_dict(tree)


def main():
    if len(sys.argv) < 2:
        print("Usage: ast_to_json.py <file.py>", file=sys.stderr)
        sys.exit(1)

    filepath = sys.argv[1]

    try:
        result = parse_file(filepath)
        print(json.dumps(result))
    except SyntaxError as e:
        error = {
            "_error": "SyntaxError",
            "msg": str(e.msg),
            "lineno": e.lineno,
            "offset": e.offset,
            "text": e.text,
        }
        print(json.dumps(error))
        sys.exit(1)
    except Exception as e:
        print(json.dumps({"_error": str(e)}), file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
```

---

## Phase 2: Create Rust Python AST Types

### File: `src/ast/mod.rs` (complete rewrite)

```rust
//! Python AST types - mirrors Python's ast module exactly
//! Reference: https://docs.python.org/3/library/ast.html

use serde::{Deserialize, Serialize};

/// Location information for AST nodes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
    #[serde(default)]
    pub lineno: Option<u32>,
    #[serde(default)]
    pub col_offset: Option<u32>,
    #[serde(default)]
    pub end_lineno: Option<u32>,
    #[serde(default)]
    pub end_col_offset: Option<u32>,
}

/// Top-level module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub type_ignores: Vec<TypeIgnore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeIgnore {
    pub lineno: u32,
    pub tag: String,
}

// ============================================================================
// Statements
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum Stmt {
    FunctionDef(FunctionDef),
    AsyncFunctionDef(AsyncFunctionDef),
    ClassDef(ClassDef),
    Return(Return),
    Delete(Delete),
    Assign(Assign),
    AugAssign(AugAssign),
    AnnAssign(AnnAssign),
    For(For),
    AsyncFor(AsyncFor),
    While(While),
    If(If),
    With(With),
    AsyncWith(AsyncWith),
    Match(Match),
    Raise(Raise),
    Try(Try),
    TryStar(TryStar),
    Assert(Assert),
    Import(Import),
    ImportFrom(ImportFrom),
    Global(Global),
    Nonlocal(Nonlocal),
    Expr(ExprStmt),
    Pass(Pass),
    Break(Break),
    Continue(Continue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDef {
    pub name: String,
    pub args: Arguments,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub decorator_list: Vec<Expr>,
    pub returns: Option<Expr>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncFunctionDef {
    pub name: String,
    pub args: Arguments,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub decorator_list: Vec<Expr>,
    pub returns: Option<Expr>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDef {
    pub name: String,
    #[serde(default)]
    pub bases: Vec<Expr>,
    #[serde(default)]
    pub keywords: Vec<Keyword>,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub decorator_list: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Return {
    pub value: Option<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delete {
    pub targets: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assign {
    pub targets: Vec<Expr>,
    pub value: Expr,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AugAssign {
    pub target: Expr,
    pub op: Operator,
    pub value: Expr,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnAssign {
    pub target: Expr,
    pub annotation: Expr,
    pub value: Option<Expr>,
    pub simple: i32,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct For {
    pub target: Expr,
    pub iter: Expr,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub orelse: Vec<Stmt>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncFor {
    pub target: Expr,
    pub iter: Expr,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub orelse: Vec<Stmt>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct While {
    pub test: Expr,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub orelse: Vec<Stmt>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct If {
    pub test: Expr,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub orelse: Vec<Stmt>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct With {
    pub items: Vec<WithItem>,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncWith {
    pub items: Vec<WithItem>,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub subject: Expr,
    pub cases: Vec<MatchCase>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Raise {
    pub exc: Option<Expr>,
    pub cause: Option<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Try {
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub handlers: Vec<ExceptHandler>,
    #[serde(default)]
    pub orelse: Vec<Stmt>,
    #[serde(default)]
    pub finalbody: Vec<Stmt>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStar {
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub handlers: Vec<ExceptHandler>,
    #[serde(default)]
    pub orelse: Vec<Stmt>,
    #[serde(default)]
    pub finalbody: Vec<Stmt>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assert {
    pub test: Expr,
    pub msg: Option<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub names: Vec<Alias>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportFrom {
    pub module: Option<String>,
    pub names: Vec<Alias>,
    pub level: i32,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Global {
    pub names: Vec<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nonlocal {
    pub names: Vec<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExprStmt {
    pub value: Expr,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pass {
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Break {
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Continue {
    #[serde(flatten)]
    pub location: Location,
}

// ============================================================================
// Expressions
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum Expr {
    BoolOp(BoolOp),
    NamedExpr(NamedExpr),
    BinOp(BinOp),
    UnaryOp(UnaryOp),
    Lambda(Lambda),
    IfExp(IfExp),
    Dict(Dict),
    Set(Set),
    ListComp(ListComp),
    SetComp(SetComp),
    DictComp(DictComp),
    GeneratorExp(GeneratorExp),
    Await(Await),
    Yield(Yield),
    YieldFrom(YieldFrom),
    Compare(Compare),
    Call(Call),
    FormattedValue(FormattedValue),
    JoinedStr(JoinedStr),
    Constant(Constant),
    Attribute(Attribute),
    Subscript(Subscript),
    Starred(Starred),
    Name(Name),
    List(List),
    Tuple(Tuple),
    Slice(Slice),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoolOp {
    pub op: BoolOpKind,
    pub values: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedExpr {
    pub target: Box<Expr>,
    pub value: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinOp {
    pub left: Box<Expr>,
    pub op: Operator,
    pub right: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryOp {
    pub op: UnaryOpKind,
    pub operand: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lambda {
    pub args: Arguments,
    pub body: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfExp {
    pub test: Box<Expr>,
    pub body: Box<Expr>,
    pub orelse: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dict {
    pub keys: Vec<Option<Expr>>,  // None for **kwargs unpacking
    pub values: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Set {
    pub elts: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListComp {
    pub elt: Box<Expr>,
    pub generators: Vec<Comprehension>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetComp {
    pub elt: Box<Expr>,
    pub generators: Vec<Comprehension>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictComp {
    pub key: Box<Expr>,
    pub value: Box<Expr>,
    pub generators: Vec<Comprehension>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorExp {
    pub elt: Box<Expr>,
    pub generators: Vec<Comprehension>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Await {
    pub value: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Yield {
    pub value: Option<Box<Expr>>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldFrom {
    pub value: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compare {
    pub left: Box<Expr>,
    pub ops: Vec<CmpOp>,
    pub comparators: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    pub func: Box<Expr>,
    #[serde(default)]
    pub args: Vec<Expr>,
    #[serde(default)]
    pub keywords: Vec<Keyword>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattedValue {
    pub value: Box<Expr>,
    pub conversion: i32,
    pub format_spec: Option<Box<Expr>>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinedStr {
    pub values: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constant {
    pub value: ConstantValue,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConstantValue {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Bytes { _bytes: Vec<u8> },
    Complex { real: f64, imag: f64 },
    Ellipsis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub value: Box<Expr>,
    pub attr: String,
    pub ctx: ExprContext,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscript {
    pub value: Box<Expr>,
    pub slice: Box<Expr>,
    pub ctx: ExprContext,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Starred {
    pub value: Box<Expr>,
    pub ctx: ExprContext,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    pub id: String,
    pub ctx: ExprContext,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub elts: Vec<Expr>,
    pub ctx: ExprContext,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tuple {
    pub elts: Vec<Expr>,
    pub ctx: ExprContext,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slice {
    pub lower: Option<Box<Expr>>,
    pub upper: Option<Box<Expr>>,
    pub step: Option<Box<Expr>>,
    #[serde(flatten)]
    pub location: Location,
}

// ============================================================================
// Operators
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum BoolOpKind {
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum Operator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum UnaryOpKind {
    Invert,
    Not,
    UAdd,
    USub,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum CmpOp {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum ExprContext {
    Load,
    Store,
    Del,
}

impl Default for ExprContext {
    fn default() -> Self {
        ExprContext::Load
    }
}

// ============================================================================
// Other nodes
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comprehension {
    pub target: Expr,
    pub iter: Expr,
    #[serde(default)]
    pub ifs: Vec<Expr>,
    #[serde(default)]
    pub is_async: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptHandler {
    #[serde(rename = "type")]
    pub type_: Option<Expr>,
    pub name: Option<String>,
    pub body: Vec<Stmt>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arguments {
    #[serde(default)]
    pub posonlyargs: Vec<Arg>,
    #[serde(default)]
    pub args: Vec<Arg>,
    pub vararg: Option<Arg>,
    #[serde(default)]
    pub kwonlyargs: Vec<Arg>,
    #[serde(default)]
    pub kw_defaults: Vec<Option<Expr>>,
    pub kwarg: Option<Arg>,
    #[serde(default)]
    pub defaults: Vec<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arg {
    pub arg: String,
    pub annotation: Option<Expr>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword {
    pub arg: Option<String>,  // None for **kwargs
    pub value: Expr,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alias {
    pub name: String,
    pub asname: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithItem {
    pub context_expr: Expr,
    pub optional_vars: Option<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Vec<Stmt>,
}

// Pattern matching (Python 3.10+)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum Pattern {
    MatchValue(MatchValue),
    MatchSingleton(MatchSingleton),
    MatchSequence(MatchSequence),
    MatchMapping(MatchMapping),
    MatchClass(MatchClass),
    MatchStar(MatchStar),
    MatchAs(MatchAs),
    MatchOr(MatchOr),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchValue {
    pub value: Expr,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchSingleton {
    pub value: ConstantValue,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchSequence {
    pub patterns: Vec<Pattern>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchMapping {
    pub keys: Vec<Expr>,
    pub patterns: Vec<Pattern>,
    pub rest: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchClass {
    pub cls: Expr,
    pub patterns: Vec<Pattern>,
    pub kwd_attrs: Vec<String>,
    pub kwd_patterns: Vec<Pattern>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchStar {
    pub name: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchAs {
    pub pattern: Option<Box<Pattern>>,
    pub name: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchOr {
    pub patterns: Vec<Pattern>,
    #[serde(flatten)]
    pub location: Location,
}

// ============================================================================
// Parsing
// ============================================================================

/// Parse JSON from Python ast_to_json.py into Module
pub fn parse_json(json: &str) -> Result<Module, String> {
    // The JSON has _type: "Module" at top level
    #[derive(Deserialize)]
    struct ModuleWrapper {
        body: Vec<Stmt>,
        #[serde(default)]
        type_ignores: Vec<TypeIgnore>,
    }

    let wrapper: ModuleWrapper = serde_json::from_str(json)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    Ok(Module {
        body: wrapper.body,
        type_ignores: wrapper.type_ignores,
    })
}
```

---

## Phase 3: Update CodeGen to Work with Python AST

The codegen visitor needs to be rewritten to work with Python AST nodes.

### Key Changes

| Old (Custom AST) | New (Python AST) |
|------------------|------------------|
| `Statement::VarDecl` | `Stmt::AnnAssign` with `simple=1` |
| `Statement::Assignment` | `Stmt::Assign` |
| `Statement::If` with `elif_clauses` | `Stmt::If` with nested `orelse` |
| `Expression::IntLit(i64)` | `Expr::Constant` with `ConstantValue::Int` |
| `Expression::BinOp { op, left, right }` | `Expr::BinOp { left, op, right }` |
| `BinaryOp::Eq` (comparison) | Separate `Expr::Compare` node |
| `Expression::Ternary` | `Expr::IfExp` |
| Chained comparisons flattened | `Compare { left, ops: [...], comparators: [...] }` |

### Example: If Statement Handling

Old:
```rust
fn visit_if(
    &mut self,
    condition: &Expression,
    then_block: &[Statement],
    elif_clauses: &[(Expression, Vec<Statement>)],
    else_block: &Option<Vec<Statement>>,
)
```

New (Python AST has no `elif`, just nested `If` in `orelse`):
```rust
fn visit_if(&mut self, if_stmt: &ast::If) -> Result<(), String> {
    // if_stmt.test is the condition
    // if_stmt.body is the then block
    // if_stmt.orelse contains either:
    //   - Empty vec (no else)
    //   - Single If statement (elif chain)
    //   - Multiple statements (else block)
}
```

---

## Phase 4: Files to Delete

1. `src/grammar.pest` - Pest grammar
2. `src/ast/parser.rs` - Pest → AST converter
3. `src/ast/visitor.rs` - Old visitor trait (will be rewritten)
4. `src/preprocessor/mod.rs` - Indentation preprocessor
5. `src/preprocessor/` - Directory

## Phase 5: Files to Modify

1. `src/ast/mod.rs` - Complete rewrite (Python AST types)
2. `src/lib.rs` - Remove pest, add serde
3. `src/module.rs` - Use Python subprocess for parsing
4. `src/codegen/visitor/mod.rs` - Rewrite for Python AST
5. `src/codegen/visitor/expressions.rs` - Rewrite for Python AST
6. `src/codegen/visitor/statements.rs` - Rewrite for Python AST
7. `src/codegen/visitor/program.rs` - Rewrite for Python AST
8. `src/codegen/visitor/generator.rs` - Update for Python AST
9. `Cargo.toml` - Add serde/serde_json, remove pest

## Phase 6: Update Cargo.toml

```toml
[package]
name = "tpy"
version = "0.1.0"
edition = "2021"
build = "build.rs"

[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
inkwell = { version = "0.7.0", features = ["llvm21-1"] }
either = "1.9"
tempfile = "3"
lazy_static = "1.4"
```

---

## Migration Checklist

- [x] **Phase 1**: Create `scripts/ast_to_json.py`
- [x] **Phase 2**: Rewrite `src/ast/mod.rs` with Python AST types
- [x] **Phase 3**: Update `Cargo.toml` dependencies
- [x] **Phase 4**: Update `src/lib.rs` (remove pest)
- [x] **Phase 5**: Update `src/module.rs` to call Python
- [x] **Phase 7**: Delete old files (grammar.pest, parser.rs, preprocessor/, visitor.rs)
- [ ] **Phase 6**: Rewrite `src/codegen/` for Python AST (414 errors remaining)
- [ ] **Phase 8**: Run tests and fix issues

---

## Benefits

1. **100% Python compatibility** - Every valid Python file parses correctly
2. **No grammar maintenance** - Python handles all syntax evolution
3. **Better error messages** - Python's mature parser with line numbers
4. **Future-proof** - New Python features (match, walrus, f-strings) automatically available
5. **Simpler codebase** - Remove ~2800 lines of parsing code
6. **Location info** - Full source location for error reporting

## Considerations

1. **Runtime dependency** - Requires Python 3 at compile time
2. **Subprocess overhead** - One Python call per file (cacheable)
3. **Larger refactor** - CodeGen needs significant updates

## Incremental Strategy

To minimize risk, migrate incrementally:

1. First, get parsing working (Phase 1-5)
2. Create adapter layer that converts Python AST → old AST (temporary)
3. Verify all tests pass
4. Gradually update CodeGen to use Python AST directly
5. Remove adapter layer once complete
