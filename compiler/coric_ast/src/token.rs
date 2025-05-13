use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Literal(LiteralKind), // Literal
    Ident(String),        // Identifier

    Semi,         // `;`
    Comma,        // `,`
    Dot,          // `.`
    DotDot,       // `..`
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
    Ne,           // `!=`
    Colon,        // `:`
    PathSep,      // `::`
    RArrow,       // `->`
    LArrow,       // `<-`
    FatArrow,     // `=>`
    Minus,        // `-`
    Plus,         // `+`
    Star,         // `*`
    Slash,        // `/`
    Caret,        // `^`
    Percent,      // `%`
    Dollar,       // `$`
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
