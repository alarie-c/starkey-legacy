use core::str;

use crate::lexer::token::{self, Span, Token, TokenKind};

pub struct Lexer<'a> {
    pub src: &'a [u8],
    pub idx: usize,
    pub output: Vec<Token>,
}

impl<'a> Lexer<'a> {
    // Return a new lexer with the string slice as a slice of u8
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.as_bytes(),
            idx: 0usize,
            output: Vec::new(),
        }
    }

    pub fn scan(&mut self) -> &Vec<Token> {
        loop {
            // Push EOF + break when EOF condition reached
            if self.idx >= self.src.len() {
                self.push_token(TokenKind::EndOfFile, self.idx, 1);
                break;
            }

            dbg!(self.src[self.idx] as char);

            // Match current slice
            match &self.src[self.idx..] {
                // Ignore useless chars
                [b' ', ..] | [b'\n', ..] | [b'\t', ..] | [b'\r', ..] => self.idx += 1,

                [b'+', b'=', ..] => self.push_token(TokenKind::PlusEqual, self.idx, 2),
                [b'-', b'=', ..] => self.push_token(TokenKind::MinusEqual, self.idx, 2),
                [b'-', b'>', ..] => self.push_token(TokenKind::RArrow, self.idx, 2),
                [b'/', b'/', ..] => self.push_token(TokenKind::SlashSlash, self.idx, 2),
                [b'<', b'-', ..] => self.push_token(TokenKind::LArrow, self.idx, 2),
                [b':', b':', ..] => self.push_token(TokenKind::ColonColon, self.idx, 2),
                [b':', b'=', ..] => self.push_token(TokenKind::ColonEqual, self.idx, 2),
                [b'<', b'=', ..] => self.push_token(TokenKind::LessEqual, self.idx, 2),
                [b'>', b'=', ..] => self.push_token(TokenKind::MoreEqual, self.idx, 2),
                [b'=', b'=', ..] => self.push_token(TokenKind::EqualEqual, self.idx, 2),
                [b'!', b'=', ..] => self.push_token(TokenKind::BangEqual, self.idx, 2),
                [b'<', ..] => self.push_token(TokenKind::Less, self.idx, 1),
                [b'>', ..] => self.push_token(TokenKind::More, self.idx, 1),
                [b'!', ..] => self.push_token(TokenKind::Bang, self.idx, 1),
                [b'=', ..] => self.push_token(TokenKind::Equal, self.idx, 1),
                [b'(', ..] => self.push_token(TokenKind::LPar, self.idx, 1),
                [b')', ..] => self.push_token(TokenKind::RPar, self.idx, 1),
                [b'{', ..] => self.push_token(TokenKind::LCurl, self.idx, 1),
                [b'}', ..] => self.push_token(TokenKind::RCurl, self.idx, 1),
                [b'[', ..] => self.push_token(TokenKind::LBrac, self.idx, 1),
                [b']', ..] => self.push_token(TokenKind::RBrac, self.idx, 1),
                [b'+', ..] => self.push_token(TokenKind::Plus, self.idx, 1),
                [b'-', ..] => self.push_token(TokenKind::Minus, self.idx, 1),
                [b'/', ..] => self.push_token(TokenKind::Slash, self.idx, 1),
                [b'*', ..] => self.push_token(TokenKind::Star, self.idx, 1),
                [b'^', ..] => self.push_token(TokenKind::Caret, self.idx, 1),
                [b'%', ..] => self.push_token(TokenKind::Modulo, self.idx, 1),
                [b',', ..] => self.push_token(TokenKind::Comma, self.idx, 1),
                [b'.', ..] => self.push_token(TokenKind::Dot, self.idx, 1),
                [b':', ..] => self.push_token(TokenKind::Colon, self.idx, 1),
                [b';', ..] => self.push_token(TokenKind::Semicolon, self.idx, 1),
                [b'@', ..] => self.push_token(TokenKind::At, self.idx, 1),
                [b'#', ..] => self.push_token(TokenKind::Hash, self.idx, 1),
                [b'&', ..] => self.push_token(TokenKind::Ampersand, self.idx, 1),

                // Look for identifiers & keywords
                [b'"', ..] => self.take_literal(),
                [x, ..] if x.is_ascii_digit() => self.take_number(),
                [x, ..] if x.is_ascii_alphabetic() => {
                    // Save current index for token span
                    let i0 = self.idx;
                    let id = self.take_ident();
                    dbg!(&id.len());

                    // Push tokens based on keyword match result
                    if let Some(kind) = TokenKind::get_keyword(&id) {
                        self.output.push(Token {
                            kind,
                            span: Span::from(i0, id.len()),
                        });
                    } else {
                        self.push_ident(i0, id.len(), id);
                    }

                    continue;
                }
                _ => {}
            }
        }
        &self.output
    }

    fn take_number(&mut self) {
        let mut buf = String::from(self.src[self.idx] as char);
        let i0 = self.idx;
        self.idx += 1;

        // Push every char until something that isn't num or _/. found
        while self.idx < self.src.len()
            && (self.src[self.idx].is_ascii_digit()
                || self.src[self.idx] == b'_'
                || self.src[self.idx] == b'.')
        {
            buf.push(self.src[self.idx] as char);
            self.idx += 1;
        }

        // Push token to output
        let len = buf.len();
        self.output.push(Token {
            kind: TokenKind::Number { value: buf },
            span: Span::from(i0, len),
        });
    }

    fn take_literal(&mut self) {
        let mut buf = String::new();
        let i0 = self.idx;
        self.idx += 1; // advance past the "

        // Push every char until idx out of bounds or " found
        while self.idx < self.src.len() && self.src[self.idx] != b'"' {
            buf.push(self.src[self.idx] as char);
            self.idx += 1;
        }

        // Push token to output
        self.output.push(Token {
            kind: TokenKind::Literal { value: buf },
            span: Span::from(i0, self.idx),
        });

        // Skip the enclosing "
        self.idx += 1;
    }

    fn take_ident(&mut self) -> String {
        let mut buf = String::from(self.src[self.idx] as char);
        self.idx += 1;

        // Push every char until idx out of bounds or condition not satisfied
        while self.idx < self.src.len()
            && (self.src[self.idx].is_ascii_alphanumeric() || self.src[self.idx] == b'_')
        {
            buf.push(self.src[self.idx] as char);
            self.idx += 1;
        }
        buf
    }

    fn push_token(&mut self, kind: TokenKind, start: usize, len: usize) {
        self.output.push(Token {
            kind,
            span: Span::from(start, len),
        });

        // Advance index by length of token
        self.idx += len;
    }

    fn push_ident(&mut self, start: usize, len: usize, value: String) {
        self.output.push(Token {
            kind: TokenKind::Ident { value },
            span: Span::from(start, len),
        });
    }
}
