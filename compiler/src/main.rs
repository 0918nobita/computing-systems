extern crate compiler;

use compiler::{compile, get_io_info, IOInfo};
use std::{
    env, fs,
    process::{self, Command},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let first_arg = args.get(1).unwrap_or_else(|| {
        eprintln!("Please specify a source file");
        process::exit(1)
    });

    let IOInfo {
        input: input_info,
        output: output_info,
    } = get_io_info(first_arg)?;

    let content = fs::read_to_string(input_info.src_path).unwrap_or_else(|_| {
        eprintln!("Failed to read the source file");
        process::exit(1);
    });

    let asm_output = compile(&content).unwrap_or_else(|msg| {
        eprintln!("{}", msg);
        process::exit(1);
    });

    fs::write(&output_info.asm_path, asm_output)?;

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
