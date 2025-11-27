use clap::Parser as ClapParser;
use log::debug;
use std::path::PathBuf;
use std::process::Command;
use tpy::pipeline::{compile_and_link, CompileOptions};

#[derive(ClapParser, Debug)]
#[command(name = "tpy")]
#[command(about = "TypePython compiler - compiles and runs .py files", long_about = None)]
pub struct Args {
    /// Input source file (.py)
    pub input: PathBuf,

    /// Show the preprocessed source (with INDENT/DEDENT markers)
    #[arg(long)]
    pub show_pp: bool,

    /// Show the PEST parse tree
    #[arg(long)]
    pub show_pest: bool,

    /// Show the AST structure
    #[arg(long)]
    pub show_ast: bool,

    /// Show LLVM IR output
    #[arg(long)]
    pub show_ir: bool,

    /// Compile only, don't run the executable
    #[arg(short = 'c', long)]
    pub compile_only: bool,

    /// Output executable name (default: same as input filename)
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,
}

fn main() {
    // Initialize logger
    env_logger::init();

    let args = Args::parse();

    // Verify input file has .py extension
    if args.input.extension().and_then(|s| s.to_str()) != Some("py") {
        eprintln!("Error: Input file must have .py extension");
        std::process::exit(1);
    }

    // Set up compilation options
    let options = CompileOptions {
        show_preprocessed: args.show_pp,
        show_pest: args.show_pest,
        show_ast: args.show_ast,
        show_ir: args.show_ir,
    };

    // Determine output executable name
    let output_path = args.output.unwrap_or_else(|| args.input.with_extension(""));

    // Compile using the pipeline
    debug!(
        "Compiling {} -> {}",
        args.input.display(),
        output_path.display()
    );

    if let Err(e) = compile_and_link(&args.input, &output_path, &options) {
        eprintln!("Compilation error: {}", e);
        std::process::exit(1);
    }

    debug!("Compilation successful!");

    // Run the executable (unless compile-only mode)
    if !args.compile_only {
        debug!("Running {}", output_path.display());
        let run_status = Command::new(&output_path).status().unwrap_or_else(|e| {
            eprintln!("Failed to run executable: {}", e);
            std::process::exit(1);
        });

        std::process::exit(run_status.code().unwrap_or(1));
    }
}
