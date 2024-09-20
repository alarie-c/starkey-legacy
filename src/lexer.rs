use crate::token::{TextSpan, Token, TokenKind};

pub struct Lexer<'a> {
    src: &'a [u8],
    idx: usize,
    output: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src: src.as_bytes(), idx: 0usize, output: Vec::new() }
    }

    pub fn scan(&mut self) -> &Vec<Token> {       
        loop {
            // Break the loop if we reach the end
            if self.idx >= self.src.len() {
                self.add_token(TokenKind::EndOfFile, self.idx, 1, None);
                break;
            }

            // Match current chars
            match &self.src[self.idx..] {
                // Ignore whitespace & newline chars
                [b' ', ..] | [b'\t', ..] | [b'\n', ..] | [b'\r', ..] => self.idx += 1,
                
                // Grouping tokens
                [b'(', ..] => self.add_token(TokenKind::LPar, self.idx, 1, None),
                [b')', ..] => self.add_token(TokenKind::RPar, self.idx, 1, None),
                [b'[', ..] => self.add_token(TokenKind::LBrac, self.idx, 1, None),
                [b']', ..] => self.add_token(TokenKind::RBrac, self.idx, 1, None),
                [b'{', ..] => self.add_token(TokenKind::LCurl, self.idx, 1, None),
                [b'}', ..] => self.add_token(TokenKind::RCurl, self.idx, 1, None),

                // Operator tokens
                [b'/', b'/', ..] => self.add_token(TokenKind::SlashSlash, self.idx, 2, None),
                [b'+', b'=', ..] => self.add_token(TokenKind::PlusEqual, self.idx, 2, None),
                [b'-', b'=', ..] => self.add_token(TokenKind::MinusEqual, self.idx, 2, None),
                [b'-', b'>', ..] => self.add_token(TokenKind::Arrow, self.idx, 2, None),
                [b':', b':', ..] => self.add_token(TokenKind::ColonColon, self.idx, 2, None),
                [b'+', ..] => self.add_token(TokenKind::Plus, self.idx, 1, None),
                [b'-', ..] => self.add_token(TokenKind::Minus, self.idx, 1, None),
                [b'%', ..] => self.add_token(TokenKind::Modulo, self.idx, 1, None),
                [b'&', ..] => self.add_token(TokenKind::Ampersand, self.idx, 1, None),
                [b'*', ..] => self.add_token(TokenKind::Star, self.idx, 1, None),
                [b'/', ..] => self.add_token(TokenKind::Slash, self.idx, 1, None),
                [b'#', ..] => self.add_token(TokenKind::Hash, self.idx, 1, None),
                [b'$', ..] => self.add_token(TokenKind::Logger, self.idx, 1, None),
                [b'^', ..] => self.add_token(TokenKind::Exponent, self.idx, 1, None),
                [b';', ..] => self.add_token(TokenKind::Semicolon, self.idx, 1, None),
                [b'.', ..] => self.add_token(TokenKind::Dot, self.idx, 1, None),
                [b',', ..] => self.add_token(TokenKind::Comma, self.idx, 1, None),
                [b':', ..] => self.add_token(TokenKind::Colon, self.idx, 1, None),
                
                // Comparison tokens
                [b'=', b'=', ..] => self.add_token(TokenKind::EqualEqual, self.idx, 2, None),
                [b'<', b'=', ..] => self.add_token(TokenKind::LessEqual, self.idx, 2, None),
                [b'>', b'=', ..] => self.add_token(TokenKind::MoreEqual, self.idx, 2, None),
                [b'!', b'=', ..] => self.add_token(TokenKind::BangEqual, self.idx, 2, None),
                [b'=', ..] => self.add_token(TokenKind::Equal, self.idx, 1, None),
                [b'<', ..] => self.add_token(TokenKind::Less, self.idx, 1, None),
                [b'>', ..] => self.add_token(TokenKind::More, self.idx, 1, None),
                [b'!', ..] => self.add_token(TokenKind::Bang, self.idx, 1, None),

                // Literals
                [b'"', ..] => self.str_literal(),
                [x, ..] if x.is_ascii_digit() => self.num_literal(),
                [x, ..] if x.is_ascii_alphabetic() => {
                    // Save current index for token span
                    let begin = self.idx;
                    let identifier = self.take_ident();

                    // Check for keyword matches
                    match identifier.as_str() {
                        "if" => self.static_add_token(TokenKind::If, begin, 2, None),
                        "else" => self.static_add_token(TokenKind::Else, begin, 4, None),
                        "elif" => self.static_add_token(TokenKind::Elif, begin, 4, None),
                        "while" => self.static_add_token(TokenKind::While, begin, 5, None),
                        "let" => self.static_add_token(TokenKind::Let, begin, 3, None),
                        "mut" => self.static_add_token(TokenKind::Mut, begin, 3, None),
                        "for" => self.static_add_token(TokenKind::For, begin, 3, None),
                        _ => self.static_add_token(TokenKind::Identifier, begin, self.idx - begin, Some(identifier)),
                    }
                },

                // Other
                _ => panic!("! Bad token !"),
            }
        }

        &self.output
    }

    // Pushes a token and advances idx according to length of span
    fn add_token(&mut self, kind: TokenKind, begin: usize, len: usize, value: Option<String>) {
        self.output.push(Token { kind, span: TextSpan::from(begin, len), value });
        self.idx += len;
    }

    // Pushes token but idx remains unchanged
    fn static_add_token(&mut self, kind: TokenKind, begin: usize, len: usize, value: Option<String>) {
        self.output.push(Token { kind, span: TextSpan::from(begin, len), value });
    }

    fn str_literal(&mut self) {
        let begin = self.idx;
        let mut buf = String::from(self.src[self.idx] as char);
        self.idx += 1;
        loop {
            if self.idx >= self.src.len() {
                panic!("! Non-terminating string literal !");
            } else if self.src[self.idx] == b'"' {
                buf.push(self.src[self.idx] as char);
                self.idx += 1;
                break;
            } else {
                buf.push(self.src[self.idx] as char);
                self.idx += 1;
            }
        }

        // Push new token
        self.static_add_token(TokenKind::Literal, begin, self.idx - begin, Some(buf.to_owned()));
    }

    fn num_literal(&mut self) {
        let begin = self.idx;
        let mut buf = String::from(self.src[self.idx] as char);
        self.idx += 1;
        
        while self.idx < self.src.len() {
            if self.src[self.idx].is_ascii_digit() || self.src[self.idx] == b'_' || self.src[self.idx] == b'.' {
                buf.push(self.src[self.idx] as char);
                self.idx += 1;
            } else {
                break;
            }
        }

        // Push new token
        self.static_add_token(TokenKind::Number, begin, self.idx - begin, Some(buf.to_owned()));
    }

    fn take_ident(&mut self) -> String {
        let mut buf = String::from(self.src[self.idx] as char);
        self.idx += 1;

        // Push all consecutive alphanumeric characters
        while self.idx < self.src.len() {
            if self.src[self.idx].is_ascii_alphanumeric() || self.src[self.idx] == b'_' {
                buf.push(self.src[self.idx] as char);
                self.idx += 1;
            } else {
                break;
            }
        }

        buf
    }
}