use crate::cpu::*;

pub fn load_ac(cpu: &mut MOS6502, value: u8) {
    cpu.set_zn(value);
    cpu.reg.ac = value;
}

pub fn load_ix(cpu: &mut MOS6502, value: u8) {
    cpu.set_zn(value);
    cpu.reg.ix = value;
}

pub fn load_iy(cpu: &mut MOS6502, value: u8) {
    cpu.set_zn(value);
    cpu.reg.iy = value;
}

pub fn store_ac(cpu: &mut MOS6502) -> u8 {
    cpu.reg.ac
}

pub fn store_ix(cpu: &mut MOS6502) -> u8 {
    cpu.reg.ix
}

pub fn store_iy(cpu: &mut MOS6502) -> u8 {
    cpu.reg.iy
}

pub fn transfer_ac_to_x(cpu: &mut MOS6502) {
    cpu.reg.ix = cpu.reg.ac;
    cpu.set_zn(cpu.reg.ac);
}

pub fn transfer_ac_to_y(cpu: &mut MOS6502) {
    cpu.reg.iy = cpu.reg.ac;
    cpu.set_zn(cpu.reg.ac);
}

pub fn transfer_x_to_ac(cpu: &mut MOS6502) {
    cpu.reg.ac = cpu.reg.ix;
    cpu.set_zn(cpu.reg.ac);
}

pub fn transfer_y_to_ac(cpu: &mut MOS6502) {
    cpu.reg.ac = cpu.reg.iy;
    cpu.set_zn(cpu.reg.ac);
}
