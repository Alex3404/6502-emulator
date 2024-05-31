use crate::emulator::*;

pub fn jmp(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let jmp_address: u16 = fetch_instruction_abs_address(mode, context, memory);
    jump_to_new_pc(context, jmp_address);
}

pub fn jump_to_subroutine(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let jmp_address: u16 = fetch_instruction_abs_address(mode, context, memory);
    push_word(context, memory, context.reg.pc + 1);
    context.reg.pc = jmp_address;
}

#[allow(unused_variables)]
pub fn return_from_subroutine(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let old_pc: u16 = pop_word(context, memory) + 1;
    jump_to_new_pc(context, old_pc);
}

pub fn branch_if_carry_clear(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let jmp_address: u16 = fetch_instruction_abs_address(mode, context, memory);
    if !context.flags.carry {
        jump_to_new_pc(context, jmp_address);
    }
}

pub fn branch_if_carry_set(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let jmp_address: u16 = fetch_instruction_abs_address(mode, context, memory);
    if context.flags.carry {
        jump_to_new_pc(context, jmp_address);
    }
}

pub fn branch_if_equal(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let jmp_address: u16 = fetch_instruction_abs_address(mode, context, memory);
    if context.flags.zero {
        jump_to_new_pc(context, jmp_address);
    }
}

pub fn branch_if_not_equal(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let jmp_address: u16 = fetch_instruction_abs_address(mode, context, memory);
    if !context.flags.zero {
        jump_to_new_pc(context, jmp_address);
    }
}

pub fn branch_if_minus(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let jmp_address: u16 = fetch_instruction_abs_address(mode, context, memory);
    if context.flags.negative {
        jump_to_new_pc(context, jmp_address);
    }
}

pub fn branch_if_positive(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let jmp_address: u16 = fetch_instruction_abs_address(mode, context, memory);
    if !context.flags.negative {
        jump_to_new_pc(context, jmp_address);
    }
}

pub fn branch_if_overflow_clear(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let jmp_address: u16 = fetch_instruction_abs_address(mode, context, memory);
    if !context.flags.overflow {
        jump_to_new_pc(context, jmp_address);
    }
}

pub fn branch_if_overflow_set(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let jmp_address: u16 = fetch_instruction_abs_address(mode, context, memory);
    if context.flags.overflow {
        jump_to_new_pc(context, jmp_address);
    }
}
