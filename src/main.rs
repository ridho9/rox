pub mod ast;

use crate::ast::{Metadata, Node, NodeList, AST};
use tree_sitter::{Node as TSNode, Parser};

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rox::language())
        .expect("error loading grammar");

    let source = "print 5;";
    let tree = parser.parse(source, None).expect("error parsing");
    let program = tree.root_node();

    let ast = parse_program(&program, source);
}

fn parse_program(node: &TSNode, source: &str) -> AST {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        parse_stmt(&child, source);
    }
    todo!()
}

fn parse_stmt(node: &TSNode, source: &str) -> AST {
    let stmt = node.child(0).unwrap();
    println!("{:?} {}", stmt, stmt.to_sexp());

    match stmt.kind() {
        "print_stmt" => {
            let expr = stmt.child(1).unwrap();
            parse_expr(&expr, source)
        }
        kind => unreachable!("unexpected stmt kind: {:?}", kind),
    }
}

fn parse_expr(node: &TSNode, source: &str) -> AST {
    let expr = node.child(0).unwrap();
    println!("{:?} {}", expr, expr.to_sexp());
    let start_pos = expr.start_position();
    let meta = Metadata {
        linecol: (start_pos.row, start_pos.column),
    };

    let anode = match expr.kind() {
        "number" => {
            let child: f64 = source[expr.start_byte()..expr.end_byte()].parse().unwrap();
            Node::Number(child)
        }
        kind => unreachable!("unexpected expr kind: {:?}", kind),
    };
    AST {
        list: NodeList(vec![anode]),
        meta: vec![meta],
    }
}
