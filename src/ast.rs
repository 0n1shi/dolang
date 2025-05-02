#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Identifier(String),
    String(String),
}
