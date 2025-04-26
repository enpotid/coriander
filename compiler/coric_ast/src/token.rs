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
    Gt,           // `>`
    Ge,           // `>=`
    Shr,          // `>>`
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
    ShlEq,        // `<<=`
    ShrEq,        // `>>=`
    PlusEq,       // `+=`
    MinusEq,      // `-=`
    StarEq,       // `*=`
    SlashEq,      // `/=`
    PercentEq,    // `%=`
    CaretEq,      // `^=`
    OrEq,         // `|=`
    AndEq,        // `&=`

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
    Nec,    // nec std;
    Return, // return x;
    Match,  // match x {}
}

impl KeywordKind {
    pub fn from_str(str: &str) -> Option<Self> {
        match str {
            "let" => Some(KeywordKind::Let),
            "index" => Some(KeywordKind::Index),
            "nec" => Some(KeywordKind::Nec),
            "return" => Some(KeywordKind::Return),
            "match" => Some(KeywordKind::Match),
            _ => None,
        }
    }
}
