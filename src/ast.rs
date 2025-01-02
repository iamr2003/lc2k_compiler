#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i8),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Var(String),
    Assign(String, Box<Expr>) // for now we aren't allowing lhs complex expressions
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Mul, // to be implemented, requires a particular decomposition
    Div, // to be implemented, requires a particular decomposition
    Add,
    Sub,
}
