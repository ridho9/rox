#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub linecol: (usize, usize),
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Nil,
    Bool(bool),
    Number(f64),
    BinaryOp(Box<Node>, BinaryOp, Box<Node>),
    UnaryOp(UnaryOp, Box<Node>),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    EqEq,
    Neq,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
}
