pub mod ast;
pub mod env;
pub mod interpreter;
pub mod parser_pest;

use std::io::{self, BufRead, Read, Write};

use color_eyre::eyre::Result;
use interpreter::Interpreter;
use parser_pest::{parse_program, RoxParser, Rule};
use pest::Parser;

fn main() -> Result<()> {
    let mut intp = Interpreter::new();

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: rox [script]");
        std::process::exit(64);
    }

    if args.len() == 2 {
        run_file(&mut intp, &args[1])
    } else {
        if atty::is(atty::Stream::Stdin) {
            run_repl(&mut intp)
        } else {
            let mut program = String::new();
            io::stdin().read_to_string(&mut program)?;
            run_file_content(&mut intp, "stdin", &program)
        }
    }
}

fn run_file(intp: &mut Interpreter, filename: &str) -> Result<()> {
    let program = std::fs::read_to_string(filename)?;
    run_file_content(intp, filename, &program)
}

fn run_file_content(intp: &mut Interpreter, _filename: &str, content: &str) -> Result<()> {
    let program = RoxParser::parse(Rule::program, &content)?;

    let ast = parse_program(program);
    ast.print_debug();

    let result = intp.interpret_ast(&ast);
    match result {
        Ok(_) => (),
        Err(re) => eprintln!("{}", re),
    }
    Ok(())
}

fn run_repl(intp: &mut Interpreter) -> Result<()> {
    let mut linecount = 0;
    let mut lines = io::stdin().lock().lines();
    loop {
        linecount += 1;
        let fname = format!("repl[{}]", linecount);
        print!("repl[{}]> ", linecount);
        io::stdout().flush()?;

        let line = match lines.next() {
            Some(line) => line?,
            None => break,
        };

        let program = match RoxParser::parse(Rule::program, &line) {
            Ok(program) => program,
            Err(e) => {
                eprintln!("parse error: {}", e.with_path(&fname));
                continue;
            }
        };

        let ast = parse_program(program);
        ast.print_debug();

        let result = intp.interpret_ast(&ast);
        match result {
            Ok(_) => (),
            Err(re) => eprintln!("{}", re),
        }
    }
    Ok(())
}
