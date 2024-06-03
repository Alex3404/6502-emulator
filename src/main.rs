#![feature(unchecked_math)]
#![feature(bigint_helper_methods)]

use std::fs::File;

mod cpu;
mod disassembler;
mod memory;
mod tests;

const TEST_FILE_PATH: &str = "tests\\6502_functional_test.bin";

fn main() {
    let mut file = File::open(TEST_FILE_PATH).unwrap();

    let memory = memory::memory_from_file(&mut file, true);

    let mut cpu = cpu::MOS6502::new(Box::new(memory));
    cpu.reset();
    cpu.set_pc(0x400);
    loop {
        let disassembly = disassembler::disassemble_instruction(&mut cpu.mem, cpu.reg.pc);
        if let Some(str) = disassembly {
            println!("Executing {}", str)
        }

        cpu.step();
        if cpu.is_trapped() {
            break;
        }
    }
    println!("CPU Trapped!");
}
