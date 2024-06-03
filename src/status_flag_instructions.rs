use crate::cpu::*;

pub fn clear_carry(context: &mut MOS6502) {
    context.flags.carry = false;
}

pub fn clear_decimal(context: &mut MOS6502) {
    context.flags.decimal_mode = false;
}

pub fn clear_int_disable(context: &mut MOS6502) {
    context.flags.int_disable = false;
}

pub fn clear_overflow(context: &mut MOS6502) {
    context.flags.overflow = false;
}

pub fn set_carry(context: &mut MOS6502) {
    context.flags.carry = true;
}

pub fn set_decimal(context: &mut MOS6502) {
    context.flags.decimal_mode = true;
}

pub fn set_int_disable(context: &mut MOS6502) {
    context.flags.int_disable = true;
}
