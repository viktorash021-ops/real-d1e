#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    StringLit(String),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Print(Expr),
    Assign(String, Expr),
    Hash { algo: String, input: Expr, out_var: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
