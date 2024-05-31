use crate::emulator::*;

pub fn inc_memory(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    let value = read(memory, address);
    let new_value = ((value as u16 + 1) & 0xFF) as u8;

    context.flags.zero = new_value == 0;
    context.flags.negative = (new_value & (1 << 7)) != 0;

    write(memory, address, new_value);
}

#[allow(unused_variables)]
pub fn inc_ix(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let new_ix = context.reg.ix + 1;

    context.flags.zero = new_ix == 0;
    context.flags.negative = (new_ix & (1 << 7)) != 0;

    context.reg.ix = new_ix
}

#[allow(unused_variables)]
pub fn inc_iy(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let new_ix = context.reg.ix + 1;

    context.flags.zero = new_ix == 0;
    context.flags.negative = (new_ix & (1 << 7)) != 0;

    context.reg.ix = new_ix
}

pub fn dec_memory(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    let value = read(memory, address);
    let new_value = ((value as u16 - 1) & 0xFF) as u8;

    context.flags.zero = new_value == 0;
    context.flags.negative = (new_value & (1 << 7)) != 0;

    write(memory, address, new_value);
}

#[allow(unused_variables)]
pub fn dec_ix(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let new_ix = ((context.reg.ix as u16 - 1) & 0xFF) as u8;

    context.flags.zero = new_ix == 0;
    context.flags.negative = (new_ix & (1 << 7)) != 0;

    context.reg.ix = new_ix
}

#[allow(unused_variables)]
pub fn dec_iy(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let new_ix = ((context.reg.iy as u16 - 1) & 0xFF) as u8;

    context.flags.zero = new_ix == 0;
    context.flags.negative = (new_ix & (1 << 7)) != 0;

    context.reg.ix = new_ix
}
