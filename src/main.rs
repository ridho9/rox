use std::io::{self, BufRead};

use lazy_static::lazy_static;
use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "rox.pest"]
pub struct RoxParser;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};

        PrattParser::new()
            .op(Op::infix(Rule::add, Left) | Op::infix(Rule::sub, Left))
            .op(Op::infix(Rule::mul, Left) | Op::infix(Rule::div, Left))
            .op(Op::prefix(Rule::neg))
    };
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Expr::Number(primary.as_str().parse().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            r => unreachable!("primary rule {:?}", r),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => BinaryOp::Add,
                Rule::sub => BinaryOp::Sub,
                Rule::mul => BinaryOp::Mul,
                Rule::div => BinaryOp::Div,
                _ => unreachable!(),
            };
            Expr::BinaryOp(Box::new(lhs), op, Box::new(rhs))
        })
        .map_prefix(|op, rhs| {
            let op = match op.as_rule() {
                Rule::neg => UnaryOp::Neg,
                _ => unreachable!(),
            };
            Expr::UnaryOp(op, Box::new(rhs))
        })
        .parse(pairs)
}

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match RoxParser::parse(Rule::program, &line?) {
            Ok(mut pairs) => {
                println!("{:#?}", pairs);
                println!(
                    "Parsed: {:#?}",
                    parse_expr(
                        // inner of expr
                        pairs
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .into_inner()
                    )
                );
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
    }
    Ok(())
}
