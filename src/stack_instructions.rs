use crate::emulator::*;

#[allow(unused_variables)]
pub fn transfer_sp_to_x(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    context.reg.ix = context.reg.sp;

    context.flags.zero = context.reg.ac == 0;
    context.flags.negative = (context.reg.ac & (1 << 7)) != 0;
}

#[allow(unused_variables)]
pub fn transfer_x_to_sp(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    context.reg.sp = context.reg.ix;
}

#[allow(unused_variables)]
pub fn push_ac(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    push(context, memory, context.reg.ac)
}

#[allow(unused_variables)]
pub fn push_processor(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    push_processor_status(context, memory)
}

#[allow(unused_variables)]
pub fn pull_ac(mode: AddressingMode, context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    context.reg.ac = pop(context, memory);

    context.flags.zero = context.reg.ac == 0;
    context.flags.negative = (context.reg.ac & (1 << 7)) != 0;
}

#[allow(unused_variables)]
pub fn pull_processor_status(
    mode: AddressingMode,
    context: &mut Obelisk6502Context,
    memory: &mut MemoryBank,
) {
    pop_processor_status(context, memory)
}
