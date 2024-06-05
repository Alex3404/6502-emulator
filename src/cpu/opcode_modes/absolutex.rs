use crate::cpu::opcode_modes::*;
use crate::cpu::MOS6502;

///////////////////////////
// Absolute,X Addressing //
///////////////////////////

fn fetch_absolutex_2(cpu: &mut MOS6502) -> (u16, u16) {
    // T1
    let address = cpu.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let address = address | (cpu.read(cpu.reg.pc) as u16) << 8;
    let address_x = address + cpu.reg.ix as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    (address, address_x)
}

pub fn absolutex_3read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1, T2
    let (address, address_x) = fetch_absolutex_2(cpu);

    // T3
    if !same_page(address, address_x) {
        cpu.read(address_x - 0x100); // Perform buggy read in previous page
        cpu.tick();
    }

    // T3 or T4
    let value = cpu.read(address_x);
    func(cpu, value);
    cpu.tick();
}

pub fn absolutex_4write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1, T2
    let (address, address_x) = fetch_absolutex_2(cpu);

    // T3
    cpu.read((address & 0xFF00) | (address_x & 0x00FF));
    cpu.tick();

    // T4
    let value = func(cpu);
    cpu.write(address_x, value);
    cpu.tick();
}

pub fn absolutex_6rmw(cpu: &mut MOS6502, func: &ReadWriteInst) {
    let (_, address_x) = fetch_absolutex_2(cpu);

    // T3
    cpu.read(address_x);
    cpu.tick();

    // T4
    let value = cpu.read(address_x);
    cpu.tick();

    // T5
    cpu.write(address_x, value);
    cpu.tick();

    // T6
    let value = func(cpu, value);
    cpu.write(address_x, value);
    cpu.tick();
}
