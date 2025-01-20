#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i64),
    Float(f64),
    Symbol(String),
    List(Vec<Expr>),
}
