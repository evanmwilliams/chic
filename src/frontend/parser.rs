#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::path::Path;
use ast;

#[derive(Parser)]
#[grammar = "frontend/grammar.pest"] // relative path to your grammar file
struct ChiParser;

impl ChiParser {
    pub fn parse_file(path: &Path) {}
}
