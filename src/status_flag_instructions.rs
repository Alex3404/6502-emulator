use crate::emulator::*;

#[allow(unused_variables)]
pub fn clear_carry(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    context.flags.carry = false;
}

#[allow(unused_variables)]
pub fn clear_decimal(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    context.flags.decimal_mode = false;
}

#[allow(unused_variables)]
pub fn clear_int_disable(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    context.flags.int_disable = false;
}

#[allow(unused_variables)]
pub fn clear_overflow(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    context.flags.overflow = false;
}

#[allow(unused_variables)]
pub fn set_carry(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    context.flags.carry = true;
}

#[allow(unused_variables)]
pub fn set_decimal(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    context.flags.decimal_mode = true;
}

#[allow(unused_variables)]
pub fn set_int_disable(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    context.flags.int_disable = true;
}
