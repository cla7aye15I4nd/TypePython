use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

pub use pest::iterators::Pairs;
pub use pest::Parser;

// Public modules - now organized in folders
pub mod ast;
pub mod codegen;
pub mod module;
pub mod pipeline;
pub mod preprocessor;
