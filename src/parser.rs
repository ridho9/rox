use lazy_static::lazy_static;
use pest::{iterators::Pairs, pratt_parser::PrattParser};
use pest_derive::Parser;

use crate::ast::{BinaryOp, Node, NodeList, UnaryOp};

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

pub fn parse_expr(pairs: Pairs<Rule>) -> NodeList {
    PRATT_PARSER
        .map_primary(|primary| {
            let node = match primary.as_rule() {
                Rule::expr => return parse_expr(primary.into_inner()),
                Rule::number => Node::Number(primary.as_str().parse().unwrap()),
                Rule::bool => Node::Bool(primary.as_str().parse().unwrap()),
                Rule::nil => Node::Nil,
                r => unreachable!("primary rule {:?}", r),
            };
            NodeList::from_node(node)
        })
        .map_infix(|mut lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => BinaryOp::Add,
                Rule::sub => BinaryOp::Sub,
                Rule::mul => BinaryOp::Mul,
                Rule::div => BinaryOp::Div,
                Rule::eqeq => BinaryOp::EqEq,
                Rule::neq => BinaryOp::Neq,
                _ => unreachable!(),
            };
            let lhs_ref = lhs.noderef();
            let rhs_ref = lhs.append(rhs);
            let node = Node::BinaryOp(op, lhs_ref, rhs_ref);
            lhs.push_node(node);
            lhs
        })
        .map_prefix(|op, mut rhs| {
            let op = match op.as_rule() {
                Rule::neg => UnaryOp::Neg,
                _ => unreachable!(),
            };
            let node = Node::UnaryOp(op, rhs.noderef());
            rhs.push_node(node);
            rhs
        })
        .parse(pairs)
}
