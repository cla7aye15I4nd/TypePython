use clap::Parser as ClapParser;
use std::path::PathBuf;
use tpy::cli::{compile_and_run, RunResult};

#[derive(ClapParser, Debug)]
#[command(name = "tpy")]
#[command(about = "TypePython compiler - compiles and runs .py files", long_about = None)]
pub struct Args {
    /// Input source file (.py)
    pub input: PathBuf,

    /// Compile only, don't run the executable
    #[arg(short = 'c', long)]
    pub compile_only: bool,

    /// Output executable name (default: same as input filename)
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    match compile_and_run(&args.input, args.output.as_deref(), args.compile_only) {
        RunResult::Completed(code) => std::process::exit(code),
        RunResult::CompileError(e) => {
            eprintln!("Compilation error: {}", e);
            std::process::exit(1);
        }
        RunResult::ExecError(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
