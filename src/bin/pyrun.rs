//! pyrun - Python-like interpreter interface
//!
//! Works like the Python interpreter: `pyrun script.py`

use anyhow::Result;
use clap::Parser;
use compiler::{Compiler, CompilerOptions, Target};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pyrun")]
#[command(about = "Run Python files - works like the Python interpreter")]
#[command(version)]
struct Args {
    /// Python file to run
    input: PathBuf,

    /// Target architecture (x86_64 or riscv64)
    #[arg(long, default_value = "x86_64")]
    target: String,

    /// Emit AST (for debugging)
    #[arg(long)]
    emit_ast: bool,

    /// Emit LLVM IR (for debugging)
    #[arg(long)]
    emit_llvm: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let target: Target = args.target.parse().map_err(|e| anyhow::anyhow!("{}", e))?;

    let options = CompilerOptions {
        emit_ast: args.emit_ast,
        emit_llvm: args.emit_llvm,
        target,
    };

    let compiler = Compiler::new(options);
    compiler.run(&args.input, &[])?;

    Ok(())
}
