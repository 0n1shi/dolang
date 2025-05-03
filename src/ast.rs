#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Let { name: String, val: Expr },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Comp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
}
