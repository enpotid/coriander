mod cursor;

use coric_ast::{Span, Token, TokenKind};
use coric_logger::Logger;
use cursor::*;
use std::ops::Range;

pub struct Lexer<'a> {
    logger: Logger,
    tokens: Vec<Token>,
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str, filename: String, logger: Logger) -> Self {
        Lexer {
            logger,
            tokens: Vec::new(),
            cursor: Cursor::new(src, filename),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        self.cursor.read_all();
        let mut token_iter = self.cursor.tokens.clone().into_iter().peekable();

        let mut is_error = false;

        while let Some(token) = token_iter.next() {
            match token.kind {
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

    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn push_token_range(&mut self, kind: TokenKind, range: Range<usize>) {
        self.tokens.push(Token {
            kind,
            span: Span {
                file: self.cursor.filename.clone(),
                range,
            },
        });
    }
}
