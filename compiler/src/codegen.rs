use super::asm::{Asm, DataSection, TextSection};
use super::ir::{Ir, IrInst};

pub fn gen_asm(ir: &Ir) -> Result<Asm, String> {
    let max_stack_size = ir.num_globals * 8;

    let mut dat = DataSection::default();
    let mut txt = TextSection::default();

    for (i, static_str) in ir.string_pool.iter().enumerate() {
        dat.append(format!("str{}", i), "db", format!("'{}', 0", static_str));
    }

    txt.label("_start");
    txt.inst(format!("sub rsp, {}", max_stack_size));
    txt.inst("mov rbp, rsp");

    for ir_inst in ir.insts.iter() {
        match ir_inst {
            IrInst::GetStaticStr(index) => {
                txt.inst(format!("push str{}", index));
            }
            IrInst::GetGlobal(index) => {
                txt.inst(format!("push qword[rbp+{}]", index * 8));
            }
            IrInst::SetGlobal(index) => {
                txt.inst("pop rax");
                txt.inst(format!("mov qword[rbp+{}], rax", index * 8));
            }
            IrInst::Print => {
                txt.inst("pop rdi");
                txt.inst("call printString");
            }
        }
    }

    txt.inst(format!("add rsp, {}", max_stack_size));

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
        data: dat,
        text: txt,
    })
}
