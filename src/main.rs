extern crate basic;

use basic::{compile, parse, tokenize};
use std::{fs, process};

fn main() {
    let content = fs::read_to_string("./basic/hello.bas").expect("Failed to load source file");

    let mut stmts = Vec::new();

    for line in content.split("\n").into_iter() {
        let tokens = tokenize(String::from(line));
        if tokens.is_empty() {
            continue;
        }

        match parse(tokens) {
            Ok(stmt) => {
                stmts.push(stmt);
            }
            Err(msg) => {
                eprintln!("{}", msg);
                process::exit(1);
            }
        }
    }

    match compile(&stmts) {
        Ok(asm) => println!("{}", asm),
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        },
    }
}
