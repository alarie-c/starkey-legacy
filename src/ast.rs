// Highest level component of the AST, stores every node
#[derive(Debug)]
pub struct AST {
    pub nodes: Vec<Node>,
}

impl AST {
    pub fn add(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

// At the highest level, everything is converted to a node.
// These nodes are simply placeholders for structs and enums where
// more specific data can be expressed
#[derive(Debug)]
pub enum Node {
    BinaryExpr(BinaryExpr),
    ValueExpr(Value),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiple,
    Divide,
    Module,
    Exponent,
}

// Any value of any type
#[derive(Debug)]
pub enum Value {
    Integer(i32),
    String(String),
    Boolean(bool),
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: BinaryOperator,
}