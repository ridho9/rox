use std::string::String;
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
        let node = ast.at(curref);
        let result = match node {
            Node::Nil => Value::Nil,
            Node::Bool(b) => Value::Bool(*b),
            Node::Number(n) => Value::Number(*n),
            Node::String(s) => {
                let s_parsed = String::new();

                let v = Value::String(s_parsed);
                v
            }
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
