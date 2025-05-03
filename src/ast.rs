#[derive(Debug, Clone)]
pub enum Expression {
    Number(i64),
    Variable(String),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>),
}
