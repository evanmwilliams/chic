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
        //  println!("{:?}", pest_output);

        let top_level = pest_output.into_iter().next().unwrap().into_inner();
        // println!("{:?}", top_level);

        let mut uses = vec![];
        let mut global_decls = vec![];

        for pair in top_level {
            match pair.as_rule() {
                Rule::global_decl => {
                    let decl = Self::parse_decl(pair);
                    global_decls.push(decl);
                }
                Rule::use_stmt => {
                    let use_statement = Self::parse_use_stmt(pair);
                    uses.push(use_statement);
                }
                Rule::function_decl => {}
                _ => unreachable!(),
            }
        }
        let prog = ast::Program {
            global_declarations: vec![],
            functions: vec![],
            use_statements: uses,
        };
        // println!("{:?}", prog);
        Ok(prog)
    }

    fn parse_use_stmt(pair: pest::iterators::Pair<Rule>) -> ast::UseStmt {
        let mut inner = pair.into_inner();
        match inner.peek().unwrap().as_rule() {
            Rule::identifier => {
                let identifier = inner.next().unwrap().as_str().to_string();
                ast::UseStmt { identifier }
            }
            _ => unreachable!(),
        }
    }

    fn parse_decl(pair: pest::iterators::Pair<Rule>) -> ast::Declaration {
        let mut inner = pair.into_inner();

        let mut ids = vec![];
        let mut exprs = vec![];

        let var_type = inner
            .peek()
            .unwrap()
            .into_inner()
            .peek()
            .unwrap()
            .into_inner()
            .peek()
            .unwrap()
            .as_rule();
        let decl_type = match var_type {
            Rule::primitive_type => {
                let primitive_type = inner.peek().unwrap().into_inner().next().unwrap();
                Self::parse_primitive_type(primitive_type)
            }
            Rule::array_type => {
                todo!()
            }
            _ => {
                println!("Unreachable block 1");
                unreachable!()
            }
        };

        for pair in inner.next().unwrap().into_inner() {
            match pair.as_rule() {
                Rule::decl_type => {
                    //TODO MOVE decl_type FROM EARLIER TO HERE
                }
                Rule::identifier => ids.push(pair.as_str().to_string()),
                Rule::expr => {
                    let expr = Self::parse_expr(pair);
                    println!("{:?}", expr);
                    exprs.push(expr);
                }
                Rule::semi | Rule::assign => {}
                _ => {
                    println!("Unreachable block 2");
                    unreachable!()
                }
            }
        }

        if exprs.len() > 0 && ids.len() != exprs.len() {
            panic!("Number of identifiers and expressions do not match");
        }

        ast::Declaration {
            decl_type,
            ids,
            exprs,
        }
    }

    fn parse_primitive_type(pair: pest::iterators::Pair<Rule>) -> ast::Type {
        let mut inner = pair.into_inner().next().unwrap().into_inner();
        match inner.next().unwrap().as_rule() {
            Rule::int_type => ast::Type::Int,
            Rule::char_type => ast::Type::Char,
            Rule::bool_type => ast::Type::Bool,
            _ => {
                println!("Unreachable block in parse_primitive_type");
                unreachable!()
            }
        }
    }

    fn parse_expr(pair: pest::iterators::Pair<Rule>) -> ast::Expr {
        let expr = match pair.clone().into_inner().next().unwrap().as_rule() {
            Rule::primary => Self::parse_primary(pair.into_inner().next().unwrap()),
            Rule::binary_expr => Self::parse_binary_expr(pair.into_inner().next().unwrap()),
            Rule::unary_expr => Self::parse_unary_expr(pair.into_inner().next().unwrap()),
            _ => {
                println!("Unreachable block in parse_expr");
                unreachable!()
            }
        };
        expr
    }

    fn parse_primary(pair: pest::iterators::Pair<Rule>) -> ast::Expr {
        match pair.clone().into_inner().next().unwrap().as_rule() {
            Rule::identifier => {
                let identifier = pair.as_str().to_string();
                ast::Expr::Primary(ast::Primary::Identifier(identifier))
            }
            Rule::literal => {
                let lit = Self::parse_literal(pair.into_inner().next().unwrap());
                ast::Expr::Primary(ast::Primary::Literal(lit))
            }
            _ => unreachable!(),
        }
    }

    fn parse_binary_expr(pair: pest::iterators::Pair<Rule>) -> ast::Expr {
        let mut inner = pair.into_inner();
        let lhs = Box::new(Self::parse_primary(inner.next().unwrap()));
        let op_string = inner.next().unwrap().as_str();
        let op = match op_string {
            "+" => ast::Bop::Add,
            "-" => ast::Bop::Sub,
            "*" => ast::Bop::Mul,
            "/" => ast::Bop::Div,
            "%" => ast::Bop::Mod,
            "==" => ast::Bop::Eq,
            "!=" => ast::Bop::Neq,
            "<" => ast::Bop::Lt,
            "<=" => ast::Bop::Lte,
            ">" => ast::Bop::Gt,
            ">=" => ast::Bop::Gte,
            "&" => ast::Bop::And,
            "|" => ast::Bop::Or,
            "*>>" => ast::Bop::HMul,
            _ => {
                println!("Unreachable block in parse_binary_expr");
                unreachable!()
            }
        };
        let rhs = Box::new(Self::parse_primary(inner.next().unwrap()));

        ast::Expr::Binary(ast::BExpr {
            operator: op,
            left: lhs,
            right: rhs,
        })
    }

    fn parse_unary_expr(pair: pest::iterators::Pair<Rule>) -> ast::Expr {
        todo!()
    }

    fn parse_literal(pair: pest::iterators::Pair<Rule>) -> ast::Literal {
        match pair.clone().into_inner().next().unwrap().as_rule() {
            Rule::int_lit => {
                let int_literal = pair.as_str().parse::<i64>().unwrap();
                ast::Literal::Int(int_literal)
            }
            Rule::char_lit => {
                let char_literal = pair.as_str().chars().next().unwrap();
                ast::Literal::Char(char_literal)
            }
            Rule::bool_lit => {
                let bool_literal = pair.as_str().parse::<bool>().unwrap();
                ast::Literal::Bool(bool_literal)
            }
            Rule::string_lit => {
                let string_literal = pair.as_str().to_string();
                ast::Literal::String(string_literal)
            }
            _ => {
                println!("Unreachable block in parse_literal");
                unreachable!()
            }
        }
    }
}
