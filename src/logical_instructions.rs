use crate::emulator::*;

pub fn logical_and(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);

    let new_accumulator = context.reg.ac & read(memory, address);

    // Flags
    context.flags.zero = new_accumulator == 0;
    context.flags.negative = (new_accumulator & (1 << 7)) != 0; // If the 7th bit is set

    // Regs
    context.reg.ac = new_accumulator;
}

pub fn logical_exclusive_or(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);

    let new_accumulator = context.reg.ac ^ read(memory, address);

    // Flags
    context.flags.zero = new_accumulator == 0;
    context.flags.negative = (new_accumulator & (1 << 7)) != 0; // If the 7th bit is set

    // Regs
    context.reg.ac = new_accumulator;
}

pub fn logical_inclusive_or(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);

    let new_accumulator = context.reg.ac | read(memory, address);

    // Flags
    context.flags.zero = new_accumulator == 0;
    context.flags.negative = (new_accumulator & (1 << 7)) != 0; // If the 7th bit is set

    // Regs
    context.reg.ac = new_accumulator;
}

pub fn logical_bit_test(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    let value = read(memory, address);

    // Flags
    context.flags.zero = (context.reg.ac & value) == 0;
    context.flags.negative = (value & (1 << 7)) != 0; // 7th bit
    context.flags.overflow = (value & (1 << 6)) != 0; // 6th bit
}

pub fn logical_shift_left(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    // Addressing Mode Accumulator
    if mode == AddressingMode::Accumulator {
        let new_accumulator = context.reg.ac << 1;

        // Flags
        context.flags.carry = (context.reg.ac & (1 << 7)) != 0; // 7th bit for carry
        context.flags.zero = context.reg.ac == 0;
        context.flags.negative = (new_accumulator & (1 << 7)) != 0; // If the 7th bit is set

        // Regs
        context.reg.ac = new_accumulator;
    } else {
        let address: u16 = fetch_instruction_abs_address(mode, context, memory);

        let value = read(memory, address);
        let new_value = value << 1;

        // Flags
        context.flags.carry = (value & (1 << 7)) != 0; // 7th bit for carry
        context.flags.zero = value == 0;
        context.flags.negative = (new_value & (1 << 7)) != 0; // If the 7th bit is set

        // Store Memory
        write(memory, address, new_value);
    }
}

pub fn logical_shift_right(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    // Addressing Mode Accumulator
    if mode == AddressingMode::Accumulator {
        let new_accumulator = context.reg.ac >> 1;

        // Flags
        context.flags.carry = (context.reg.ac & 1) != 0; // 0th bit for carry
        context.flags.zero = new_accumulator == 0;
        context.flags.negative = false; // Since the 7th bit is always 0 negative is always unset

        // Regs
        context.reg.ac = new_accumulator;
    } else {
        let address: u16 = fetch_instruction_abs_address(mode, context, memory);

        let value = read(memory, address);
        let new_value = value >> 1;

        // Flags
        context.flags.carry = (value & 1) != 0; // 0th bit for carry
        context.flags.zero = value == 0;
        context.flags.negative = false; // Since the 7th bit is always 0 negative is always unset

        // Store Memory
        write(memory, address, new_value);
    }
}

pub fn logical_rotate_left(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let carry_bit = match context.flags.carry {
        true => 1_u8,
        false => 0_u8,
    };

    // Addressing Mode Accumulator
    if mode == AddressingMode::Accumulator {
        let new_accumulator = (context.reg.ac << 1) | carry_bit;

        // Flags
        context.flags.carry = (context.reg.ac & (1 << 7)) != 0; // 7th bit for carry
        context.flags.zero = new_accumulator == 0;
        context.flags.negative = (new_accumulator & (1 << 7)) != 0; // If the 7th bit is set

        // Regs
        context.reg.ac = new_accumulator;
    } else {
        let address: u16 = fetch_instruction_abs_address(mode, context, memory);
        let value = read(memory, address);

        let new_value = (value << 1) | carry_bit;

        // Flags
        context.flags.carry = (value & (1 << 7)) != 0; // 7th bit for carry
        context.flags.zero = value == 0;
        context.flags.negative = (new_value & (1 << 7)) != 0; // If the 7th bit is set

        // Store Memory
        write(memory, address, new_value);
    }
}

pub fn logical_rotate_right(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let carry_bit = match context.flags.carry {
        true => 1_u8,
        false => 0_u8,
    };

    // Addressing Mode Accumulator
    if mode == AddressingMode::Accumulator {
        let new_accumulator = (context.reg.ac >> 1) | (carry_bit << 7);

        // Flags
        context.flags.carry = (context.reg.ac & 1) != 0; // 0th bit for carry
        context.flags.zero = new_accumulator == 0;
        context.flags.negative = (new_accumulator & (1 << 7)) != 0; // If the 7th bit is set

        // Regs
        context.reg.ac = new_accumulator;
    } else {
        let address: u16 = fetch_instruction_abs_address(mode, context, memory);
        let value = read(memory, address);

        let new_value = (value >> 1) | (carry_bit << 7);

        // Flags
        context.flags.carry = (value & 1) != 0; // 7th bit for carry
        context.flags.zero = value == 0;
        context.flags.negative = (new_value & (1 << 7)) != 0; // If the 7th bit is set

        // Store Memory
        write(memory, address, new_value);
    }
}
