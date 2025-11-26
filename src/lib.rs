use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

pub use pest::iterators::Pairs;
pub use pest::Parser;
