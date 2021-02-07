mod asm;
pub mod ast;
pub mod codegen;
mod ir;
mod location;
pub mod parser;
pub mod sem_analysis;
pub mod term_color;
mod token;
pub mod tokenizer;

use codegen::gen_asm;
use parser::parse;
use sem_analysis::sem_analysis;
use std::path::PathBuf;
use tokenizer::tokenize;

pub struct IOInfo {
    pub input: InputInfo,
    pub output: OutputInfo,
}

pub struct InputInfo {
    pub src_path: PathBuf,
}

pub struct OutputInfo {
    pub ast_path: PathBuf,
    pub asm_path: PathBuf,
    pub obj_path: PathBuf,
    pub bin_path: PathBuf,
}

pub fn get_io_info<F: Into<PathBuf>>(filename: F) -> Result<IOInfo, String> {
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

    Ok(IOInfo {
        input: input_info,
        output: output_info,
    })
}

pub fn compile(src: &str) -> Result<String, String> {
    let tokens = tokenize(src)?;
    let ast = parse(&tokens)?;
    let ir = sem_analysis(&ast)?;
    let asm = gen_asm(&ir)?;
    Ok(asm.stringify())
}
