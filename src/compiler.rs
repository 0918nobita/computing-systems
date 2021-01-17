use super::asm::{Asm, DataSection, DataSectionItem, TextSection, TextSectionItem};
use super::ast::{ExprAst, StmtAst};

pub fn compile(stmts: &[StmtAst]) -> Result<Asm, String> {
    let mut current_dat_index = 0;
    let mut data_section_items = Vec::<DataSectionItem>::new();
    let mut text_section_items = vec![TextSectionItem::Label(String::from("_start"))];

    for stmt in stmts.into_iter() {
        match stmt {
            StmtAst::CallProc(proc, args) if proc.name.eq("PRINT") => {
                if args.len() > 1 {
                    return Err(String::from("Failed to compile: Too many arguments"));
                }
                if let Some(head) = args.first() {
                    match head {
                        ExprAst::StrLit(str_lit) => {
                            data_section_items.push(DataSectionItem {
                                name: format!("dat{}", current_dat_index),
                                size: String::from("db"),
                                values: format!("'{}', 10, 0", str_lit.value),
                            });
                            text_section_items.push(TextSectionItem::Instruction(format!(
                                "mov rdi, dat{}",
                                current_dat_index
                            )));
                            text_section_items.push(TextSectionItem::Instruction(String::from(
                                "call printString",
                            )));
                            current_dat_index += 1;
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

    // exit
    text_section_items.push(TextSectionItem::Instruction(String::from("mov rax, 60")));
    text_section_items.push(TextSectionItem::Instruction(String::from("xor rdi, rdi")));
    text_section_items.push(TextSectionItem::Instruction(String::from("syscall")));

    // printString
    text_section_items.push(TextSectionItem::Label(String::from("printString")));
    text_section_items.push(TextSectionItem::Instruction(String::from(
        "call stringLength",
    )));
    text_section_items.push(TextSectionItem::Instruction(String::from("mov rdx, rax")));
    text_section_items.push(TextSectionItem::Instruction(String::from("mov rax, 1")));
    text_section_items.push(TextSectionItem::Instruction(String::from("mov rsi, rdi")));
    text_section_items.push(TextSectionItem::Instruction(String::from("mov rdi, 1")));
    text_section_items.push(TextSectionItem::Instruction(String::from("syscall")));
    text_section_items.push(TextSectionItem::Instruction(String::from("ret")));

    // stringLength
    text_section_items.push(TextSectionItem::Label(String::from("stringLength")));
    text_section_items.push(TextSectionItem::Instruction(String::from("xor rax, rax")));
    text_section_items.push(TextSectionItem::Label(String::from(".loop")));
    text_section_items.push(TextSectionItem::Instruction(String::from(
        "cmp byte[rdi+rax], 0",
    )));
    text_section_items.push(TextSectionItem::Instruction(String::from("je .end")));
    text_section_items.push(TextSectionItem::Instruction(String::from("inc rax")));
    text_section_items.push(TextSectionItem::Instruction(String::from("jmp .loop")));
    text_section_items.push(TextSectionItem::Label(String::from(".end")));
    text_section_items.push(TextSectionItem::Instruction(String::from("ret")));

    Ok(Asm {
        data: DataSection {
            items: data_section_items,
        },
        text: TextSection {
            items: text_section_items,
        },
    })
}
