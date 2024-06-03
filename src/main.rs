#![feature(unchecked_math)]
#![feature(bigint_helper_methods)]

use std::fs::File;

#[allow(unused_variables)]
mod addressing_mode_test;
mod arithmetic_instructions;
mod branching_instructions;
mod cpu;
mod inc_dec_instructions;
mod logical_instructions;
mod memory;
mod opcode_modes;
mod stack_instructions;
mod status_flag_instructions;
mod transfer_load_store_instructions;

const TEST_FILE_PATH: &str = "6502_functional_test.bin";

fn main() {
    let mut file = File::open(TEST_FILE_PATH).unwrap();

    let memory = memory::memory_from_file(&mut file, true);

    let mut cpu = cpu::MOS6502::new(Box::new(memory));
    cpu.execute(0x400);
}
