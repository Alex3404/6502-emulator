use std::boxed::Box;
use std::time::Instant;

use crate::{
    arithmetic_instructions, branching_instructions, inc_dec_instructions, logical_instructions,
    memory::MemoryBus, opcode_modes, stack_instructions, status_flag_instructions,
    transfer_load_store_instructions,
};

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

pub struct MOS6502Registers {
    pub pc: u16,
    pub sp: u8,
    pub ac: u8,
    pub ix: u8,
    pub iy: u8,
}

pub struct MOS6502Flags {
    pub carry: bool,
    pub zero: bool,
    pub int_disable: bool,
    pub decimal_mode: bool,
    pub break_cmd: bool,
    pub overflow: bool,
    pub negative: bool,
}

pub struct MOS6502 {
    pub reg: MOS6502Registers,
    pub flags: MOS6502Flags,
    pub mem: Box<dyn MemoryBus>,
    last_clock: Instant,
}

pub fn same_page(addr1: u16, addr2: u16) -> bool {
    (addr1 & 0xFF00) == (addr2 & 0xFF00)
}

impl MOS6502 {
    pub fn new(memory: Box<dyn MemoryBus>) -> Self {
        Self {
            reg: MOS6502Registers {
                pc: 0,
                sp: 0xFF,
                ac: 0,
                ix: 0,
                iy: 0,
            },
            flags: MOS6502Flags {
                carry: false,
                zero: false,
                int_disable: false,
                decimal_mode: false,
                break_cmd: false,
                overflow: false,
                negative: false,
            },
            mem: memory,
            last_clock: Instant::now(),
        }
    }

    pub fn execute(&mut self, start_address: u16) {
        self.reset();
        self.reg.pc = start_address;

        loop {
            step(self);
        }
    }

    pub fn stack_peek(&mut self) -> u8 {
        self.mem.read(STACK_BASE + self.reg.sp as u16)
    }

    #[allow(dead_code)]
    pub fn stack_write(&mut self, value: u8) {
        self.mem.write(STACK_BASE + self.reg.sp as u16, value);
    }

    pub fn stack_pop_no_read(&mut self) {
        self.reg.sp = unsafe { self.reg.sp.unchecked_add(1) };
    }

    #[allow(dead_code)]
    pub fn stack_push_no_read(&mut self) {
        self.reg.sp = unsafe { self.reg.sp.unchecked_sub(1) };
    }

    pub fn stack_push(&mut self, value: u8) {
        self.mem.write(STACK_BASE + self.reg.sp as u16, value);
        self.reg.sp = unsafe { self.reg.sp.unchecked_sub(1) };
    }

    #[allow(dead_code)]
    pub fn stack_pop(&mut self) -> u8 {
        self.reg.sp = unsafe { self.reg.sp.unchecked_add(1) };
        self.mem.read(STACK_BASE + self.reg.sp as u16)
    }

    pub fn reset(&mut self) {
        self.reg.sp = 0x00;
        self.reg.pc =
            self.mem.read(RESET_VECTOR) as u16 | ((self.mem.read(RESET_VECTOR) as u16) << 8);
        self.flags.int_disable = true;
    }

    pub fn push_processor_status(&mut self) {
        let mut value = UNUSED_FLAG | BREAK_FLAG;

        if self.flags.carry {
            value |= CARRY_FLAG;
        }
        if self.flags.zero {
            value |= ZERO_FLAG;
        }
        if self.flags.int_disable {
            value |= INT_DISABLE_FLAG;
        }
        if self.flags.decimal_mode {
            value |= DECIMAL_FLAG;
        }
        if self.flags.overflow {
            value |= OVERFLOW_FLAG;
        }
        if self.flags.negative {
            value |= NEGATIVE_FLAG;
        }

        self.stack_push(value);
    }

    pub fn set_proccessor_status(&mut self, value: u8) {
        self.flags.carry = value & CARRY_FLAG != 0;
        self.flags.zero = value & ZERO_FLAG != 0;
        self.flags.int_disable = value & INT_DISABLE_FLAG != 0;
        self.flags.decimal_mode = value & DECIMAL_FLAG != 0;
        self.flags.overflow = value & OVERFLOW_FLAG != 0;
        self.flags.negative = value & NEGATIVE_FLAG != 0;
        self.flags.break_cmd = true;
    }

    pub fn tick(&mut self) {
        loop {
            let now = Instant::now();
            let time = now - self.last_clock;
            if time.subsec_nanos() > 950 {
                break;
            }
        }
    }

    pub fn set_zn(&mut self, value: u8) {
        self.flags.zero = value == 0;
        self.flags.negative = (value & (1 << 7)) != 0;
    }
}

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

fn step(context: &mut MOS6502) {
    // T1
    let opcode: u8 = context.mem.read(context.reg.pc);
    context.reg.pc = (context.reg.pc as u32 + 1) as u16;
    context.tick();

    let addressing_mode = opcode_modes::get_addressing_mode(opcode);
    match opcode {
        0x0 => branching_instructions::break_interrupt(context), // Custom preface no opcode_modes::instruction_implied() used
        0x40 => opcode_modes::instruction_implied(
            context,
            &branching_instructions::return_from_interrupt,
        ),
        0xEA => println!("NOP"),

        // Logical
        0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => opcode_modes::instruction_read(
            context,
            addressing_mode,
            &logical_instructions::logical_and,
        ),
        0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => opcode_modes::instruction_read(
            context,
            addressing_mode,
            &logical_instructions::logical_exclusive_or,
        ),
        0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
            opcode_modes::instruction_read(
                context,
                addressing_mode,
                &logical_instructions::logical_inclusive_or,
            );
        }
        0x24 | 0x2C => opcode_modes::instruction_read(
            context,
            addressing_mode,
            &logical_instructions::logical_bit_test,
        ),

        // Shifts
        0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
            opcode_modes::instruction_read_move_write(
                context,
                addressing_mode,
                &logical_instructions::logical_shift_left,
            );
        }
        0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
            opcode_modes::instruction_read_move_write(
                context,
                addressing_mode,
                &logical_instructions::logical_shift_right,
            );
        }
        0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
            opcode_modes::instruction_read_move_write(
                context,
                addressing_mode,
                &logical_instructions::logical_rotate_left,
            );
        }
        0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
            opcode_modes::instruction_read_move_write(
                context,
                addressing_mode,
                &logical_instructions::logical_rotate_right,
            );
        }

        // Arithmetic
        0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
            opcode_modes::instruction_read(
                context,
                addressing_mode,
                &arithmetic_instructions::add_with_carry,
            );
        }
        0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
            opcode_modes::instruction_read(
                context,
                addressing_mode,
                &arithmetic_instructions::sub_with_carry,
            );
        }
        0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
            opcode_modes::instruction_read(
                context,
                addressing_mode,
                &arithmetic_instructions::compare_ac,
            );
        }
        0xE0 | 0xE4 | 0xEC => {
            opcode_modes::instruction_read(
                context,
                addressing_mode,
                &arithmetic_instructions::compare_ix,
            );
        }
        0xC0 | 0xC4 | 0xCC => {
            opcode_modes::instruction_read(
                context,
                addressing_mode,
                &arithmetic_instructions::compare_iy,
            );
        }

        // Increment & Decrement Instructions
        0xE6 | 0xF6 | 0xEE | 0xFE => opcode_modes::instruction_read_move_write(
            context,
            addressing_mode,
            &inc_dec_instructions::inc_memory,
        ),
        0xE8 => {
            opcode_modes::instruction_implied(context, &inc_dec_instructions::inc_ix);
        }
        0xC8 => {
            opcode_modes::instruction_implied(context, &inc_dec_instructions::inc_iy);
        }

        0xC6 | 0xD6 | 0xCE | 0xDE => {
            opcode_modes::instruction_read_move_write(
                context,
                addressing_mode,
                &inc_dec_instructions::dec_memory,
            );
        }
        0xCA => {
            opcode_modes::instruction_implied(context, &inc_dec_instructions::dec_ix);
        }
        0x88 => {
            opcode_modes::instruction_implied(context, &inc_dec_instructions::dec_iy);
        }

        // Load/Store instructions
        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
            opcode_modes::instruction_read(
                context,
                addressing_mode,
                &transfer_load_store_instructions::load_ac,
            );
        }

        0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
            opcode_modes::instruction_read(
                context,
                addressing_mode,
                &transfer_load_store_instructions::load_ix,
            );
        }

        0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
            opcode_modes::instruction_read(
                context,
                addressing_mode,
                &transfer_load_store_instructions::load_iy,
            );
        }

        0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
            opcode_modes::instruction_write(
                context,
                addressing_mode,
                &transfer_load_store_instructions::store_ac,
            );
        }

        0x86 | 0x96 | 0x8E => {
            opcode_modes::instruction_write(
                context,
                addressing_mode,
                &transfer_load_store_instructions::store_ix,
            );
        }

        0x84 | 0x94 | 0x8C => {
            opcode_modes::instruction_write(
                context,
                addressing_mode,
                &transfer_load_store_instructions::store_iy,
            );
        }

        // Transfer instructions
        0xAA => opcode_modes::instruction_implied(
            context,
            &transfer_load_store_instructions::transfer_ac_to_x,
        ),
        0xA8 => opcode_modes::instruction_implied(
            context,
            &transfer_load_store_instructions::transfer_ac_to_y,
        ),
        0x8A => opcode_modes::instruction_implied(
            context,
            &transfer_load_store_instructions::transfer_x_to_ac,
        ),
        0x98 => opcode_modes::instruction_implied(
            context,
            &transfer_load_store_instructions::transfer_y_to_ac,
        ),

        // Jumps & Calls
        0x4C => branching_instructions::jmp_absolute(context),
        0x6C => branching_instructions::jmp_indirect(context),
        0x20 => {
            opcode_modes::instruction_implied(context, &branching_instructions::jump_to_subroutine)
        }
        0x60 => opcode_modes::instruction_implied(
            context,
            &branching_instructions::return_from_subroutine,
        ),

        // Branches
        0x90 => opcode_modes::instruction_read(
            context,
            addressing_mode,
            &branching_instructions::branch_if_carry_clear,
        ),
        0xB0 => opcode_modes::instruction_read(
            context,
            addressing_mode,
            &branching_instructions::branch_if_carry_set,
        ),
        0xF0 => opcode_modes::instruction_read(
            context,
            addressing_mode,
            &branching_instructions::branch_if_equal,
        ),
        0xD0 => opcode_modes::instruction_read(
            context,
            addressing_mode,
            &branching_instructions::branch_if_not_equal,
        ),
        0x30 => opcode_modes::instruction_read(
            context,
            addressing_mode,
            &branching_instructions::branch_if_minus,
        ),
        0x10 => opcode_modes::instruction_read(
            context,
            addressing_mode,
            &branching_instructions::branch_if_positive,
        ),
        0x50 => opcode_modes::instruction_read(
            context,
            addressing_mode,
            &branching_instructions::branch_if_overflow_clear,
        ),
        0x70 => opcode_modes::instruction_read(
            context,
            addressing_mode,
            &branching_instructions::branch_if_overflow_set,
        ),

        // Stack instructions
        0xBA => opcode_modes::instruction_implied(context, &stack_instructions::transfer_sp_to_x),
        0x9A => opcode_modes::instruction_implied(context, &stack_instructions::transfer_x_to_sp),
        0x48 => opcode_modes::instruction_implied(context, &stack_instructions::push_ac),
        0x08 => opcode_modes::instruction_implied(context, &stack_instructions::push_processor),
        0x68 => opcode_modes::instruction_implied(context, &stack_instructions::pull_ac),
        0x28 => {
            opcode_modes::instruction_implied(context, &stack_instructions::pull_processor_status)
        }

        // Status Flags
        0x18 => opcode_modes::instruction_implied(context, &status_flag_instructions::clear_carry),
        0xD8 => {
            opcode_modes::instruction_implied(context, &status_flag_instructions::clear_decimal)
        }
        0x58 => {
            opcode_modes::instruction_implied(context, &status_flag_instructions::clear_int_disable)
        }
        0xB8 => {
            opcode_modes::instruction_implied(context, &status_flag_instructions::clear_overflow)
        }
        0x38 => opcode_modes::instruction_implied(context, &status_flag_instructions::set_carry),
        0xF8 => opcode_modes::instruction_implied(context, &status_flag_instructions::set_decimal),
        0x78 => {
            opcode_modes::instruction_implied(context, &status_flag_instructions::set_int_disable)
        }

        _ => {
            panic!(
                "Illegal instruction {:x} at {:x}",
                opcode,
                context.reg.pc - 1
            );
        }
    };
}
