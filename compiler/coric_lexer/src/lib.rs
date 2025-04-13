mod cursor;

use coric_ast::{Span, Token, TokenKind};
use cursor::*;
use std::ops::Range;

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str, filename: String) -> Self {
        Lexer {
            tokens: Vec::new(),
            cursor: Cursor::new(src, filename),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        self.cursor.read_all();
        let mut token_iter = self.cursor.tokens.clone().into_iter().peekable();

        while let Some(token) = token_iter.next() {
            match token {
                o => self.push_token(o),
            }
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
