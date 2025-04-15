use coric_codegen::Target;
use coric_lexer::Lexer;
use coric_logger::Logger;
use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("wrong!");
        println!("coric <main.dr>");
        return;
    }

    let src = match fs::read_to_string(&args[1]) {
        Ok(f) => f,
        Err(_) => {
            println!("can't read file");
            println!("coric <main.dr>");
            return;
        }
    };

    let logger = Logger::new(src.clone(), args[1].clone());

    let mut lexer = Lexer::new(&src, args[1].clone(), logger);
    let tokens = lexer.tokenize();

    println!("{tokens:#?}");

    /*
    let ast = match coric_parse::parse_program(&tokens) {
        Some(ast) => ast,
        None => {
            println!("invalid program!");
            return;
        }
    };

    let output = coric_codegen::generate(ast, Target::Asm);

    fs::write("out.asm", output).unwrap();
    Command::new("nasm")
        .args(["-felf64", "out.asm"])
        .spawn()
        .unwrap();
    Command::new("ld")
        .args(["out.o", "-o", "out"])
        .spawn()
        .unwrap();
    */
}
