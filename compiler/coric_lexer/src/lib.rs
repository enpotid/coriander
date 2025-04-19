mod cursor;

use coric_ast::{Span, Token, TokenKind};
use coric_logger::Logger;
use cursor::*;
use std::{iter::Peekable, ops::Range, vec::IntoIter};

enum P {
    One(TokenKind, TokenKind),
    Two((TokenKind, TokenKind), (TokenKind, TokenKind)),
}

use P::*;

pub struct Lexer {
    logger: Logger,
    tokens: Vec<Token>,
    token_iter: Peekable<IntoIter<Token>>,
    filename: String,
}

impl Lexer {
    pub fn new(src: &str, filename: String, logger: Logger) -> Self {
        let mut cursor = Cursor::new(src, filename.clone());
        cursor.read_all();
        let token_iter = cursor.tokens.clone().into_iter().peekable();

        Lexer {
            logger,
            tokens: Vec::new(),
            token_iter,
            filename,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut is_error = false;

        while let Some(token) = self.token_iter.next() {
            match token.kind {
                TokenKind::Or => {
                    self.if_push(token, vec![One(TokenKind::Or, TokenKind::OrOr)]);
                }
                TokenKind::And => {
                    self.if_push(token, vec![One(TokenKind::And, TokenKind::AndAnd)]);
                }
                TokenKind::Lt => {
                    self.if_push(
                        token,
                        vec![
                            One(TokenKind::Eq, TokenKind::Le),
                            Two(
                                (TokenKind::Lt, TokenKind::Shl),
                                (TokenKind::Eq, TokenKind::ShlEq),
                            ),
                        ],
                    );
                }
                TokenKind::Gt => {
                    self.if_push(
                        token,
                        vec![
                            One(TokenKind::Eq, TokenKind::Ge),
                            Two(
                                (TokenKind::Gt, TokenKind::Shr),
                                (TokenKind::Eq, TokenKind::ShrEq),
                            ),
                        ],
                    );
                }
                TokenKind::Eq => {
                    self.if_push(token, vec![One(TokenKind::Eq, TokenKind::EqEq)]);
                }
                TokenKind::Not => {
                    self.if_push(
                        token,
                        vec![
                            One(TokenKind::Not, TokenKind::PathSep),
                            One(TokenKind::Eq, TokenKind::Ne),
                        ],
                    );
                }
                TokenKind::Whitespace | TokenKind::Comment => {}
                TokenKind::Unknown => {
                    self.logger.error("unknown token", token.span.range);
                    is_error = true;
                }
                _ => self.push_token(token),
            }
        }

        if is_error {
            Logger::exit();
        }

        self.tokens.clone()
    }

    fn if_push(&mut self, token: Token, list: Vec<P>) {
        let token_peek = self.token_iter.peek().unwrap().clone();
        for p in list {
            match p {
                One(if_token_kind, push_token_kind) => {
                    if token_peek.kind == if_token_kind {
                        self.push_token_range(
                            push_token_kind,
                            token.span.range.start..token_peek.span.range.end,
                        );
                        self.token_iter.next();
                        return;
                    }
                }
                Two((if_token_kind_a, push_token_kind_a), (if_token_kind_b, push_token_kind_b)) => {
                    if token_peek.kind == if_token_kind_a {
                        self.token_iter.next();
                        let second_token_peek = self.token_iter.peek().unwrap().clone();
                        if second_token_peek.kind == if_token_kind_b {
                            self.push_token_range(
                                push_token_kind_b,
                                token.span.range.start..second_token_peek.span.range.end,
                            );
                            self.token_iter.next();
                            return;
                        } else {
                            self.push_token_range(
                                push_token_kind_a,
                                token.span.range.start..token_peek.span.range.end,
                            );
                            return;
                        }
                    }
                }
            }
        }
        self.push_token(token);
    }

    fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn push_token_range(&mut self, kind: TokenKind, range: Range<usize>) {
        self.tokens.push(Token {
            kind,
            span: Span {
                file: self.filename.clone(),
                range,
            },
        });
    }
}
