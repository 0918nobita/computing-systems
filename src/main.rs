extern crate basic;

use basic::{compiler::compile, parser::parse, tokenizer::tokenize};
use serde_json;
use std::{
    env, fs,
    path::Path,
    process::{self, Command},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let first_arg = args.get(1).expect("Please specify a source file");

    let src_path = Path::new(first_arg);
    let src_dir = src_path.parent().expect("Failed to get directory info");
    let src_filename = src_path
        .file_name()
        .expect("Failed to get name of the source file");

    let out_ast_path = src_dir.join(src_filename).with_extension("ast.json");
    let out_ast_path = out_ast_path.to_str().unwrap();

    let out_asm_path = src_dir.join(src_filename).with_extension("s");
    let out_asm_path = out_asm_path.to_str().unwrap();

    let out_obj_path = src_dir.join(src_filename).with_extension("o");
    let out_obj_path = out_obj_path.to_str().unwrap();

    let out_bin_path = src_dir.join(src_filename).with_extension("bin");
    let out_bin_path = out_bin_path.to_str().unwrap();

    let content = fs::read_to_string(src_path).expect("Failed to load the source file");

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

    fs::write(
        out_ast_path,
        serde_json::to_string_pretty(&stmts).unwrap() + "\n",
    )
    .expect("Failed to write .ast.json file");

    match compile(&stmts) {
        Ok(asm) => {
            fs::write(out_asm_path, asm.stringify()).expect("Failed to write output file");
        }
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    }

    Command::new("nasm")
        .args(&["-f", "elf64", out_asm_path])
        .output()
        .expect("Failed to execute `nasm`");

    Command::new("ld")
        .args(&["-o", out_bin_path, out_obj_path])
        .output()
        .expect("Failed to execute `ld`");
}
