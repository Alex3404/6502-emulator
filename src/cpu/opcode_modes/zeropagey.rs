use crate::cpu::opcode_modes::*;
use crate::cpu::MOS6502;

///////////////////////////
// ZeroPage,Y Addressing //
///////////////////////////

fn fetch_zeropagey_2(cpu: &mut MOS6502) -> u16 {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    let address_y = (address + cpu.reg.iy as u16) & 0xFF;
    cpu.reg.pc += 1;
    cpu.tick();
    // T2
    cpu.bus.read(address);
    cpu.tick();

    address_y
}

pub fn zeropagey_3write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1, T2
    let address_y = fetch_zeropagey_2(cpu);
    // T3
    let value = func(cpu);
    cpu.bus.write(address_y, value);
    cpu.tick();
}

pub fn zeropagey_3read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1, T2
    let address_y = fetch_zeropagey_2(cpu);
    // T3
    let value = cpu.bus.read(address_y);
    func(cpu, value);
    cpu.tick();
}
