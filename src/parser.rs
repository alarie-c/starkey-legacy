use crate::{ast::{Node, Value, AST}, token::{Token, TokenKind}};

pub struct Parser<'a> {
    pub ast: AST,
    src: &'a Vec<Token>,
    idx: usize,
    t: &'a Token,
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a Vec<Token>) -> Self {
        Self {
            ast: AST { nodes: Vec::new() },
            src,
            idx: 0usize,
            t: &src.get(0).unwrap(),
            offset: 0usize,
        }
    }

    pub fn parse(&mut self) {
        loop {
            // Add a node if we parse a valid one
            if let Some(n) = self.parse_expr() {
                self.ast.add(n);
            }

            // Advance the iterator
            if self.idx + 1 < self.src.len() {
                self.idx += 1;
                self.idx += self.offset; // offset if applicable
                self.offset = 0;         // reset offset to 0
                self.t = self.src.get(self.idx).unwrap();
                continue;
            } else {
                break;
            }
        }
    }

    // Will match anything except a leaf node
    // Leaf nodes require a branch node (like a binary op) to parse them
    fn parse_expr(&mut self) -> Option<Node> {
        match &self.t.kind {
            TokenKind::Plus => Some(Node::ValueExpr(Value::Integer(1))),
            _ => panic!("Unexpected token"),
        }
    }
}