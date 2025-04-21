use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Keyword(KeywordKind), // Keyword
    Literal(LiteralKind), // Literal
    Ident(String),        // Identifier

    Semi,         // `;`
    Comma,        // `,`
    Dot,          // `.`
    Tilde,        // `~`
    At,           // `@`
    Pound,        // `#`
    OpenParen,    // `(`
    CloseParen,   // `)`
    OpenBrace,    // `{`
    CloseBrace,   // `}`
    OpenBracket,  // `[`
    CloseBracket, // `]`
    Eq,           // `=`
    EqEq,         // `==`
    Lt,           // `<`
    Le,           // `<=`
    Shl,          // `<<`
    ShlEq,        // `<<=`
    Gt,           // `>`
    Ge,           // `>=`
    Shr,          // `>>`
    ShrEq,        // `>>=`
    Question,     // `?`
    Not,          // `!`
    PathSep,      // `!!`
    Ne,           // `!=`
    Colon,        // `:`
    RArrow,       // `->`
    LArrow,       // `<-`
    FatArrow,     // `=>`
    Minus,        // `-`
    Plus,         // `+`
    Star,         // `*`
    Slash,        // `/`
    Caret,        // `^`
    Percent,      // `%`
    Or,           // `|`
    OrOr,         // `||`
    And,          // `&`
    AndAnd,       // `&&`

    Whitespace, // Any whitespace
    Comment,    // // coment
    Unknown,    // Unknown token, e.g. "№"
    Eof,        // End of file
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralKind {
    Int(String),   // `1234`
    Float(String), // `12.34`
    Str(String),   // `"hello"`
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordKind {
    Let,    // let a = 1;
    Index,  // index std!!io;
    Mode,   // mode main;
    Nec,    // nec std;
    Return, // return x;
}

impl KeywordKind {
    pub fn from_str(str: &str) -> Option<Self> {
        match str {
            "let" => Some(KeywordKind::Let),
            "index" => Some(KeywordKind::Index),
            "mode" => Some(KeywordKind::Mode),
            "nec" => Some(KeywordKind::Nec),
            "return" => Some(KeywordKind::Return),
            _ => None,
        }
    }
}
