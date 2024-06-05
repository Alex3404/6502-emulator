use crate::cpu::opcode_modes::*;
use crate::cpu::MOS6502;

/////////////////////////
// Absolute Addressing //
/////////////////////////

fn fetch_absolute_2(cpu: &mut MOS6502) -> u16 {
    // T1
    let address = cpu.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let address = address | (cpu.read(cpu.reg.pc) as u16) << 8;
    cpu.reg.pc += 1;
    cpu.tick();

    address
}

pub fn absolute_3read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1, T2
    let address = fetch_absolute_2(cpu);

    // T3
    let value = cpu.read(address);
    func(cpu, value);
    cpu.tick();
}

pub fn absolute_3write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1, T2
    let address = fetch_absolute_2(cpu);

    // T3
    let value = func(cpu);
    cpu.write(address, value);
    cpu.tick();
}

pub fn absolute_5rmw(cpu: &mut MOS6502, func: &ReadWriteInst) {
    // T1, T2
    let address = fetch_absolute_2(cpu);

    // T3
    let value = cpu.read(address);
    cpu.tick();

    // T4
    cpu.write(address, value);
    cpu.tick();

    // T5
    let value = func(cpu, value);
    cpu.write(address, value);
    cpu.tick();
}
