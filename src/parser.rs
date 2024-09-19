use crate::{ast::{BinaryExpr, BinaryOp, Node, AST}, token::{Token, TokenKind}};

pub enum ParseResult<'a> {
    Ok,
    Err(&'a str)
}

pub struct Parser<'a> {
    pub ast: AST,
    src: &'a Vec<Token>,
    idx: usize,
    token: &'a Token,
    offset: usize,
}

impl<'a> Parser<'a> {
    /// Create a new instance of the parser that contains a reference to the output of the lexer
    /// Note that the lexer still owns it's output, the parser simply takes a reference to it's ouput.
    pub fn new(src: &'a Vec<Token>) -> Self {
        Self {
            ast: AST { nodes: Vec::new() },
            src,
            idx: 0usize,
            token: &src.get(0).unwrap(),
            offset: 0usize,
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        // Quickly return if there's no entry point
        if self.idx >= self.src.len() { return ParseResult::Err("Missing entry point."); }

        'parse: loop {
            // Get the token of the current index
            self.token = self.src.get(self.idx).unwrap();
            
            // Break if EOF reached
            if self.token.kind == TokenKind::EndOfFile { break 'parse; }

            // Parse tokens for real now
            // Noteworthy information: do not parse leaf nodes
            if self.token.kind.is_leaf_node() {
                match self.parse_expression(&self.token) {
                    Some(node) => self.ast.add(node),
                    None => {},
                }
            }
            
            // Apply the accumulated idx advance offset from helper methods
            self.idx += self.offset;

            // Advance the index
            if self.idx + 1 > self.src.len() {
                break 'parse;
            } else {
                // Reset offset and continue
                self.offset = 0;
                self.idx += 1;
                continue 'parse;
            }
        }

        ParseResult::Ok
    }

    /// This is a parent method for all expressions and is generally the
    /// primary method that is recursively called, e.g. `let lhs = parse_expression()`
    fn parse_expression(&mut self, token: &Token) -> Option<Node> {
        match token.kind {
            // Binary operators for binary expressions
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Exponent
                | TokenKind::Modulo
            => self.n_binary_expr(token, None),

            // Float and integer literals
            TokenKind::Number(_) => self.parse_number(token),

            // Anything else
            _ => panic!("??? lol")
        }
    }

    fn get_binary_op(&mut self, token: &Token) -> Option<(BinaryOp, u8)> {
        // Get operator and precedence
        match token.kind {
            TokenKind::Plus => Some((BinaryOp::Plus, 5u8)),
            TokenKind::Minus => Some((BinaryOp::Minus, 4u8)),
            TokenKind::Modulo => Some((BinaryOp::Modulo, 3u8)),
            TokenKind::Star => Some((BinaryOp::Multiply, 2u8)),
            TokenKind::Slash => Some((BinaryOp::Divide, 1u8)),
            TokenKind::Exponent => Some((BinaryOp::Exponent, 0u8)),
            _ => None,
        }
    }

    fn n_binary_expr(&mut self, token: &Token, lhs: Option<Node>) -> Option<Node> {
        let (op, prec) = self.get_binary_op(token).unwrap();        

        // Get LHS, which is the previous token
        if self.idx == 0 { panic!("Binary expression requires a left-hand value") }
        
        // Check to make sure we aren't being given the LHS and if we are, use that instead of trying to
        // parse the last token.
        let maybe_lhs: Option<Node>;
        if lhs.is_some() {
            maybe_lhs = lhs;
        } else {
            maybe_lhs = self.parse_expression(self.src.get(self.idx - 1).unwrap());
        }
        
        // Get RHS, which is the next token
        if self.idx + 1 >= self.src.len() { panic!("Binary expression requires a right-hand value") }
        let maybe_rhs = self.parse_expression(self.src.get(self.idx + 1).unwrap());
        self.offset += 1;
        
        // Advance offset so parse() knows to skip ahead a few tokens
        // Comes from `self.idx+1` in the maybe_rhs getter
        // self.offset += 1;

        /*
        Now we have to determine if there's another binary expression in this sequence
        We're going to look for a binary operator after the final RHS token has been reached
        This includes the offset because of RHS is something like an itendifier it could be
        composed of who knows how many tokens
        */

        self.idx += 1; // Takes us to AT the rhs start token
                      // LHS + RHS
                     //        ^^^ we are here

        let peek_idx = self.idx + self.offset; // Takes us to PAST the rhs start token
                                                     // LHS + RHS +
                                                    //        ||| ^ peek_idx is here
                                                   //         ^^^ we are actually here

        // Make sure we aren't out of bounds
        println!("peek_idx: {peek_idx} & len: {}", self.src.len());
        if peek_idx >= self.src.len() {
            println!("Uh oh");
            // Peek_idx is out of bounds

            if maybe_rhs.is_some() && maybe_lhs.is_some() {
                return Some(Node::BinaryExpr(self.s_binary_expr(
                    maybe_lhs.unwrap(),
                    maybe_rhs.unwrap(),
                    op,
                    prec
                )))
            } else {
                panic!("Either LHS or RHS are not valis expressions");
            }
        } else {
            println!("New expr");
            // Peek_idx is within bounds, so we check for a BinaryOp
            let peeked_token = self.src.get(peek_idx).unwrap();
            dbg!(peeked_token);
            match self.get_binary_op(peeked_token) {
                // Parse a new binary expressio and set it as the RHS of THIS expr
                Some(_) => {
                    println!("Got a binary op!");
                    // Unwrap current RHS, this will be the LHS of the new expression
                    let rhs = maybe_rhs.expect("RHS was not a valid expression");
                    
                    // Consume the peeked token moron!!!!!
                    self.idx = peek_idx; // hate myself for missing this line

                    // Parse a new BinaryExpr using the unwrapped RHS as the LHS for the new expression
                    let new_rhs = self.n_binary_expr(peeked_token, Some(rhs));

                    if new_rhs.is_some() && maybe_lhs.is_some() {
                    // ^^^^^^^ bin_expr  // ^^^^^^^^^ the original LHS
                        return Some(Node::BinaryExpr(self.s_binary_expr(
                            maybe_lhs.unwrap(),
                            new_rhs.unwrap(),
                            op,
                            prec
                        )))
                    } else {
                        panic!("Either LHS or RHS are not valis expressions");
                    }
                }

                // If there is no new binary operator successfully returned
                None => {
                    return Some(Node::BinaryExpr(self.s_binary_expr(
                        maybe_lhs.unwrap(),
                        maybe_rhs.unwrap(),
                        op,
                        prec
                    )))
                }
            }
        }
    }

    fn parse_number(&mut self, token: &Token) -> Option<Node> {
        match &token.kind {
            TokenKind::Number(val) => {
                // Parse float values
                if val.contains(".") {
                    let num: f32 = val.parse().expect("Error parsing float");
                    Some(Node::Float(num))
                    
                // Parse integer values
                } else {
                    let num: i32 = val.parse().expect("Error parsing integer");
                    Some(Node::Integer(num))
                }
            }
            _ => None,
        }
    }

    fn s_binary_expr(&mut self, lhs: Node, rhs: Node, op: BinaryOp, prec: u8) -> BinaryExpr {
        BinaryExpr {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
            prec,
        }
    }
}