extern crate basic;

use basic::{compiler::compile, parser::parse, tokenizer::tokenize};
use serde_json;
use std::{
    env, fs,
    process::{self, Command},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please specify a source file");
    let content = fs::read_to_string(filename).expect("Failed to load the source file");

    let mut stmts = Vec::new();

    for (i, line) in content.split("\n").into_iter().enumerate() {
        match tokenize(String::from(line), i as i32) {
            Ok(tokens) => {
                if tokens.is_empty() {
                    continue;
                }

                match parse(&tokens) {
                    Ok(stmt) => {
                        stmts.push(stmt);
                    }
                    Err(msg) => {
                        eprintln!("{}", msg);
                        process::exit(1);
                    }
                }
            }
            Err(msg) => {
                eprintln!("{}", msg);
                process::exit(1);
            }
        }
    }

    println!("{}", serde_json::to_string_pretty(&stmts).unwrap());

    match compile(&stmts) {
        Ok(asm) => {
            fs::write("out.s", asm.stringify()).expect("Failed to write output file");
        }
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    }

    Command::new("nasm")
        .args(&["-f", "elf64", "out.s"])
        .output()
        .expect("Failed to execute `nasm`");

    Command::new("ld")
        .args(&["-o", "out.bin", "out.o"])
        .output()
        .expect("Failed to execute `ld`");
}
