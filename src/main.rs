// main.rs
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "frontend/grammar.pest"] // relative path to your grammar file
struct MyParser;

fn main() {
    let successful_parse = MyParser::parse(Rule::number, "12345");
    match successful_parse {
        Ok(parsed) => println!("Parsed successfully: {:?}", parsed),
        Err(e) => println!("Parsing failed: {}", e),
    }
}

