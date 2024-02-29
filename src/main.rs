pub mod ast;
pub mod interpreter;
pub mod parser;

use std::io::{self, BufRead};

use parser::{RoxParser, Rule};
use pest::Parser;

use crate::{interpreter::Interpreter, parser::parse_program};

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match RoxParser::parse(Rule::program, &line?) {
            Ok(mut program) => {
                let ast = parse_program(program.next().unwrap().into_inner());

                #[cfg(debug)]
                ast.print_debug();

                let mut intp = Interpreter::new();
                let result = intp.interpret_ast(&ast);
                match result {
                    Ok(_) => (),
                    Err(re) => eprintln!("{}", re),
                }
            }
            Err(e) => {
                eprintln!("parse failed: {}", e);
            }
        }
    }
    Ok(())
}
