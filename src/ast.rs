#[derive(Debug)]
pub struct ASTree {
    pub nodes: Vec<Node>,
}

#[derive(Debug)]
pub enum Node {
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: BinaryOperator,
    pub prec: u8,
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub rhs: Box<Node>,
    pub op: UnaryOperator,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negator,
    LogicalNot,
    LengthOf,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Exponent,
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Plus => 2,
            BinaryOperator::Minus => 2,
            BinaryOperator::Multiply => 1,
            BinaryOperator::Divide => 1,
            BinaryOperator::Modulo => 1,
            BinaryOperator::Exponent => 0,
        }
    }
}
