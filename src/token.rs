use std::{fmt::Display, ops::Range};

// Span should be used for error reporting and records a range of indicies from
// the raw data of the source code. Readable ranges will need to be constructed by
// counting newlines and columns.
#[derive(Debug)]
pub struct TextSpan {
    pub range: Range<usize>
}

impl TextSpan {
    pub fn from(start: usize, end: usize) -> Self {
        Self {
            range: Range::from(start..end)
        }
    }
}

// Holds information for every token created by the lexer, including a span
pub struct Token {
    pub span: TextSpan,
    pub kind: TokenKind,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} :: {:?} :: {}", self.span, self.kind, self.get_lexeme())
    }
}


impl Token {
    pub fn from(kind: TokenKind, start: usize, end: usize) -> Self {
        let span = TextSpan::from(start, end);
        Self { span, kind }
    }

    pub fn get_lexeme(&self) -> &str {
        match &self.kind {
            TokenKind::LPar => "(",
            TokenKind::RPar => ")",
            TokenKind::Plus => "+",
            TokenKind::Colon => ":",
            TokenKind::EndOfFile => "<EOF>",
            _ => " "
        }
    }
}

// Defines each kind of token to be parsed
// Some, like literals and identifiers hold string values which are the lexeme
// from the source code, generally the name of an identifier or the value of a literal
#[derive(Debug)]
pub enum TokenKind {
    // Grouping
    LPar,
    RPar,

    // Operators
    Plus,

    // Symbols
    Colon,

    // Keywords
    Let,
    Mut,

    // Other
    EndOfFile,
}