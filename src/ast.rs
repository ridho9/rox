#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub linecol: (usize, usize),
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Nil,
    Bool(bool),
    Number(f64),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
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
