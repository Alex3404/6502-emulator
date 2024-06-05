mod absolute;
mod absolutex;
mod absolutey;
mod indirectx;
mod indirecty;
mod readwrite;
mod zeropage;
mod zeropagex;
mod zeropagey;

use crate::cpu::*;
use absolute::*;
use absolutex::*;
use absolutey::*;
use indirectx::*;
use indirecty::*;
use readwrite::*;
use zeropage::*;
use zeropagex::*;
use zeropagey::*;

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
    cpu.read(cpu.reg.pc);
    cpu.tick();
}

fn immediate_1read(cpu: &mut MOS6502, func: &ReadInst) {
    // T1
    let value = cpu.read(cpu.reg.pc);
    cpu.reg.pc += 1;
    func(cpu, value);
    cpu.tick();
}
