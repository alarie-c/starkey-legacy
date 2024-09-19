/// Highest level component of the AST, stores every node
#[derive(Debug)]
pub struct AST {
    pub nodes: Vec<Node>,
}

impl AST {
    pub fn add(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

/// At the highest level, everything is converted to a node.
/// These nodes are simply placeholders for structs and enums where
/// more specific data can be expressed.
#[derive(Debug)]
pub enum Node {
    /// Binary expressions require a BinaryExpr struct which stores the
    /// operator and precedence of that expression.
    /// 
    /// Binary expressons also strictly use boxed nodes to refer to
    /// the values subject to the operator and thus other BinaryExpr nodes can be nested
    /// in a BinaryExpr node.
    BinaryExpr(BinaryExpr),
    
    UnaryExpr(UnaryExpr),
    Integer(i32),
    Float(f32),
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: BinaryOp,
    pub prec: u8,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub rhs: Box<Node>,
    pub op: UnaryOp,
}

#[derive(Debug)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Exponent,
}

#[derive(Debug)]
pub enum UnaryOp {
    Minus,
    Hash,       // length of an array-derived type
    Bang,       // negative logical
    Ampersand,  // address
    Star,       // pointer
}
