<p align="center">
<img src="https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&logo=rust" alt="Rust">
<img src="https://img.shields.io/badge/backend-LLVM-blue?style=for-the-badge&logo=llvm" alt="LLVM">
<img src="https://img.shields.io/badge/parser-PyO3-green?style=for-the-badge&logo=python" alt="PyO3">
</p>
<h1 align="center">
🐍⚡ <code>tpy</code> — TypePython Compiler
</h1>

<p align="center">
<strong>A statically-typed Python compiler that produces native machine code via LLVM</strong>
</p>

<p align="center">
<em>Write Python. Get type safety. Compile to native code.</em>
</p>

-----

## ✨ What is TypePython?

**TypePython** (`tpy`) is a statically-typed subset of Python that compiles directly to native machine code via LLVM. The goal is to be **compatible with original Python syntax** while enforcing static typing for performance and safety.

**Key Features:**
  - 🎯 **Python-compatible syntax** — Uses Python's official AST parser
  - 🔒 **Static typing** — All variables must be type-annotated
  - ⚡ **Native performance** — Compiles to optimized machine code via LLVM
  - 🦀 **Rust-powered** — Memory-safe compiler implementation
  - 📦 **Multi-module support** — Import and compile multiple Python files

**Current Status:**
  - ✅ Core language features (functions, classes, control flow, context managers)
  - ✅ 50+ builtin functions and 100+ builtin methods
  - ✅ Collections with full slicing support (list, dict, set, tuple)
  - ✅ Exception handling (try/except/finally/raise)
  - ✅ Module imports with recursive compilation
  - ✅ Assert, del, pass, with statements
  - ✅ String formatting (% operator)
  - ❌ **No union types** (e.g., `int | None`) — Phase 1 limitation
  - ❌ No decorators, async/await, or f-strings

-----

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                     TypePython Compilation Pipeline                 │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────────┐     │
│  │ Source   │──▶│  PyO3    │──▶│ Serde    │──▶│ Module       │     │
│  │  .py     │   │ ast.parse│   │ JSON AST │   │ Resolution   │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────┬───────┘     │
│                                                       │             │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐          │             │
│  │ Native   │◀──│  Clang   │◀──│  Inkwell │◀─────────┘             │
│  │ Binary   │   │   +LTO   │   │ LLVM IR  │                        │
│  └──────────┘   └──────────┘   └──────────┘                        │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

| Stage | Technology | Description |
|-------|------------|-------------|
| **Parsing** | [PyO3](https://pyo3.rs) + Python AST | Uses Python's official `ast` module for 100% syntax compatibility |
| **AST Conversion** | Serde JSON | Deserializes Python AST into Rust structures |
| **Module Resolution** | Custom | BFS-based discovery of all imported modules |
| **Code Generation** | [Inkwell](https://github.com/TheDan64/inkwell) | Safe Rust bindings to LLVM 21 |
| **Linking** | Clang + LTO | Link-time optimization for minimal binary size |

**Why PyO3?**
By leveraging Python's official AST parser, TypePython guarantees syntax compatibility with CPython. Any valid Python syntax is correctly parsed—only semantic features (like dynamic typing) are restricted.

-----

## 📊 Feature Implementation Status

### ✅ Fully Implemented

#### Core Types
- **Primitives**: `int`, `float`, `bool`, `str`, `bytes`, `None`
- **Collections**: `list[T]`, `dict[K,V]`, `set[T]`, `tuple[T1,T2,...]`
- **Advanced**: `range`, user-defined classes, function types, modules

#### Operators
- **Arithmetic**: `+`, `-`, `*`, `/`, `//`, `%`, `**`
- **Comparison**: `==`, `!=`, `<`, `<=`, `>`, `>=`, `is`, `is not`, `in`, `not in`
- **Logical**: `and`, `or`, `not`
- **Bitwise**: `&`, `|`, `^`, `~`, `<<`, `>>` (integers only)
- **Augmented assignment**: `+=`, `-=`, `*=`, `/=`, `//=`, `%=`, `**=`, `&=`, `|=`, `^=`, `<<=`, `>>=`

#### Control Flow
- **Conditionals**: `if`/`elif`/`else`, ternary expressions
- **Loops**: `while`, `for...in` (with `break`, `continue`, `else` clause)
- **Exception Handling**: `try`/`except`/`else`/`finally`, `raise`
- **Context Managers**: `with` statement for resource management
- **Utility Statements**: `assert`, `del`, `pass`
- **Functions**: `def`, `return`, recursion, higher-order functions
- **Generators**: `yield`, `yield from`

#### Classes & Objects
- Class definition with `__init__`
- Instance methods and attributes
- Method calls with `self`
- Instance creation and attribute access

#### Comprehensions
- List: `[x for x in iterable if condition]`
- Dict: `{k: v for k, v in iterable if condition}`
- Set: `{x for x in iterable if condition}`
- Generator: `(x for x in iterable)`

#### Imports & Modules
- `import module`
- `from module import name`
- Multi-level imports
- Recursive module compilation

#### Variables & Scoping
- Type-annotated declarations: `x: int = 5`
- Tuple unpacking: `a, b = (1, 2)`
- Multiple assignment: `x = y = z = 5`
- Starred expressions: `a, *rest, b = [1, 2, 3, 4, 5]`
- `global` and `nonlocal` keywords
- Proper block scoping

#### Indexing & Slicing
- Subscript access: `list[0]`, `dict[key]`, `str[i]`
- Slice notation: `list[start:stop:step]`
- Negative indices: `list[-1]`
- Slice examples: `list[2:5]`, `list[:3]`, `list[::2]`, `list[::-1]`

#### String Features
- String literals: single/double quotes, bytes literals
- String formatting: `"Hello %s" % name`, `"%d + %d = %d" % (a, b, c)`
- Escape sequences: `\n`, `\t`, `\\`, etc.

#### Built-in Functions (50+)
- **I/O**: `print()`
- **Type conversions**: `int()`, `float()`, `bool()`, `str()`, `bytes()`
- **Math**: `abs()`, `round()`, `pow()`, `divmod()`, `min()`, `max()`, `sum()`
- **Boolean**: `any()`, `all()`
- **String/Number**: `bin()`, `hex()`, `oct()`, `chr()`, `ord()`, `ascii()`
- **Sequences**: `len()`, `sorted()`, `reversed()`
- **Iterators**: `range()`, `enumerate()`, `zip()`, `filter()`, `iter()`, `next()`
- **Collections**: `list()`, `dict()`, `set()`, `tuple()`, `frozenset()`
- **Introspection**: `id()`, `repr()`, `getattr()`, `hasattr()`

#### Built-in Methods
**String/Bytes methods** (~30 methods):
- Case: `upper()`, `lower()`, `capitalize()`, `title()`, `swapcase()`
- Padding: `ljust()`, `rjust()`, `center()`, `zfill()`
- Stripping: `strip()`, `lstrip()`, `rstrip()`
- Search: `find()`, `count()`, `startswith()`, `endswith()`
- Predicates: `isalnum()`, `isalpha()`, `isdigit()`, `isspace()`, `isupper()`, `islower()`
- Transform: `replace()`, `split()`, `join()`

**List methods**:
- `append()`, `pop()`, `clear()`, `insert()`, `extend()`, `remove()`, `index()`, `count()`, `reverse()`, `sort()`

**Dict methods**:
- `get()`, `keys()`, `values()`, `items()`, `pop()`, `clear()`, `update()`

**Set methods**:
- `add()`, `remove()`, `discard()`, `pop()`, `clear()`, `union()`, `intersection()`, `difference()`, `issubset()`, `issuperset()`, `isdisjoint()`

### ❌ Not Implemented (Phase 1 Limitations)

#### Type System Limitations
- ❌ **Union types** (`int | None`, `str | int`, etc.)
- ❌ **Optional types** (`Optional[int]`)
- ❌ **Type aliases** (`type MyInt = int`)
- ❌ **Generic functions** (no TypeVar)
- ❌ **Protocol/structural typing**

#### Language Features
- ❌ **Decorators** (parsed but not applied)
- ⚠️ **Lambda functions** (parsed but limited support)
- ❌ **Pattern matching** (`match` statement)
- ❌ **Async/await** (async def, async for, async with)
- ❌ **F-strings** (formatted string literals)
- ❌ **Walrus operator** (`:=` assignment expressions)
- ❌ **Keyword arguments** (`func(x=10)`)
- ❌ **Default parameters** (`def func(x=5)`)
- ❌ **Variable arguments** (`*args`, `**kwargs`)

#### Object-Oriented Features
- ❌ **Inheritance** (base classes parsed but not compiled)
- ❌ **Multiple inheritance**
- ❌ **Class variables**
- ❌ **Static methods** (`@staticmethod`)
- ❌ **Class methods** (`@classmethod`)
- ❌ **Properties** (`@property`)
- ❌ **Operator overloading** (`__add__`, `__eq__`, etc.)
- ❌ **Metaclasses**

#### Advanced Features
- ❌ **Dynamic attribute creation**
- ❌ **Monkey patching**
- ❌ **Reflection** (beyond basic `getattr`/`hasattr`)
- ❌ **Module reloading**
- ❌ **Eval/exec**
- ❌ **Descriptors**
- ❌ **Weak references**

-----

## 🗺️ Development Roadmap

### Phase 1: Core Language (CURRENT) ✅
**Goal**: Statically-typed Python subset with essential features

**Status**: ~85% Complete
- ✅ Core types and operators
- ✅ Control flow (if/while/for/try)
- ✅ Functions and basic classes
- ✅ 50+ builtin functions
- ✅ Module imports
- ✅ Comprehensive test suite (81% coverage)

**Remaining Phase 1 Tasks**:
1. Fix memory leaks in collection types
2. Improve error messages (show Python source location)
3. Complete lambda function support
4. Add more string methods (`format()`, `maketrans()`)
5. Test and validate complex number support

### Phase 2: Advanced Features (PLANNED)
**Goal**: Expand language capabilities while maintaining static typing

**Priorities**:
1. **Full lambda support** — Anonymous functions for `filter()`, `map()`, etc.
2. **Decorators** — Function and class decorators
3. **Keyword arguments** — Named parameters in function calls
4. **Default parameters** — Optional function parameters
5. **Inheritance** — Single inheritance for classes
6. **Class/static methods** — `@classmethod` and `@staticmethod`
7. **Properties** — `@property` decorator
8. **Operator overloading** — `__add__`, `__eq__`, etc. in user classes
9. **F-string support** — String interpolation
10. **Walrus operator** — Assignment expressions (`:=`)

### Phase 3: Type System Enhancement (FUTURE)
**Goal**: Richer type system for better expressiveness

**Potential Features**:
- Union types (`int | None`, `str | int`)
- Optional types (`Optional[T]`)
- Type aliases (`type MyInt = int`)
- Generic functions (TypeVar)
- Protocol/structural typing
- Type narrowing in conditionals

**Note**: This phase requires careful design to maintain compile-time type checking without runtime overhead.

### Phase 4: Performance & Optimization (FUTURE)
**Goal**: Competitive performance with C/Rust

**Ideas**:
- SIMD vectorization for numeric operations
- Custom memory allocators
- Better cache locality for collections
- Escape analysis for stack allocation
- Inline caching for method dispatch
- Profile-guided optimization

### Phase 5: Ecosystem & Tooling (FUTURE)
**Goal**: Production-ready toolchain

**Components**:
- Package manager integration
- LSP (Language Server Protocol) for IDE support
- Debugger with source-level debugging
- Profiler and coverage tools
- Foreign Function Interface (FFI) for C libraries
- Standard library expansion

-----

## 🚀 Quick Start

### Prerequisites

  - **Rust** 1.70+ (install via [rustup](https://rustup.rs))
  - **LLVM 21** with development headers
  - **Clang** (part of LLVM)
  - **Python 3.8+** (for PyO3 parser)
  - **Dependencies**: `build-essential`, `pkg-config`, `libssl-dev`, `libzstd-dev`

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/tpy.git
cd tpy

# Set LLVM path (adjust for your system)
export LLVM_SYS_211_PREFIX=/usr/lib/llvm-21

# Build the compiler
cargo build --release

# Add to PATH (optional)
export PATH="$PATH:$(pwd)/target/release"
```

### Your First Program

**hello.py**:
```python
def main() -> None:
    name: str = "TypePython"
    version: int = 1
    print("Hello from", name, version)

main()
```

**Compile and run**:
```bash
$ tpy hello.py
Hello from TypePython 1
```

### Example: Fibonacci

**fib.py**:
```python
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    else:
        a: int = 0
        b: int = 1
        i: int = 2
        while i <= n:
            temp: int = a + b
            a = b
            b = temp
            i = i + 1
        return b

result: int = fibonacci(10)
print("Fibonacci(10) =", result)
```

```bash
$ tpy fib.py
Fibonacci(10) = 55
```

### Example: Classes

**person.py**:
```python
class Person:
    def __init__(self, name: str, age: int) -> None:
        self.name: str = name
        self.age: int = age

    def greet(self) -> None:
        print("Hello, my name is", self.name)
        print("I am", self.age, "years old")

p: Person = Person("Alice", 30)
p.greet()
```

```bash
$ tpy person.py
Hello, my name is Alice
I am 30 years old
```

### Example: Collections

**collections.py**:
```python
# Lists
numbers: list[int] = [1, 2, 3, 4, 5]
squares: list[int] = [x * x for x in numbers]
print("Squares:", squares)

# Dictionaries
ages: dict[str, int] = {"Alice": 30, "Bob": 25}
ages["Charlie"] = 35
print("Ages:", ages)

# Sets
unique: set[int] = {1, 2, 3, 2, 1}
print("Unique:", unique)

# Tuples
point: tuple[int, int] = (10, 20)
x: int
y: int
x, y = point
print("Point:", x, y)
```

### Example: Modules

**math_utils.py**:
```python
def add(a: int, b: int) -> int:
    return a + b

def multiply(a: int, b: int) -> int:
    return a * b
```

**main.py**:
```python
from math_utils import add, multiply

result1: int = add(5, 3)
result2: int = multiply(4, 7)
print("5 + 3 =", result1)
print("4 * 7 =", result2)
```

```bash
$ tpy main.py
5 + 3 = 8
4 * 7 = 28
```

### More Examples

Check out [tests/fixtures/valid](tests/fixtures/valid) for 169+ working example programs covering:
- All primitive types and operators
- Control flow variations (if/while/for/with/try)
- Exception handling (try/except/finally/raise)
- Comprehensions (list/dict/set/generator)
- Generator functions with yield
- Class definitions and methods
- Module imports
- Slicing and indexing
- String formatting
- Assert and del statements

-----

## 🧪 Testing

TypePython has comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run specific test category
cargo test test_valid      # Valid programs (should compile)
cargo test test_invalid    # Invalid programs (should fail)
cargo test test_cli        # CLI interface tests

# Run with coverage report
cargo llvm-cov --html
```

**Test Statistics**:
- **169 valid tests** — Programs that should compile and run
- **124 invalid tests** — Programs that should fail compilation
- **7 CLI tests** — Command-line interface behavior
- **Overall coverage**: 81.21%

-----

## 📖 Language Guide

### Type Annotations

**All variables must be type-annotated**:

```python
# ✅ Correct
x: int = 10
name: str = "Alice"
items: list[int] = [1, 2, 3]

# ❌ Wrong (no type inference)
y = 10  # Error: missing type annotation
```

### Function Signatures

**Functions must declare parameter and return types**:

```python
# ✅ Correct
def add(a: int, b: int) -> int:
    return a + b

def greet(name: str) -> None:
    print("Hello,", name)

# ❌ Wrong
def bad_func(x):  # Error: missing parameter type
    return x * 2
```

### Collections

**Use type parameters for generic collections**:

```python
# Lists (homogeneous)
numbers: list[int] = [1, 2, 3]
names: list[str] = ["Alice", "Bob"]

# Dictionaries
ages: dict[str, int] = {"Alice": 30}

# Sets
unique: set[int] = {1, 2, 3}

# Tuples (heterogeneous)
point: tuple[int, int] = (10, 20)
mixed: tuple[str, int, bool] = ("Alice", 30, True)
```

### Type Restrictions

**No union types in Phase 1**:

```python
# ❌ Not supported yet
def find(items: list[int], target: int) -> int | None:
    # Union types (int | None) not implemented
    pass

# ✅ Workaround: use sentinel values
def find(items: list[int], target: int) -> int:
    for i in range(len(items)):
        if items[i] == target:
            return i
    return -1  # Use -1 as "not found"
```

### Control Flow

**Standard Python control flow is supported**:

```python
# If/elif/else
if x > 0:
    print("positive")
elif x < 0:
    print("negative")
else:
    print("zero")

# While loops
i: int = 0
while i < 10:
    print(i)
    i += 1

# For loops
for x in [1, 2, 3, 4, 5]:
    if x == 3:
        continue
    print(x)

# Try/except
try:
    risky_operation()
except ValueError:
    print("Value error occurred")
except Exception:
    print("Other error")
finally:
    cleanup()

# With statement (context managers)
with open("file.txt") as f:
    content: str = f.read()
    print(content)

# Assert statement
assert x > 0, "x must be positive"

# Del statement
my_dict: dict[str, int] = {"a": 1, "b": 2}
del my_dict["a"]

my_list: list[int] = [1, 2, 3, 4, 5]
del my_list[0]

# Slicing
numbers: list[int] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
print(numbers[2:5])      # [2, 3, 4]
print(numbers[:3])       # [0, 1, 2]
print(numbers[::2])      # [0, 2, 4, 6, 8]
print(numbers[::-1])     # [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]

# String formatting
name: str = "Alice"
age: int = 30
print("Hello %s" % name)
print("%s is %d years old" % (name, age))
```

-----

## 🔧 Compiler Options

```bash
# Compile and run
tpy script.py

# Compile only (produce executable)
tpy -o output script.py

# Show LLVM IR
tpy --emit-llvm script.py

# Verbose output
tpy -v script.py

# Show help
tpy --help
```

-----

## 🐛 Common Issues

### "Union types not supported"

```python
# ❌ Error
def func(x: int | None) -> None:
    pass
```

**Solution**: Phase 1 doesn't support union types. Use alternative designs like sentinel values or multiple functions.

### "Missing type annotation"

```python
# ❌ Error
x = 10
```

**Solution**: Always annotate variable types:
```python
# ✅ Correct
x: int = 10
```

### "Cannot infer type"

```python
# ❌ Error
items = []
```

**Solution**: Specify the element type:
```python
# ✅ Correct
items: list[int] = []
```

-----

## 🤝 Contributing

Contributions are welcome! Here's how to help:

### Areas for Contribution
- 🐛 **Bug fixes** — Report issues or submit fixes
- ✨ **Phase 2 features** — Implement lambdas, decorators, etc.
- 📚 **Documentation** — Improve guides and examples
- 🧪 **Testing** — Add more test cases
- ⚡ **Performance** — Optimize codegen or runtime

### Development Setup

```bash
# Clone and build
git clone https://github.com/yourusername/tpy.git
cd tpy
cargo build

# Run tests
cargo test

# Run with coverage
cargo llvm-cov

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Contribution Guidelines
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Add tests for new features
4. Ensure `cargo test` passes
5. Run `cargo fmt` and `cargo clippy`
6. Submit a pull request

-----

## 📚 Technical Details

### Memory Management
- **Stack allocation**: Local variables use LLVM `alloca`
- **Heap allocation**: Collections (list/dict/set) allocated on heap
- **No GC**: C-style manual memory (potential for leaks)

### Function Mangling
Functions are mangled to handle imports:
```
module_name::function_name → module_name_function_name
```

### Type Dispatch
Operations dispatch to type-specific C functions:
```
print(int)       → print_int
print(str)       → print_str
list[int].append → list_append
list[str].append → str_list_append
```

### LLVM Optimization
- Clang with `-O2` optimization
- Link-Time Optimization (LTO) enabled
- Dead code elimination
- Inline expansion

-----

## 🙏 Acknowledgments

Built with these excellent projects:

  - [**PyO3**](https://pyo3.rs) — Rust bindings for Python
  - [**Inkwell**](https://github.com/TheDan64/inkwell) — Safe LLVM bindings for Rust
  - [**LLVM**](https://llvm.org) — Compiler infrastructure
  - [**Serde**](https://serde.rs) — Serialization framework

-----

## 📄 License

MIT License - see [LICENSE](LICENSE) for details

-----

<p align="center">
<strong>Made with 🦀 and ❤️</strong>
</p>

<p align="center">
<sub>TypePython: Where Python syntax meets native performance.</sub>
</p>
