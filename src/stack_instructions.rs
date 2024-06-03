use crate::cpu::*;

pub fn transfer_sp_to_x(context: &mut MOS6502) {
    context.reg.ix = context.reg.sp;
    context.set_zn(context.reg.ix);
    context.tick();
}

pub fn transfer_x_to_sp(context: &mut MOS6502) {
    context.reg.sp = context.reg.ix;
    context.tick();
}

pub fn push_ac(context: &mut MOS6502) {
    // T1
    context.stack_push(context.reg.ac);
    context.tick();
}

pub fn pull_ac(context: &mut MOS6502) {
    // T1
    context.stack_peek();
    context.stack_pop_no_read();
    context.tick();

    // T2
    context.reg.ac = context.stack_peek();
    context.set_zn(context.reg.ac);
    context.tick();
}

pub fn push_processor(context: &mut MOS6502) {
    println!("Push proccessor!");
    // T1
    context.push_processor_status();
    context.tick();
}

pub fn pull_processor_status(context: &mut MOS6502) {
    // T1
    context.stack_peek();
    context.stack_pop_no_read();
    context.tick();

    // T2
    let value = context.stack_peek();
    context.set_proccessor_status(value);
    context.tick();
}
