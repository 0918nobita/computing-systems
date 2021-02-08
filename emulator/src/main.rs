extern crate emulator;
use emulator::{get_inst_table, Emulator};

use std::{env, fs};

static MEMORY_SIZE: usize = 1024 * 1024; // 1MiB

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please specify a source file");

    let mut emu = Emulator::new(MEMORY_SIZE, 0x00_00_00_00, 0x00_00_7c_00);

    emu.store_bytes(&fs::read(filename).expect("Unable to open the source file"));
    emu.dump_registers();
    emu.dump_memory();

    let insts = get_inst_table();
    emu.run(&insts);
}
