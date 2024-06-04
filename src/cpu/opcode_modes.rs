use crate::cpu::*;

type Inst = dyn Fn(&mut MOS6502);
type ReadInst = dyn Fn(&mut MOS6502, u8);
type WriteInst = dyn Fn(&mut MOS6502) -> u8;
type ReadWriteInst = dyn Fn(&mut MOS6502, u8) -> u8;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum AddressingMode {
    Accumulator,
    Immediate,
    Indirect,
    Relative,
    Implied,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

// Matches all 151 opcodes to their correct addressing_mode
pub fn get_addressing_mode(opcode: u8) -> AddressingMode {
    if opcode == 0x6C {
        return AddressingMode::Indirect;
    }
    if opcode == 0x20 {
        return AddressingMode::Absolute;
    }
    if opcode == 0xC0 || opcode == 0xA2 || opcode == 0xA0 || opcode == 0xE0 {
        return AddressingMode::Immediate;
    }
    if opcode == 0x4A || opcode == 0xA || opcode == 0x2A || opcode == 0x6A {
        return AddressingMode::Accumulator;
    }
    if opcode == 0xB6 || opcode == 0x96 {
        return AddressingMode::ZeroPageY;
    }

    // Lots of magic numbers (Might be a better way of doing this but whatever its shorter then 151 lines)
    if (opcode & 0b11111) == 0b10000 {
        return AddressingMode::Relative;
    }
    if (opcode & 0b11111) == 0b10001 {
        return AddressingMode::IndirectY;
    }
    if (opcode & 0b11111) == 0b00001 {
        return AddressingMode::IndirectX;
    }
    if (opcode & 0b101) == 0 {
        return AddressingMode::Implied;
    }
    if (opcode & 0b1111) == 0b1010 {
        return AddressingMode::Accumulator;
    }
    if (opcode & 0b11100) == 0b01100 {
        return AddressingMode::Absolute;
    }
    if (opcode & 0b10100) == 0 {
        return AddressingMode::Immediate;
    }
    if (opcode & 0b11100) == 0b00100 {
        return AddressingMode::ZeroPage;
    }
    if (opcode & 0b11100) == 0b10100 {
        return AddressingMode::ZeroPageX;
    }
    if (opcode & 0b11100) == 0b11000 || opcode == 0b10111110 {
        return AddressingMode::AbsoluteY;
    }
    if (opcode & 0b11100) == 0b11100 {
        return AddressingMode::AbsoluteX;
    }

    AddressingMode::Implied
}

pub fn instruction_implied(cpu: &mut MOS6502, func: &Inst) {
    implied_1read(cpu);
    func(cpu);
}

pub fn instruction_read(cpu: &mut MOS6502, addressing_mode: AddressingMode, func: &ReadInst) {
    match addressing_mode {
        AddressingMode::Immediate => immediate_1read(cpu, func),
        AddressingMode::Relative => immediate_1read(cpu, func),
        AddressingMode::Absolute => absolute_3read(cpu, func),
        AddressingMode::ZeroPage => zeropage_2read(cpu, func),
        AddressingMode::ZeroPageX => zeropagex_3read(cpu, func),
        AddressingMode::ZeroPageY => zeropagey_3read(cpu, func),
        AddressingMode::AbsoluteX => absolutex_3read(cpu, func),
        AddressingMode::AbsoluteY => absolutey_3read(cpu, func),
        AddressingMode::IndirectX => indirectx_5read(cpu, func),
        AddressingMode::IndirectY => indirecty_5read(cpu, func),
        _ => panic!("Invalid instruction read addressing mode!"),
    }
}

pub fn instruction_write(cpu: &mut MOS6502, addressing_mode: AddressingMode, func: &WriteInst) {
    match addressing_mode {
        AddressingMode::Absolute => absolute_3write(cpu, func),
        AddressingMode::ZeroPage => zeropage_2write(cpu, func),
        AddressingMode::ZeroPageX => zeropagex_3write(cpu, func),
        AddressingMode::ZeroPageY => zeropagey_3write(cpu, func),
        AddressingMode::AbsoluteX => absolutex_4write(cpu, func),
        AddressingMode::AbsoluteY => absolutey_4write(cpu, func),
        AddressingMode::IndirectX => indirectx_5write(cpu, func),
        AddressingMode::IndirectY => indirecty_5write(cpu, func),
        _ => panic!("Invalid instruction write addressing mode!"),
    }
}

pub fn instruction_read_move_write(
    cpu: &mut MOS6502,
    addressing_mode: AddressingMode,
    func: &ReadWriteInst,
) {
    match addressing_mode {
        AddressingMode::Accumulator => ac1_rmw(cpu, func),
        AddressingMode::ZeroPage => zeropage_4rmw(cpu, func),
        AddressingMode::ZeroPageX => zeropagex_5rmw(cpu, func),
        AddressingMode::Absolute => absolute_5rmw(cpu, func),
        AddressingMode::AbsoluteX => absolutex_6rmw(cpu, func),
        _ => panic!("Invalid instruction read move write addressing mode!"),
    }
}

fn implied_1read(cpu: &mut MOS6502) {
    cpu.bus.read(cpu.reg.pc);
    cpu.tick();
}

fn immediate_1read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1
    let value = cpu.bus.read(cpu.reg.pc);
    cpu.reg.pc += 1;
    func(cpu, value);
    cpu.tick();
}

fn absolute_3read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let address = address | (cpu.bus.read(cpu.reg.pc) as u16) << 8;
    cpu.reg.pc += 1;
    cpu.tick();

    // T3
    let value = cpu.bus.read(address);
    func(cpu, value);
    cpu.tick();
}

fn absolute_3write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let address = address | (cpu.bus.read(cpu.reg.pc) as u16) << 8;
    cpu.reg.pc += 1;
    cpu.tick();

    // T3
    let value = func(cpu);
    cpu.bus.write(address, value);
    cpu.tick();
}

fn zeropage_2read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let value = cpu.bus.read(address);
    func(cpu, value); // Perform the operation
    cpu.tick();
}

fn zeropage_2write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let value = func(cpu);
    cpu.bus.write(address, value);
    cpu.tick();
}

fn absolutex_3read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let address = address | (cpu.bus.read(cpu.reg.pc) as u16) << 8;
    let address_x = address + cpu.reg.ix as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T3
    if !same_page(address, address_x) {
        cpu.bus.read(address_x - 0x100); // Perform buggy read in previous page
        cpu.tick();
    }

    // T3 or T4
    let value = cpu.bus.read(address_x);
    func(cpu, value);
    cpu.tick();
}

fn absolutex_4write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let address = address | (cpu.bus.read(cpu.reg.pc) as u16) << 8;
    let address_x = address + cpu.reg.ix as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T3
    cpu.bus.read((address & 0xFF00) | (address_x & 0x00FF));
    cpu.tick();

    // T4
    let value = func(cpu);
    cpu.bus.write(address_x, value);
    cpu.tick();
}

fn absolutey_3read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let address = address | (cpu.bus.read(cpu.reg.pc) as u16) << 8;
    let address_y = address + cpu.reg.iy as u16;

    cpu.reg.pc += 1;
    cpu.tick();

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

fn absolutey_4write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let address = address | (cpu.bus.read(cpu.reg.pc) as u16) << 8;
    let address_y = address + cpu.reg.iy as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T3
    cpu.bus.read((address & 0xFF00) | (address_y & 0x00FF));
    cpu.tick();

    // T4
    let value = func(cpu);
    cpu.bus.write(address_y, value);
    cpu.tick();
}

fn zeropagex_3write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    let address_x = (address + cpu.reg.ix as u16) & 0xFF;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    cpu.bus.read(address);
    cpu.tick();

    // T3
    let value = func(cpu);
    cpu.bus.write(address_x, value);
    cpu.tick();
}

fn zeropagex_3read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    let address_x = (address + cpu.reg.ix as u16) & 0xFF;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    cpu.bus.read(address);
    cpu.tick();

    // T3
    let value = cpu.bus.read(address_x);
    func(cpu, value);
    cpu.tick();
}

fn zeropagey_3write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    let address_y = (address + cpu.reg.iy as u16) & 0xFF;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    cpu.bus.read(address);
    cpu.tick();

    // T3
    let value = func(cpu);
    cpu.bus.write(address_y, value);
    cpu.tick();
}

fn zeropagey_3read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    let address_y = (address + cpu.reg.iy as u16) & 0xFF;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    cpu.bus.read(address);
    cpu.tick();

    // T3
    let value = cpu.bus.read(address_y);
    func(cpu, value);
    cpu.tick();
}

fn indirecty_5read(cpu: &mut MOS6502, func: &ReadInst) {
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

fn indirecty_5write(cpu: &mut MOS6502, func: &WriteInst) {
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

    // T4
    cpu.bus.read((address & 0xFF00) | (address_y | 0x00FF));
    cpu.tick();

    // T5
    let value = func(cpu);
    cpu.bus.write(address_y, value);
    cpu.tick();
}

fn indirectx_5read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1
    let indirect_address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    cpu.bus.read(indirect_address);
    cpu.tick();

    // T3
    let zp_address = (indirect_address + cpu.reg.ix as u16) & 0xFF;
    let address = cpu.bus.read(zp_address) as u16;
    cpu.tick();

    // T4
    let zp_address = (indirect_address + cpu.reg.ix as u16 + 1) & 0xFF;
    let address = address | ((cpu.bus.read(zp_address) as u16) << 8);
    cpu.tick();

    // T5
    let value = cpu.bus.read(address);
    func(cpu, value);
    cpu.tick();
}

fn indirectx_5write(cpu: &mut MOS6502, func: &WriteInst) {
    // T1
    let indirect_address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    cpu.bus.read(indirect_address);
    cpu.tick();

    // T3
    let zp_address = (indirect_address + cpu.reg.ix as u16) & 0xFF;
    let address = cpu.bus.read(zp_address) as u16;
    cpu.tick();

    // T4
    let zp_address = (indirect_address + cpu.reg.ix as u16 + 1) & 0xFF;
    let address = address | ((cpu.bus.read(zp_address) as u16) << 8);
    cpu.tick();

    // T5
    let value = func(cpu);
    cpu.bus.write(address, value);
    cpu.tick();
}

fn ac1_rmw(cpu: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    cpu.bus.read(cpu.reg.pc);
    cpu.reg.ac = func(cpu, cpu.reg.ac);
    cpu.tick();
}

fn zeropage_4rmw(cpu: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let value = cpu.bus.read(address);
    cpu.tick();

    // T3
    cpu.bus.write(address, value);
    cpu.tick();

    // T4
    let value = func(cpu, value);
    cpu.bus.write(address, value);
    cpu.tick();
}

fn absolute_5rmw(cpu: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let address = address | ((cpu.bus.read(cpu.reg.pc) as u16) << 8);
    cpu.reg.pc += 1;
    cpu.tick();

    // T3
    let value = cpu.bus.read(address);
    cpu.tick();

    // T4
    cpu.bus.write(address, value);
    cpu.tick();

    // T5
    let value = func(cpu, value);
    cpu.bus.write(address, value);
    cpu.tick();
}

fn zeropagex_5rmw(cpu: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    let address_x = (address + cpu.reg.ix as u16) & 0xFF;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    cpu.bus.read(address);
    cpu.tick();

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

fn absolutex_6rmw(cpu: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = cpu.bus.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let address = address | (cpu.bus.read(cpu.reg.pc) as u16) << 8;
    let address = address + cpu.reg.ix as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T3
    cpu.bus.read(address);
    cpu.tick();

    // T4
    let value = cpu.bus.read(address);
    cpu.tick();

    // T5
    cpu.bus.write(address, value);
    cpu.tick();

    // T6
    let value = func(cpu, value);
    cpu.bus.write(address, value);
    cpu.tick();
}
