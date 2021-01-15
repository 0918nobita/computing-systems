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
            Ok(_) => (),
            Err(msg) => {
                eprintln!("{}", msg);
                process::exit(1);
            }
        }
    }

    println!("bits 64");
    println!("global _start");
    println!("section .data");
    println!("section .text");
    println!("_start:");
    println!("    mov rax, 60");
    println!("    xor rdi, rdi");
    println!("    syscall");
}
