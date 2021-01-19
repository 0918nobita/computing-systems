use super::asm::{Asm, DataSection, DataSectionItem, TextSection};
use super::ast::{ExprAst, Locatable, StmtAst};
use std::collections::HashMap;

struct CompilationContext {
    current_dat_index: i32,
    current_var_index: i32,
    var_mappings: HashMap<String, i32>,
}

pub fn compile(stmts: &[StmtAst]) -> Result<Asm, String> {
    let mut dat_items = Vec::<DataSectionItem>::new();
    let mut txt = TextSection::new();

    let mut context = CompilationContext {
        current_dat_index: 0,
        current_var_index: 0,
        var_mappings: HashMap::new(),
    };

    for stmt in stmts.into_iter() {
        match stmt {
            StmtAst::ProcCall(proc, args) if proc.name == "PRINT" => {
                if args.len() > 1 {
                    return Err(format!(
                        "Compile error: ({}) Too many arguments",
                        proc.locate()
                    ));
                }
                if let Some(head) = args.first() {
                    compile_expr(head, &mut context, &mut dat_items, &mut txt)?;
                    txt.inst("pop rdi");
                    txt.inst("call printString");
                } else {
                    return Err(format!(
                        "Compile error: ({}) Too few arguments",
                        proc.locate()
                    ));
                }
            }
            StmtAst::ProcCall(proc, _) => {
                return Err(format!(
                    "Compile error: ({}) Unknown procedure `{}`",
                    proc.locate(),
                    proc.name
                ))
            }
            StmtAst::VarDecl(var, init_expr) => {
                compile_expr(init_expr, &mut context, &mut dat_items, &mut txt)?;
                txt.inst("pop rax");
                txt.inst(format!(
                    "mov qword[rsp+{}], rax  ; intialize {}",
                    context.current_var_index * 8,
                    var.name
                ));
                context
                    .var_mappings
                    .insert(var.name.clone(), context.current_var_index);
                context.current_var_index += 1;
            }
            _ => return Err(String::from("Compile error: Not implemented")),
        }
    }

    let mut new_txt = TextSection::new();
    new_txt.label("_start");
    new_txt.inst(format!("sub rsp, {}", context.current_var_index * 8));
    new_txt.inst("mov rbp, rsp");
    new_txt.extend(txt);
    txt = new_txt;

    txt.inst(format!("add rsp, {}", context.current_var_index * 8));

    // exit
    txt.inst("mov rax, 60");
    txt.inst("xor rdi, rdi");
    txt.inst("syscall");

    // printString
    txt.label("printString");
    txt.inst("call stringLength");
    txt.inst("mov rdx, rax");
    txt.inst("mov rax, 1");
    txt.inst("mov rsi, rdi");
    txt.inst("mov rdi, 1");
    txt.inst("syscall");
    txt.inst("ret");

    // stringLength
    txt.label("stringLength");
    txt.inst("xor rax, rax");
    txt.label(".loop");
    txt.inst("cmp byte[rdi+rax], 0");
    txt.inst("je .end");
    txt.inst("inc rax");
    txt.inst("jmp .loop");
    txt.label(".end");
    txt.inst("ret");

    Ok(Asm {
        data: DataSection { items: dat_items },
        text: txt,
    })
}

fn compile_expr(
    expr_ast: &ExprAst,
    context: &mut CompilationContext,
    dat_items: &mut Vec<DataSectionItem>,
    txt: &mut TextSection,
) -> Result<(), String> {
    match expr_ast {
        ExprAst::StrLit(str_lit) => {
            dat_items.push(DataSectionItem {
                name: format!("dat{}", context.current_dat_index),
                size: String::from("db"),
                values: format!("'{}', 0", str_lit.value),
            });
            txt.inst(format!("push dat{}", context.current_dat_index));
            context.current_dat_index += 1;
            Ok(())
        }
        ExprAst::Ident(ident) => {
            if let Some(var_index) = context.var_mappings.get(&ident.name) {
                txt.inst(format!("push qword[rbp+{}]", var_index * 8));
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
