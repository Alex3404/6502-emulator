use crate::cpu::*;

pub fn inc_memory(cpu: &mut MOS6502, value: u8) -> u8 {
    let new_value = (value as u16 + 1) as u8;
    cpu.set_zn(new_value);
    new_value
}

pub fn inc_ix(cpu: &mut MOS6502) {
    let new_ix = (cpu.reg.ix as u16 + 1) as u8;
    cpu.set_zn(new_ix);
    cpu.reg.ix = new_ix
}

pub fn inc_iy(cpu: &mut MOS6502) {
    let new_iy = (cpu.reg.iy as u16 + 1) as u8;
    cpu.set_zn(new_iy);
    cpu.reg.iy = new_iy
}

pub fn dec_memory(cpu: &mut MOS6502, value: u8) -> u8 {
    let new_value = unsafe { value.unchecked_sub(1) };
    cpu.set_zn(new_value);
    new_value
}

pub fn dec_ix(cpu: &mut MOS6502) {
    let new_ix = unsafe { cpu.reg.ix.unchecked_sub(1) };
    cpu.set_zn(new_ix);
    cpu.reg.ix = new_ix
}

pub fn dec_iy(cpu: &mut MOS6502) {
    let new_iy = unsafe { cpu.reg.iy.unchecked_sub(1) };
    cpu.set_zn(new_iy);
    cpu.reg.iy = new_iy
}
