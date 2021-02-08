use std::{collections::HashMap, convert::TryFrom};

type NativeInst = fn(&mut Emulator) -> ();

#[derive(Default)]
pub struct InstTable {
    member: HashMap<u8, NativeInst>,
}

#[derive(Default)]
pub struct Emulator {
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
    pub fn new(size: usize, eip: u32, esp: u32) -> Self {
        let mut emu = Emulator::default();
        emu.eip = eip;
        emu.esp = esp;
        emu.memory.resize_with(size, Default::default);
        emu
    }

    pub fn dump_registers(&self) {
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

    pub fn dump_memory(&self) {
        println!("Memory[0..=5]: {:?}", &self.memory[0..=5]);
    }

    fn read_code_u8(&self, displacement: u32) -> u8 {
        self.memory[(self.eip + displacement) as usize]
    }

    fn read_code_i8(&self, displacement: u32) -> i8 {
        self.memory[(self.eip + displacement) as usize] as i8
    }

    pub fn store_bytes(&mut self, content: &[u8]) {
        for (i, b) in content.iter().enumerate() {
            self.memory[i] = *b;
        }
    }

    pub fn run(&mut self, insts: &InstTable) {
        while self.eip < self.memory.len() as u32 {
            let code = self.read_code_u8(0);
            let inst = insts.member.get(&code).expect("Unknown instruction");
            inst(self);

            if self.eip == 0x00_00_00_00 {
                println!("end of program");
                break;
            }
        }
    }
}

pub fn get_inst_table() -> InstTable {
    let mut insts = InstTable::default();

    insts.member.insert(0xff, |emu| {
        println!("inc");
        // TODO: implement `inc` instruction
        emu.eip += 3;
    });

    insts.member.insert(0xeb, |emu| {
        println!("short_jmp");
        let diff = emu.read_code_i8(1);
        emu.eip += 2;
        if diff > 0 {
            emu.eip += u32::try_from(diff).unwrap();
        } else if diff < 0 {
            emu.eip = emu
                .eip
                .checked_sub(u32::try_from(diff * (-1)).unwrap())
                .expect("Failed to jump");
        }
    });

    insts
}
