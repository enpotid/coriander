use crate::*;
use std::process::exit;

pub fn tokenize(src: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let chars: Vec<char> = src.chars().collect();
    let mut buffer: String = String::new();
    let mut i = 0;
    while peak(&chars, i, 0).is_some() {
        if peak(&chars, i, 0).unwrap().is_alphabetic() {
            buffer.push(consume(&chars, &mut i));
            while peak(&chars, i, 0).is_some() && peak(&chars, i, 0).unwrap().is_alphabetic() {
                buffer.push(consume(&chars, &mut i));
            }
            if buffer == String::from("exit") {
                tokens.push(Token {
                    ttype: TokenType::Exit,
                    value: None,
                });
                buffer.clear();
                continue;
            } else {
                println!("wrong!");
                exit(1);
            }
        } else if peak(&chars, i, 0).unwrap().is_digit(10) {
            buffer.push(consume(&chars, &mut i));
            while peak(&chars, i, 0).is_some() && peak(&chars, i, 0).unwrap().is_digit(10) {
                buffer.push(consume(&chars, &mut i));
            }
            tokens.push(Token {
                ttype: TokenType::IntLit,
                value: Some(buffer.clone()),
            });
            buffer.clear();
            continue;
        } else if peak(&chars, i, 0).unwrap() == ';' {
            consume(&chars, &mut i);
            tokens.push(Token {
                ttype: TokenType::Semi,
                value: None,
            });
            continue;
        } else if peak(&chars, i, 0).unwrap().is_whitespace() {
            consume(&chars, &mut i);
            continue;
        } else {
            println!("wrong!");
            exit(1);
        }
    }
    tokens
}

fn peak(chars: &Vec<char>, i: usize, offset: usize) -> Option<char> {
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
