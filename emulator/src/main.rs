use std::{collections::HashMap, convert::TryFrom, env, fs};

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
    fn new(size: u32, eip: u32, esp: u32) -> Self {
        let mut emu = Emulator::default();
        emu.eip = eip;
        emu.esp = esp;
        emu.memory.resize_with(size as usize, Default::default);
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

    fn read_code_u8(&self, displacement: u32) -> u8 {
        self.memory[(self.eip + displacement) as usize]
    }

    fn read_code_i8(&self, displacement: u32) -> i8 {
        self.memory[(self.eip + displacement) as usize] as i8
    }

    fn run(&mut self, insts: &InstructionTable) {
        while self.eip < MEMORY_SIZE {
            let code = self.read_code_u8(0);
            let inst = insts.get(&code).expect("Unknown instruction");
            inst(self);

            if self.eip == 0x00_00_00_00 {
                println!("end of program");
                break;
            }
        }
    }
}

type Instruction = fn(&mut Emulator) -> ();

type InstructionTable = HashMap<u8, Instruction>;

static MEMORY_SIZE: u32 = 1024 * 1024; // 1MiB

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please specify a source file");

    let mut emu = Emulator::new(MEMORY_SIZE, 0x00_00_00_00, 0x00_00_7c_00);

    load_binary(&mut emu, filename);

    emu.dump_registers();
    println!("Memory[0..=5]: {:?}", &emu.memory[0..=5]);

    let insts = get_inst_table();
    emu.run(&insts);
}

fn load_binary(emu: &mut Emulator, filename: &str) {
    let content = fs::read(filename).expect("Unable to open the source file");

    for (i, b) in content.iter().enumerate() {
        emu.memory[i] = *b;
    }
}

fn get_inst_table() -> InstructionTable {
    let mut insts: InstructionTable = HashMap::new();

    insts.insert(0xff, |emu| {
        println!("inc");
        // TODO: implement `inc` instruction
        emu.eip += 3;
    });

    insts.insert(0xeb, |emu| {
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
