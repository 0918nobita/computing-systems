use super::ast::{ExprAst, StmtAst};

struct DataSectionItem {
    name: String,
    size: String,
    values: String,
}

struct DataSection {
    items: Vec<DataSectionItem>,
}

enum TextSectionItem {
    Label(String),
    Instruction(String),
}

struct TextSection {
    items: Vec<TextSectionItem>,
}

pub struct Asm {
    data: DataSection,
    text: TextSection,
}

impl Asm {
    pub fn stringify(&self) -> String {
        let mut result = String::from("bits 64\nglobal _start\n\nsection .data\n");
        for item in self.data.items.iter() {
            result.push_str(format!("    {} {} {}\n", item.name, item.size, item.values).as_str());
        }
        result.push_str("\nsection .text\n");
        for item in self.text.items.iter() {
            match item {
                TextSectionItem::Label(label_name) => {
                    result.push_str(format!("{}:\n", label_name).as_str());
                }
                TextSectionItem::Instruction(inst) => {
                    result.push_str(format!("    {}\n", inst).as_str());
                }
            }
        }
        result
    }
}

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
