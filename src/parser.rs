use std::borrow::Borrow;

use crate::{
    ast::{Ast, Node, VariableExpr},
    token::{Token, TokenKind},
};

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
            if self.this().kind == Tk::EndOfFile {
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
            [Tk::Number, ..] => self.parse_number(),
            [Tk::Var, Tk::Identifier, Tk::ColonColon, ..] => self.parse_val_annotated(true),
            [Tk::Val, Tk::Identifier, Tk::ColonColon, ..] => self.parse_val_annotated(false),
            [Tk::Var, Tk::Identifier, Tk::Equal, ..] => self.parse_val(true),
            [Tk::Val, Tk::Identifier, Tk::Equal, ..] => self.parse_val(false),
            [Tk::Identifier, ..] => self.parse_identifier(),
            _ => None,
        }
    }

    fn parse_identifier(&mut self) -> Option<Node> {
        let name = self.this().value.as_ref().unwrap();

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
        let this = self.this();

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

    fn this(&self) -> &'a Token {
        self.as_token.get(self.idx).unwrap()
    }

    fn at(&self, offset: isize) -> Option<&'a Token> {
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
}
