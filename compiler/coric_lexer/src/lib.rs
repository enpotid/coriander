use coric_ast::{KeywordKind, Span, Token, TokenKind};

pub fn tokenize(src: &str, filename: String) -> Vec<Token> {
    let mut chars = src.char_indices().peekable();
    let mut tokens = Vec::new();

    let mut ident_buffer = String::new();
    while let Some((i, c)) = chars.next() {
        match c {
            _ if c.is_alphabetic() || c == '_' => {
                ident_buffer.push(c);
                while let Some(&(ei, c)) = chars.peek() {
                    if c.is_alphabetic() || c.is_digit(10) || c == '_' {
                        ident_buffer.push(c);
                        chars.next();
                    } else {
                        if let Some(keyword) = KeywordKind::from_str(&ident_buffer) {
                            tokens.push(Token {
                                kind: TokenKind::Keyword(keyword),
                                span: Span {
                                    file: filename.clone(),
                                    range: i..ei,
                                },
                            });
                        } else {
                            tokens.push(Token {
                                kind: TokenKind::Ident(ident_buffer.clone()),
                                span: Span {
                                    file: filename.clone(),
                                    range: i..ei,
                                },
                            });
                        }
                        ident_buffer.clear();
                        break;
                    }
                }
            }
            _ => tokens.push(Token {
                kind: TokenKind::Unknown,
                span: Span {
                    file: filename.clone(),
                    range: i..i + 1,
                },
            }),
        }
    }
    tokens.push(Token {
        kind: TokenKind::Eof,
        span: Span {
            file: filename.clone(),
            range: src.len()..src.len(),
        },
    });

    tokens
}
