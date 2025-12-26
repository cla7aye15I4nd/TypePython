# TODO: Making TypePython More Like Python

This document tracks planned features to make TypePython more compatible with standard Python syntax and semantics.

> **Note**: Some Python features are intentionally **not planned** to maintain TypePython's static typing guarantees:
> - `None` type (no null values)
> - Union types (`Union[A, B]`, `A | B`)
> - `__init__.py` package initialization
> - Dynamic typing / `Any` type

## High Priority

### 1. Implement Dictionary Type
- [ ] Add `dict[K, V]` type to the type system
- [ ] Implement dict runtime in C (hash table with open addressing)
- [ ] Support dict literals: `{"key": value}`
- [ ] Implement dict methods: `get()`, `keys()`, `values()`, `items()`, `pop()`, `update()`
- [ ] Support dict iteration: `for k in d:`, `for k, v in d.items():`
- [ ] Add `in` operator for dict membership

### 2. Implement Set Type
- [ ] Add `set[T]` type to the type system
- [ ] Implement set runtime in C (hash set)
- [ ] Support set literals: `{1, 2, 3}`
- [ ] Implement set methods: `add()`, `remove()`, `discard()`, `pop()`, `union()`, `intersection()`
- [ ] Support set operations: `|`, `&`, `-`, `^`
- [ ] Add `in` operator for set membership

### 3. Tuple Support
- [ ] Add tuple types: `tuple[T1, T2, ...]`
- [ ] Support tuple literals: `(1, "hello", 3.14)`
- [ ] Implement tuple unpacking: `a, b, c = my_tuple`
- [ ] Support tuple indexing
- [ ] Add `len()` support for tuples

### 4. Multiple Assignment / Unpacking
- [ ] Support `a, b = b, a` (swap)
- [ ] Support `a, b, c = some_list`
- [ ] Support `first, *rest = items`
- [ ] Support `*start, last = items`

## Medium Priority

### 5. List Comprehensions
- [ ] Parse list comprehension syntax: `[expr for x in iter]`
- [ ] Support conditionals: `[x for x in items if x > 0]`
- [ ] Support nested comprehensions: `[x*y for x in a for y in b]`
- [ ] Add dict comprehensions: `{k: v for k, v in items}`
- [ ] Add set comprehensions: `{x for x in items}`

### 6. Lambda Expressions
- [ ] Parse lambda syntax: `lambda x: x * 2`
- [ ] Implement closure capture for lambdas
- [ ] Support lambdas as function arguments
- [ ] Type inference for lambda parameters

### 7. Generator Functions
- [ ] Parse `yield` and `yield from` statements
- [ ] Implement generator state machine transformation
- [ ] Create generator runtime support
- [ ] Support `StopIteration` integration
- [ ] Add generator expressions: `(x for x in items)`

### 8. Decorators
- [ ] Parse decorator syntax: `@decorator`
- [ ] Support decorator arguments: `@decorator(arg)`
- [ ] Implement compile-time decorator application
- [ ] Support built-in decorators: `@staticmethod`, `@classmethod`, `@property`

### 9. String Formatting
- [ ] Support f-strings: `f"Hello, {name}!"`
- [ ] Support format specifiers: `f"{value:.2f}"`
- [ ] Support `.format()` method
- [ ] Support `%` formatting (legacy)

### 10. More String Methods
- [ ] `split()`, `join()`, `strip()`, `lstrip()`, `rstrip()`
- [ ] `startswith()`, `endswith()`, `find()`, `replace()`
- [ ] `upper()`, `lower()`, `capitalize()`, `title()`
- [ ] `isdigit()`, `isalpha()`, `isalnum()`, `isspace()`
- [ ] String slicing: `s[1:5]`, `s[::2]`, `s[::-1]`

### 11. More List Methods
- [ ] `insert()`, `remove()`, `pop(index)`, `index()`, `count()`
- [ ] `sort()`, `reverse()`, `copy()`, `clear()`
- [ ] `extend()` with iterables
- [ ] List slicing: `l[1:5]`, `l[::2]`
- [ ] Negative indexing: `l[-1]`, `l[-2:]`

## Lower Priority

### 12. Multiple Inheritance
- [ ] Support multiple base classes
- [ ] Implement Method Resolution Order (MRO)
- [ ] Handle diamond inheritance
- [ ] Support `super()` with MRO

### 13. Context Managers
- [ ] Parse `with` statement
- [ ] Support `__enter__` and `__exit__` protocols
- [ ] Support multiple context managers: `with a, b:`
- [ ] Support `as` binding: `with open(f) as file:`

### 14. Async/Await
- [ ] Parse `async def`, `await`, `async for`, `async with`
- [ ] Implement coroutine transformation
- [ ] Create async runtime (event loop)
- [ ] Support `asyncio`-like primitives

### 15. Star Arguments
- [ ] Support `*args` in function definitions
- [ ] Support `**kwargs` in function definitions
- [ ] Support `*` and `**` in function calls
- [ ] Implement variadic argument passing at LLVM level
