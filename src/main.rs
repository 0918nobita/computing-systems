extern crate basic;

use basic::{parse, tokenize, ExprAst, StmtAst};
use std::{fs, process};

fn main() {
    let content = fs::read_to_string("./hello.bas").expect("Failed to load source file");

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

    let mut data_section = String::from("section .data\n");
    let mut text_section = String::from("section .text\n_start:\n");

    for (i, stmt) in stmts.into_iter().enumerate() {
        match stmt {
            StmtAst::CallProc(proc, ExprAst::StrLit(str_lit)) if proc.eq("PRINT") => {
                data_section.push_str(format!("    dat{} db '{}', 0x0A\n", i, str_lit).as_str());
                text_section.push_str(
                    format!(
                        "    mov rax, 1\n    mov rdi, 1\n    mov rsi, dat{}\n    mov rdx, {}\n    syscall\n",
                        i,
                        str_lit.len() + 1
                    ).as_str()
                );
            }
            _ => {
                eprintln!("Failed to compile: Unknown procedure");
                process::exit(1);
            }
        }
    }

    text_section.push_str("    mov rax, 60\n");
    text_section.push_str("    xor rdi, rdi\n");
    text_section.push_str("    syscall\n");

    println!("bits 64");
    println!("global _start");
    print!("{}", data_section);
    print!("{}", text_section);
}
