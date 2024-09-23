use std::{borrow::Borrow, fmt::Binary};

use crate::{
    ast::{Ast, BinaryExpr, BinaryOp, FunctionExpr, FunctionSignature, Node, VariableExpr},
    token::{Token, TokenKind},
};

// Im lazy :P
type Tk = TokenKind;

/// The parser is the highest level data structure for the actual generation of the AST
/// The parser works in the following way:
///
/// 1. It takes input in the form a vector of `Token`
/// 2. It creates a new vector that contains references to the `TokenKind` of those `Token`
/// 3. The parser takes a slice of `[index ..]` and matches on slices of `TokenKind`
///
///  YAY! :)
pub struct Parser<'a> {
    pub ast: Ast,
    as_token: &'a Vec<Token>,
    as_kind: Vec<&'a TokenKind>,
    idx: usize,
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a Vec<Token>) -> Self {
        Self {
            ast: Ast { nodes: Vec::new() },
            as_token: src,
            as_kind: src.iter().map(|x| &x.kind).collect(),
            idx: 0usize,
            offset: 0usize,
        }
    }

    /// The game plan is as follows:
    ///
    /// We are going to look for patterns that we would want to parse, EXCLUDING leaf nodes.
    /// Leafs nodes aren't going to be parsed until we can be certain they're going to live under something
    /// There is a expression() method which will be the main componenet of this but this one also parses leaf nodes
    /// because it's the one we wanna call recursively/.
    pub fn parse(&mut self) -> Option<()> {
        loop {
            // End if EOF reached
            if self._this().kind == Tk::EndOfFile {
                break;
            }

            // If not a leaf node, parse it
            if !self.as_kind.get(self.idx).unwrap().leaf_node() {
                match self.expression() {
                    Some(n) => self.ast.add(n),
                    None => (),
                }
            }

            self.idx += 1;
            self.idx += self.offset;
        }

        Some(())
    }

    fn expression(&mut self) -> Option<Node> {
        if self.idx >= self.as_kind.len() {
            return None;
        }

        match self.as_kind.as_slice()[self.idx..] {
            [Tk::Plus, ..]
            | [Tk::Minus, ..]
            | [Tk::Star, ..]
            | [Tk::Slash, ..]
            | [Tk::Exponent, ..]
            | [Tk::Modulo, ..] => self.parse_arithmetic(None),

            [Tk::Number, ..] => self.parse_number(),
            [Tk::Var, Tk::Identifier, Tk::ColonColon, ..] => self.parse_val_annotated(true),
            [Tk::Val, Tk::Identifier, Tk::ColonColon, ..] => self.parse_val_annotated(false),
            [Tk::Var, Tk::Identifier, Tk::Equal, ..] => self.parse_val(true),
            [Tk::Val, Tk::Identifier, Tk::Equal, ..] => self.parse_val(false),

            // func name()
            [Tk::Func, Tk::Identifier, Tk::LPar, ..] => self.parse_function(false),
            // mut func name()
            [Tk::Mut, Tk::Func, Tk::Identifier, Tk::LPar, ..] => self.parse_function(true),

            [Tk::Identifier, ..] => self.parse_identifier(),
            _ => None,
        }
    }

    fn parse_function(&mut self, mutable: bool) -> Option<Node> {
        // Advance to Tk::Identifier
        if mutable {
            self.idx += 2;
        } else {
            self.idx += 1
        }

        // Get the function name
        let f_name = self.expression(); // can safely unwrap, we know it's an identifier
        let f_name_as_string = match f_name.unwrap() {
            Node::Identifier(s) => s,
            _ => panic!("Error getting function name"),
        };

        // Look for some parameters
        // Check 2 ahead for ending par
        if self._at(2).is_some_and(|x| x.kind == Tk::RPar) {
            // This scenario: No params
            let params: Vec<Node> = Vec::new();
            if self.idx + 4 < self._len() {
                self.idx += 4;
                dbg!(self._this());
                let maybe_returns = self.expression();

                let signature: FunctionSignature;
                if maybe_returns.is_some() {
                    signature = FunctionSignature {
                        name: f_name_as_string,
                        params,
                        returns: Box::new(maybe_returns.unwrap()),
                        mutable,
                    };

                    self.idx += 1;

                    // Get the rest of the function
                    let body = self._parse_block();
                    if body.is_none() {
                        panic!("Non-terminating code block")
                    }

                    // Create function node
                    Some(Node::Function(FunctionExpr {
                        signature,
                        body: body?,
                    }))
                } else {
                    panic!("Could not evaluate return type");
                }
            } else {
                panic!("Missing return type");
            }
        } else {
            None
        }
    }

    fn _parse_block(&mut self) -> Option<Vec<Node>> {
        let mut body = Vec::<Node>::new();

        loop {
            // End if EOF reached
            if self._this().kind == Tk::EndOfFile {
                return None;
            }

            if self._this().kind == Tk::RCurl {
                return Some(body);
            }

            // If not a leaf node, parse it
            if !self.as_kind.get(self.idx).unwrap().leaf_node() {
                match self.expression() {
                    Some(n) => body.push(n),
                    None => (),
                }
            }

            self.idx += 1;
            self.idx += self.offset;
        }
    }

    fn parse_parameter(&mut self) -> Option<Node> {
        None
    }

    fn parse_arithmetic(&mut self, lhs: Option<Node>) -> Option<Node> {
        let (op, prec) = self._get_operator_and_precedence().unwrap();

        if self.idx == 0 {
            panic!("Binary arithmetic needs an LHS")
        }

        // Get LHS by initializing and selection
        let maybe_lhs: Option<Node>;
        if lhs.is_some() {
            maybe_lhs = lhs;
        } else {
            // TODO: Keep going back until we find something that is a , ; ( ) etc.
            self.idx -= 1;
            maybe_lhs = self.expression();
            self.idx += 1;
        }

        self.idx += 1;
        let maybe_rhs = self.expression();

        // Advance if possible
        if self.idx < self.as_kind.len() {
            self.idx += self.offset;
            self.idx += 1;
        }

        // Construct node and struct
        if self._get_operator_and_precedence().is_some() {
            let new_rhs = self.parse_arithmetic(maybe_rhs);
            if maybe_lhs.is_some() && new_rhs.is_some() {
                Some(Node::BinaryExpr(BinaryExpr {
                    lhs: Box::new(maybe_lhs.unwrap()),
                    rhs: Box::new(new_rhs.unwrap()),
                    op,
                    prec,
                }))
            } else {
                None
            }
        } else {
            if maybe_lhs.is_some() && maybe_rhs.is_some() {
                Some(Node::BinaryExpr(BinaryExpr {
                    lhs: Box::new(maybe_lhs.unwrap()),
                    rhs: Box::new(maybe_rhs.unwrap()),
                    op,
                    prec,
                }))
            } else {
                None
            }
        }
    }

    fn parse_identifier(&mut self) -> Option<Node> {
        let name = self._this().value.as_ref().unwrap();

        // We want to check to make sure there's no more stuff going on here
        match self.as_kind.as_slice()[self.idx..] {
            // Access member
            [Tk::Identifier, Tk::Dot, ..] => {
                self.idx += 2; // skip the dot
                let maybe_member = self.expression();
                let ident = Node::Identifier(name.to_string());

                // Construct node
                if maybe_member.is_some() {
                    Some(Node::AccessMember(
                        Box::new(ident),
                        Box::new(maybe_member.unwrap()),
                    ))
                } else {
                    Some(Node::Identifier(name.to_string()))
                }
            }
            // Invoke member
            [Tk::Identifier, Tk::Colon, ..] => {
                self.idx += 2; // skip the colon
                let maybe_method = self.expression();
                let ident = Node::Identifier(name.to_string());

                // Construct node
                if maybe_method.is_some() {
                    Some(Node::InvokeMember(
                        Box::new(ident),
                        Box::new(maybe_method.unwrap()),
                    ))
                } else {
                    Some(Node::Identifier(name.to_string()))
                }
            }
            // Just return the identifier
            _ => Some(Node::Identifier(name.to_string())),
        }
    }

    fn parse_val(&mut self, mutable: bool) -> Option<Node> {
        self.idx += 1;
        let maybe_name = self.expression();
        dbg!(&maybe_name);

        self.idx += 2;
        let maybe_value = self.expression();
        dbg!(&maybe_value);

        // Construct node and struct
        if maybe_name.is_some() && maybe_value.is_some() {
            Some(Node::ValueDeclaration(VariableExpr {
                key: Box::new(maybe_name.unwrap()),
                value: Box::new(maybe_value.unwrap()),
                mutable,
                annotation: None,
            }))
        } else {
            None
        }
    }

    fn parse_val_annotated(&mut self, mutable: bool) -> Option<Node> {
        self.idx += 1;
        let maybe_name = self.expression();

        self.idx += 2;
        let maybe_type = self.expression();

        // Apply offset from getting the type and advance once past the `=`
        self.idx += self.offset + 2;
        let maybe_value = self.expression();

        // Construct node and struct
        if maybe_name.is_some() && maybe_type.is_some() && maybe_value.is_some() {
            Some(Node::ValueDeclaration(VariableExpr {
                key: Box::new(maybe_name.unwrap()),
                value: Box::new(maybe_value.unwrap()),
                mutable,
                annotation: Some(Box::new(maybe_type.unwrap())),
            }))
        } else {
            None
        }
    }

    fn parse_number(&mut self) -> Option<Node> {
        let this = self._this();

        // Floating point values
        if this.value.as_ref().is_some_and(|v| v.contains(".")) {
            let val: f32 = this
                .value
                .as_ref()
                .unwrap()
                .parse()
                .expect("Error parsing float");
            Some(Node::Float(val))

        // Integers
        } else {
            let val: i32 = this
                .value
                .as_ref()
                .unwrap()
                .parse()
                .expect("Error parsing integer");
            Some(Node::Integer(val))
        }
    }

    fn _get_operator_and_precedence(&mut self) -> Option<(BinaryOp, u8)> {
        match self._this().kind {
            Tk::Plus => Some((BinaryOp::Plus, 2)),
            Tk::Minus => Some((BinaryOp::Minus, 2)),
            Tk::Star => Some((BinaryOp::Multiply, 1)),
            Tk::Slash => Some((BinaryOp::Divide, 1)),
            Tk::Modulo => Some((BinaryOp::Modulo, 1)),
            Tk::Exponent => Some((BinaryOp::Exponent, 0)),
            _ => None,
        }
    }

    fn _this(&self) -> &'a Token {
        self.as_token.get(self.idx).unwrap()
    }

    fn _at(&self, offset: isize) -> Option<&'a Token> {
        if self.idx as isize + offset >= 0 {
            Some(
                self.as_token
                    .get((self.idx as isize + offset) as usize)
                    .unwrap(),
            )
        } else {
            None
        }
    }

    fn _len(&self) -> usize {
        self.as_kind.len()
    }
}
