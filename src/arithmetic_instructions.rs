use crate::cpu::*;

pub fn add_with_carry(context: &mut MOS6502, value: u8) {
    // A,Z,C,N,V = A+M+C
    let (new_ac, carry) = context.reg.ac.carrying_add(value, context.flags.carry);
    context.flags.overflow = carry;
    context.flags.carry = carry;
    context.set_zn(new_ac);
    context.reg.ac = new_ac;
}

pub fn sub_with_carry(context: &mut MOS6502, value: u8) {
    // A,Z,C,N,V = A-M-(1-C)
    let (new_value, carry) = context.reg.ac.borrowing_sub(value, !context.flags.carry);
    context.flags.overflow = ((context.reg.ac ^ new_value) & (value ^ new_value) & 0x80) != 0;
    context.flags.carry = carry;
    context.set_zn(new_value);
    context.reg.ac = new_value;
}

pub fn compare_ac(context: &mut MOS6502, value: u8) {
    println!("CMP: {:X}", value);
    let subtracted_value = unsafe { context.reg.ac.unchecked_sub(value) };
    context.flags.carry = context.reg.ac >= value;
    context.set_zn(subtracted_value);
}

pub fn compare_ix(context: &mut MOS6502, value: u8) {
    let subtracted_value = unsafe { context.reg.ix.unchecked_sub(value) };
    context.flags.carry = context.reg.ix >= value;
    context.set_zn(subtracted_value);
}

pub fn compare_iy(context: &mut MOS6502, value: u8) {
    let subtracted_value = unsafe { context.reg.iy.unchecked_sub(value) };
    context.flags.carry = context.reg.iy >= value;
    context.set_zn(subtracted_value);
}
