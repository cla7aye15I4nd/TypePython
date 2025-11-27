use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

pub use pest::iterators::Pairs;
pub use pest::Parser;

// Public modules
pub mod ast;
pub mod codegen;
pub mod pest_to_ast;
pub mod preprocessor;
