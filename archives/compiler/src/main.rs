extern crate compiler;

use compiler::{compile, get_io_info, term_color::red_bold, IOInfo, Options};
use std::{
    fs,
    process::{self, Command},
};

fn main() {
    let opts: Options = Options::parse();

    if opts.verbose {
        println!("Target: {}", opts.target)
    }

    let IOInfo {
        input: input_info,
        output: output_info,
    } = get_io_info(opts.input).unwrap_or_else(|msg| exit_failure(&msg));

    let content = fs::read_to_string(input_info.src_path)
        .unwrap_or_else(|_| exit_failure("Failed to read the source file"));

    let asm_output = compile(&content).unwrap_or_else(|msg| exit_failure(&msg));

    fs::write(&output_info.asm_path, asm_output).unwrap_or_else(|err| {
        exit_failure(&format!(
            "{}\nError occurs while outputting assembly program",
            err
        ))
    });

    let status = Command::new("nasm")
        .args(&["-f", "elf64", output_info.asm_path.to_str().unwrap()])
        .status()
        .unwrap_or_else(|err| {
            exit_failure(&format!(
                "{}\nUnable to find `nasm`, perhaps install NASM and set PATH",
                err
            ))
        });

    if !status.success() {
        exit_failure("Error occurs while executing `nasm`");
    }

    let status = Command::new("ld")
        .args(&[
            "-o",
            output_info.bin_path.to_str().unwrap(),
            output_info.obj_path.to_str().unwrap(),
        ])
        .status()
        .unwrap_or_else(|err| {
            exit_failure(&format!(
                "{}\nUnable to find `ld`, perhaps install Linker and set PATH",
                err
            ))
        });

    if !status.success() {
        exit_failure("Error occurs while executing `ld`");
    }
}

fn exit_failure(msg: &str) -> ! {
    eprintln!("{}", red_bold(msg));
    process::exit(1);
}
