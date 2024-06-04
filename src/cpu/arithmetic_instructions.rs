use crate::cpu::*;

pub fn add_with_carry(cpu: &mut MOS6502, value: u8) {
    assert!(
        !cpu.is_set(CPUFLAGS::DECIMAL),
        "Decimal mode not implemented!"
    );

    let carry = cpu.is_set(CPUFLAGS::CARRY);
    // A,Z,C,N,V = A+M+C
    let (result, carry) = cpu.reg.ac.carrying_add(value, carry);

    // let same_sign_operands: bool = ((value ^ cpu.reg.ac) & 0x80) == 0;
    // let different_sign_result = ((cpu.reg.ac ^ new_value as u8) & 0x80) != 0;
    // let overflow = same_sign_operands & different_sign_result;
    let overflow = ((cpu.reg.ac ^ result) & (!(value ^ cpu.reg.ac)) & 0x80) != 0;
    cpu.set(CPUFLAGS::OVERFLOW, overflow);
    cpu.set(CPUFLAGS::CARRY, carry);
    cpu.set_zn(result);

    cpu.reg.ac = result;
}

pub fn sub_with_carry(cpu: &mut MOS6502, value: u8) {
    assert!(
        !cpu.is_set(CPUFLAGS::DECIMAL),
        "Decimal mode not implemented!"
    );

    let carry = cpu.is_set(CPUFLAGS::CARRY);
    // A,Z,C,N = A-M-(1-C)
    let (result, carry) = cpu.reg.ac.borrowing_sub(value, !carry);

    // let different_sign_operands: bool = ((value ^ cpu.reg.ac) & 0x80) != 0;
    // let different_sign_result = ((cpu.reg.ac ^ new_value as u8) & 0x80) != 0;
    // let overflow = different_sign_operands & different_sign_result;
    let overflow = (result ^ cpu.reg.ac) & (value ^ cpu.reg.ac) & 0x80 != 0;
    cpu.set(CPUFLAGS::OVERFLOW, overflow);
    cpu.set(CPUFLAGS::CARRY, !carry);
    cpu.set_zn(result);

    cpu.reg.ac = result;
}

pub fn compare_ac(cpu: &mut MOS6502, value: u8) {
    // println!("CMP {:02X} == {:02X}", cpu.reg.ac, value);

    let subtracted_value = unsafe { cpu.reg.ac.unchecked_sub(value) };

    cpu.set(CPUFLAGS::CARRY, cpu.reg.ac >= value);
    cpu.set_zn(subtracted_value);
}

pub fn compare_ix(cpu: &mut MOS6502, value: u8) {
    let subtracted_value = unsafe { cpu.reg.ix.unchecked_sub(value) };

    cpu.set(CPUFLAGS::CARRY, cpu.reg.ix >= value);
    cpu.set_zn(subtracted_value);
}

pub fn compare_iy(cpu: &mut MOS6502, value: u8) {
    let subtracted_value = unsafe { cpu.reg.iy.unchecked_sub(value) };

    cpu.set(CPUFLAGS::CARRY, cpu.reg.iy >= value);
    cpu.set_zn(subtracted_value);
}
