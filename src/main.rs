use clap::Parser as ClapParser;
use inkwell::context::Context;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tpy::{codegen::CodeGen, pest_to_ast, preprocessor, LangParser, Parser, Rule};

#[derive(ClapParser, Debug)]
#[command(name = "tpy")]
#[command(about = "A compiler for TypePython language", long_about = None)]
pub struct Args {
    /// Input source files (optional - if none provided, runs examples)
    pub input: Vec<PathBuf>,

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

    /// Arguments passed directly to LLVM
    #[arg(last = true)]
    pub llvm_args: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let mut bc_files: Vec<String> = vec![];
    let mut other_files: Vec<String> = vec![];

    if !args.input.is_empty() {
        for path in &args.input {
            if path.extension().and_then(|s| s.to_str()) != Some("py") {
                other_files.push(path.to_string_lossy().to_string());
                continue;
            }

            let source = fs::read_to_string(path).unwrap_or_else(|e| {
                eprintln!("Error reading {}: {}", path.display(), e);
                std::process::exit(1);
            });

            // Preprocess: convert indentation to explicit tokens
            let preprocessed = match preprocessor::preprocess(&source) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Preprocessing error in {}:\n{}", path.display(), e);
                    std::process::exit(1);
                }
            };

            if args.show_pp {
                println!("\n--- Preprocessed Source ---");
                println!("{}", preprocessed);
            }

            let pairs = match LangParser::parse(Rule::program, &preprocessed).map_err(Box::new) {
                Ok(pairs) => pairs,
                Err(e) => {
                    eprintln!("Parse error in {}:\n{}", path.display(), e);
                    std::process::exit(1);
                }
            };

            if args.show_pest {
                println!("\n--- PEST Parse Tree ---");
                for pair in pairs.clone() {
                    println!("{:#?}", pair);
                }
            }

            // Parse Pest AST to our AST
            let program = pest_to_ast::build_program(pairs);

            if args.show_ast {
                println!("\n--- AST ---");
                println!("{:#?}", program);
            }

            // Generate LLVM IR
            let context = Context::create();
            let mut codegen = CodeGen::new(&context, path.file_stem().unwrap().to_str().unwrap());

            if let Err(e) = codegen.generate(&program) {
                eprintln!("Code generation error: {}", e);
                std::process::exit(1);
            }

            if args.show_ir {
                println!("\n--- LLVM IR ---");
                println!("{}", codegen.get_module().print_to_string().to_string());
            }

            if let Err(e) = codegen.get_module().verify() {
                eprintln!("Module verification failed: {}", e);
                std::process::exit(1);
            }

            let bc_path = path.with_extension("bc");
            codegen.get_module().write_bitcode_to_path(&bc_path);
            bc_files.push(bc_path.to_string_lossy().to_string());
        }
    }

    match std::env::var("LLVM_SYS_211_PREFIX") {
        Err(_) => {
            eprintln!("Error: LLVM_SYS_211_PREFIX environment variable is not set.");
            std::process::exit(1);
        }
        Ok(prefix) => {
            let llc = format!("{}/bin/clang", prefix);

            let status = Command::new(&llc)
                .args(&bc_files)
                .args(&other_files)
                .args(&args.llvm_args)
                .status()
                .expect("failed to execute llc");

            std::process::exit(status.code().unwrap_or(1));
        }
    }
}
