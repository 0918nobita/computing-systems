extern crate basic;

use basic::{ast::StmtAst, compiler::compile, parser::parse, tokenizer::tokenize};
use serde_json;
use std::{
    env, fs,
    path::Path,
    process::{self, Command},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let first_arg = args.get(1).expect("Please specify a source file");

    let (input_info, output_info) = get_io_info(first_arg)?;
    let content = fs::read_to_string(input_info.src_path).expect("Failed to read the source file");
    let mut stmts = Vec::<StmtAst>::new();

    for (i, line) in content.split("\n").into_iter().enumerate() {
        let tokens = tokenize(String::from(line), i as i32)?;
        if tokens.is_empty() {
            continue;
        }
        match parse(&tokens) {
            Ok(stmt) => stmts.push(stmt),
            Err(msg) => {
                eprintln!("{}", msg);
                process::exit(1);
            }
        }
    }

    fs::write(
        output_info.ast_path,
        serde_json::to_string_pretty(&stmts).unwrap() + "\n",
    )?;

    let asm = compile(&stmts)?;
    fs::write(&output_info.asm_path, asm.stringify())?;

    Command::new("nasm")
        .args(&["-f", "elf64", &output_info.asm_path])
        .output()?;

    Command::new("ld")
        .args(&["-o", &output_info.bin_path, &output_info.obj_path])
        .output()?;
    Ok(())
}

struct InputInfo {
    src_path: String,
}

struct OutputInfo {
    ast_path: String,
    asm_path: String,
    obj_path: String,
    bin_path: String,
}

fn get_io_info(src_filename: &str) -> Result<(InputInfo, OutputInfo), String> {
    let src_path = Path::new(src_filename);
    let src_dir = src_path.parent().expect("Failed to get directory info");
    let src_filename = src_path
        .file_name()
        .ok_or("Failed to get name of the source file")?;

    let ast_path = src_dir.join(src_filename).with_extension("ast.json");
    let ast_path = ast_path.to_str().unwrap();

    let asm_path = src_dir.join(src_filename).with_extension("s");
    let asm_path = asm_path.to_str().unwrap();

    let obj_path = src_dir.join(src_filename).with_extension("o");
    let obj_path = obj_path.to_str().unwrap();

    let bin_path = src_dir.join(src_filename).with_extension("bin");
    let bin_path = bin_path.to_str().unwrap();

    let input_info = InputInfo {
        src_path: String::from(src_path.to_str().unwrap()),
    };

    let output_info = OutputInfo {
        ast_path: String::from(ast_path),
        asm_path: String::from(asm_path),
        obj_path: String::from(obj_path),
        bin_path: String::from(bin_path),
    };

    Ok((input_info, output_info))
}
