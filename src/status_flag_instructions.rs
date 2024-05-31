use crate::emulator::*;

#[allow(unused_variables)]
pub fn clear_carry(context: &mut MOS6502, mode: AddressingMode) {
    context.flags.carry = false;
}

#[allow(unused_variables)]
pub fn clear_decimal(context: &mut MOS6502, mode: AddressingMode) {
    context.flags.decimal_mode = false;
}

#[allow(unused_variables)]
pub fn clear_int_disable(context: &mut MOS6502, mode: AddressingMode) {
    context.flags.int_disable = false;
}

#[allow(unused_variables)]
pub fn clear_overflow(context: &mut MOS6502, mode: AddressingMode) {
    context.flags.overflow = false;
}

#[allow(unused_variables)]
pub fn set_carry(context: &mut MOS6502, mode: AddressingMode) {
    context.flags.carry = true;
}

#[allow(unused_variables)]
pub fn set_decimal(context: &mut MOS6502, mode: AddressingMode) {
    context.flags.decimal_mode = true;
}

#[allow(unused_variables)]
pub fn set_int_disable(context: &mut MOS6502, mode: AddressingMode) {
    context.flags.int_disable = true;
}
