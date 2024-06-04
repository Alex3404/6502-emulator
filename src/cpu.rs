use bitflags::bitflags;
use std::boxed::Box;
use std::time::Instant;

mod arithmetic_instructions;
mod branching_instructions;
mod inc_dec_instructions;
mod logical_instructions;
pub mod opcode_modes;
mod stack_instructions;
mod status_instructions;
mod transfer_load_store_instructions;

use crate::address_bus::AddressBus;

pub const RESET_VECTOR: u16 = 0xFFFC;
pub const IRQ_VECTOR: u16 = 0xFFFE;
pub const STACK_BASE: u16 = 0x100;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct CPUFLAGS : u8 {
        const CARRY = 1;
        const ZERO = 2;
        const INT_DISABLE = 4;
        const DECIMAL = 8;
        const BREAK = 16;
        const UNUSED = 32;
        const OVERFLOW = 64;
        const NEGATIVE = 128;

        // The source may set any bits
        const _ = !0;
    }
}

pub struct MOS6502Registers {
    pub pc: u16,
    pub sp: u8,
    pub ac: u8,
    pub ix: u8,
    pub iy: u8,
    pub status: CPUFLAGS,
}

pub struct MOS6502 {
    pub reg: MOS6502Registers,
    pub bus: Box<dyn AddressBus>,
    trapped: bool,
    last_clock: Instant,
}

pub fn same_page(addr1: u16, addr2: u16) -> bool {
    (addr1 & 0xFF00) == (addr2 & 0xFF00)
}

#[allow(dead_code)]
impl MOS6502 {
    pub fn new(memory: Box<dyn AddressBus>) -> Self {
        Self {
            reg: MOS6502Registers {
                pc: 0,
                sp: 0xFF,
                ac: 0,
                ix: 0,
                iy: 0,
                status: CPUFLAGS::UNUSED,
            },
            bus: memory,
            last_clock: Instant::now(),
            trapped: false,
        }
    }

    fn set(&mut self, flag: CPUFLAGS, value: bool) {
        self.reg.status.set(flag, value);
    }

    fn is_set(&mut self, flag: CPUFLAGS) -> bool {
        self.reg.status.intersects(flag)
    }

    fn trapped(&mut self) {
        self.trapped = true;
    }

    pub fn is_trapped(&mut self) -> bool {
        self.trapped
    }

    pub fn set_pc(&mut self, address: u16) {
        self.trapped = false;
        self.reg.pc = address;
    }

    pub fn execute(&mut self, start_address: u16) {
        self.reset();
        self.reg.pc = start_address;

        loop {
            self.step();
        }
    }

    fn stack_peek(&mut self) -> u8 {
        self.bus.read(STACK_BASE + self.reg.sp as u16)
    }

    #[allow(dead_code)]
    fn stack_write(&mut self, value: u8) {
        self.bus.write(STACK_BASE + self.reg.sp as u16, value);
    }

    fn stack_pop_no_read(&mut self) {
        self.reg.sp = unsafe { self.reg.sp.unchecked_add(1) };
    }

    #[allow(dead_code)]
    fn stack_push_no_read(&mut self) {
        self.reg.sp = unsafe { self.reg.sp.unchecked_sub(1) };
    }

    fn stack_push(&mut self, value: u8) {
        self.bus.write(STACK_BASE + self.reg.sp as u16, value);
        self.reg.sp = unsafe { self.reg.sp.unchecked_sub(1) };
    }

    #[allow(dead_code)]
    fn stack_pop(&mut self) -> u8 {
        self.reg.sp = unsafe { self.reg.sp.unchecked_add(1) };
        self.bus.read(STACK_BASE + self.reg.sp as u16)
    }

    pub fn reset(&mut self) {
        self.reg.sp = 0x00;
        self.reg.pc =
            self.bus.read(RESET_VECTOR) as u16 | ((self.bus.read(RESET_VECTOR + 1) as u16) << 8);
        self.reg.status |= CPUFLAGS::INT_DISABLE;
    }

    fn push_processor_status(&mut self) {
        let flags = self.reg.status | CPUFLAGS::UNUSED | CPUFLAGS::BREAK;
        self.stack_push(flags.bits());
    }

    fn set_proccessor_status(&mut self, value: u8) {
        self.reg.status = CPUFLAGS::from_bits_truncate(value) | CPUFLAGS::UNUSED | CPUFLAGS::BREAK;
    }

    fn tick(&mut self) {
        loop {
            let now = Instant::now();
            let time = now - self.last_clock;
            if time.subsec_nanos() > 950 {
                break;
            }
        }
    }

    fn set_zn(&mut self, value: u8) {
        self.set(CPUFLAGS::ZERO, value == 0);
        self.set(CPUFLAGS::NEGATIVE, (value & (1 << 7)) != 0);
    }

    pub fn step(&mut self) {
        use arithmetic_instructions::*;
        use branching_instructions::*;
        use inc_dec_instructions::*;
        use logical_instructions::*;
        use opcode_modes::*;
        use stack_instructions::*;
        use status_instructions::*;
        use transfer_load_store_instructions::*;

        // T1
        let opcode: u8 = self.bus.read(self.reg.pc);
        self.reg.pc = (self.reg.pc as u32 + 1) as u16;
        self.tick();

        let mode = opcode_modes::get_addressing_mode(opcode);
        match opcode {
            0x0 => break_interrupt(self), // Custom preface no instruction_implied() used
            0x40 => instruction_implied(self, &return_from_interrupt),
            0xEA => (),

            // Logical
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                instruction_read(self, mode, &logical_and)
            }
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                instruction_read(self, mode, &logical_exclusive_or)
            }
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                instruction_read(self, mode, &logical_inclusive_or);
            }
            0x24 | 0x2C => instruction_read(self, mode, &logical_bit_test),

            // Shifts
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                instruction_read_move_write(self, mode, &logical_shift_left);
            }
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                instruction_read_move_write(self, mode, &logical_shift_right);
            }
            0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
                instruction_read_move_write(self, mode, &logical_rotate_left);
            }
            0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
                instruction_read_move_write(self, mode, &logical_rotate_right);
            }

            // Arithmetic
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                instruction_read(self, mode, &add_with_carry);
            }
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                instruction_read(self, mode, &sub_with_carry);
            }
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                instruction_read(self, mode, &compare_ac);
            }
            0xE0 | 0xE4 | 0xEC => {
                instruction_read(self, mode, &compare_ix);
            }
            0xC0 | 0xC4 | 0xCC => {
                instruction_read(self, mode, &compare_iy);
            }

            // Increment & Decrement Instructions
            0xE6 | 0xF6 | 0xEE | 0xFE => instruction_read_move_write(self, mode, &inc_memory),
            0xE8 => {
                instruction_implied(self, &inc_ix);
            }
            0xC8 => {
                instruction_implied(self, &inc_iy);
            }

            0xC6 | 0xD6 | 0xCE | 0xDE => {
                instruction_read_move_write(self, mode, &dec_memory);
            }
            0xCA => {
                instruction_implied(self, &dec_ix);
            }
            0x88 => {
                instruction_implied(self, &dec_iy);
            }

            // Load/Store instructions
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                instruction_read(self, mode, &load_ac);
            }

            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                instruction_read(self, mode, &load_ix);
            }

            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                instruction_read(self, mode, &load_iy);
            }

            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                instruction_write(self, mode, &store_ac);
            }

            0x86 | 0x96 | 0x8E => {
                instruction_write(self, mode, &store_ix);
            }

            0x84 | 0x94 | 0x8C => {
                instruction_write(self, mode, &store_iy);
            }

            // Transfer instructions
            0xAA => instruction_implied(self, &transfer_ac_to_x),
            0xA8 => instruction_implied(self, &transfer_ac_to_y),
            0x8A => instruction_implied(self, &transfer_x_to_ac),
            0x98 => instruction_implied(self, &transfer_y_to_ac),

            // Jumps & Calls
            0x4C => jmp_absolute(self),
            0x6C => jmp_indirect(self),
            0x20 => instruction_implied(self, &jump_to_subroutine),
            0x60 => instruction_implied(self, &return_from_subroutine),

            // Branches
            0x90 => instruction_read(self, mode, &branch_if_carry_clear),
            0xB0 => instruction_read(self, mode, &branch_if_carry_set),
            0xF0 => instruction_read(self, mode, &branch_if_equal),
            0xD0 => instruction_read(self, mode, &branch_if_not_equal),
            0x30 => instruction_read(self, mode, &branch_if_minus),
            0x10 => instruction_read(self, mode, &branch_if_positive),
            0x50 => instruction_read(self, mode, &branch_if_overflow_clear),
            0x70 => instruction_read(self, mode, &branch_if_overflow_set),

            // Stack instructions
            0xBA => instruction_implied(self, &transfer_sp_to_x),
            0x9A => instruction_implied(self, &transfer_x_to_sp),
            0x48 => instruction_implied(self, &push_ac),
            0x08 => instruction_implied(self, &push_processor),
            0x68 => instruction_implied(self, &pull_ac),
            0x28 => instruction_implied(self, &pull_processor_status),

            // Status Flags
            0x18 => instruction_implied(self, &clear_carry),
            0xD8 => instruction_implied(self, &clear_decimal),
            0x58 => instruction_implied(self, &clear_int_disable),
            0xB8 => instruction_implied(self, &clear_overflow),
            0x38 => instruction_implied(self, &set_carry),
            0xF8 => instruction_implied(self, &set_decimal),
            0x78 => instruction_implied(self, &set_int_disable),

            _ => {
                panic!("Illegal instruction {:x} at {:x}", opcode, self.reg.pc - 1);
            }
        };
    }
}
