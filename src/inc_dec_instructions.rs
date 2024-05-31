use crate::emulator::*;

pub fn inc_memory(context: &mut MOS6502, mode: AddressingMode) {
    let address: u16 = context.fetch_instruction_abs_address(mode);
    let value = context.read(address);
    let new_value = ((value as u16 + 1) & 0xFF) as u8;

    context.flags.zero = new_value == 0;
    context.flags.negative = (new_value & (1 << 7)) != 0;

    context.write(address, new_value);
}

#[allow(unused_variables)]
pub fn inc_ix(context: &mut MOS6502, mode: AddressingMode) {
    let new_ix = context.reg.ix + 1;

    context.flags.zero = new_ix == 0;
    context.flags.negative = (new_ix & (1 << 7)) != 0;

    context.reg.ix = new_ix
}

#[allow(unused_variables)]
pub fn inc_iy(context: &mut MOS6502, mode: AddressingMode) {
    let new_ix = context.reg.ix + 1;

    context.flags.zero = new_ix == 0;
    context.flags.negative = (new_ix & (1 << 7)) != 0;

    context.reg.ix = new_ix
}

pub fn dec_memory(context: &mut MOS6502, mode: AddressingMode) {
    let address: u16 = context.fetch_instruction_abs_address(mode);
    let value = context.read(address);
    let new_value = ((value as u16 - 1) & 0xFF) as u8;

    context.flags.zero = new_value == 0;
    context.flags.negative = (new_value & (1 << 7)) != 0;

    context.write(address, new_value);
}

#[allow(unused_variables)]
pub fn dec_ix(context: &mut MOS6502, mode: AddressingMode) {
    let new_ix = ((context.reg.ix as u16 - 1) & 0xFF) as u8;

    context.flags.zero = new_ix == 0;
    context.flags.negative = (new_ix & (1 << 7)) != 0;

    context.reg.ix = new_ix
}

#[allow(unused_variables)]
pub fn dec_iy(context: &mut MOS6502, mode: AddressingMode) {
    let new_ix = ((context.reg.iy as u16 - 1) & 0xFF) as u8;

    context.flags.zero = new_ix == 0;
    context.flags.negative = (new_ix & (1 << 7)) != 0;

    context.reg.ix = new_ix
}
