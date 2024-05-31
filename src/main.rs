#![feature(unchecked_math)]
#![feature(bigint_helper_methods)]
#[allow(unused_variables)]
mod addressing_mode_test;
mod arithmetic_instructions;
mod branching_instructions;
mod emulator;
mod inc_dec_instructions;
mod logical_instructions;
mod stack_instructions;
mod status_flag_instructions;
mod transfer_load_store_instructions;

fn main() {
    emulator::load_and_execute([0_u8; emulator::MEMORY_SIZE]);
}
