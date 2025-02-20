mod generation;
mod parser;
mod tokenization;

use std::env;
use std::fs;
use std::process::Command;

#[derive(PartialEq, Clone)]
enum TokenType {
    Exit,
    IntLit,
    Semi,
}

#[derive(Clone)]
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

    let tokens = tokenization::tokenize(contents);

    let tree = parser::parse(&tokens);
    if tree.is_none() {
        println!("wrong!");
        return;
    }

    let output = generation::generate(tree.unwrap());

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
