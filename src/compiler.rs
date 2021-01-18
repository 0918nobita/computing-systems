use super::asm::{Asm, DataSection, DataSectionItem, TextSection, TextSectionItem};
use super::ast::{ExprAst, Locatable, StmtAst};
use std::collections::HashMap;

struct CompilationContext {
    current_dat_index: i32,
    current_var_index: i32,
    var_mappings: HashMap<String, i32>,
}

pub fn compile(stmts: &[StmtAst]) -> Result<Asm, String> {
    let mut dat_items = Vec::<DataSectionItem>::new();
    let mut txt_items = Vec::<TextSectionItem>::new();

    let mut context = CompilationContext {
        current_dat_index: 0,
        current_var_index: 0,
        var_mappings: HashMap::new(),
    };

    for stmt in stmts.into_iter() {
        match stmt {
            StmtAst::ProcCall(proc, args) if proc.name == "PRINT" => {
                if args.len() > 1 {
                    return Err(String::from("Failed to compile: Too many arguments"));
                }
                if let Some(head) = args.first() {
                    compile_expr(head, &mut context, &mut dat_items, &mut txt_items)?;
                    txt_items.push(TextSectionItem::Instruction(String::from("pop rdi")));
                    txt_items.push(TextSectionItem::Instruction(String::from(
                        "call printString",
                    )));
                } else {
                    return Err(String::from("Failed to compile: Too few arguments"));
                }
            }
            StmtAst::VarDecl(var, init_expr) => {
                compile_expr(init_expr, &mut context, &mut dat_items, &mut txt_items)?;
                txt_items.push(TextSectionItem::Instruction(String::from("pop rax")));
                txt_items.push(TextSectionItem::Instruction(format!(
                    "mov qword[rsp+{}], rax  ; intialize {}",
                    context.current_var_index * 8,
                    var.name
                )));
                context
                    .var_mappings
                    .insert(var.name.clone(), context.current_var_index);
                context.current_var_index += 1;
            }
            _ => {
                return Err(String::from("Failed to compile: Unknown procedure"));
            }
        }
    }

    if context.current_var_index >= 1 {
        let mut new_txt_items = Vec::<TextSectionItem>::new();
        new_txt_items.push(TextSectionItem::Label(String::from("_start")));
        new_txt_items.push(TextSectionItem::Instruction(format!(
            "sub rsp, {}",
            context.current_var_index * 8
        )));
        new_txt_items.push(TextSectionItem::Instruction(String::from("mov rbp, rsp")));
        new_txt_items.extend(txt_items);
        txt_items = new_txt_items;
    }

    // exit
    txt_items.push(TextSectionItem::Instruction(String::from("mov rax, 60")));
    txt_items.push(TextSectionItem::Instruction(String::from("xor rdi, rdi")));
    txt_items.push(TextSectionItem::Instruction(String::from("syscall")));

    // printString
    txt_items.push(TextSectionItem::Label(String::from("printString")));
    txt_items.push(TextSectionItem::Instruction(String::from(
        "call stringLength",
    )));
    txt_items.push(TextSectionItem::Instruction(String::from("mov rdx, rax")));
    txt_items.push(TextSectionItem::Instruction(String::from("mov rax, 1")));
    txt_items.push(TextSectionItem::Instruction(String::from("mov rsi, rdi")));
    txt_items.push(TextSectionItem::Instruction(String::from("mov rdi, 1")));
    txt_items.push(TextSectionItem::Instruction(String::from("syscall")));
    txt_items.push(TextSectionItem::Instruction(String::from("ret")));

    // stringLength
    txt_items.push(TextSectionItem::Label(String::from("stringLength")));
    txt_items.push(TextSectionItem::Instruction(String::from("xor rax, rax")));
    txt_items.push(TextSectionItem::Label(String::from(".loop")));
    txt_items.push(TextSectionItem::Instruction(String::from(
        "cmp byte[rdi+rax], 0",
    )));
    txt_items.push(TextSectionItem::Instruction(String::from("je .end")));
    txt_items.push(TextSectionItem::Instruction(String::from("inc rax")));
    txt_items.push(TextSectionItem::Instruction(String::from("jmp .loop")));
    txt_items.push(TextSectionItem::Label(String::from(".end")));
    txt_items.push(TextSectionItem::Instruction(String::from("ret")));

    Ok(Asm {
        data: DataSection { items: dat_items },
        text: TextSection { items: txt_items },
    })
}

fn compile_expr(
    expr_ast: &ExprAst,
    context: &mut CompilationContext,
    dat_items: &mut Vec<DataSectionItem>,
    txt_items: &mut Vec<TextSectionItem>,
) -> Result<(), String> {
    match expr_ast {
        ExprAst::StrLit(str_lit) => {
            dat_items.push(DataSectionItem {
                name: format!("dat{}", context.current_dat_index),
                size: String::from("db"),
                values: format!("'{}', 0", str_lit.value),
            });
            txt_items.push(TextSectionItem::Instruction(format!(
                "push dat{}",
                context.current_dat_index
            )));
            context.current_dat_index += 1;
            Ok(())
        }
        ExprAst::Ident(ident) => {
            if let Some(var_index) = context.var_mappings.get(&ident.name) {
                txt_items.push(TextSectionItem::Instruction(format!(
                    "push qword[rbp+{}]",
                    var_index * 8
                )));
                Ok(())
            } else {
                Err(format!(
                    "Failed to compile: ({}) {} is not defined",
                    ident.locate(),
                    ident.name
                ))
            }
        }
    }
}
