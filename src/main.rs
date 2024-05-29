// main.rs
#[macro_use]
extern crate pest_derive;

use std::env; 

pub mod frontend {
    pub mod ast;
    pub mod parser;
}

use frontend::parser::ChiParser;

fn main() {
    for arg in env::args().skip(1) {
        frontend::parser::ChiParser::parse_file(arg.as_ref()).unwrap();
    }
}
