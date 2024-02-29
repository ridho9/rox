pub mod ast;
pub mod interpreter;
pub mod parser;

use std::io::{self, BufRead};

use parser::{RoxParser, Rule};
use pest::Parser;

use crate::{interpreter::Interpreter, parser::parse_expr};

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match RoxParser::parse(Rule::program, &line?) {
            Ok(mut pairs) => {
                // println!("{:#?}", pairs);
                let ast = parse_expr(
                    pairs
                        .next() // program
                        .unwrap()
                        .into_inner()
                        .next() // expr
                        .unwrap()
                        .into_inner(),
                );
                ast.print();

                let mut intp = Interpreter::new();
                println!("=====>");
                let result = intp.interpret_ast(&ast);
                match result {
                    Ok(result) => println!("{:?}", result),
                    Err(re) => println!("{}", re),
                }
            }
            Err(e) => {
                eprintln!("Parse failed: {}", e);
            }
        }
    }
    Ok(())
}
