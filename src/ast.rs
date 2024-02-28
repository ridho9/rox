use std::{ops::Add, vec};

#[derive(Debug, Clone, Copy)]
pub enum Node {
    Nil,
    Bool(bool),
    Number(f64),
    BinaryOp(BinaryOp, NodeRef, NodeRef),
    UnaryOp(UnaryOp, NodeRef),
}

impl Add<usize> for Node {
    type Output = Node;

    fn add(self, t: usize) -> Self::Output {
        use Node::*;
        match self {
            Nil => Nil,
            Bool(b) => Bool(b),
            Number(n) => Number(n),
            BinaryOp(op, lhs, rhs) => BinaryOp(op, lhs + t, rhs + t),
            UnaryOp(op, i) => UnaryOp(op, i + t),
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

#[derive(Debug, Clone, Copy)]
pub struct NodeRef(pub usize);

impl Add<usize> for NodeRef {
    type Output = NodeRef;

    fn add(self, rhs: usize) -> Self::Output {
        NodeRef(self.0 + rhs)
    }
}

impl Into<NodeRef> for usize {
    fn into(self) -> NodeRef {
        NodeRef(self)
    }
}

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
            node = node + start_ref;
            self.0.push(node);
        }
        self.noderef()
    }
}
