pub static BINARY_OPS: [u8; 6] = [
    '+' as u8,
    '-' as u8,
    '*' as u8,
    '/' as u8,
    '%' as u8,
    '^' as u8,
];

#[derive(Debug)]
pub enum Node {
    Integer(i32),
    Number(f32),

    BinaryExpr(BinaryExpr),
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub op: u8,
    pub prec: u8,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}