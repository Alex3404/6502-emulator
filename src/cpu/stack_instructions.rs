use crate::cpu::*;

pub fn transfer_sp_to_x(cpu: &mut MOS6502) {
    cpu.reg.ix = cpu.reg.sp;
    cpu.set_zn(cpu.reg.ix);
    cpu.tick();
}

pub fn transfer_x_to_sp(cpu: &mut MOS6502) {
    cpu.reg.sp = cpu.reg.ix;
    cpu.tick();
}

pub fn push_ac(cpu: &mut MOS6502) {
    // T1
    cpu.stack_push(cpu.reg.ac);
    cpu.tick();
}

pub fn pull_ac(cpu: &mut MOS6502) {
    // T1
    cpu.stack_peek();
    cpu.stack_pop_no_read();
    cpu.tick();

    // T2
    cpu.reg.ac = cpu.stack_peek();
    cpu.set_zn(cpu.reg.ac);
    cpu.tick();
}

pub fn push_processor(cpu: &mut MOS6502) {
    // T1
    cpu.push_processor_status();
    cpu.tick();
}

pub fn pull_processor_status(cpu: &mut MOS6502) {
    // T1
    cpu.stack_peek();
    cpu.stack_pop_no_read();
    cpu.tick();

    // T2
    let value = cpu.stack_peek();
    cpu.set_proccessor_status(value);
    cpu.tick();
}
