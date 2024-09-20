use std::{fmt::Display, ops::Range};

// Span should be used for error reporting and records a range of indicies from
// the raw data of the source code. Readable ranges will need to be constructed by
// counting newlines and columns.
#[derive(Debug)]
pub struct TextSpan {
    pub range: Range<usize>
}

impl TextSpan {
    pub fn from(start: usize, len: usize) -> Self {
        Self {
            range: Range::from(start..start + (len - 1))
        }
    }
}

// Holds information for every token created by the lexer, including a span
#[derive(Debug)]
pub struct Token {
    pub span: TextSpan,
    pub kind: TokenKind,
    pub value: Option<String>,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} :: {:?} :: {}", self.span, self.kind, self.get_lexeme())
    }
}


impl Token {
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
#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Grouping
    LPar,
    RPar,
    LBrac,
    RBrac,
    LCurl,
    RCurl,

    // Operators
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Equal,
    EqualEqual,
    Star,
    Slash,
    SlashSlash,
    Hash,
    Ampersand,
    Exponent,
    Modulo,

    // Comparisons
    More,
    MoreEqual,
    Less,
    LessEqual,
    Bang,
    BangEqual,

    // Symbols
    Colon,
    ColonColon,
    Semicolon,
    Comma,
    Dot,
    Arrow,
    Logger,

    // Literals
    Literal,
    Number,
    Identifier,

    // Keywords
    Let,
    Mut,
    If,
    Elif,
    Else,
    While,
    For,

    // Other
    EndOfFile,
}