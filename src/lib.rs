extern crate pest;
#[macro_use] extern crate pest_derive;
use pest::Parser;
use std::collections::HashMap;
#[macro_use] extern crate failure;

pub type FilterCallback = fn(String) -> String;
pub type FilterHashMap = HashMap<String, FilterCallback>;
pub type VarHashMap = HashMap<String, String>;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("./ident.pest");

#[derive(Parser)]
#[grammar = "ident.pest"]
struct IdentParser;

///Given a template string parse the shit out of it.
pub fn parse(template: &str)  -> Result<pest::iterators::Pairs<Rule>, pest::Error<Rule>> {
   IdentParser::parse(Rule::ident_list, template)
}

pub type PestError<'a> = pest::Error<'a,Rule>;

pub mod errors;
pub mod parser;
pub mod utils;
