use crate::cpu::opcode_modes::*;
use crate::cpu::MOS6502;

/////////////////////////
// ZeroPage Addressing //
/////////////////////////

fn fetch_zeropage1(cpu: &mut MOS6502) -> u16 {
    // T1
    let address = cpu.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    address
}

pub fn zeropage_2read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = fetch_zeropage1(cpu);

    // T2
    let value = cpu.read(address);
    func(cpu, value); // Perform the operation
    cpu.tick();
}

pub fn zeropage_2write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = fetch_zeropage1(cpu);

    // T2
    let value = func(cpu);
    cpu.write(address, value);
    cpu.tick();
}

pub fn zeropage_4rmw(cpu: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = fetch_zeropage1(cpu);

    // T2
    let value = cpu.read(address);
    cpu.tick();

    // T3
    cpu.write(address, value);
    cpu.tick();

    // T4
    let value = func(cpu, value);
    cpu.write(address, value);
    cpu.tick();
}
