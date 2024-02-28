use crate::ast::{Node, NodeRef, AST};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn exec_ast(&mut self, ast: &AST) -> Value {
        self.run_ast(ast, ast.tailref())
    }

    fn run_ast(&mut self, ast: &AST, curref: NodeRef) -> Value {
        use Node::*;
        let node = ast.at(curref);
        match *node {
            Nil => Value::Nil,
            Bool(b) => Value::Bool(b),
            Number(n) => Value::Number(n),
            _ => unimplemented!("{:?}", node),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
}
