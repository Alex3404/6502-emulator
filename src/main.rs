#![feature(unchecked_math)]
#![feature(bigint_helper_methods)]

use std::fs;

use emulator::MemoryBank;
#[allow(unused_variables)]
mod addressing_mode_test;
mod arithmetic_instructions;
mod branching_instructions;
mod emulator;
mod inc_dec_instructions;
mod logical_instructions;
mod opcode_modes;
mod stack_instructions;
mod status_flag_instructions;
mod transfer_load_store_instructions;

fn main() {
    let file_path = "6502_functional_test.bin";
    let mut contents = fs::read(file_path).unwrap();
    contents.resize(emulator::MEMORY_SIZE, 0);
    let memory: MemoryBank = contents.try_into().unwrap();
    emulator::load_and_execute(memory);
}
