#[derive(Debug)]
pub enum Expr {
    Nil,
    Bool(bool),
    Number(f64),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    EqEq,
    Neq,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}
