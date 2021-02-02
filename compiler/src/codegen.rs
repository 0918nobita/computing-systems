use super::asm::{Asm, DataSection, TextSection};
use super::ir::{Ir, IrInst};

/// 中間表現からアセンブリの内部表現を生成する
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
                txt.inst("push TYPE_STR");
            }
            IrInst::GetGlobal(index) => {
                txt.inst(format!("push qword[rbp+{}]", index * 8));
                txt.inst("push TYPE_STR");
            }
            IrInst::SetGlobal(index) => {
                txt.inst("add rsp, 8");
                txt.inst("pop rax");
                txt.inst(format!("mov qword[rbp+{}], rax", index * 8));
            }
            IrInst::Print => {
                txt.inst("pop rax");
                txt.inst("cmp rax, TYPE_STR");
                txt.inst("jnz type_error");
                txt.inst("pop rdi");
                txt.inst("call print_string");
            }
        }
    }

    // スタックを実行前の状態に戻す
    txt.inst(format!("add rsp, {}", max_stack_size));

    // exit
    txt.inst("mov rax, SYS_EXIT");
    txt.inst("xor rdi, rdi  ; exit code");
    txt.inst("syscall");

    // print_string
    txt.label("print_string");
    txt.inst("call string_length");
    txt.inst("mov rdx, rax  ; length");
    txt.inst("mov rax, SYS_WRITE");
    txt.inst("mov rsi, rdi  ; address");
    txt.inst("mov rdi, FD_STDOUT");
    txt.inst("syscall");
    txt.inst("ret");

    // string_length
    txt.label("string_length");
    txt.inst("xor rax, rax");
    txt.label(".loop");
    txt.inst("cmp byte[rdi+rax], 0");
    txt.inst("je .end");
    txt.inst("inc rax");
    txt.inst("jmp .loop");
    txt.label(".end");
    txt.inst("ret");

    // type_error
    txt.label("type_error");
    txt.inst("mov rax, SYS_WRITE");
    txt.inst("mov rdi, FD_STDERR");
    txt.inst("mov rsi, err_msg");
    txt.inst("mov rdx, ERR_MSG_CNT");
    txt.inst("syscall");
    txt.inst("mov rax, SYS_EXIT");
    txt.inst("mov rdi, EXIT_FAILURE");
    txt.inst("syscall");

    Ok(Asm {
        data: dat,
        text: txt,
    })
}
