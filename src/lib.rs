#[macro_use] extern crate failure;
#[macro_use] extern crate pest_derive;
#[macro_use] extern crate log;
extern crate pest;

use pest::Parser;
use std::collections::HashMap;

// types
pub type FilterCallback = fn(String) -> String;
pub type FilterHashMap = HashMap<String, FilterCallback>;
pub type VarHashMap = HashMap<String, String>;
pub type PestError<'a> = pest::Error<'a,Rule>;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("./ident.pest");

#[derive(Parser)]
#[grammar = "ident.pest"]
struct IdentParser;

/// Given a template &str, parse it using the IdentParser generated with the Parser
/// procedural macro.
// NB This function needs to stay in this module, as the parse method isn't generated
// in time to import IdentParser into another module.
pub fn parse(template: &str)  -> Result<pest::iterators::Pairs<Rule>, pest::Error<Rule>> {
   IdentParser::parse(Rule::ident_list, template)
}

// modules
pub mod errors;
pub mod parser;
pub mod utils;
