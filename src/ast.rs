#[derive(Debug, Clone)]
pub enum Expr {
    Comparison(Box<Expr>, Vec<(CmpOp, Box<Expr>)>),
    Additive(Box<Expr>, Vec<(AddOp, Box<Expr>)>),
    Multiplicative(Box<Expr>, Vec<(MulOp, Box<Expr>)>),
    Pipe(Box<Expr>, Vec<Box<Expr>>),
    Application(Box<Expr>, Vec<Box<Expr>>),
    Range(Box<Expr>, Option<Box<Expr>>),
    Simple(SimpleExpr),
}

#[derive(Debug, Clone)]
pub enum SimpleExpr {
    Match(Box<Expr>, Vec<(Pattern, Expr)>),
    For(String, Box<Expr>, Box<Expr>),
    Let(String, Box<Expr>),
    Lambda(String, Box<Expr>),
    Record(Vec<(String, Expr)>),
    Access(Box<Expr>, String),
    List(Vec<Expr>),
    Tuple(Vec<Expr>),
    IfElse {
        cond: Box<Expr>,
        then: Box<Expr>,
        els: Option<Box<Expr>>,
    },
    Literal(Literal),
    Ident(String),
    Grouped(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Ident(String),
    Tuple(Vec<Pattern>),
    Wildcard,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum CmpOp {
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
}
#[derive(Debug, Clone)]
pub enum AddOp {
    Add,
    Sub,
}
#[derive(Debug, Clone)]
pub enum MulOp {
    Mul,
    Div,
    Mod,
}
