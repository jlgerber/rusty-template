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

pub type PestError<'a> = pest::Error<'a,Rule>;

pub mod errors;
pub mod parser;
pub mod utils;

/// Given a template string parse the shit out of it.
pub fn parse(template: &str)  -> Result<pest::iterators::Pairs<Rule>, pest::Error<Rule>> {
    IdentParser::parse(Rule::ident_list, template)
}

// pub fn printout(parsed: Result<pest::iterators::Pairs<Rule>, pest::Error<Rule>>) {
//     let pairs = parsed.unwrap_or_else(|e| panic!("{}", e));
//     // Because ident_list is silent, the iterator will contain idents
//     for pair in pairs {
//         let span = pair.clone().into_span();

//         // A pair is a combination of the rule which matched and a span of input
//         println!("Rule:    {:?}", pair.as_rule());
//         println!("Span:    {:?}", span);
//         println!("Text:    {}", span.as_str());

//         // A pair can be converted to an iterator of the tokens which make it up:
//         for inner_pair in pair.into_inner() {
//             let inner_span = inner_pair.clone().into_span();
//             match inner_pair.as_rule() {
//                 Rule::alpha => println!("Letter:  {}", inner_span.as_str()),
//                 Rule::digit => println!("Digit:   {}", inner_span.as_str()),
//                 Rule::word =>  println!("Word:    {}", inner_span.as_str()),
//                 Rule::trans => println!("trans:   {}", inner_span.as_str()),
//                 Rule::var =>   println!("var:     {}", inner_span.as_str()),
//                 Rule::path =>  println!("path:    {}", inner_span.as_str()),
//                 _ => unreachable!()
//             };
//         }
//         println!("");
//     }
// }