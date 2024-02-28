use lazy_static::lazy_static;
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};
use pest_derive::Parser;

use crate::ast::{BinaryOp, Node, NodeKind, UnaryOp};

#[derive(Parser)]
#[grammar = "rox.pest"]
pub struct RoxParser;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};

        PrattParser::new()
            .op(Op::infix(Rule::eqeq, Left) | Op::infix(Rule::neq, Left))
            .op(Op::infix(Rule::add, Left) | Op::infix(Rule::sub, Left))
            .op(Op::infix(Rule::mul, Left) | Op::infix(Rule::div, Left))
            .op(Op::prefix(Rule::neg))
    };
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Node {
    PRATT_PARSER
        .map_primary(|primary| {
            let linecol = primary.line_col();
            let kind = match primary.as_rule() {
                Rule::number => NodeKind::Number(primary.as_str().parse().unwrap()),
                Rule::expr => parse_expr(primary.into_inner()).kind,
                Rule::bool => NodeKind::Bool(primary.as_str().parse().unwrap()),
                Rule::nil => NodeKind::Nil,
                r => unreachable!("primary rule {:?}", r),
            };
            Node { kind, linecol }
        })
        .map_infix(|lhs, op, rhs| {
            let linecol = lhs.linecol;
            let op = match op.as_rule() {
                Rule::add => BinaryOp::Add,
                Rule::sub => BinaryOp::Sub,
                Rule::mul => BinaryOp::Mul,
                Rule::div => BinaryOp::Div,
                Rule::eqeq => BinaryOp::EqEq,
                Rule::neq => BinaryOp::Neq,
                _ => unreachable!(),
            };
            let kind = NodeKind::BinaryOp(Box::new(lhs), op, Box::new(rhs));
            Node { kind, linecol }
        })
        .map_prefix(|op, rhs| {
            let linecol = op.line_col();
            let op = match op.as_rule() {
                Rule::neg => UnaryOp::Neg,
                _ => unreachable!(),
            };
            let kind = NodeKind::UnaryOp(op, Box::new(rhs));
            Node { kind, linecol }
        })
        .parse(pairs)
}
