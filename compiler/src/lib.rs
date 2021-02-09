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

use clap::{app_from_crate, Arg};
use codegen::gen_asm;
use parser::parse;
use sem_analysis::sem_analysis;
use std::{fmt, path::PathBuf, str};
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

pub struct Options {
    pub input: String,
    pub verbose: bool,
    pub target: Target,
}

impl Options {
    pub fn parse() -> Self {
        let matches = app_from_crate!()
            .arg(Arg::new("INPUT").required(true).about("Source file"))
            .arg(
                Arg::new("verbose")
                    .long("verbose")
                    .about("Outputs verbose messages on internal operations"),
            )
            .arg(
                Arg::new("target")
                    .long("target")
                    .takes_value(true)
                    .possible_values(&["x86_64-linux", "x86_64-darwin", "dotnet"])
                    .default_value(&Target::default().to_string())
                    .about("Builds for the target triple"),
            )
            .get_matches();

        let input = matches.value_of("INPUT").unwrap();
        let verbose = matches.is_present("verbose");
        let target: Target = matches.value_of("target").unwrap().parse().unwrap();

        Options {
            input: input.to_owned(),
            verbose,
            target,
        }
    }
}

pub fn compile(src: &str) -> Result<String, String> {
    let tokens = tokenize(src)?;
    let ast = parse(&tokens)?;
    let ir = sem_analysis(&ast)?;
    let asm = gen_asm(&ir)?;
    Ok(asm.stringify())
}

pub enum Target {
    X64Darwin,
    X64Linux,
    Dotnet,
}

#[derive(Debug)]
pub struct InvalidTargetError;

impl Default for Target {
    fn default() -> Self {
        if cfg!(all(target_arch = "x86_64", target_os = "linux")) {
            Target::X64Linux
        } else if cfg!(all(target_arch = "x86_64", target_os = "macos")) {
            Target::X64Darwin
        } else {
            Target::Dotnet
        }
    }
}

impl str::FromStr for Target {
    type Err = InvalidTargetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "x86_64-linux" {
            Ok(Target::X64Linux)
        } else if s == "x86_64-darwin" {
            Ok(Target::X64Darwin)
        } else {
            Err(InvalidTargetError)
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let target_name = match self {
            Target::X64Linux => "x86_64-linux",
            Target::X64Darwin => "x86_64-darwin",
            Target::Dotnet => "dotnet",
        };
        write!(f, "{}", target_name)
    }
}
