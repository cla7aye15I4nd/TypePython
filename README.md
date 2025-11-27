<p align="center">
<img src="https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&logo=rust" alt="Rust">
<img src="https://img.shields.io/badge/backend-LLVM-blue?style=for-the-badge&logo=llvm" alt="LLVM">
<img src="https://img.shields.io/badge/parser-Pest-green?style=for-the-badge" alt="Pest">
</p>
<h1 align="center">
🐍⚡ <code>tpy</code> — TypePython Compiler
</h1>

<p align="center">
<strong>A blazingly fast compiler for statically-typed Python, powered by LLVM</strong>
</p>

<p align="center">
<em>Write Python. Get type safety. Compile to native code.</em>
</p>

-----

## ✨ What is TypePython?

**TypePython** (`tpy`) bridges the gap between Python's elegant syntax and systems-level performance. It's a statically-typed subset of Python that compiles directly to native machine code via LLVM, giving you:

  - 🎯 **Python-like syntax** — Familiar, readable, and expressive
  - 🔒 **Static typing** — Catch errors at compile time, not runtime
  - ⚡ **Native performance** — LLVM-optimized machine code
  - 🦀 **Rust-powered toolchain** — Memory-safe compiler implementation

-----

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         tpy Compiler Pipeline                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│   ┌──────────┐    ┌──────────┐    ┌─────────┐    ┌──────────────┐   │
│   │  Source  │───▶│Preprocess│───▶│  Parse  │───▶│   Build AST  │   │
│   │   .py    │    │ (indent) │    │  (Pest) │    │              │   │
│   └──────────┘    └──────────┘    └─────────┘    └──────┬───────┘   │
│                                                         │           │
│   ┌──────────┐    ┌──────────┐    ┌─────────┐           │           │
│   │  Native  │◀───│   Link   │◀───│ LLVM IR │◀─────────-┘           │
│   │  Binary  │    │  (clang) │    │ CodeGen │                       │
│   └──────────┘    └──────────┘    └─────────┘                       │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

| Stage | Technology | Description |
|-------|------------|-------------|
| **Preprocessing** | Custom | Converts Python indentation to explicit `⟨INDENT⟩`/`⟨DEDENT⟩` tokens |
| **Parsing** | [Pest](https://pest.rs) | PEG-based grammar for clean, maintainable parser definitions |
| **AST** | Rust | Strongly-typed abstract syntax tree representation |
| **Code Generation** | [Inkwell](https://github.com/TheDan64/inkwell) | Safe Rust bindings to LLVM |
| **Linking** | Clang/LLVM | Final native binary generation |

-----

## 🚀 Quick Start

#### Prerequisites

  - **Rust** Stable (install via [rustup](https://rustup.rs))
  - **LLVM 21** (including development headers)
  - **Dependencies**: `build-essential`, `pkg-config`, `libssl-dev`, `libzstd-dev`
  - **Clang** (part of LLVM toolchain)

#### Build Steps

```bash
# Clone the repository
git clone https://github.com/yourusername/tpy.git
cd tpy

# Set LLVM path (example for Ubuntu/Debian with llvm.sh)
export LLVM_SYS_211_PREFIX=/usr/lib/llvm-21

# Build the compiler
cargo build --release

# Add to PATH (optional)
export PATH="$PATH:$(pwd)/target/release"
```

### Example
```shell
$ cat > fib.py << 'EOF'
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    
    a: int = 0
    b: int = 1
    
    # Use tuple unpacking and a for-loop for cleaner iteration
    for _ in range(n - 1):
        a, b = b, a + b
        
    return b

if __name__ == "__main__":
    n: int = 10
    print(f"10th Fibonacci number: {fibonacci(n)}")
EOF

$ tpy fib.py -o fib
$ ./fib
# Output: 10th Fibonacci number: 55
```

## 🤝 Contributing

Contributions are welcome\! Here are some areas where help is appreciated:

  - 🐛 **Bug fixes** — Found an issue? Open a PR\!
  - ✨ **New features** — Arrays, structs, imports, etc.
  - 📚 **Documentation** — Improve examples and guides
  - 🧪 **Testing** — Add more test cases

## 🙏 Acknowledgments

Built with these amazing projects:

  - [**Pest**](https://pest.rs) — The elegant parser generator
  - [**Inkwell**](https://github.com/TheDan64/inkwell) — Safe LLVM bindings for Rust
  - [**LLVM**](https://llvm.org) — The compiler infrastructure

-----

<p align="center">
<strong>Made with 🦀 and ❤️</strong>
</p>

<p align="center">
<sub>TypePython: Where Python meets performance.</sub>
</p>