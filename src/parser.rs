use crate::{
    ast::{ASTree, BinaryExpression, BinaryOperator, Node},
    token::{self, Token, TokenKind},
};

/// The parser is the highest level data structure for the actual generation of the AST
/// The parser works in the following way:
///
/// 1. It takes input in the form a vector of `Token`
/// 2. It creates a new vector that contains references to the `TokenKind` of those `Token`
/// 3. The parser takes a slice of `[index ..]` and matches on slices of `TokenKind`
pub struct Parser<'a> {
    pub ast: ASTree,
    tokens: &'a Vec<Token>,
    kinds: Vec<&'a TokenKind>,
    idx: usize,
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a Vec<Token>) -> Self {
        Self {
            ast: ASTree { nodes: Vec::new() },
            tokens: src,
            kinds: src.iter().map(|x| &x.kind).collect(),
            idx: 0usize,
            offset: 0usize,
        }
    }

    pub fn parse(&mut self) -> Option<()> {
        loop {
            // Break if EOF condition reached or idx out of bounds
            if self.idx >= self.len() || self.now() == &TokenKind::EndOfFile {
                break;
            }

            // Check for non-leaf nodes and parse
            if !token::is_leaf_node(self.now()) {
                if let Some(n) = self.parse_expression() {
                    self.ast.nodes.push(n);
                }
            }

            // Advance idx
            self.idx += self.offset + 1;
        }

        Some(())
    }

    fn parse_expression(&mut self) -> Option<Node> {
        match self.kinds.as_slice()[self.idx..] {
            [TokenKind::Plus, ..] => self.parse_binary_expr(None),

            _ => None,
        }
    }

    fn parse_binary_expr(&mut self, lhs: Option<Node>) -> Option<Node> {
        // Bounds check
        if !self.bounds_lr(1) {
            panic!("Missing RHS or LHS")
        }

        // Save original index of + token
        let idx_origin = self.idx;

        // Go back 1 and get LHS
        self.idx -= 1;
        let maybe_lhs: Option<Node>;
        if lhs.is_some() {
            maybe_lhs = lhs;
        } else {
            maybe_lhs = self.parse_expression();
        }

        // Reset idx and look for RHS
        self.idx = idx_origin + 1;
        let maybe_rhs: Option<Node> = self.parse_expression();
        let new_rhs: Option<Node>;

        // Advance to one token past the RHS and look for binary op
        self.idx += 1;
        if self.binary_operator().is_some() {
            new_rhs = self.parse_binary_expr(maybe_rhs);
        } else {
            new_rhs = None;
        }

        // Get operator and precedence
        let op = self.binary_operator()?;
        let prec = op.precedence();

        None
    }

    fn binary_operator(&self) -> Option<BinaryOperator> {
        match self.now() {
            TokenKind::Plus => Some(BinaryOperator::Plus),
            TokenKind::Minus => Some(BinaryOperator::Minus),
            TokenKind::Star => Some(BinaryOperator::Multiply),
            TokenKind::Slash => Some(BinaryOperator::Divide),
            TokenKind::Modulo => Some(BinaryOperator::Modulo),
            TokenKind::Caret => Some(BinaryOperator::Exponent),
            _ => None,
        }
    }

    /// Takes an offset of type `usize` and returns whether or not
    /// the current index + offset is out of bounds.
    ///
    /// Will also return false in the scenario idx is 0.
    fn bounds_lr(&self, offset: usize) -> bool {
        !(self.idx == 0 || self.idx + offset >= self.len())
    }

    /// Much like `bounds_lr()` but this does not return false if
    /// idx is 0 -- it only checks to the right
    fn bounds_r(&self, offset: usize) -> bool {
        !(self.idx + offset >= self.len())
    }

    /// Much like `bounds_r()` but this checks to the left instead.
    /// Will return false if idx is 0 or if idx - offset is less than 0
    fn bounds_l(&self, offset: usize) -> bool {
        !(self.idx == 0 || self.idx as isize - offset as isize <= 0)
    }

    fn now(&self) -> &TokenKind {
        unsafe { self.kinds.get_unchecked(self.idx) }
    }

    fn len(&self) -> usize {
        self.kinds.len()
    }
}
