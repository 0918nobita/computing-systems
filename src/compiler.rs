use super::ast::{ExprAst, StmtAst};

type Asm = String;

pub fn compile(stmts: &[StmtAst]) -> Result<Asm, String> {
    let mut data_section = String::from("section .data\n");
    let mut text_section = String::from("\nsection .text\n_start:\n");

    for (i, stmt) in stmts.into_iter().enumerate() {
        match stmt {
            StmtAst::CallProc(proc, args) if proc.name.eq("PRINT") => {
                if args.len() > 1 {
                    return Err(String::from("Failed to compile: Too many arguments"));
                }
                if let Some(head) = args.first() {
                    match head {
                        ExprAst::StrLit(str_lit) => {
                            data_section.push_str(
                                format!("    dat{} db '{}', 10, 0\n", i, str_lit.value).as_str(),
                            );
                            text_section.push_str(
                                format!("    mov rdi, dat{}\n    call printString\n", i).as_str(),
                            );
                        }
                    }
                } else {
                    return Err(String::from("Failed to compile: Too few arguments"));
                }
            }
            _ => {
                return Err(String::from("Failed to compile: Unknown procedure"));
            }
        }
    }

    text_section.push_str("    mov rax, 60\n"); // sys_exit
    text_section.push_str("    xor rdi, rdi\n");
    text_section.push_str("    syscall\n\n");
    text_section.push_str("printString:\n");
    text_section.push_str("    call stringLength\n");
    text_section.push_str("    mov rdx, rax\n");
    text_section.push_str("    mov rax, 1\n");
    text_section.push_str("    mov rsi, rdi\n");
    text_section.push_str("    mov rdi, 1\n");
    text_section.push_str("    syscall\n");
    text_section.push_str("    ret\n\n");
    text_section.push_str("stringLength:\n");
    text_section.push_str("    xor rax, rax\n");
    text_section.push_str(".loop:\n");
    text_section.push_str("    cmp byte[rdi+rax], 0\n");
    text_section.push_str("    je .end\n");
    text_section.push_str("    inc rax\n");
    text_section.push_str("    jmp .loop\n");
    text_section.push_str(".end:\n");
    text_section.push_str("    ret\n");

    Ok(format!(
        "bits 64\nglobal _start\n{}{}",
        data_section, text_section
    ))
}
