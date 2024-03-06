use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rox::language())
        .expect("error loading grammar");

    let source = "hello";
    let tree = parser.parse(source, None).expect("error parsing");

    println!("{:?}", tree.root_node().to_sexp())
}
