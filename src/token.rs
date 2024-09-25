use std::ops::Range;

#[derive(Debug)]
pub struct Span {
    range: Range<usize>,
}

impl Span {
    pub fn from(start: usize, len: usize) -> Self {
        Self {
            range: Range {
                start,
                end: (start + len - 1),
            },
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum TokenKind {
    // Grouping tokens
    LPar,
    RPar,
    LBrac,
    RBrac,
    LCurl,
    RCurl,

    // Arithmetic tokens
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    Slash,
    SlashSlash,
    Caret,
    Modulo,

    // Symbol tokens
    LArrow,
    RArrow,
    Hash,
    At,
    Ampersand,
    Colon,
    ColonColon,
    ColonEqual,
    Semicolon,
    Comma,
    Dot,

    // Comparison tokens
    More,
    MoreEqual,
    Less,
    LessEqual,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,

    // Keyword tokens
    If,
    Else,
    Elif,
    For,
    While,
    New,
    Mut,
    Func,

    // Literal tokens
    Literal { value: String },
    Number { value: String },
    Ident { value: String },

    // Other tokens
    EndOfFile,
}

/// Takes a string as input and returns and option type containing the TokenKind that string pertains to.
///
/// For key/reserved words only.
pub fn get_keyword(src: &String) -> Option<TokenKind> {
    match src.as_str() {
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "elif" => Some(TokenKind::Elif),
        "for" => Some(TokenKind::For),
        "while" => Some(TokenKind::While),
        "new" => Some(TokenKind::New),
        "mut" => Some(TokenKind::Mut),
        "func" => Some(TokenKind::Func),
        _ => None,
    }
}
