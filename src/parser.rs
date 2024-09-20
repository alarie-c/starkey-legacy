use std::borrow::Borrow;

use crate::{ast::{Ast, Node}, token::{Token, TokenKind}};

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
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a Vec<Token>) -> Self {
        Self {
            ast: Ast { nodes: Vec::new() },
            as_token: src,
            as_kind: src.iter().map(|x| &x.kind).collect(),
            idx: 0usize,
        }
    }

    /// The game plan is as follows:
    /// 
    /// We are going to look for patterns that we would want to parse, EXCLUDING leaf nodes.
    /// Leafs nodes aren't going to be parsed until we can be certain they're going to live under something
    /// There is a expression() method which will be the main componenet of this but this one also parses leaf nodes
    /// because it's the one we wanna call recursively/.
    pub fn parse() -> Option<()> {
        




        Some(())
    }

    fn expression(&mut self) -> Option<Node> {
        match self.as_kind.as_slice()[self.idx ..] {
            [Tk::Number, ..] => self.parse_number(),
            
            _ => None,
        }
    }

    /// Once again we are assembling a game plan:
    /// 
    /// There's a couple different routes this could go.
    /// 1. There's a chance it's `name = value`
    /// 2. There's a chance it's `name :: type = value`
    /// 3. There's a chance it's `mut name = value`
    /// 4. There's a chance it's `mut name :: type = value`
    /// 
    /// So we start by identifying the beginning only
    /// 1. `[Mut, Ident, ColonColon, ..]` means we're doing mut and type
    /// 2. `[Mut, Ident, ..]` means we're doing mut only
    /// 3. `[Ident, ColonColon, ..]` means we're doing type only
    /// 4. `[Ident, ..]` means we're doing nothing
    /// So with this we can identify what route we're taking and make our nodes from there
    fn parse_variable_decl(&mut self) -> Option<Node> {
        if self.idx <= 1 {
            // This means there's only an identifier and an equals

        } else if self.idx 
    }

    fn parse_number(&mut self) -> Option<Node> {
        let this = self.this();

        // Floating point values
        if this.value.as_ref().is_some_and(|v| v.contains(".")) {
            let val: f32 = this.value.as_ref().unwrap().parse().expect("Error parsing float");
            Some(Node::Float(val))
        
        // Integers
        } else {
            let val: i32 = this.value.as_ref().unwrap().parse().expect("Error parsing integer");
            Some(Node::Integer(val))
        }
    }

    fn this(&self) -> &'a Token {
        self.as_token.get(self.idx).unwrap()
    }
}