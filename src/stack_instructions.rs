use crate::emulator::*;

#[allow(unused_variables)]
pub fn transfer_sp_to_x(context: &mut MOS6502, mode: AddressingMode) {
    context.reg.ix = context.reg.sp;

    context.flags.zero = context.reg.ac == 0;
    context.flags.negative = (context.reg.ac & (1 << 7)) != 0;
}

#[allow(unused_variables)]
pub fn transfer_x_to_sp(context: &mut MOS6502, mode: AddressingMode) {
    context.reg.sp = context.reg.ix;
}

#[allow(unused_variables)]
pub fn push_ac(context: &mut MOS6502, mode: AddressingMode) {
    context.push(context.reg.ac);
}

#[allow(unused_variables)]
pub fn push_processor(context: &mut MOS6502, mode: AddressingMode) {
    context.push_processor_status();
}

#[allow(unused_variables)]
pub fn pull_ac(context: &mut MOS6502, mode: AddressingMode) {
    context.reg.ac = context.pop();

    context.flags.zero = context.reg.ac == 0;
    context.flags.negative = (context.reg.ac & (1 << 7)) != 0;
}

#[allow(unused_variables)]
pub fn pull_processor_status(context: &mut MOS6502, mode: AddressingMode) {
    context.pop_processor_status();
}
