use crate::lexer::token::{Token, TokenKind};

use super::node::{BinaryExpr, Node};

// Im lazy :P
type Tk = TokenKind;

pub struct Parser<'a> {
    src: &'a Vec<Token>,
    idx: usize,
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a Vec<Token>) -> Self {
        // Get the first token
        let current = src.get(0).unwrap();

        // Return new parser
        Self {
            src,
            idx: 0usize,
            offset: 0usize,
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut ast: Vec<Node> = Vec::new();

        'parse: loop {
            // Check for EOF
            if self.idx >= self.src.len() - 1 {
                break 'parse;
            }

            // Get current token
            let current = self.src.get(self.idx).unwrap();

            // Attempt to parse an expression
            if !current.kind.is_leaf_node() {
                dbg!(current);
                match self.parse_expr(current) {
                    Some(n) => ast.push(n),
                    None => {},
                }
            }

            self.idx += 1;
            self.idx += self.offset;
            continue 'parse;
        }

        ast
    }

    fn parse_expr(&mut self, token: &Token) -> Option<Node> {
        match &token.kind {
            Tk::Number { value: v } => self.parse_number(v),

            // Arithmetic
            Tk::Plus => self.parse_binary_expr(token, None),
            Tk::Minus => self.parse_binary_expr(token, None),
            Tk::Star => self.parse_binary_expr(token, None),
            Tk::Slash => self.parse_binary_expr(token, None),
            Tk::Modulo => self.parse_binary_expr(token, None),
            Tk::Caret => self.parse_binary_expr(token, None),
            _ => panic!("Unexpected token"),
        }
    }

    fn parse_number(&mut self, val: &String) -> Option<Node> {
        if val.contains(".") {
            // Indicates a floating point number
            match val.parse::<f32>() {
                Ok(v) => Some(Node::Number(v)),
                Err(_) => panic!("Error parsing number"),
            }
        } else {
            // Indicates an integer number
            match val.parse::<i32>() {
                Ok(v) => Some(Node::Integer(v)),
                Err(_) => panic!("Error parsing number"),
            }
        }
    }

    fn parse_binary_expr(&mut self, token: &Token, maybe_lhs: Option<Node>) -> Option<Node> {
        // Get operator and precedence
        let (op, prec) = token.kind.binary_operator();
        if op == 0 { panic!("Invalid token for BinaryExpr") }
        
        // ----------------- GET CHILD NODES ----------------- \\ 
        
        // Get LHS node
        let lhs: Node;

        if maybe_lhs.is_some() {
            lhs = maybe_lhs.unwrap();
        } else {
            // Get LHS token
            self.idx -= 1;
            let lhs_token = self.src.get(self.idx).unwrap_or_else(|| {
                panic!("Missing LHS token");
            });

            lhs = self.parse_expr(lhs_token).unwrap_or_else(|| {
                panic!("LHS node is invalid");
            });
        }

        // Get RHS token
        self.idx += 2;
        let rhs_token = self.src.get(self.idx).unwrap_or_else(|| {
            panic!("Missing RHS token");
        });
        
        // Get RHS node
        let rhs = self.parse_expr(rhs_token).unwrap_or_else(|| {
            panic!("RHS node is invalid");
        });

        // ----------------- LOOKAHEAD ----------------- \\ 

        // Look ahead for possible other binary expressions
        match self.src.get(self.idx + 1) {
            Some(t) => {
                // If the lookahead is a binary operator...
                if t.kind.binary_operator().0 != 0 {
                    
                    // Get inner BinaryExpr
                    let inner_expr = self.parse_binary_expr(t, Some(rhs)).unwrap_or_else(|| {
                        panic!("Error parsing inner BinaryExpr");
                    });

                    return Some(Node::BinaryExpr(BinaryExpr {
                        lhs: Box::new(lhs),
                        rhs: Box::new(inner_expr),
                        op,
                        prec,
                    }));
                }
            }

            None => {}
        }

        // ----------------- CREATE NODE ----------------- \\ 

        return Some(Node::BinaryExpr(BinaryExpr {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
            prec,
        }));
    }

    /// Advances self.idx until it comes across a token that isn't a leaf node
    fn next_non_leaf(&mut self) -> Option<&'a Token> {
        loop {
            self.idx += 1;
            match self.src.get(self.idx) {
                Some(t) => {
                    if !t.kind.is_leaf_node() {
                        return Some(t);
                    } else {
                        continue;
                    }
                }
                None => return None,
            }
        }
    }
}