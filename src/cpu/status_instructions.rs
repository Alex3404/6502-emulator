use crate::cpu::*;

pub fn clear_carry(context: &mut MOS6502) {
    context.flags.set(CPUFLAGS::CARRY, false);
}

pub fn clear_decimal(context: &mut MOS6502) {
    context.flags.set(CPUFLAGS::DECIMAL, false);
}

pub fn clear_int_disable(context: &mut MOS6502) {
    context.flags.set(CPUFLAGS::INT_DISABLE, false);
}

pub fn clear_overflow(context: &mut MOS6502) {
    context.flags.set(CPUFLAGS::OVERFLOW, false);
}

pub fn set_carry(context: &mut MOS6502) {
    context.flags.set(CPUFLAGS::CARRY, true);
}

pub fn set_decimal(context: &mut MOS6502) {
    context.flags.set(CPUFLAGS::DECIMAL, true);
}

pub fn set_int_disable(context: &mut MOS6502) {
    context.flags.set(CPUFLAGS::INT_DISABLE, true);
}
