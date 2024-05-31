use crate::{
    arithmetic_instructions, branching_instructions, inc_dec_instructions, logical_instructions,
    stack_instructions, status_flag_instructions, transfer_load_store_instructions,
};

pub const NMI_VECTOR: u16 = 0xFFFA;
pub const RESET_VECTOR: u16 = 0xFFFC;
pub const IRQ_VECTOR: u16 = 0xFFFE;

pub const STACK_BASE: u16 = 0x100;

pub const CARRY_FLAG: u8 = 1;
pub const ZERO_FLAG: u8 = 2;
pub const INT_DISABLE_FLAG: u8 = 4;
pub const DECIMAL_FLAG: u8 = 8;
pub const BREAK_FLAG: u8 = 16;
pub const UNUSED_FLAG: u8 = 32;
pub const OVERFLOW_FLAG: u8 = 64;
pub const NEGATIVE_FLAG: u8 = 128;

pub struct Obelisk6502Registers {
    pub pc: u16,
    pub sp: u8,
    pub ac: u8,
    pub ix: u8,
    pub iy: u8,
}

pub struct Obelisk6502Flags {
    pub carry: bool,
    pub zero: bool,
    pub int_disable: bool,
    pub decimal_mode: bool,
    pub break_cmd: bool,
    pub overflow: bool,
    pub negative: bool,
}

pub struct Obelisk6502Context {
    pub reg: Obelisk6502Registers,
    pub flags: Obelisk6502Flags,
    incremented_pc: bool,
}

impl Default for Obelisk6502Context {
    fn default() -> Self {
        Self {
            reg: Obelisk6502Registers {
                pc: 0,
                sp: 0xFF,
                ac: 0,
                ix: 0,
                iy: 0,
            },
            flags: Obelisk6502Flags {
                carry: false,
                zero: false,
                int_disable: false,
                decimal_mode: false,
                break_cmd: false,
                overflow: false,
                negative: false,
            },
            incremented_pc: false,
        }
    }
}

// Entire memory
pub type MemoryBank = [u8; (u16::MAX as usize) + 1];

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

// Matches all 151 opcodes too their correct addressing_mode
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

pub fn fetch_instruction_abs_address(
    mode: AddressingMode,
    context: &Obelisk6502Context,
    memory: &MemoryBank,
) -> u16 {
    let pc: u16 = context.reg.pc;

    match mode {
        AddressingMode::Accumulator => pc + 1,
        AddressingMode::Immediate => pc + 1,
        AddressingMode::ZeroPage => read(memory, pc + 1) as u16,
        AddressingMode::ZeroPageX => (read(memory, pc + 1) as u16 + context.reg.ix as u16) & 0xFF,
        AddressingMode::ZeroPageY => (read(memory, pc + 1) as u16 + context.reg.iy as u16) & 0xFF,
        AddressingMode::Absolute => read_word(memory, pc + 1),
        AddressingMode::AbsoluteX => read_word(memory, pc + 1) + context.reg.ix as u16,
        AddressingMode::AbsoluteY => read_word(memory, pc + 1) + context.reg.iy as u16,
        AddressingMode::IndirectX => {
            let zero_page_address = read(memory, pc + 1) as u16;
            let zero_page_address = (zero_page_address + context.reg.ix as u16) & 0xFF;
            read_word(memory, zero_page_address)
        }
        AddressingMode::IndirectY => {
            let zero_page_address = read(memory, pc + 1) as u16;
            read_word(memory, zero_page_address) + context.reg.iy as u16
        }
        AddressingMode::Indirect => {
            let address = read_word(memory, pc + 1);
            read_word(memory, address)
        }
        AddressingMode::Relative => {
            let relative_offset = read(memory, pc + 1) as u16;

            if relative_offset & (1 << 7) != 0 {
                // 2's compliment of the relative offset since its negative
                let relative_offset = (relative_offset & 0x7F).wrapping_neg();
                (pc as u32 + 2 + relative_offset as u32) as u16
            } else {
                (pc as u32 + 2 + relative_offset as u32) as u16
            }
        }
        _ => panic!("Undefined Addressing Mode"),
    }
}

pub fn set_zn(context: &mut Obelisk6502Context, value: u8) {
    context.flags.zero = value == 0;
    context.flags.negative = (value & (1 << 7)) != 0;
}

pub fn read(memory: &MemoryBank, address: u16) -> u8 {
    memory[address as usize]
}

pub fn read_word(memory: &MemoryBank, address: u16) -> u16 {
    let lsb = memory[address as usize] as u16;
    let msb = memory[(address as usize) + 1] as u16;
    lsb | (msb << 8)
}

pub fn write(memory: &mut MemoryBank, address: u16, value: u8) {
    memory[address as usize] = value;
}

pub fn write_word(memory: &mut MemoryBank, address: u16, value: u16) {
    memory[address as usize] = value as u8;
    memory[(address as usize) + 1] = (value >> 8) as u8;
}

pub fn push(context: &mut Obelisk6502Context, memory: &mut MemoryBank, value: u8) {
    write(memory, STACK_BASE + context.reg.sp as u16, value);
    context.reg.sp = unsafe { context.reg.sp.unchecked_sub(1) };
}

pub fn push_word(context: &mut Obelisk6502Context, memory: &mut MemoryBank, value: u16) {
    write_word(memory, STACK_BASE + context.reg.sp as u16, value);
    context.reg.sp = unsafe { context.reg.sp.unchecked_sub(2) };
}

pub fn pop(context: &mut Obelisk6502Context, memory: &mut MemoryBank) -> u8 {
    context.reg.sp = unsafe { context.reg.sp.unchecked_add(1) };
    read(memory, STACK_BASE + context.reg.sp as u16)
}

pub fn pop_word(context: &mut Obelisk6502Context, memory: &mut MemoryBank) -> u16 {
    context.reg.sp = unsafe { context.reg.sp.unchecked_add(2) };
    read_word(memory, STACK_BASE + context.reg.sp as u16)
}

pub fn jump_to_new_pc(context: &mut Obelisk6502Context, address: u16) {
    context.incremented_pc = true;
    context.reg.pc = address;
}

pub fn push_processor_status(context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let mut value = UNUSED_FLAG;

    if context.flags.carry {
        value |= CARRY_FLAG;
    }
    if context.flags.zero {
        value |= ZERO_FLAG;
    }
    if context.flags.int_disable {
        value |= INT_DISABLE_FLAG;
    }
    if context.flags.decimal_mode {
        value |= DECIMAL_FLAG;
    }
    if context.flags.break_cmd {
        value |= BREAK_FLAG
    }
    if context.flags.overflow {
        value |= OVERFLOW_FLAG;
    }
    if context.flags.negative {
        value |= NEGATIVE_FLAG;
    }

    push(context, memory, value)
}

pub fn pop_processor_status(context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let value = pop(context, memory);

    context.flags.carry = value & CARRY_FLAG != 0;
    context.flags.zero = value & ZERO_FLAG != 0;
    context.flags.int_disable = value & INT_DISABLE_FLAG != 0;
    context.flags.decimal_mode = value & DECIMAL_FLAG != 0;
    context.flags.break_cmd = value & BREAK_FLAG != 0;
    context.flags.overflow = value & OVERFLOW_FLAG != 0;
    context.flags.negative = value & NEGATIVE_FLAG != 0;
}

fn break_interrupt(context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    push_word(context, memory, context.reg.pc);
    push_processor_status(context, memory);

    let irq = read_word(memory, IRQ_VECTOR);
    println!("Interrupt handler: {:x}", irq);
    jump_to_new_pc(context, irq);
    context.flags.break_cmd = true;
}

fn return_from_interrupt(context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    pop_processor_status(context, memory);
    let old_pc = pop_word(context, memory);
    jump_to_new_pc(context, old_pc);
}

fn increment_pc(context: &mut Obelisk6502Context, addressing_mode: AddressingMode) {
    if context.incremented_pc {
        panic!("Double increment of Program Counter in same execute!")
    }

    context.incremented_pc = true;
    context.reg.pc += match addressing_mode {
        AddressingMode::Accumulator => 1,
        AddressingMode::Implied => 1,

        AddressingMode::Immediate => 2,
        AddressingMode::ZeroPage => 2,
        AddressingMode::ZeroPageX => 2,
        AddressingMode::ZeroPageY => 2,
        AddressingMode::IndirectX => 2,
        AddressingMode::IndirectY => 2,
        AddressingMode::Relative => 2,

        AddressingMode::Absolute => 3,
        AddressingMode::AbsoluteX => 3,
        AddressingMode::AbsoluteY => 3,
        AddressingMode::Indirect => 3,
    };
}

fn execute(context: &mut Obelisk6502Context, memory: &mut MemoryBank) {
    let pc: u16 = context.reg.pc;
    let opcode: u8 = memory[pc as usize];
    let addressing_mode = get_addressing_mode(opcode);
    context.incremented_pc = false;

    match opcode {
        0x0 => {
            break_interrupt(context, memory);
        }
        0x40 => {
            return_from_interrupt(context, memory);
        }

        0xEA => {
            print!("NOP!");
        }

        // Logical
        0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
            logical_instructions::logical_and(addressing_mode, context, memory);
        }
        0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
            logical_instructions::logical_exclusive_or(addressing_mode, context, memory);
        }
        0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
            logical_instructions::logical_inclusive_or(addressing_mode, context, memory);
        }
        0x24 | 0x2C => {
            logical_instructions::logical_bit_test(addressing_mode, context, memory);
        }

        // Shifts
        0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
            logical_instructions::logical_shift_left(addressing_mode, context, memory)
        }
        0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
            logical_instructions::logical_shift_right(addressing_mode, context, memory)
        }
        0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
            logical_instructions::logical_rotate_left(addressing_mode, context, memory)
        }
        0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
            logical_instructions::logical_rotate_right(addressing_mode, context, memory)
        }

        // Arithmetic
        0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
            arithmetic_instructions::add_with_carry(addressing_mode, context, memory);
        }
        0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
            arithmetic_instructions::sub_with_carry(addressing_mode, context, memory);
        }
        0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
            arithmetic_instructions::compare_ac(addressing_mode, context, memory);
        }
        0xE0 | 0xE4 | 0xEC => {
            arithmetic_instructions::compare_ix(addressing_mode, context, memory);
        }
        0xC0 | 0xC4 | 0xCC => {
            arithmetic_instructions::compare_iy(addressing_mode, context, memory);
        }

        // Increment & Decrement Instructions
        0xE6 | 0xF6 | 0xEE | 0xFE => {
            inc_dec_instructions::inc_memory(addressing_mode, context, memory);
        }
        0xE8 => {
            inc_dec_instructions::inc_ix(addressing_mode, context, memory);
        }
        0xC8 => {
            inc_dec_instructions::inc_iy(addressing_mode, context, memory);
        }

        0xC6 | 0xD6 | 0xCE | 0xDE => {
            inc_dec_instructions::dec_memory(addressing_mode, context, memory);
        }
        0xCA => {
            inc_dec_instructions::dec_ix(addressing_mode, context, memory);
        }
        0x88 => {
            inc_dec_instructions::dec_iy(addressing_mode, context, memory);
        }

        // Load/Store instructions
        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
            transfer_load_store_instructions::load_ac(addressing_mode, context, memory);
        }

        0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
            transfer_load_store_instructions::load_ix(addressing_mode, context, memory);
        }

        0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
            transfer_load_store_instructions::load_iy(addressing_mode, context, memory);
        }

        0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
            transfer_load_store_instructions::store_ac(addressing_mode, context, memory);
        }

        0x86 | 0x96 | 0x8E => {
            transfer_load_store_instructions::store_ix(addressing_mode, context, memory);
        }

        0x84 | 0x94 | 0x8C => {
            transfer_load_store_instructions::store_iy(addressing_mode, context, memory);
        }

        // Transfer instructions
        0xAA => {
            transfer_load_store_instructions::transfer_ac_to_x(addressing_mode, context, memory)
        }

        0xA8 => {
            transfer_load_store_instructions::transfer_ac_to_y(addressing_mode, context, memory)
        }

        0x8A => {
            transfer_load_store_instructions::transfer_x_to_ac(addressing_mode, context, memory)
        }

        0x98 => {
            transfer_load_store_instructions::transfer_y_to_ac(addressing_mode, context, memory)
        }

        // Jumps & Calls
        0x4C | 0x6C => {
            branching_instructions::jmp(addressing_mode, context, memory);
        }
        0x20 => {
            branching_instructions::jump_to_subroutine(addressing_mode, context, memory);
        }
        0x60 => {
            branching_instructions::return_from_subroutine(addressing_mode, context, memory);
        }

        // Branches
        0x90 => branching_instructions::branch_if_carry_clear(addressing_mode, context, memory),
        0xB0 => branching_instructions::branch_if_carry_set(addressing_mode, context, memory),
        0xF0 => branching_instructions::branch_if_equal(addressing_mode, context, memory),
        0xD0 => branching_instructions::branch_if_not_equal(addressing_mode, context, memory),
        0x30 => branching_instructions::branch_if_minus(addressing_mode, context, memory),
        0x10 => branching_instructions::branch_if_positive(addressing_mode, context, memory),
        0x50 => branching_instructions::branch_if_overflow_clear(addressing_mode, context, memory),
        0x70 => branching_instructions::branch_if_overflow_set(addressing_mode, context, memory),

        // Stack instructions
        0xBA => stack_instructions::transfer_sp_to_x(addressing_mode, context, memory),
        0x9A => stack_instructions::transfer_x_to_sp(addressing_mode, context, memory),
        0x48 => stack_instructions::push_ac(addressing_mode, context, memory),
        0x08 => stack_instructions::push_processor(addressing_mode, context, memory),
        0x68 => stack_instructions::pull_ac(addressing_mode, context, memory),
        0x28 => stack_instructions::pull_processor_status(addressing_mode, context, memory),

        // Status Flags
        0x18 => status_flag_instructions::clear_carry(addressing_mode, context, memory),
        0xD8 => status_flag_instructions::clear_decimal(addressing_mode, context, memory),
        0x58 => status_flag_instructions::clear_int_disable(addressing_mode, context, memory),
        0xB8 => status_flag_instructions::clear_overflow(addressing_mode, context, memory),
        0x38 => status_flag_instructions::set_carry(addressing_mode, context, memory),
        0xF8 => status_flag_instructions::set_decimal(addressing_mode, context, memory),
        0x78 => status_flag_instructions::set_int_disable(addressing_mode, context, memory),

        _ => {
            panic!("Illegal instruction {:x}", opcode);
        }
    };

    if !context.incremented_pc {
        increment_pc(context, addressing_mode);
    }
}

pub fn load_and_execute(binary: MemoryBank) {
    let mut memory = binary;
    let mut context = Obelisk6502Context::default();

    loop {
        execute(&mut context, &mut memory);
    }
}
