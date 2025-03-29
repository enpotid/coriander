mod generation;
mod parser;
mod tokenization;

use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("wrong!");
        println!("ander <main.dr>");
        return;
    }

    let contents = match fs::read_to_string(&args[1]) {
        Ok(f) => f,
        Err(_) => {
            println!("can't read file");
            println!("ander <main.dr>");
            return;
        }
    };

    let tokens = tokenization::tokenize(contents);

    let prog = parser::parse_prog(&tokens);
    if prog.is_none() {
        println!("invalid program!");
        return;
    }

    let output = generation::gen_prog(prog.unwrap());

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
