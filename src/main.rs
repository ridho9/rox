pub mod ast;
pub mod parser;

use std::io::{self, BufRead};

use parser::{RoxParser, Rule};
use pest::Parser;

use crate::parser::parse_expr;

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match RoxParser::parse(Rule::program, &line?) {
            Ok(mut pairs) => {
                // println!("{:#?}", pairs);
                println!(
                    "Parsed: {:#?}",
                    parse_expr(
                        // inner of expr
                        pairs
                            .next() // program
                            .unwrap()
                            .into_inner()
                            .next() // expr
                            .unwrap()
                            .into_inner()
                    )
                );
            }
            Err(e) => {
                eprintln!("Parse failed: {}", e);
            }
        }
    }
    Ok(())
}
