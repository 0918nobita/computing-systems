extern crate basic;

use basic::{compile, parse, tokenize};
use std::{fs, process};
use serde_json;

fn main() {
    let content = fs::read_to_string("./basic/hello.bas").expect("Failed to load source file");

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
            fs::write("out.s", asm).expect("Failed to write output file");
        },
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    }
}
