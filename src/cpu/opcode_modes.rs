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

pub fn instruction_implied(context: &mut MOS6502, func: &Inst) {
    implied_1read(context);
    func(context);
}

pub fn instruction_read(context: &mut MOS6502, addressing_mode: AddressingMode, func: &ReadInst) {
    match addressing_mode {
        AddressingMode::Immediate => immediate_1read(context, func),
        AddressingMode::Relative => immediate_1read(context, func),
        AddressingMode::Absolute => absolute_3read(context, func),
        AddressingMode::ZeroPage => zeropage_2read(context, func),
        AddressingMode::ZeroPageX => zeropagex_3read(context, func),
        AddressingMode::ZeroPageY => zeropagey_3read(context, func),
        AddressingMode::AbsoluteX => absolutex_3read(context, func),
        AddressingMode::AbsoluteY => absolutey_3read(context, func),
        AddressingMode::IndirectX => indirectx_5read(context, func),
        AddressingMode::IndirectY => indirecty_5read(context, func),
        _ => panic!("Invalid instruction read addressing mode!"),
    }
}

pub fn instruction_write(context: &mut MOS6502, addressing_mode: AddressingMode, func: &WriteInst) {
    match addressing_mode {
        AddressingMode::Absolute => absolute_3write(context, func),
        AddressingMode::ZeroPage => zeropage_2write(context, func),
        AddressingMode::ZeroPageX => zeropagex_3write(context, func),
        AddressingMode::ZeroPageY => zeropagey_3write(context, func),
        AddressingMode::AbsoluteX => absolutex_4write(context, func),
        AddressingMode::AbsoluteY => absolutey_4write(context, func),
        AddressingMode::IndirectX => indirectx_5write(context, func),
        AddressingMode::IndirectY => indirecty_5write(context, func),
        _ => panic!("Invalid instruction write addressing mode!"),
    }
}

pub fn instruction_read_move_write(
    context: &mut MOS6502,
    addressing_mode: AddressingMode,
    func: &ReadWriteInst,
) {
    match addressing_mode {
        AddressingMode::Accumulator => ac1_rmw(context, func),
        AddressingMode::ZeroPage => zeropage_4rmw(context, func),
        AddressingMode::ZeroPageX => zeropagex_5rmw(context, func),
        AddressingMode::Absolute => absolute_5rmw(context, func),
        AddressingMode::AbsoluteX => absolutex_6rmw(context, func),
        _ => panic!("Invalid instruction read move write addressing mode!"),
    }
}

fn implied_1read(context: &mut MOS6502) {
    context.mem.read(context.reg.pc);
    context.tick();
}

fn immediate_1read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let value = context.mem.read(context.reg.pc);
    context.reg.pc += 1;
    func(context, value);
    context.tick();
}

fn absolute_3read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.mem.read(context.reg.pc) as u16) << 8;
    context.reg.pc += 1;
    context.tick();

    // T3
    let value = context.mem.read(address);
    func(context, value);
    context.tick();
}

fn absolute_3write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.mem.read(context.reg.pc) as u16) << 8;
    context.reg.pc += 1;
    context.tick();

    // T3
    let value = func(context);
    context.mem.write(address, value);
    context.tick();
}

fn zeropage_2read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let value = context.mem.read(address);
    func(context, value); // Perform the operation
    context.tick();
}

fn zeropage_2write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let value = func(context);
    context.mem.write(address, value);
    context.tick();
}

fn absolutex_3read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.mem.read(context.reg.pc) as u16) << 8;
    let address_x = address + context.reg.ix as u16;
    context.reg.pc += 1;
    context.tick();

    // T3
    if !same_page(address, address_x) {
        context.mem.read(address_x - 0x100); // Perform buggy read in previous page
        context.tick();
    }

    // T3 or T4
    let value = context.mem.read(address_x);
    func(context, value);
    context.tick();
}

fn absolutex_4write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.mem.read(context.reg.pc) as u16) << 8;
    let address_x = address + context.reg.ix as u16;
    context.reg.pc += 1;
    context.tick();

    // T3
    context.mem.read((address & 0xFF00) | (address_x & 0x00FF));
    context.tick();

    // T4
    let value = func(context);
    context.mem.write(address, value);
    context.tick();
}

fn absolutey_3read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.mem.read(context.reg.pc) as u16) << 8;
    let address_y = address + context.reg.iy as u16;

    context.reg.pc += 1;
    context.tick();

    // T3
    if !same_page(address, address_y) {
        context.mem.read(address_y - 0x100); // Perform buggy read in previous page
        context.tick();
    }

    // T3 or T4
    let value = context.mem.read(address_y);
    func(context, value);
    context.tick();
}

fn absolutey_4write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.mem.read(context.reg.pc) as u16) << 8;
    let address_y = address + context.reg.iy as u16;
    context.reg.pc += 1;
    context.tick();

    // T3
    context.mem.read((address & 0xFF00) | (address_y & 0x00FF));
    context.tick();

    // T4
    let value = func(context);
    context.mem.write(address, value);
    context.tick();
}

fn zeropagex_3write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    let address_x = (address + context.reg.ix as u16) & 0xFF;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.mem.read(address);
    context.tick();

    // T3
    let value = func(context);
    context.mem.write(address_x, value);
    context.tick();
}

fn zeropagex_3read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    let address_x = (address + context.reg.ix as u16) & 0xFF;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.mem.read(address);
    context.tick();

    // T3
    let value = context.mem.read(address_x);
    func(context, value);
    context.tick();
}

fn zeropagey_3write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    let address_y = (address + context.reg.iy as u16) & 0xFF;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.mem.read(address);
    context.tick();

    // T3
    let value = func(context);
    context.mem.write(address_y, value);
    context.tick();
}

fn zeropagey_3read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    let address_y = (address + context.reg.iy as u16) & 0xFF;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.mem.read(address);
    context.tick();

    // T3
    let value = context.mem.read(address_y);
    func(context, value);
    context.tick();
}

fn indirecty_5read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let indirect_address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = context.mem.read(indirect_address) as u16;
    context.tick();

    // T3
    let address = address | ((context.mem.read(indirect_address + 1) as u16) << 8);
    let address_y = address + context.reg.iy as u16;
    context.tick();

    // T4
    if !same_page(address, address_y) {
        context.mem.read((address & 0xFF00) | (address_y | 0x00FF));
        context.tick();
    }

    // T4 or T5
    let value = context.mem.read(address_y);
    func(context, value);
    context.tick();
}

fn indirecty_5write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let indirect_address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = context.mem.read(indirect_address) as u16;
    context.tick();

    // T3
    let address = address | ((context.mem.read(indirect_address + 1) as u16) << 8);
    let address_y = address + context.reg.iy as u16;
    context.tick();

    // T4
    context.mem.read((address & 0xFF00) | (address_y | 0x00FF));
    context.tick();

    // T5
    let value = func(context);
    context.mem.write(address_y, value);
    context.tick();
}

fn indirectx_5read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let indirect_address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.mem.read(indirect_address);
    context.tick();

    // T3
    let address = context
        .mem
        .read((indirect_address + context.reg.ix as u16) & 0xFF) as u16;
    context.tick();

    // T4
    let address = address
        | context
            .mem
            .read((indirect_address + context.reg.ix as u16 + 1) & 0xFF) as u16;
    context.tick();

    // T5
    let value = context.mem.read(address);
    func(context, value);
    context.tick();
}

fn indirectx_5write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let indirect_address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.mem.read(indirect_address);
    context.tick();

    // T3
    let address = context
        .mem
        .read((indirect_address + context.reg.ix as u16) & 0xFF) as u16;
    context.tick();

    // T4
    let address = address
        | context
            .mem
            .read((indirect_address + context.reg.ix as u16 + 1) & 0xFF) as u16;
    context.tick();

    // T5
    let value = func(context);
    context.mem.write(address, value);
    context.tick();
}

fn ac1_rmw(context: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    context.mem.read(context.reg.pc);
    context.reg.ac = func(context, context.reg.ac);
    context.tick();
}

fn zeropage_4rmw(context: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let value = context.mem.read(address);
    context.tick();

    // T3
    context.mem.write(address, value);
    context.tick();

    // T4
    let value = func(context, value);
    context.mem.write(address, value);
    context.tick();
}

fn absolute_5rmw(context: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | ((context.mem.read(context.reg.pc) as u16) << 8);
    context.reg.pc += 1;
    context.tick();

    // T3
    let value = context.mem.read(address);
    context.tick();

    // T4
    context.mem.write(address, value);
    context.tick();

    // T5
    let value = func(context, value);
    context.mem.write(address, value);
    context.tick();
}

fn zeropagex_5rmw(context: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    let address_x = (address + context.reg.ix as u16) & 0xFF;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.mem.read(address);
    context.tick();

    // T3
    let value = context.mem.read(address_x);
    context.tick();

    // T4
    context.mem.write(address, value);
    context.tick();

    // T5
    let value = func(context, value);
    context.mem.write(address, value);
    context.tick();
}

fn absolutex_6rmw(context: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.mem.read(context.reg.pc) as u16) << 8;
    let address = address + context.reg.ix as u16;
    context.reg.pc += 1;
    context.tick();

    // T3
    context.mem.read(address);
    context.tick();

    // T4
    let value = context.mem.read(address);
    context.tick();

    // T5
    context.mem.write(address, value);
    context.tick();

    // T6
    let value = func(context, value);
    context.mem.write(address, value);
    context.tick();
}
