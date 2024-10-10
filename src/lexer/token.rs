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

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
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

impl TokenKind {
    /// Takes a string as input and returns and option type containing the TokenKind that string pertains to.
    pub fn get_keyword(src: &String) -> Option<TokenKind> {
        match src.as_str() {
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "elif" => Some(Self::Elif),
            "for" => Some(Self::For),
            "while" => Some(Self::While),
            "new" => Some(Self::New),
            "mut" => Some(Self::Mut),
            "func" => Some(Self::Func),
            _ => None,
        }
    }

    /// Returns `true` or `false` based on if the given variant is a leaf node in the AST
    pub fn is_leaf_node(&self) -> bool {
        match self {
            Self::Literal { value: _ } => true,
            Self::Number { value: _ } => true,
            Self::Ident { value: _ } => true,
            _ => false,
        }
    }

    /// Determines if the given variant is a binary operator, and if so returns it's precedence/index
    pub fn binary_operator(&self) -> (u8, u8) {
        match self {
            Self::Plus => ('+' as u8, 2),
            Self::Minus => ('-' as u8, 2),
            Self::Star => ('*' as u8, 1),
            Self::Slash => ('/' as u8, 1),
            Self::Modulo => ('%' as u8, 1),
            Self::Caret => ('^' as u8, 0),
            _ => (0u8, 0u8),
        }
    }
}





