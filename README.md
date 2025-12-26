# TypePython

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![LLVM](https://img.shields.io/badge/LLVM-Backend-blue.svg)](https://llvm.org/)
[![Platform](https://img.shields.io/badge/Platform-x86__64%20|%20RISC--V64-green.svg)](#cross-compilation-risc-v-64)
[![Python Syntax](https://img.shields.io/badge/Syntax-Python%203.x-3776ab.svg)](https://www.python.org/)

A statically-typed Python compiler that compiles Python source code to native executables.

TypePython enforces type annotations at compile time and generates efficient native code through LLVM, supporting both x86_64 and RISC-V 64-bit targets via static linking with musl libc.

## Features

### Type System
- **Static typing**: All variables, function parameters, and return types must have explicit type annotations
- **Type inference**: Local variables can have their types inferred from context
- **Compile-time type checking**: Type errors are caught at compile time, not runtime

### Supported Types
- **Primitives**: `int` (64-bit), `float` (64-bit), `bool`, `str`
- **Binary data**: `bytes` (immutable), `bytearray` (mutable)
- **Collections**: `list[T]` (homogeneous, type-checked)
- **Classes**: User-defined classes with single inheritance
- **Iterators**: `range()` for numeric iteration

### Language Features

#### Control Flow
```python
# if/elif/else
if x > 0:
    print("positive")
elif x < 0:
    print("negative")
else:
    print("zero")

# while loops
while x > 0:
    x = x - 1

# for loops with range
for i in range(10):
    print(i)

# for loops with lists
items: list[int] = [1, 2, 3]
for item in items:
    print(item)
```

#### Functions
```python
def add(a: int, b: int) -> int:
    return a + b

def greet(name: str) -> None:
    print("Hello, " + name)
```

#### Classes and Inheritance
```python
class Animal:
    name: str

    def __init__(self, name: str) -> None:
        self.name = name

    def speak(self) -> str:
        return "..."

class Dog(Animal):
    def __init__(self, name: str) -> None:
        super().__init__(name)

    def speak(self) -> str:
        return "Woof!"
```

#### Exception Handling
```python
class MyError(Exception):
    pass

def risky_operation() -> int:
    raise MyError("something went wrong")

try:
    result = risky_operation()
except MyError:
    print("caught MyError")
except Exception:
    print("caught generic exception")
finally:
    print("cleanup")
```

#### Modules and Imports
```python
# Simple import
import mymodule

# Aliased import
import mymodule as m

# Specific imports
from mypackage import func1, func2
```

### Operators
- **Arithmetic**: `+`, `-`, `*`, `/`, `//`, `%`, `**`
- **Comparison**: `==`, `!=`, `<`, `<=`, `>`, `>=`
- **Logical**: `and`, `or`, `not`
- **Bitwise**: `&`, `|`, `^`, `<<`, `>>` (integers only)
- **Augmented assignment**: `+=`, `-=`, `*=`, `/=`, `%=`

### Built-in Functions
- `print(*args)` - Print values to stdout
- `len(obj)` - Length of list, string, bytes, or bytearray
- `range(stop)`, `range(start, stop)`, `range(start, stop, step)` - Create range iterator
- `iter(iterable)` - Get iterator from iterable
- `next(iterator)` - Get next item from iterator

## Installation

### Prerequisites
- Rust (latest stable)
- Python 3.x (for AST parsing via PyO3)
- LLVM 21 (with clang, llvm-ar, llvm-link, llvm-ranlib)
- Build tools (make, gcc for x86_64)
- musl libc (built automatically from source)

### Building from Source

#### Using Dev Container (Recommended)

The easiest way to build TypePython is using the included dev container, which has all dependencies pre-installed including a pre-built musl:

```bash
# Open in VS Code with Dev Containers extension
code .
# Then: Ctrl+Shift+P -> "Dev Containers: Reopen in Container"

# Build the compiler
cargo build --release
```

#### Manual Build

```bash
# Clone the repository
git clone https://github.com/example/TypePython.git
cd TypePython

# Build the compiler (musl will be downloaded and built automatically)
cargo build --release

# The binaries will be at:
# - target/release/pyrun   (compile and run)
# - target/release/pycc    (compile to executable)
```

#### Using Pre-built musl

If you have musl pre-built (e.g., from a package manager or Docker), you can skip the automatic build by setting environment variables:

```bash
# Set paths to pre-built musl installations
export MUSL_X86_64_PREFIX=/path/to/musl-x86_64
export MUSL_RISCV64_PREFIX=/path/to/musl-riscv64

# Build (will use pre-built musl)
cargo build --release
```

Each musl prefix directory should contain:
- `lib/crt1.o`, `lib/crti.o`, `lib/crtn.o`, `lib/libc.a`
- `include/` with musl headers

## Usage

### Compile and Run
```bash
# Run a Python file directly
./target/release/pyrun examples/hello.py

# With verbose output
./target/release/pyrun -v examples/hello.py
```

### Compile to Executable
```bash
# Compile to native executable
./target/release/pycc examples/hello.py -o hello

# Run the compiled executable
./hello
```

### Cross-Compilation (RISC-V 64)
```bash
# Compile for RISC-V 64-bit
./target/release/pycc --target riscv64 examples/hello.py -o hello_riscv

# Run with QEMU
qemu-riscv64 ./hello_riscv
```

## Architecture

TypePython uses a multi-stage compilation pipeline:

```
Python Source
     │
     ▼
┌─────────────────┐
│ Python AST      │  ← PyO3 bindings to Python's ast module
│ (via PyO3)      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Internal AST    │  ← Rust-native AST representation
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ TIR             │  ← Typed Intermediate Representation
│ (Type-checked)  │     with constraint-based type inference
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ LLVM IR         │  ← Generated via Inkwell
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Native Code     │  ← Linked with musl libc + runtime
└─────────────────┘
```

### Key Components

| Component | Description |
|-----------|-------------|
| `compiler/src/ast/` | Python AST to internal AST conversion |
| `compiler/src/tir/` | Typed IR with type inference and resolution |
| `compiler/src/codegen/` | LLVM IR generation via Inkwell |
| `runtime/src/` | C runtime library for Python objects |

## Testing

```bash
# Run all tests
cargo test

# Run specific test category
cargo test basic
cargo test algorithm
cargo test exceptions
```

The test suite includes 158 Python test files covering:
- Basic operations (primitives, operators, control flow)
- Algorithms (factorial, fibonacci, recursion)
- Data structures (HashMap, HashSet, BST, MinHeap)
- Exception handling
- Class inheritance
- Module imports
- Type inference
- Stress tests

## Limitations

TypePython intentionally does **not** support certain Python features by design:

### By Design (Not Planned)
These features are intentionally excluded to maintain TypePython's static typing guarantees:
- **`None` type**: No null values - all variables must have concrete values
- **Union types**: No `Union[A, B]` or `A | B` - each variable has exactly one type
- **`__init__.py` packages**: Modules are files, not directories with init files
- **Dynamic typing**: No `Any` type or runtime type changes

### Not Yet Implemented
These features may be added in future versions:
- Multiple inheritance
- Decorators
- Generators (`yield`)
- Lambda expressions
- List comprehensions
- Async/await
- `with` statements (context managers)
- `*args` and `**kwargs` (except for `print`)
- Global/nonlocal declarations
- Dictionaries and sets

## Project Structure

```
TypePython/
├── compiler/           # Main compiler crate
│   └── src/
│       ├── ast/       # AST types and conversion
│       ├── tir/       # Typed IR and lowering
│       └── codegen/   # LLVM code generation
├── runtime/           # C runtime library
│   └── src/
│       ├── list.c     # List implementation
│       ├── str.c      # String implementation
│       ├── bytes.c    # Bytes implementation
│       ├── range.c    # Range iterator
│       └── exception.c # Exception handling
├── src/               # CLI tools (pyrun, pycc)
├── test/              # Python test files
└── tests/             # Rust integration tests
```

## License

Apache License 2.0

## Roadmap

See the following documents for planned features:
- [TODO-python-features.md](TODO-python-features.md) - Making TypePython more like Python
- [TODO-nostd.md](TODO-nostd.md) - nostd mode for OS development
