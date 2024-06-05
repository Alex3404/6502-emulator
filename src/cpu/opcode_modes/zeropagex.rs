use crate::cpu::opcode_modes::*;
use crate::cpu::MOS6502;

///////////////////////////
// ZeroPage,X Addressing //
///////////////////////////

fn fetch_zeropagex_2(cpu: &mut MOS6502) -> u16 {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    let address_x = (address + cpu.reg.ix as u16) & 0xFF;
    cpu.reg.pc += 1;
    cpu.tick();
    // T2
    cpu.bus.read(address);
    cpu.tick();

    address_x
}

pub fn zeropagex_3write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1, T2
    let address_x = fetch_zeropagex_2(cpu);
    // T3
    let value = func(cpu);
    cpu.bus.write(address_x, value);
    cpu.tick();
}

pub fn zeropagex_3read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1, T2
    let address_x = fetch_zeropagex_2(cpu);
    // T3
    let value = cpu.bus.read(address_x);
    func(cpu, value);
    cpu.tick();
}

pub fn zeropagex_5rmw(cpu: &mut MOS6502, func: &ReadWriteInst) {
    // T1, T2
    let address_x = fetch_zeropagex_2(cpu);
    // T3
    let value = cpu.bus.read(address_x);
    cpu.tick();
    // T4
    cpu.bus.write(address_x, value);
    cpu.tick();
    // T5
    let value = func(cpu, value);
    cpu.bus.write(address_x, value);
    cpu.tick();
}
