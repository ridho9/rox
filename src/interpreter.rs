use thiserror::Error;

use crate::ast::{Node, NodeRef, AST};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn exec_ast(&mut self, ast: &AST) -> Result<Value, RuntimeError> {
        self.run_ast(ast, ast.tailref())
    }

    fn run_ast(&mut self, ast: &AST, curref: NodeRef) -> Result<Value, RuntimeError> {
        use Node::*;
        let node = ast.at(curref);
        let result = match node {
            Nil => Value::Nil,
            Bool(b) => Value::Bool(*b),
            Number(n) => Value::Number(*n),
            String(s) => Value::String(s.clone()),
            _ => unimplemented!("{:?}", node),
        };
        Ok(result)
    }
}

#[derive(Debug)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

#[derive(Error, Debug)]
pub enum RuntimeError {}
