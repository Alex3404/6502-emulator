use crate::cpu::*;

pub fn logical_and(cpu: &mut MOS6502, value: u8) {
    let new_accumulator = cpu.reg.ac & value;
    cpu.set_zn(new_accumulator);
    cpu.reg.ac = new_accumulator;
}

pub fn logical_exclusive_or(cpu: &mut MOS6502, value: u8) {
    let new_accumulator = cpu.reg.ac ^ value;
    cpu.set_zn(new_accumulator);
    cpu.reg.ac = new_accumulator;
}

pub fn logical_inclusive_or(cpu: &mut MOS6502, value: u8) {
    let new_accumulator = cpu.reg.ac | value;
    cpu.set_zn(new_accumulator);
    cpu.reg.ac = new_accumulator;
}

pub fn logical_bit_test(cpu: &mut MOS6502, value: u8) {
    cpu.set(CPUFLAGS::ZERO, (value & cpu.reg.ac) == 0);
    cpu.set(CPUFLAGS::NEGATIVE, (value & (1 << 7)) != 0);
    cpu.set(CPUFLAGS::OVERFLOW, (value & (1 << 6)) != 0);
}

pub fn logical_shift_left(cpu: &mut MOS6502, value: u8) -> u8 {
    let new_value = value << 1;
    cpu.set(CPUFLAGS::CARRY, (value & (1 << 7)) != 0); // 7th bit for carry
    cpu.set_zn(new_value);
    new_value
}

pub fn logical_shift_right(cpu: &mut MOS6502, value: u8) -> u8 {
    let new_value = value >> 1;
    cpu.set(CPUFLAGS::CARRY, (value & 1) != 0); // 0th bit for carry
    cpu.set_zn(new_value);
    new_value
}

pub fn logical_rotate_left(cpu: &mut MOS6502, value: u8) -> u8 {
    let carry_bit = match cpu.is_set(CPUFLAGS::CARRY) {
        true => 1_u8,
        false => 0_u8,
    };
    let new_value = (value << 1) | carry_bit;
    cpu.set(CPUFLAGS::CARRY, (value & (1 << 7)) != 0); // 7th bit for carry
    cpu.set_zn(new_value);
    new_value
}

pub fn logical_rotate_right(cpu: &mut MOS6502, value: u8) -> u8 {
    let carry_bit = match cpu.is_set(CPUFLAGS::CARRY) {
        true => 1_u8,
        false => 0_u8,
    };

    let new_value = (value >> 1) | (carry_bit << 7);
    cpu.set(CPUFLAGS::CARRY, (value & 1) != 0); // 0th bit for carry
    cpu.set_zn(new_value);
    new_value
}
