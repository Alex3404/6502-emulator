use crate::cpu::*;

pub fn clear_carry(cpu: &mut MOS6502) {
    cpu.set(CPUFLAGS::CARRY, false);
}

pub fn clear_decimal(cpu: &mut MOS6502) {
    cpu.set(CPUFLAGS::DECIMAL, false);
}

pub fn clear_int_disable(cpu: &mut MOS6502) {
    cpu.set(CPUFLAGS::INT_DISABLE, false);
}

pub fn clear_overflow(cpu: &mut MOS6502) {
    cpu.set(CPUFLAGS::OVERFLOW, false);
}

pub fn set_carry(cpu: &mut MOS6502) {
    cpu.set(CPUFLAGS::CARRY, true);
}

pub fn set_decimal(cpu: &mut MOS6502) {
    cpu.set(CPUFLAGS::DECIMAL, true);
}

pub fn set_int_disable(cpu: &mut MOS6502) {
    cpu.set(CPUFLAGS::INT_DISABLE, true);
}
