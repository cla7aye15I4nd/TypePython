//! pycc - Compile Python files to executables
//!
//! Works like gcc: `pycc input.py -o output`

use anyhow::Result;
use clap::Parser;
use compiler::{Compiler, CompilerOptions, Target};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pycc")]
#[command(about = "Compile Python files to executables")]
#[command(version)]
struct Args {
    /// Python file to compile
    input: PathBuf,

    /// Output executable path
    #[arg(short, long)]
    output: PathBuf,

    /// Target architecture (x86_64 or riscv64)
    #[arg(long, default_value = "x86_64")]
    target: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let target: Target = args.target.parse().map_err(|e| anyhow::anyhow!("{}", e))?;

    let options = CompilerOptions {
        target,
        ..Default::default()
    };

    let compiler = Compiler::new(options);
    compiler.compile(&args.input, &args.output)?;

    Ok(())
}
