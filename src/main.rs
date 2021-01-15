extern crate basic;

use basic::{parse, tokenize};
use std::{fs, process};

fn main() {
    let content = fs::read_to_string("./hello.bas").expect("Failed to load source file");

    for line in content.split("\n").into_iter() {
        let tokens = tokenize(String::from(line));
        if tokens.is_empty() {
            continue;
        }

        match parse(tokens) {
            Ok(stmt) => {
                println!("{:?}", stmt);
            }
            Err(msg) => {
                eprintln!("{}", msg);
                process::exit(1);
            }
        }
    }
}
