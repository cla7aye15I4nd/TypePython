use clap::Parser as ClapParser;
use std::path::PathBuf;
use std::process::Command;
use tpy::pipeline::CompileOptions;

#[derive(ClapParser, Debug)]
#[command(name = "tpy")]
#[command(about = "TypePython compiler - compiles and runs .py files", long_about = None)]
pub struct Args {
    /// Input source file (.py)
    pub input: PathBuf,

    /// Dump the preprocessed source (with INDENT/DEDENT markers) to file
    #[arg(long)]
    pub dump_pp: bool,

    /// Dump the PEST parse tree to file
    #[arg(long)]
    pub dump_pest: bool,

    /// Dump the AST structure to file
    #[arg(long)]
    pub dump_ast: bool,

    /// Dump LLVM IR output to file
    #[arg(long)]
    pub dump_ir: bool,

    /// Compile only, don't run the executable
    #[arg(short = 'c', long)]
    pub compile_only: bool,

    /// Output executable name (default: same as input filename)
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    // Verify input file has .py extension
    if args.input.extension().and_then(|s| s.to_str()) != Some("py") {
        eprintln!("Error: Input file must have .py extension");
        std::process::exit(1);
    }

    // Set up compilation options
    let options = CompileOptions {
        dump_preprocessed: args.dump_pp,
        dump_pest: args.dump_pest,
        dump_ast: args.dump_ast,
        dump_ir: args.dump_ir,
    };

    // Determine output executable name
    let output_path = args.output.unwrap_or_else(|| args.input.with_extension(""));

    // Compile using the module-aware pipeline

    if let Err(e) = tpy::pipeline::compile(&args.input, &output_path, &options) {
        eprintln!("Compilation error: {}", e);
        std::process::exit(1);
    }

    // Run the executable (unless compile-only mode)
    if !args.compile_only {
        let run_status = Command::new(&output_path).status().unwrap_or_else(|e| {
            eprintln!("Failed to run executable: {}", e);
            std::process::exit(1);
        });

        std::process::exit(run_status.code().unwrap_or(1));
    }
}
