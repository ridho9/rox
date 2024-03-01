use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
    string::String,
};
use thiserror::Error;

use crate::ast::{BinaryOp, Node, NodeRef, UnaryOp, AST};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret_ast(&mut self, ast: &AST) -> Result<Value, RuntimeError> {
        self.eval_ast_ref(ast, ast.tailref())
    }

    fn eval_ast_ref(&mut self, ast: &AST, curref: NodeRef) -> Result<Value, RuntimeError> {
        let node = ast.at(curref);
        let result = match node {
            Node::Nil => Value::Nil,
            Node::Bool(b) => Value::Bool(*b),
            Node::Number(n) => Value::Number(*n),
            Node::String(s) => Value::String(s.to_string()),
            Node::UnaryOp(op, rhs) => {
                let rhs_res = self.eval_ast_ref(ast, *rhs)?;
                match op {
                    UnaryOp::Neg => (-rhs_res)?,
                    UnaryOp::Not => (!rhs_res)?,
                }
            }
            Node::BinaryOp(op, lhs_ref, rhs_ref) => {
                let lhs = self.eval_ast_ref(ast, *lhs_ref)?;
                let rhs = self.eval_ast_ref(ast, *rhs_ref)?;
                match op {
                    BinaryOp::Add => (lhs + rhs)?,
                    BinaryOp::Sub => (lhs - rhs)?,
                    BinaryOp::Mul => (lhs * rhs)?,
                    BinaryOp::Div => (lhs / rhs)?,
                    BinaryOp::EqEq => Value::Bool(lhs == rhs),
                    BinaryOp::Neq => Value::Bool(lhs != rhs),
                    BinaryOp::Less => match (lhs, rhs) {
                        (Value::Number(lhs), Value::Number(rhs)) => Value::Bool(lhs < rhs),
                        (lhs, rhs) => {
                            return Err(RuntimeError {
                                kind: RuntimeErrorKind::TypeError {
                                    op: "<",
                                    types: vec![lhs.type_name(), rhs.type_name()],
                                },
                            })
                        }
                    },
                    BinaryOp::LessEq => match (lhs, rhs) {
                        (Value::Number(lhs), Value::Number(rhs)) => Value::Bool(lhs <= rhs),
                        (lhs, rhs) => {
                            return Err(RuntimeError {
                                kind: RuntimeErrorKind::TypeError {
                                    op: "<=",
                                    types: vec![lhs.type_name(), rhs.type_name()],
                                },
                            })
                        }
                    },
                    BinaryOp::Greater => match (lhs, rhs) {
                        (Value::Number(lhs), Value::Number(rhs)) => Value::Bool(lhs > rhs),
                        (lhs, rhs) => {
                            return Err(RuntimeError {
                                kind: RuntimeErrorKind::TypeError {
                                    op: ">",
                                    types: vec![lhs.type_name(), rhs.type_name()],
                                },
                            })
                        }
                    },
                    BinaryOp::GreaterEq => match (lhs, rhs) {
                        (Value::Number(lhs), Value::Number(rhs)) => Value::Bool(lhs >= rhs),
                        (lhs, rhs) => {
                            return Err(RuntimeError {
                                kind: RuntimeErrorKind::TypeError {
                                    op: ">=",
                                    types: vec![lhs.type_name(), rhs.type_name()],
                                },
                            })
                        }
                    },
                }
            }
            Node::PrintStmt(expr) => {
                let result = self.eval_ast_ref(ast, *expr)?;
                println!("{}", result);
                result
            }
            Node::Statements(refs) => {
                for stmt_ref in refs {
                    self.eval_ast_ref(ast, *stmt_ref)?;
                }
                Value::Nil
            }
        };
        Ok(result)
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Bool(_) => "bool",
            Value::Number(_) => "number",
            Value::String(_) => "string",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }
}

impl Add for Value {
    type Output = Result<Self, RuntimeError>;

    fn add(self, rhs: Self) -> Self::Output {
        let res = match (self, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs + rhs),
            (Value::String(lhs), Value::String(rhs)) => Value::String(lhs + &rhs),
            (lhs, rhs) => {
                return Err(RuntimeError {
                    kind: RuntimeErrorKind::TypeError {
                        op: "+",
                        types: vec![lhs.type_name(), rhs.type_name()],
                    },
                })
            }
        };
        Ok(res)
    }
}

impl Sub for Value {
    type Output = Result<Self, RuntimeError>;

    fn sub(self, rhs: Self) -> Self::Output {
        let res = match (self, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs - rhs),
            (lhs, rhs) => {
                return Err(RuntimeError {
                    kind: RuntimeErrorKind::TypeError {
                        op: "-",
                        types: vec![lhs.type_name(), rhs.type_name()],
                    },
                })
            }
        };
        Ok(res)
    }
}

impl Mul for Value {
    type Output = Result<Self, RuntimeError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let res = match (self, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs - rhs),
            (lhs, rhs) => {
                return Err(RuntimeError {
                    kind: RuntimeErrorKind::TypeError {
                        op: "-",
                        types: vec![lhs.type_name(), rhs.type_name()],
                    },
                })
            }
        };
        Ok(res)
    }
}

impl Div for Value {
    type Output = Result<Self, RuntimeError>;

    fn div(self, rhs: Self) -> Self::Output {
        let res = match (self, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                if rhs == 0.0 {
                    return Err(RuntimeError {
                        kind: RuntimeErrorKind::DivisionByZeroError,
                    });
                }
                Value::Number(lhs / rhs)
            }
            (lhs, rhs) => {
                return Err(RuntimeError {
                    kind: RuntimeErrorKind::TypeError {
                        op: "-",
                        types: vec![lhs.type_name(), rhs.type_name()],
                    },
                })
            }
        };
        Ok(res)
    }
}

impl Not for Value {
    type Output = Result<Self, RuntimeError>;

    fn not(self) -> Self::Output {
        Ok(Value::Bool(!self.is_truthy()))
    }
}

impl Neg for Value {
    type Output = Result<Self, RuntimeError>;

    fn neg(self) -> Self::Output {
        let value = match self {
            Value::Number(n) => Value::Number(-n),
            _ => {
                return Err(RuntimeError {
                    kind: RuntimeErrorKind::TypeError {
                        op: "-",
                        types: vec![self.type_name()],
                    },
                })
            }
        };
        Ok(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Error, Debug)]
#[error("runtime error: {kind}")]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
}

#[derive(Error, Debug)]
pub enum RuntimeErrorKind {
    #[error("type error: unsupported operand types for '{op}': {}", types.join(", "))]
    TypeError {
        op: &'static str,
        types: Vec<&'static str>,
    },
    #[error("division by zero")]
    DivisionByZeroError,
}
