extern crate compiler;

use compiler::{codegen::gen_asm, parser::parse, sem_analysis::sem_analysis, tokenizer::tokenize};
use std::{
    env, fs,
    path::PathBuf,
    process::{self, Command},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let first_arg = args.get(1).unwrap_or_else(|| {
        eprintln!("Please specify a source file");
        process::exit(1)
    });

    let (input_info, output_info) = get_io_info(first_arg)?;
    let content = fs::read_to_string(input_info.src_path).unwrap_or_else(|_| {
        eprintln!("Failed to read the source file");
        process::exit(1);
    });

    let tokens = tokenize(&content).unwrap_or_else_terminate();

    let ast = parse(&tokens).unwrap_or_else_terminate();

    fs::write(
        output_info.ast_path,
        serde_json::to_string_pretty(&ast).unwrap() + "\n",
    )?;

    let ir = sem_analysis(&ast).unwrap_or_else_terminate();

    let asm = gen_asm(&ir).unwrap_or_else_terminate();

    fs::write(&output_info.asm_path, asm.stringify())?;

    let status = Command::new("nasm")
        .args(&["-f", "elf64", output_info.asm_path.to_str().unwrap()])
        .status()?;
    if !status.success() {
        eprintln!("Error occurs while executing `nasm`");
        process::exit(1);
    }

    let status = Command::new("ld")
        .args(&[
            "-o",
            output_info.bin_path.to_str().unwrap(),
            output_info.obj_path.to_str().unwrap(),
        ])
        .status()?;
    if !status.success() {
        eprintln!("Error occurs while executing `ld`");
        process::exit(1);
    }

    Ok(())
}

trait ResultExt<T> {
    fn unwrap_or_else_terminate(self) -> T;
}

impl<T> ResultExt<T> for Result<T, String> {
    fn unwrap_or_else_terminate(self) -> T {
        self.unwrap_or_else(|msg| {
            eprintln!("{}", msg);
            process::exit(1);
        })
    }
}

struct InputInfo {
    src_path: PathBuf,
}

struct OutputInfo {
    ast_path: PathBuf,
    asm_path: PathBuf,
    obj_path: PathBuf,
    bin_path: PathBuf,
}

fn get_io_info<F: Into<PathBuf>>(filename: F) -> Result<(InputInfo, OutputInfo), String> {
    let src_path = filename.into();
    let src_dir = src_path.parent().ok_or("Failed to get directory info")?;
    let mut src_dir = src_dir.to_path_buf();
    let src_filename = src_path
        .file_name()
        .ok_or("Failed to get name of the source file")?;

    src_dir.push(src_filename);

    let ast_path = src_dir.with_extension("ast.json");
    let asm_path = src_dir.with_extension("s");
    let obj_path = src_dir.with_extension("o");
    let bin_path = src_dir.with_extension("bin");

    let input_info = InputInfo { src_path };

    let output_info = OutputInfo {
        ast_path,
        asm_path,
        obj_path,
        bin_path,
    };

    Ok((input_info, output_info))
}
