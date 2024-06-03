use crate::cpu::*;

pub fn inc_memory(context: &mut MOS6502, value: u8) -> u8 {
    let new_value = (value as u16 + 1) as u8;
    context.set_zn(new_value);
    new_value
}

pub fn inc_ix(context: &mut MOS6502) {
    let new_ix = (context.reg.ix as u16 + 1) as u8;
    context.set_zn(new_ix);
    context.reg.ix = new_ix
}

pub fn inc_iy(context: &mut MOS6502) {
    let new_iy = (context.reg.iy as u16 + 1) as u8;
    context.set_zn(new_iy);
    context.reg.iy = new_iy
}

pub fn dec_memory(context: &mut MOS6502, value: u8) -> u8 {
    let new_value = unsafe { value.unchecked_sub(1) };
    context.set_zn(new_value);
    new_value
}

pub fn dec_ix(context: &mut MOS6502) {
    let new_ix = unsafe { context.reg.ix.unchecked_sub(1) };
    context.set_zn(new_ix);
    context.reg.ix = new_ix
}

pub fn dec_iy(context: &mut MOS6502) {
    let new_iy = unsafe { context.reg.iy.unchecked_sub(1) };
    context.set_zn(new_iy);
    context.reg.iy = new_iy
}
