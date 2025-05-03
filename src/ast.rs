#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Let { name: String, val: Expr },
    Print(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Logic {
        left: Box<Expr>,
        op: LogicOp,
        right: Box<Expr>,
    },
    Comp {
        left: Box<Expr>,
        op: CompOp,
        right: Box<Expr>,
    },
    Term {
        left: Box<Expr>,
        op: TermOp,
        right: Box<Expr>,
    },
    Factor {
        left: Box<Expr>,
        op: FactorOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompOp {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicOp {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TermOp {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FactorOp {
    Multiply,
    Divide,
    Modulus,
}
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Minus,
    Not,
}
