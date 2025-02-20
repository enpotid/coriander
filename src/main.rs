use std::env;
use std::fs;
use std::process::exit;
use std::process::Command;

#[derive(PartialEq)]
enum TokenType {
    Return,
    IntLit,
    Semi,
}

struct Token {
    ttype: TokenType,
    value: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("wrong!");
        println!("coriander <main.dr>");
        return;
    }

    let contents = match fs::read_to_string(&args[1]) {
        Ok(f) => f,
        Err(_) => {
            println!("can't read file");
            println!("coriander <main.dr>");
            return;
        }
    };

    let tokens = tokenize(contents);
    let output = token_to_asm(tokens);

    fs::write("out.asm", output).unwrap();
    Command::new("nasm")
        .args(["-felf64", "out.asm"])
        .spawn()
        .unwrap();
    Command::new("ld")
        .args(["out.o", "-o", "out"])
        .spawn()
        .unwrap();
}

fn tokenize(contents: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let chars: Vec<char> = contents.chars().collect();
    let mut buffer: String = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_alphabetic() {
            buffer.push(c);
            i += 1;
            while chars[i].is_alphanumeric() {
                buffer.push(chars[i]);
                i += 1;
                if i >= chars.len() {
                    continue;
                }
            }
            i -= 1;

            if buffer == String::from("return") {
                tokens.push(Token {
                    ttype: TokenType::Return,
                    value: None,
                });
                buffer.clear();
            } else {
                println!("wrong!");
                exit(1);
            }
        } else if c.is_numeric() {
            buffer.push(c);
            i += 1;
            while chars[i].is_numeric() {
                buffer.push(chars[i]);
                i += 1;
                if i >= chars.len() {
                    continue;
                }
            }
            i -= 1;

            tokens.push(Token {
                ttype: TokenType::IntLit,
                value: Some(buffer.clone()),
            });
            buffer.clear();
        } else if c == ';' {
            tokens.push(Token {
                ttype: TokenType::Semi,
                value: None,
            });
        } else if c.is_whitespace() {
        } else {
            println!("wrong!");
            exit(1);
        }
        i += 1;
    }
    tokens
}

fn token_to_asm(tokens: Vec<Token>) -> String {
    let mut output = String::from("global _start\n_start:\n");
    for i in 0..tokens.len() {
        if tokens[i].ttype == TokenType::Return {
            if i + 1 < tokens.len() && tokens[i + 1].ttype == TokenType::IntLit {
                if i + 2 < tokens.len() && tokens[i + 2].ttype == TokenType::Semi {
                    output.push_str(&format!(
                        "    mov rax, 60\n    mov rdi, {}\n    syscall\n",
                        tokens[i + 1]
                            .value
                            .clone()
                            .unwrap()
                            .parse::<usize>()
                            .unwrap()
                    ));
                }
            }
        }
    }
    output
}
