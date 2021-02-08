use std::{env, fs};

#[derive(Default)]
struct Emulator {
    // 汎用レジスタ
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esp: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    /// プログラムカウンタ
    eip: u32,
    /// EFLAGS レジスタ
    eflags: u32,
    /// メモリ (バイト列)
    memory: Vec<u8>,
}

impl Emulator {
    fn new(size: usize, eip: u32, esp: u32) -> Self {
        let mut emu = Emulator::default();
        emu.eip = eip;
        emu.esp = esp;
        emu.memory.resize_with(size, Default::default);
        emu
    }

    fn dump_registers(&self) {
        println!("EAX = {}", self.eax);
        println!("ECX = {}", self.ecx);
        println!("EDX = {}", self.edx);
        println!("EBX = {}", self.ebx);
        println!("ESP = {}", self.esp);
        println!("EBP = {}", self.ebp);
        println!("ESI = {}", self.esi);
        println!("EDI = {}", self.edi);
        println!("EIP = {}", self.eip);
        println!("EFLAGS = {}", self.eflags);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please specify a source file");

    let mut emu = Emulator::new(1024 * 1024 /* 1MiB */, 0x0000, 0x7c00);

    load_binary(&mut emu, filename);

    emu.dump_registers();
    println!("Memory[0..=5]: {:?}", &emu.memory[0..=5]);
}

fn load_binary(emu: &mut Emulator, filename: &str) {
    if let Ok(content) = fs::read(filename) {
        for (i, b) in content.iter().enumerate() {
            emu.memory[i] = *b;
        }
    } else {
        panic!("Unable to open input.bin")
    }
}
