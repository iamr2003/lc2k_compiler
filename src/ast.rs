#[derive(Debug,PartialEq)]
pub enum Expr {
    Number(i8),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug,PartialEq)]
pub enum Opcode {
    Mul, // to be implemented, requires a particular decomposition
    Div, // to be implemented, requires a particular decomposition
    Add,
    Sub,
}
