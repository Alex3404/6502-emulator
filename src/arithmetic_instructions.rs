use crate::emulator::*;

pub fn add_with_carry(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    let value = read(memory, address);

    let (new_ac, carry) = context.reg.ac.carrying_add(value, context.flags.carry);

    // Flags
    context.flags.overflow = carry;
    context.flags.carry = carry;

    set_zn(context, new_ac);

    // Regs
    context.reg.ac = new_ac;
}

pub fn sub_with_carry(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    let value = read(memory, address);

    // A,Z,C,N = A-M-(1-C)
    let (new_ac, carry) = context.reg.ac.borrowing_sub(value, !context.flags.carry);

    // Flags
    context.flags.overflow = ((context.reg.ac ^ value) & (context.reg.ac ^ new_ac) & 0x80) != 0;
    context.flags.carry = carry;

    set_zn(context, new_ac);

    // Regs
    context.reg.ac = new_ac;
}

pub fn compare_ac(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    let value = read(memory, address);

    let subtracted_value = (context.reg.ac as u16 - value as u16) as u8;

    context.flags.carry = context.reg.ac >= value;
    set_zn(context, subtracted_value);
}

pub fn compare_ix(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    let value = read(memory, address);

    let subtracted_value = (context.reg.ix as u16 - value as u16) as u8;

    context.flags.carry = context.reg.ix >= value;
    set_zn(context, subtracted_value);
}

pub fn compare_iy(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let address: u16 = fetch_instruction_abs_address(mode, context, memory);
    let value = read(memory, address);

    let subtracted_value = (context.reg.iy as u16 - value as u16) as u8;

    context.flags.carry = context.reg.iy >= value;
    set_zn(context, subtracted_value);
}
