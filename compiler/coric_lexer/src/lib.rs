mod cursor;

use coric_ast::{Span, Token, TokenKind};
use coric_logger::Logger;
use cursor::*;
use std::{iter::Peekable, ops::Range, vec::IntoIter};

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
                    self.if_push(token, vec![(TokenKind::Or, TokenKind::OrOr)]);
                }
                TokenKind::And => {
                    self.if_push(token, vec![(TokenKind::And, TokenKind::AndAnd)]);
                }
                TokenKind::Lt => {
                    self.if_push(token, vec![(TokenKind::Eq, TokenKind::Le)]);
                }
                TokenKind::Gt => {
                    self.if_push(token, vec![(TokenKind::Eq, TokenKind::Ge)]);
                }
                TokenKind::Eq => {
                    self.if_push(token, vec![(TokenKind::Eq, TokenKind::EqEq)]);
                }
                TokenKind::Not => {
                    self.if_push(
                        token,
                        vec![
                            (TokenKind::Not, TokenKind::PathSep),
                            (TokenKind::Eq, TokenKind::Ne),
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

    fn if_push(&mut self, token: Token, list: Vec<(TokenKind, TokenKind)>) {
        let token_peek = self.token_iter.peek().unwrap().clone();
        for (if_token_kind, push_token_kind) in list {
            if token_peek.kind == if_token_kind {
                self.push_token_range(
                    push_token_kind,
                    token.span.range.start..token_peek.span.range.end,
                );
                self.token_iter.next();
                return;
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
