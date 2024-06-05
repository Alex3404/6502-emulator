use crate::cpu::*;

pub fn jmp_absolute(cpu: &mut MOS6502) {
    // T1
    let address = cpu.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let address = address | ((cpu.read(cpu.reg.pc) as u16) << 8);

    if address + 2 == cpu.reg.pc {
        cpu.trapped();
    }

    cpu.reg.pc += 1;
    cpu.reg.pc = address;

    cpu.tick()
}

pub fn jmp_indirect(cpu: &mut MOS6502) {
    // T1
    let indirect_address = cpu.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1;
    cpu.tick();

    // T2
    let indirect_address = indirect_address | ((cpu.read(cpu.reg.pc) as u16) << 8);
    cpu.reg.pc += 1;
    cpu.tick();

    // T3
    let address = cpu.read(indirect_address) as u16;

    // T4
    // See https://www.nesdev.org/obelisk-6502-guide/reference.html
    // An original 6502 has does not correctly fetch the target address if the indirect
    // vector falls on a page boundary (e.g. $xxFF where xx is any value from $00 to $FF).
    // In this case fetches the LSB from $xxFF as expected but takes the MSB from $xx00.
    // This is fixed in some later chips like the 65SC02 so for compatibility always
    // ensure the indirect vector is not at the end of the page.

    // I'm going to use the 65SC02 version in the sake of simplicity
    let address = address | ((cpu.read(indirect_address + 1) as u16) << 8);
    cpu.reg.pc = address;
    cpu.tick();
}

pub fn jump_to_subroutine(cpu: &mut MOS6502) {
    // T1
    let address = cpu.read(cpu.reg.pc) as u16;
    cpu.reg.pc += 1; // Only increment by 1 (and the instruction is 3) because we push the next pc - 1
    cpu.tick();

    // T2
    cpu.stack_peek();
    cpu.tick();

    // T3
    cpu.stack_push((cpu.reg.pc >> 8) as u8);
    cpu.tick();

    // T4
    cpu.stack_push(cpu.reg.pc as u8);
    cpu.tick();

    // T5
    let address = address | ((cpu.read(cpu.reg.pc) as u16) << 8);
    cpu.reg.pc = address;
    cpu.tick();
}

pub fn return_from_subroutine(cpu: &mut MOS6502) {
    // T1
    cpu.read(cpu.reg.pc);
    cpu.tick();
    // T2
    cpu.stack_peek();
    cpu.stack_pop_no_read();
    cpu.tick();
    // T3
    let address = cpu.stack_peek() as u16;
    cpu.stack_pop_no_read();
    cpu.tick();
    // T4
    let address = address | ((cpu.stack_peek() as u16) << 8);
    cpu.tick();
    // T5
    cpu.read(address);
    cpu.reg.pc = address + 1;
    cpu.tick();
}

pub fn return_from_interrupt(cpu: &mut MOS6502) {
    // T1
    cpu.stack_peek();
    cpu.stack_pop_no_read();
    cpu.tick();
    // T2
    let value = cpu.stack_peek();
    cpu.set_proccessor_status(value);
    cpu.stack_pop_no_read();
    cpu.tick();
    // T3
    let address = cpu.stack_peek() as u16;
    cpu.stack_pop_no_read();
    cpu.tick();
    // T4
    let address = address | ((cpu.stack_peek() as u16) << 8);
    cpu.reg.pc = address;
    cpu.tick();
}

pub fn branch(cpu: &mut MOS6502, relative_offset: u8) {
    // T1
    cpu.read(cpu.reg.pc);
    cpu.tick();

    let old_pc = cpu.reg.pc;
    let new_pc = if relative_offset & (1 << 7) != 0 {
        let offset = !(relative_offset - 1);
        unsafe { cpu.reg.pc.unchecked_sub(offset as u16) }
    } else {
        unsafe { cpu.reg.pc.unchecked_add(relative_offset as u16) }
    };

    cpu.reg.pc = new_pc;

    if !same_page(cpu.reg.pc, new_pc) {
        // T2
        cpu.read((old_pc & 0xFF00) | (new_pc & 0x00FF));
        cpu.tick();
    }

    if relative_offset == 0xFE
    /* -2 in 2's compilement */
    {
        cpu.trapped();
    }
}

pub fn branch_if_carry_clear(cpu: &mut MOS6502, relative_address: u8) {
    if !cpu.is_set(CPUFLAGS::CARRY) {
        branch(cpu, relative_address);
    }
}

pub fn branch_if_carry_set(cpu: &mut MOS6502, relative_address: u8) {
    if cpu.is_set(CPUFLAGS::CARRY) {
        branch(cpu, relative_address);
    }
}

pub fn branch_if_equal(cpu: &mut MOS6502, relative_address: u8) {
    if cpu.is_set(CPUFLAGS::ZERO) {
        branch(cpu, relative_address);
    }
}

pub fn branch_if_not_equal(cpu: &mut MOS6502, relative_address: u8) {
    if !cpu.is_set(CPUFLAGS::ZERO) {
        branch(cpu, relative_address);
    }
}

pub fn branch_if_minus(cpu: &mut MOS6502, relative_address: u8) {
    if cpu.is_set(CPUFLAGS::NEGATIVE) {
        branch(cpu, relative_address);
    }
}

pub fn branch_if_positive(cpu: &mut MOS6502, relative_address: u8) {
    if !cpu.is_set(CPUFLAGS::NEGATIVE) {
        branch(cpu, relative_address);
    }
}

pub fn branch_if_overflow_clear(cpu: &mut MOS6502, relative_address: u8) {
    if !cpu.is_set(CPUFLAGS::OVERFLOW) {
        branch(cpu, relative_address);
    }
}

pub fn branch_if_overflow_set(cpu: &mut MOS6502, relative_address: u8) {
    if cpu.is_set(CPUFLAGS::OVERFLOW) {
        branch(cpu, relative_address);
    }
}
