use lazy_static::lazy_static;
use pest::{iterators::Pairs, pratt_parser::PrattParser};
use pest_derive::Parser;

use crate::ast::{BinaryOp, Metadata, Node, NodeList, UnaryOp, AST};

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
            .op(Op::prefix(Rule::neg) | Op::prefix(Rule::not))
    };
}

pub fn parse_expr(pairs: Pairs<Rule>) -> AST {
    PRATT_PARSER
        .map_primary(|primary| {
            let meta = Metadata {
                linecol: primary.line_col(),
            };
            let node = match primary.as_rule() {
                Rule::expr => return parse_expr(primary.into_inner()),
                Rule::number => Node::Number(primary.as_str().parse().unwrap()),
                Rule::bool => Node::Bool(primary.as_str().parse().unwrap()),
                Rule::nil => Node::Nil,
                Rule::string => Node::String({
                    let mut result = String::new();
                    let tree = primary.into_inner().next().unwrap().into_inner();
                    for ch in tree {
                        let inner = ch.into_inner().next().unwrap();
                        match inner.as_rule() {
                            Rule::char_noesc => result.push_str(inner.as_str()),
                            Rule::char_esc => {
                                let conv = match inner.as_str() {
                                    r#"\""# => "\"",
                                    r#"\\"# => "\\",
                                    r#"\/"# => "/",
                                    r#"\n"# => "\n",
                                    r#"\r"# => "\r",
                                    r#"\t"# => "\t",
                                    _ => unreachable!(),
                                };
                                result.push_str(conv)
                            }
                            _ => unreachable!(),
                        }
                    }
                    result
                }),
                r => unreachable!("primary rule {:?}", r),
            };
            AST {
                list: NodeList(vec![node]),
                meta: vec![meta],
            }
        })
        .map_infix(|mut lhs, op, rhs| {
            let meta = Metadata {
                linecol: op.line_col(),
            };
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
            lhs.push(node, meta);
            lhs
        })
        .map_prefix(|op, mut rhs| {
            let meta = Metadata {
                linecol: op.line_col(),
            };
            let op = match op.as_rule() {
                Rule::neg => UnaryOp::Neg,
                Rule::not => UnaryOp::Not,
                _ => unreachable!(),
            };
            let node = Node::UnaryOp(op, rhs.noderef());
            rhs.push(node, meta);
            rhs
        })
        .parse(pairs)
}
