use crate::cpu::opcode_modes::*;
use crate::cpu::MOS6502;

///////////////////////////
// Indirect,Y Addressing //
///////////////////////////

fn fetch_indirecty_3(cpu: &mut MOS6502) -> (u16, u16) {
    // T1
    let zp_address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();
    // T2
    let address = cpu.bus.read(zp_address) as u16;
    cpu.tick();
    // T3
    let address = address | ((cpu.bus.read((zp_address + 1) & 0xFF) as u16) << 8);
    let address_y = address + cpu.reg.iy as u16;
    cpu.tick();

    (address, address_y)
}

pub fn indirecty_5read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1, T2, T3
    let (address, address_y) = fetch_indirecty_3(cpu);

    // T4
    if !same_page(address, address_y) {
        cpu.bus.read((address & 0xFF00) | (address_y | 0x00FF));
        cpu.tick();
    }

    // T4 or T5
    let value = cpu.bus.read(address_y);
    func(cpu, value);
    cpu.tick();
}

pub fn indirecty_5write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1, T2, T3
    let (address, address_y) = fetch_indirecty_3(cpu);

    // T4
    cpu.bus.read((address & 0xFF00) | (address_y | 0x00FF));
    cpu.tick();

    // T5
    let value = func(cpu);
    cpu.bus.write(address_y, value);
    cpu.tick();
}
