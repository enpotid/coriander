use coric_ast::{KeywordKind, LiteralKind, Span, Token, TokenKind};
use std::{iter::Peekable, ops::Range, str::CharIndices};

pub struct Cursor<'a> {
    pub tokens: Vec<Token>,
    pub chars: Peekable<CharIndices<'a>>,
    pub src_len: usize,
    pub filename: String,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str, filename: String) -> Self {
        Cursor {
            tokens: Vec::new(),
            chars: src.char_indices().peekable(),
            src_len: src.len(),
            filename,
        }
    }

    pub fn read_all(&mut self) {
        let mut buffer = String::new();
        'n: while let Some((i, c)) = self.chars.next() {
            match c {
                _ if c == '/' => {
                    if let Some(&(ei, c)) = self.chars.peek() {
                        if c == '/' {
                            self.chars.next();
                            let mut e = ei + 1;
                            'c: while let Some(&(ei, c)) = self.chars.peek() {
                                e = ei + 1;
                                if c != '\n' {
                                    self.chars.next();
                                } else {
                                    e -= 1;
                                    break 'c;
                                }
                            }

                            self.push_token(TokenKind::Comment, i..e);
                            continue 'n;
                        }
                    }
                    self.push_token_char(TokenKind::Slash, i);
                }

                _ if c.is_alphabetic() || c == '_' => {
                    buffer.push(c);
                    let mut e = i + 1;
                    'i: while let Some(&(ei, c)) = self.chars.peek() {
                        e = ei + 1;
                        if c.is_alphabetic() || c.is_digit(10) || c == '_' {
                            buffer.push(c);
                            self.chars.next();
                        } else {
                            e -= 1;
                            break 'i;
                        }
                    }

                    if let Some(keyword) = KeywordKind::from_str(&buffer) {
                        self.push_token(TokenKind::Keyword(keyword), i..e);
                    } else {
                        self.push_token(TokenKind::Ident(buffer.clone()), i..e);
                    }
                    buffer.clear();
                }

                '0'..='9' => {
                    buffer.push(c);
                    let mut isf = false;
                    let mut e = i + 1;
                    'l: while let Some(&(ei, c)) = self.chars.peek() {
                        e = ei + 1;
                        match c {
                            '0'..='9' => {
                                buffer.push(c);
                                self.chars.next();
                            }
                            '.' => {
                                if !isf {
                                    isf = true;
                                    buffer.push(c);
                                    self.chars.next();
                                } else {
                                    e -= 1;
                                    break 'l;
                                }
                            }
                            _ => {
                                e -= 1;
                                break 'l;
                            }
                        }
                    }

                    if isf {
                        self.push_token(
                            TokenKind::Literal(LiteralKind::Float(buffer.clone())),
                            i..e,
                        );
                    } else {
                        self.push_token(TokenKind::Literal(LiteralKind::Int(buffer.clone())), i..e);
                    }
                    buffer.clear();
                }

                '"' => {
                    let mut e = i + 1;
                    while let Some(&(ei, c)) = self.chars.peek() {
                        e = ei + 1;
                        if c != '"' {
                            buffer.push(c);
                            self.chars.next();
                        } else {
                            self.chars.next();
                            self.push_token(
                                TokenKind::Literal(LiteralKind::Str(buffer.clone())),
                                i..e,
                            );
                            buffer.clear();
                            continue 'n;
                        }
                    }

                    self.push_token(TokenKind::Unknown, i..e);
                    buffer.clear();
                }

                ';' => self.push_token_char(TokenKind::Semi, i),
                ',' => self.push_token_char(TokenKind::Comma, i),
                '.' => self.push_token_char(TokenKind::Dot, i),
                '~' => self.push_token_char(TokenKind::Tilde, i),
                '@' => self.push_token_char(TokenKind::At, i),
                '#' => self.push_token_char(TokenKind::Pound, i),
                '(' => self.push_token_char(TokenKind::OpenParen, i),
                ')' => self.push_token_char(TokenKind::CloseParen, i),
                '{' => self.push_token_char(TokenKind::OpenBrace, i),
                '}' => self.push_token_char(TokenKind::CloseBrace, i),
                '[' => self.push_token_char(TokenKind::OpenBracket, i),
                ']' => self.push_token_char(TokenKind::CloseBracket, i),
                '=' => self.push_token_char(TokenKind::Eq, i),
                '<' => self.push_token_char(TokenKind::Lt, i),
                '>' => self.push_token_char(TokenKind::Gt, i),
                '?' => self.push_token_char(TokenKind::Question, i),
                '!' => self.push_token_char(TokenKind::Not, i),
                ':' => self.push_token_char(TokenKind::Colon, i),
                '-' => self.push_token_char(TokenKind::Minus, i),
                '+' => self.push_token_char(TokenKind::Plus, i),
                '*' => self.push_token_char(TokenKind::Star, i),
                '^' => self.push_token_char(TokenKind::Caret, i),
                '%' => self.push_token_char(TokenKind::Percent, i),
                '$' => self.push_token_char(TokenKind::Dollar, i),
                '|' => self.push_token_char(TokenKind::Or, i),
                '&' => self.push_token_char(TokenKind::And, i),

                _ if c.is_whitespace() => self.push_token_char(TokenKind::Whitespace, i),
                _ => self.push_token_char(TokenKind::Unknown, i),
            }
        }
        self.push_eof();
    }

    fn push_token(&mut self, kind: TokenKind, range: Range<usize>) {
        self.tokens.push(Token {
            kind,
            span: Span {
                file: self.filename.clone(),
                range,
            },
        });
    }

    fn push_token_char(&mut self, kind: TokenKind, start: usize) {
        self.push_token(kind, start..start + 1);
    }

    fn push_eof(&mut self) {
        self.push_token(TokenKind::Eof, self.src_len..self.src_len);
    }
}
