use clap::Parser as ClapParser;
use inkwell::context::Context;
use std::fs;
use std::path::PathBuf;
use tpy::{codegen::CodeGen, pest_to_ast, LangParser, Parser, Rule};

#[derive(ClapParser, Debug)]
#[command(name = "tpy")]
#[command(about = "A compiler for TypePython language", long_about = None)]
pub struct Args {
    /// Input source files (optional - if none provided, runs examples)
    pub input: Vec<PathBuf>,

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

    if !args.input.is_empty() {
        for path in &args.input {
            let source = fs::read_to_string(path).unwrap_or_else(|e| {
                eprintln!("Error reading {}: {}", path.display(), e);
                std::process::exit(1);
            });

            let pairs = match LangParser::parse(Rule::program, &source).map_err(Box::new) {
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

            // Verify the module
            if let Err(e) = codegen.get_module().verify() {
                eprintln!("Module verification failed: {}", e);
                std::process::exit(1);
            }

            println!("✓ Successfully compiled {}", path.display());
        }
    }

    if !args.llvm_args.is_empty() {
        println!("\nLLVM args: {:?}", args.llvm_args);
    }
}
