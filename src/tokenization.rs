use std::process::exit;

#[derive(PartialEq, Clone)]
pub enum TokenType {
    Exit,
    IntLit,
    Semi,
    OpenParen,
    CloseParen,
    Ident,
    Let,
    Eq,
}

#[derive(Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub value: Option<String>,
}

pub fn tokenize(src: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let chars: Vec<char> = src.chars().collect();
    let mut buffer: String = String::new();
    let mut i = 0;
    while peek(&chars, i, 0).is_some() {
        if peek(&chars, i, 0).unwrap().is_alphabetic() {
            buffer.push(consume(&chars, &mut i));
            while peek(&chars, i, 0).is_some() && peek(&chars, i, 0).unwrap().is_alphabetic() {
                buffer.push(consume(&chars, &mut i));
            }
            if buffer == String::from("exit") {
                tokens.push(Token {
                    ttype: TokenType::Exit,
                    value: None,
                });
                buffer.clear();
                continue;
            } else if buffer == String::from("let") {
                tokens.push(Token {
                    ttype: TokenType::Let,
                    value: None,
                });
                buffer.clear();
                continue;
            } else {
                tokens.push(Token {
                    ttype: TokenType::Ident,
                    value: Some(buffer.clone()),
                });
                buffer.clear();
                continue;
            }
        } else if peek(&chars, i, 0).unwrap().is_digit(10) {
            buffer.push(consume(&chars, &mut i));
            while peek(&chars, i, 0).is_some() && peek(&chars, i, 0).unwrap().is_digit(10) {
                buffer.push(consume(&chars, &mut i));
            }
            tokens.push(Token {
                ttype: TokenType::IntLit,
                value: Some(buffer.clone()),
            });
            buffer.clear();
            continue;
        } else if peek(&chars, i, 0).unwrap() == '(' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::OpenParen,
                value: None,
            });
            continue;
        } else if peek(&chars, i, 0).unwrap() == ')' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::CloseParen,
                value: None,
            });
            continue;
        } else if peek(&chars, i, 0).unwrap() == ';' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::Semi,
                value: None,
            });
            continue;
        } else if peek(&chars, i, 0).unwrap() == '=' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::Eq,
                value: None,
            });
            continue;
        } else if peek(&chars, i, 0).unwrap().is_whitespace() {
            consume(&chars, &mut i);
            continue;
        } else {
            println!("wrong!");
            exit(1);
        }
    }
    tokens
}

fn peek(chars: &Vec<char>, i: usize, offset: usize) -> Option<char> {
    if i + offset >= chars.len() {
        return None;
    } else {
        return Some(chars[i + offset]);
    }
}

fn consume(chars: &Vec<char>, i: &mut usize) -> char {
    let v = chars[*i];
    *i += 1;
    v
}
