#[derive(Debug)]
pub enum Expr {
    Number(i64),
    String(String),
    Variable(String),
    Binary(Box<Expr>,Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Let(String, Expr),
    ConsoleLog(Expr),
}