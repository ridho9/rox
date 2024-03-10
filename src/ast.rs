use std::fmt::Debug;
use std::{ops::Add, vec};

#[derive(Clone)]
pub enum Node {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Ident(String),
    BinaryOp(BinaryOp, NodeRef, NodeRef),
    UnaryOp(UnaryOp, NodeRef),

    Statements(Vec<NodeRef>),
    LetStmt(String, Option<NodeRef>),
    PrintStmt(NodeRef),
    AssignStmt(String, NodeRef),
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Nil => write!(f, "Lit nil"),
            Node::Bool(arg0) => write!(f, "Lit {}", arg0),
            Node::Number(arg0) => write!(f, "Lit {}", arg0),
            Node::String(arg0) => write!(f, "Lit {:?}", arg0),
            Node::Ident(arg0) => write!(f, "Ident {}", arg0),
            Node::BinaryOp(arg0, arg1, arg2) => {
                write!(f, "Binary {:?} {:?} {:?}", arg0, arg1, arg2)
            }
            Node::UnaryOp(arg0, arg1) => write!(f, "Unary {:?} {:?}", arg0, arg1),
            Node::PrintStmt(arg0) => write!(f, "Print {:?}", arg0),
            Node::Statements(refs) => write!(f, "Statements {:?}", refs),
            Node::LetStmt(ident, expr) => write!(f, "Let {} {:?}", ident, expr),
            Node::AssignStmt(ident, expr) => write!(f, "Assign {} {:?}", ident, expr),
        }
    }
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
            String(s) => String(s),
            Ident(s) => Ident(s),
            PrintStmt(rhs) => PrintStmt(rhs + t),
            Statements(refs) => Statements(refs.into_iter().map(|r| r + t).collect()),
            LetStmt(ident, expr) => LetStmt(ident, expr.map(|r| r + t)),
            AssignStmt(ident, expr) => AssignStmt(ident, expr + t),
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
    LessEq,
    Less,
    GreaterEq,
    Greater,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Clone, Copy)]
pub struct NodeRef(pub usize);
impl Debug for NodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct AST {
    pub list: NodeList,
    pub meta: Vec<Metadata>,
}
impl AST {
    pub fn noderef(&self) -> NodeRef {
        self.list.noderef()
    }

    pub fn push_node(&mut self, node: Node, meta: Metadata) {
        self.list.push_node(node);
        self.meta.push(meta);
    }

    pub fn append(&mut self, mut rhs: AST) -> NodeRef {
        let rhs_ref = self.list.append(rhs.list);
        self.meta.append(&mut rhs.meta);
        rhs_ref
    }

    pub fn print_debug(&self) {
        #[cfg(feature = "debug")]
        {
            let mut line = 0;
            for (node, meta) in self.list.0.iter().zip(self.meta.iter()) {
                print!("[{:4}:{:4}] ", meta.linecol.0, meta.linecol.1);
                print!("{:8} ", line);
                print!("{:?}", node);
                println!("");

                line += 1;
            }
        }
    }

    pub fn tailref(&self) -> NodeRef {
        NodeRef(self.list.0.len() - 1)
    }

    pub fn at(&self, curref: NodeRef) -> &Node {
        self.list.0.get(curref.0).expect("invalid ref")
    }

    pub fn new() -> AST {
        AST {
            list: NodeList(vec![]),
            meta: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metadata {
    pub linecol: (usize, usize),
}
