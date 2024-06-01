use crate::emulator::*;

pub fn load_ac(context: &mut MOS6502, value: u8) {
    context.set_zn(value);
    context.reg.ac = value;
}

pub fn load_ix(context: &mut MOS6502, value: u8) {
    context.set_zn(value);
    context.reg.ix = value;
}

pub fn load_iy(context: &mut MOS6502, value: u8) {
    context.set_zn(value);
    context.reg.iy = value;
}

pub fn store_ac(context: &mut MOS6502) -> u8 {
    context.reg.ac
}

pub fn store_ix(context: &mut MOS6502) -> u8 {
    context.reg.ix
}

pub fn store_iy(context: &mut MOS6502) -> u8 {
    context.reg.iy
}

pub fn transfer_ac_to_x(context: &mut MOS6502) {
    context.reg.ix = context.reg.ac;
    context.set_zn(context.reg.ac);
}

pub fn transfer_ac_to_y(context: &mut MOS6502) {
    context.reg.iy = context.reg.ac;
    context.set_zn(context.reg.ac);
}

pub fn transfer_x_to_ac(context: &mut MOS6502) {
    context.reg.ac = context.reg.ix;
    context.set_zn(context.reg.ac);
}

pub fn transfer_y_to_ac(context: &mut MOS6502) {
    context.reg.ac = context.reg.iy;
    context.set_zn(context.reg.ac);
}
