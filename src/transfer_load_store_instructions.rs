use crate::emulator::*;

pub fn load_ac(context: &mut MOS6502, mode: AddressingMode) {
    let address: u16 = context.fetch_instruction_abs_address(mode);
    let value = context.read(address);

    context.flags.zero = value == 0;
    context.flags.negative = (value & (1 << 7)) != 0;

    context.reg.ac = value;
}

pub fn load_ix(context: &mut MOS6502, mode: AddressingMode) {
    let address: u16 = context.fetch_instruction_abs_address(mode);
    let value = context.read(address);

    context.flags.zero = value == 0;
    context.flags.negative = (value & (1 << 7)) != 0;

    context.reg.ix = value;
}

pub fn load_iy(context: &mut MOS6502, mode: AddressingMode) {
    let address: u16 = context.fetch_instruction_abs_address(mode);
    let value = context.read(address);

    context.flags.zero = value == 0;
    context.flags.negative = (value & (1 << 7)) != 0;

    context.reg.iy = value;
}

pub fn store_ac(context: &mut MOS6502, mode: AddressingMode) {
    let address: u16 = context.fetch_instruction_abs_address(mode);
    context.write(address, context.reg.ac)
}

pub fn store_ix(context: &mut MOS6502, mode: AddressingMode) {
    let address: u16 = context.fetch_instruction_abs_address(mode);
    context.write(address, context.reg.ix)
}

pub fn store_iy(context: &mut MOS6502, mode: AddressingMode) {
    let address: u16 = context.fetch_instruction_abs_address(mode);
    context.write(address, context.reg.iy)
}

#[allow(unused_variables)]
pub fn transfer_ac_to_x(context: &mut MOS6502, mode: AddressingMode) {
    context.reg.ix = context.reg.ac;

    context.flags.zero = context.reg.ac == 0;
    context.flags.negative = (context.reg.ac & (1 << 7)) != 0;
}

#[allow(unused_variables)]
pub fn transfer_ac_to_y(context: &mut MOS6502, mode: AddressingMode) {
    context.reg.iy = context.reg.ac;

    context.flags.zero = context.reg.ac == 0;
    context.flags.negative = (context.reg.ac & (1 << 7)) != 0;
}

#[allow(unused_variables)]
pub fn transfer_x_to_ac(context: &mut MOS6502, mode: AddressingMode) {
    context.reg.ac = context.reg.ix;

    context.flags.zero = context.reg.ac == 0;
    context.flags.negative = (context.reg.ac & (1 << 7)) != 0;
}

#[allow(unused_variables)]
pub fn transfer_y_to_ac(context: &mut MOS6502, mode: AddressingMode) {
    context.reg.ac = context.reg.iy;

    context.flags.zero = context.reg.ac == 0;
    context.flags.negative = (context.reg.ac & (1 << 7)) != 0;
}
