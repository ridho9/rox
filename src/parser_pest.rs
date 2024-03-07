use lazy_static::lazy_static;
use pest::{error::Error, iterators::Pairs, pratt_parser::PrattParser, Parser};
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
            .op(Op::infix(Rule::le, Left)
                | Op::infix(Rule::lt, Left)
                | Op::infix(Rule::ge, Left)
                | Op::infix(Rule::gt, Left))
            .op(Op::infix(Rule::add, Left) | Op::infix(Rule::sub, Left))
            .op(Op::infix(Rule::mul, Left) | Op::infix(Rule::div, Left))
            .op(Op::prefix(Rule::neg) | Op::prefix(Rule::not))
    };
}

pub fn parse(_filename: &str, source: &str) -> Result<AST, Error<Rule>> {
    let program = RoxParser::parse(Rule::program, &source)?;
    Ok(parse_program(program))
}

fn parse_program(mut program: Pairs<Rule>) -> AST {
    let mut statements = program.next().unwrap().into_inner();
    parse_statements(statements.next().unwrap().into_inner())
}

fn parse_statements(stmts: Pairs<Rule>) -> AST {
    let mut result_ast = AST {
        list: NodeList(vec![]),
        meta: vec![],
    };
    let mut refs = vec![];

    for stmt in stmts {
        let stmt_ast = parse_statement(stmt.into_inner());
        result_ast.append(stmt_ast);
        refs.push(result_ast.noderef());
    }
    let meta = result_ast.meta[refs[0].0];
    result_ast.push_node(Node::Statements(refs), meta);
    result_ast
}

fn parse_statement(mut stmt: Pairs<Rule>) -> AST {
    let stmt = stmt.next().unwrap();
    match stmt.as_rule() {
        Rule::print_statement => {
            let meta = Metadata {
                linecol: stmt.as_span().start_pos().line_col(),
            };
            let mut expr = parse_expr(stmt.into_inner());
            let expr_ref = expr.noderef();
            expr.push_node(Node::PrintStmt(expr_ref), meta);
            expr
        }
        Rule::let_statement => {
            let meta = Metadata {
                linecol: stmt.as_span().start_pos().line_col(),
            };
            let mut stmt = stmt.into_inner();
            let ident = stmt.next().unwrap().as_str().to_string();
            let expr = stmt.next().map(|expr| parse_expr(expr.into_inner()));
            let mut result_ast = AST {
                list: NodeList(vec![]),
                meta: vec![],
            };
            let expr_ref = expr.as_ref().map(|expr| {
                let expr_ref = expr.noderef();
                result_ast.append(expr.clone());
                expr_ref
            });
            result_ast.push_node(Node::LetStmt(ident, expr_ref), meta);
            result_ast
        }
        rule => unreachable!("rule {:?} {:?}", rule, stmt),
    }
}

fn parse_expr(exprs: Pairs<Rule>) -> AST {
    PRATT_PARSER
        .map_primary(|primary| {
            let meta = Metadata {
                linecol: primary.line_col(),
            };
            let node = match primary.as_rule() {
                Rule::expr => return parse_expr(primary.into_inner()),
                Rule::number => Node::Number(primary.as_str().parse().unwrap()),
                Rule::bool => Node::Bool(primary.as_str().parse().unwrap()),
                Rule::ident => Node::Ident(primary.as_str().to_string()),
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
                Rule::le => BinaryOp::LessEq,
                Rule::lt => BinaryOp::Less,
                Rule::ge => BinaryOp::GreaterEq,
                Rule::gt => BinaryOp::Greater,
                _ => unreachable!(),
            };
            let lhs_ref = lhs.noderef();
            let rhs_ref = lhs.append(rhs);
            let node = Node::BinaryOp(op, lhs_ref, rhs_ref);
            lhs.push_node(node, meta);
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
            rhs.push_node(node, meta);
            rhs
        })
        .parse(exprs)
}
