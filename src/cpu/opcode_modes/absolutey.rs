use crate::cpu::opcode_modes::*;
use crate::cpu::MOS6502;

///////////////////////////
// Absolute,Y Addressing //
///////////////////////////

fn fetch_absolutey_2(cpu: &mut MOS6502) -> (u16, u16) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let address = address | (cpu.bus.read(cpu.reg.pc) as u16) << 8;
    let address_y = address + cpu.reg.iy as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    (address, address_y)
}

pub fn absolutey_3read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1, T2
    let (address, address_y) = fetch_absolutey_2(cpu);
    // T3
    if !same_page(address, address_y) {
        cpu.bus.read(address_y - 0x100); // Perform buggy read in previous page
        cpu.tick();
    }
    // T3 or T4
    let value = cpu.bus.read(address_y);
    func(cpu, value);
    cpu.tick();
}

pub fn absolutey_4write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1, T2
    let (address, address_y) = fetch_absolutey_2(cpu);
    // T3
    cpu.bus.read((address & 0xFF00) | (address_y & 0x00FF));
    cpu.tick();
    // T4
    let value = func(cpu);
    cpu.bus.write(address_y, value);
    cpu.tick();
}
