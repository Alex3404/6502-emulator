use crate::emulator::*;

pub fn jmp(context: &mut MOS6502, mode: AddressingMode) {
    let jmp_address: u16 = context.fetch_instruction_abs_address(mode);
    context.jump_to_new_pc(jmp_address);
}

pub fn jump_to_subroutine(context: &mut MOS6502, mode: AddressingMode) {
    let jmp_address: u16 = context.fetch_instruction_abs_address(mode);
    context.push_word(context.reg.pc + 1);
    context.reg.pc = jmp_address;
}

#[allow(unused_variables)]
pub fn return_from_subroutine(context: &mut MOS6502, mode: AddressingMode) {
    let old_pc: u16 = context.pop_word() + 1;
    context.jump_to_new_pc(old_pc);
}

pub fn branch_if_carry_clear(context: &mut MOS6502, mode: AddressingMode) {
    let jmp_address: u16 = context.fetch_instruction_abs_address(mode);
    if !context.flags.carry {
        context.jump_to_new_pc(jmp_address);
    }
}

pub fn branch_if_carry_set(context: &mut MOS6502, mode: AddressingMode) {
    let jmp_address: u16 = context.fetch_instruction_abs_address(mode);
    if context.flags.carry {
        context.jump_to_new_pc(jmp_address);
    }
}

pub fn branch_if_equal(context: &mut MOS6502, mode: AddressingMode) {
    let jmp_address: u16 = context.fetch_instruction_abs_address(mode);
    if context.flags.zero {
        context.jump_to_new_pc(jmp_address);
    }
}

pub fn branch_if_not_equal(context: &mut MOS6502, mode: AddressingMode) {
    let jmp_address: u16 = context.fetch_instruction_abs_address(mode);
    if !context.flags.zero {
        context.jump_to_new_pc(jmp_address);
    }
}

pub fn branch_if_minus(context: &mut MOS6502, mode: AddressingMode) {
    let jmp_address: u16 = context.fetch_instruction_abs_address(mode);
    if context.flags.negative {
        context.jump_to_new_pc(jmp_address);
    }
}

pub fn branch_if_positive(context: &mut MOS6502, mode: AddressingMode) {
    let jmp_address: u16 = context.fetch_instruction_abs_address(mode);
    if !context.flags.negative {
        context.jump_to_new_pc(jmp_address);
    }
}

pub fn branch_if_overflow_clear(context: &mut MOS6502, mode: AddressingMode) {
    let jmp_address: u16 = context.fetch_instruction_abs_address(mode);
    if !context.flags.overflow {
        context.jump_to_new_pc(jmp_address);
    }
}

pub fn branch_if_overflow_set(context: &mut MOS6502, mode: AddressingMode) {
    let jmp_address: u16 = context.fetch_instruction_abs_address(mode);
    if context.flags.overflow {
        context.jump_to_new_pc(jmp_address);
    }
}
