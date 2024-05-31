use crate::emulator::*;

pub fn load_ac(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    let value = memory[address as usize];

    context.flags.zero = value == 0;
    context.flags.negative = (value & (1 << 7)) != 0;

    context.reg.ac = value;
}

pub fn load_ix(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    let value = memory[address as usize];

    context.flags.zero = value == 0;
    context.flags.negative = (value & (1 << 7)) != 0;

    context.reg.ix = value;
}

pub fn load_iy(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    let value = memory[address as usize];

    context.flags.zero = value == 0;
    context.flags.negative = (value & (1 << 7)) != 0;

    context.reg.iy = value;
}

pub fn store_ac(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    memory[address as usize] = context.reg.ac
}

pub fn store_ix(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    memory[address as usize] = context.reg.ix
}

pub fn store_iy(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    memory[address as usize] = context.reg.iy
}

#[allow(unused_variables)]
pub fn transfer_ac_to_x(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    context.reg.ix = context.reg.ac;

    context.flags.zero = context.reg.ac == 0;
    context.flags.negative = (context.reg.ac & (1 << 7)) != 0;
}

#[allow(unused_variables)]
pub fn transfer_ac_to_y(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    context.reg.iy = context.reg.ac;

    context.flags.zero = context.reg.ac == 0;
    context.flags.negative = (context.reg.ac & (1 << 7)) != 0;
}

#[allow(unused_variables)]
pub fn transfer_x_to_ac(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    context.reg.ac = context.reg.ix;

    context.flags.zero = context.reg.ac == 0;
    context.flags.negative = (context.reg.ac & (1 << 7)) != 0;
}

#[allow(unused_variables)]
pub fn transfer_y_to_ac(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    context.reg.ac = context.reg.iy;

    context.flags.zero = context.reg.ac == 0;
    context.flags.negative = (context.reg.ac & (1 << 7)) != 0;
}
