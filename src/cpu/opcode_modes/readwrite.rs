use crate::cpu::opcode_modes::*;
use crate::cpu::MOS6502;

///////////////////////////
// Read,Write Addressing //
///////////////////////////

pub fn ac1_rmw(cpu: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    cpu.read(cpu.reg.pc);
    cpu.reg.ac = func(cpu, cpu.reg.ac);
    cpu.tick();
}
