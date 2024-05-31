use crate::emulator::*;

pub fn logical_and(context: &mut MOS6502, mode: AddressingMode) {
    let address: u16 = context.fetch_instruction_abs_address(mode);

    let new_accumulator = context.reg.ac & context.read(address);

    // Flags
    context.flags.zero = new_accumulator == 0;
    context.flags.negative = (new_accumulator & (1 << 7)) != 0; // If the 7th bit is set

    // Regs
    context.reg.ac = new_accumulator;
}

pub fn logical_exclusive_or(context: &mut MOS6502, mode: AddressingMode) {
    let address: u16 = context.fetch_instruction_abs_address(mode);

    let new_accumulator = context.reg.ac ^ context.read(address);

    // Flags
    context.flags.zero = new_accumulator == 0;
    context.flags.negative = (new_accumulator & (1 << 7)) != 0; // If the 7th bit is set

    // Regs
    context.reg.ac = new_accumulator;
}

pub fn logical_inclusive_or(context: &mut MOS6502, mode: AddressingMode) {
    let address: u16 = context.fetch_instruction_abs_address(mode);

    let new_accumulator = context.reg.ac | context.read(address);

    // Flags
    context.flags.zero = new_accumulator == 0;
    context.flags.negative = (new_accumulator & (1 << 7)) != 0; // If the 7th bit is set

    // Regs
    context.reg.ac = new_accumulator;
}

pub fn logical_bit_test(context: &mut MOS6502, mode: AddressingMode) {
    let address = context.fetch_instruction_abs_address(mode);
    let value = context.read(address);

    // Flags
    context.flags.zero = (context.reg.ac & value) == 0;
    context.flags.negative = (value & (1 << 7)) != 0; // 7th bit
    context.flags.overflow = (value & (1 << 6)) != 0; // 6th bit
}

pub fn logical_shift_left(context: &mut MOS6502, mode: AddressingMode) {
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
        let address: u16 = context.fetch_instruction_abs_address(mode);

        let value = context.read(address);
        let new_value = value << 1;

        // Flags
        context.flags.carry = (value & (1 << 7)) != 0; // 7th bit for carry
        context.flags.zero = value == 0;
        context.flags.negative = (new_value & (1 << 7)) != 0; // If the 7th bit is set

        // Store Memory
        context.write(address, new_value);
    }
}

pub fn logical_shift_right(context: &mut MOS6502, mode: AddressingMode) {
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
        let address: u16 = context.fetch_instruction_abs_address(mode);

        let value = context.read(address);
        let new_value = value >> 1;

        // Flags
        context.flags.carry = (value & 1) != 0; // 0th bit for carry
        context.flags.zero = value == 0;
        context.flags.negative = false; // Since the 7th bit is always 0 negative is always unset

        // Store Memory
        context.write(address, new_value);
    }
}

pub fn logical_rotate_left(context: &mut MOS6502, mode: AddressingMode) {
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
        let address: u16 = context.fetch_instruction_abs_address(mode);
        let value = context.read(address);

        let new_value = (value << 1) | carry_bit;

        // Flags
        context.flags.carry = (value & (1 << 7)) != 0; // 7th bit for carry
        context.flags.zero = value == 0;
        context.flags.negative = (new_value & (1 << 7)) != 0; // If the 7th bit is set

        // Store Memory
        context.write(address, new_value);
    }
}

pub fn logical_rotate_right(context: &mut MOS6502, mode: AddressingMode) {
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
        let address: u16 = context.fetch_instruction_abs_address(mode);
        let value = context.read(address);

        let new_value = (value >> 1) | (carry_bit << 7);

        // Flags
        context.flags.carry = (value & 1) != 0; // 7th bit for carry
        context.flags.zero = value == 0;
        context.flags.negative = (new_value & (1 << 7)) != 0; // If the 7th bit is set

        // Store Memory
        context.write(address, new_value);
    }
}
