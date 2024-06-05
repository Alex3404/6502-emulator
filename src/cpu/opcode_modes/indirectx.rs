use crate::cpu::opcode_modes::*;
use crate::cpu::MOS6502;

///////////////////////////
// Indirect,X Addressing //
///////////////////////////

fn fetch_indirectx(cpu: &mut MOS6502) -> u16 {
    // T1
    let indirect_address = cpu.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();
    // T2
    cpu.read(indirect_address);
    cpu.tick();
    // T3
    let zp_address = (indirect_address + cpu.reg.ix as u16) & 0xFF;
    let address = cpu.read(zp_address) as u16;
    cpu.tick();
    // T4
    let zp_address = (indirect_address + cpu.reg.ix as u16 + 1) & 0xFF;
    let address = address | ((cpu.read(zp_address) as u16) << 8);
    cpu.tick();

    address
}

pub fn indirectx_5read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1, T2, T3, T4
    let address = fetch_indirectx(cpu);

    // T5
    let value = cpu.read(address);
    func(cpu, value);
    cpu.tick();
}

pub fn indirectx_5write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1, T2, T3, T4
    let address = fetch_indirectx(cpu);

    // T5
    let value = func(cpu);
    cpu.write(address, value);
    cpu.tick();
}
