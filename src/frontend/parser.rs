use crate::frontend::ast;
use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[grammar = "frontend/grammar.pest"] // relative path to your grammar file
pub struct ChiParser;

impl ChiParser {
    pub fn parse_file(path: &Path) -> Result<ast::Program, Rule> {
        let contents = fs::read_to_string(path).unwrap();
        let pest_output = ChiParser::parse(Rule::program, &contents).unwrap();
        println!("{:?}", pest_output);
        let prog = ast::Program {
            global_declarations: vec![],
            functions: vec![],
            use_statements: vec![],
        };
        Ok(prog)
    }
}
