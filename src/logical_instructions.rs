use crate::cpu::*;

pub fn logical_and(context: &mut MOS6502, value: u8) {
    let new_accumulator = context.reg.ac & value;
    context.set_zn(new_accumulator);
    context.reg.ac = new_accumulator;
}

pub fn logical_exclusive_or(context: &mut MOS6502, value: u8) {
    let new_accumulator = context.reg.ac ^ value;
    context.set_zn(new_accumulator);
    context.reg.ac = new_accumulator;
}

pub fn logical_inclusive_or(context: &mut MOS6502, value: u8) {
    let new_accumulator = context.reg.ac | value;
    context.set_zn(new_accumulator);
    context.reg.ac = new_accumulator;
}

pub fn logical_bit_test(context: &mut MOS6502, value: u8) {
    context.flags.zero = (value & context.reg.ac) == 0;
    context.flags.negative = (value & (1 << 7)) != 0; // 7th bit
    context.flags.overflow = (value & (1 << 6)) != 0; // 6th bit
}

pub fn logical_shift_left(context: &mut MOS6502, value: u8) -> u8 {
    let new_value = value << 1;
    context.flags.carry = (value & (1 << 7)) != 0; // 7th bit for carry
    context.set_zn(value);
    new_value
}

pub fn logical_shift_right(context: &mut MOS6502, value: u8) -> u8 {
    let new_value = value >> 1;
    context.flags.carry = (value & 1) != 0; // 0th bit for carry
    context.set_zn(value);
    new_value
}

pub fn logical_rotate_left(context: &mut MOS6502, value: u8) -> u8 {
    let carry_bit = match context.flags.carry {
        true => 1_u8,
        false => 0_u8,
    };
    let new_value = (value << 1) | carry_bit;
    context.flags.carry = (value & (1 << 7)) != 0; // 7th bit for carry
    context.set_zn(new_value);
    new_value
}

pub fn logical_rotate_right(context: &mut MOS6502, value: u8) -> u8 {
    let carry_bit = match context.flags.carry {
        true => 1_u8,
        false => 0_u8,
    };

    let new_value = (value >> 1) | (carry_bit << 7);
    context.flags.carry = (value & 1) != 0; // 0th bit for carry
    context.set_zn(new_value);
    new_value
}
