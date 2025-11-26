use clap::Parser as ClapParser;
use pest::Parser;
use std::fs;
use std::path::PathBuf;
use tpy::{LangParser, Rule};

#[derive(ClapParser, Debug)]
#[command(name = "tpy")]
#[command(about = "A compiler for TypePython language", long_about = None)]
pub struct Args {
    /// Input source files
    #[arg(required = true)]
    pub input: Vec<PathBuf>,

    /// Arguments passed directly to LLVM
    #[arg(last = true)]
    pub llvm_args: Vec<String>,
}

fn print_pairs(pairs: pest::iterators::Pairs<Rule>, indent: usize) {
    for pair in pairs {
        let rule = pair.as_rule();
        let span = pair.as_span();
        let text = span.as_str();

        println!(
            "{:indent$}{:?} {:?} @ {}..{}",
            "",
            rule,
            text,
            span.start(),
            span.end(),
            indent = indent
        );

        print_pairs(pair.into_inner(), indent + 2);
    }
}

fn main() {
    let args = Args::parse();

    for path in &args.input {
        println!("=== {} ===", path.display());

        let source = fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {}", path.display(), e);
            std::process::exit(1);
        });

        match LangParser::parse(Rule::program, &source) {
            Ok(pairs) => print_pairs(pairs, 0),
            Err(e) => {
                eprintln!("Parse error in {}:\n{}", path.display(), e);
                std::process::exit(1);
            }
        }
    }

    if !args.llvm_args.is_empty() {
        println!("\nLLVM args: {:?}", args.llvm_args);
    }
}
