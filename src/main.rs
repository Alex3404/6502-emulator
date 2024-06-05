#![feature(unchecked_math)]
#![feature(bigint_helper_methods)]

use std::fs::File;

mod address_bus;
mod cpu;
mod disassembler;
mod tests;

const TEST_FILE_PATH: &str = "tests\\6502_functional_test.bin";

fn main() {
    let mut file = File::open(TEST_FILE_PATH).unwrap();

    let memory = address_bus::memory_from_file(&mut file, true);

    let mut cpu = cpu::MOS6502::new(Box::new(memory));
    cpu.reset();
    cpu.set_pc(0x400);
    loop {
        // Commented out because it slows the cpu by alot
        // let disassembly = disassembler::disassemble_instruction(&mut cpu.bus, cpu.reg.pc);
        // if let Some(str) = disassembly {
        //     println!(
        //         "A: {:02X} X: {:02X} Y: {:02X} | {}",
        //         cpu.reg.ac, cpu.reg.ix, cpu.reg.iy, str
        //     )
        // }

        cpu.step();
        if cpu.is_trapped() {
            break;
        }
    }
    println!("CPU Trapped!");
}
