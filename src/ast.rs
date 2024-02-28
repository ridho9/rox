use std::{ops::Add, vec};

#[derive(Debug)]
pub struct NodeList(pub Vec<Node>);

impl NodeList {
    pub fn from_node(node: Node) -> NodeList {
        NodeList(vec![node])
    }

    pub fn push_node(&mut self, node: Node) {
        self.0.push(node)
    }

    pub fn noderef(&self) -> NodeRef {
        NodeRef(self.0.len() - 1)
    }

    pub fn append(&mut self, rhs: NodeList) -> NodeRef {
        let start_ref = self.0.len();
        for mut node in rhs.0 {
            node.transpose(start_ref);
            self.0.push(node);
        }
        self.noderef()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NodeRef(pub usize);

impl Add<usize> for NodeRef {
    type Output = NodeRef;

    fn add(self, rhs: usize) -> Self::Output {
        NodeRef(self.0 + rhs)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Node {
    Nil,
    Bool(bool),
    Number(f64),
    BinaryOp(BinaryOp, NodeRef, NodeRef),
    UnaryOp(UnaryOp, NodeRef),
}
impl Node {
    fn transpose(&mut self, t: usize) {
        *self = match *self {
            Node::BinaryOp(op, lhs, rhs) => Node::BinaryOp(op, lhs + t, rhs + t),
            Node::UnaryOp(op, i) => Node::UnaryOp(op, i + t),
            node => node,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    EqEq,
    Neq,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,
}
