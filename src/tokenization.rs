use std::process::exit;

#[derive(PartialEq, Clone)]
pub enum TokenType {
    IntLit,
    Semi,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Tilde,
    Ident,
    String,
    Let,
    Eq,
    Plus,
    Star,
    Sub,
    Div,
    Mod,
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
    'l: while peek(&chars, i, 0).is_some() {
        if peek(&chars, i, 0).unwrap().is_alphabetic() || peek(&chars, i, 0).unwrap() == '_' {
            buffer.push(consume(&chars, &mut i));
            while peek(&chars, i, 0).is_some()
                && (peek(&chars, i, 0).unwrap().is_alphabetic()
                    || peek(&chars, i, 0).unwrap() == '_'
                    || peek(&chars, i, 0).unwrap().is_digit(10))
            {
                buffer.push(consume(&chars, &mut i));
            }

            if buffer == String::from("let") {
                tokens.push(Token {
                    ttype: TokenType::Let,
                    value: None,
                });
                buffer.clear();
            } else {
                tokens.push(Token {
                    ttype: TokenType::Ident,
                    value: Some(buffer.clone()),
                });
                buffer.clear();
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
        } else if peek(&chars, i, 0).unwrap() == '(' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::OpenParen,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == ')' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::CloseParen,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == '{' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::OpenBrace,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == '}' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::CloseBrace,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == '[' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::OpenBracket,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == ']' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::CloseBracket,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == ';' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::Semi,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == '"' {
            consume(&chars, &mut i);
            let mut buffer = String::new();
            while peek(&chars, i, 0).is_some() {
                let x = consume(&chars, &mut i);
                if x == '"' {
                    tokens.push(Token {
                        ttype: TokenType::String,
                        value: Some(buffer.clone()),
                    });
                    continue 'l;
                }
                buffer.push(x);
            }
            println!("wrong!");
            exit(1);
        } else if peek(&chars, i, 0).unwrap() == '~' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::Tilde,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == '=' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::Eq,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == '+' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::Plus,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == '*' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::Star,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == '-' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::Sub,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == '/' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::Div,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap() == '%' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::Mod,
                value: None,
            });
        } else if peek(&chars, i, 0).unwrap().is_whitespace() {
            consume(&chars, &mut i);
        } else {
            println!("wrong!");
            exit(1);
        }
    }
    tokens
}

pub fn bin_prec(ttype: TokenType) -> Option<usize> {
    match ttype {
        TokenType::Plus => Some(0),
        TokenType::Sub => Some(0),
        TokenType::Star => Some(1),
        TokenType::Div => Some(1),
        TokenType::Mod => Some(1),
        _ => None,
    }
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
