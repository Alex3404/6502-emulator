use crate::cpu::*;

pub fn jmp_absolute(context: &mut MOS6502) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | ((context.mem.read(context.reg.pc) as u16) << 8);

    // Checks if we are trapped (For debugging)
    if address + 2 == context.reg.pc {
        panic!("Trapped!");
    }

    context.reg.pc += 1;
    context.reg.pc = address;
    println!("Jmp absolute to {:x}", address);
    context.tick()
}

pub fn jmp_indirect(context: &mut MOS6502) {
    // T1
    let indirect_address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let indirect_address = indirect_address | ((context.mem.read(context.reg.pc) as u16) << 8);
    context.reg.pc += 1;
    context.tick();

    // T3
    let address = context.mem.read(indirect_address) as u16;

    // T4
    // See https://www.nesdev.org/obelisk-6502-guide/reference.html
    // An original 6502 has does not correctly fetch the target address if the indirect
    // vector falls on a page boundary (e.g. $xxFF where xx is any value from $00 to $FF).
    // In this case fetches the LSB from $xxFF as expected but takes the MSB from $xx00.
    // This is fixed in some later chips like the 65SC02 so for compatibility always
    // ensure the indirect vector is not at the end of the page.

    // I'm going to use the 65SC02 version in the sake of simplicity
    let address = address | ((context.mem.read(indirect_address + 1) as u16) << 8);
    context.reg.pc = address;
    context.tick();
}

pub fn jump_to_subroutine(context: &mut MOS6502) {
    // T1
    let address = context.mem.read(context.reg.pc) as u16;
    context.reg.pc += 1; // Only increment by 1 (and the instruction is 3) because we push the next pc - 1
    context.tick();

    // T2
    context.stack_peek();
    context.tick();

    // T3
    context.stack_push((context.reg.pc >> 8) as u8);
    context.tick();

    // T4
    context.stack_push(context.reg.pc as u8);
    context.tick();

    // T5
    let address = address | ((context.mem.read(context.reg.pc) as u16) << 8);
    context.reg.pc = address;
    context.tick();
}

pub fn return_from_subroutine(context: &mut MOS6502) {
    // T1
    context.mem.read(context.reg.pc);
    context.tick();
    // T2
    context.stack_peek();
    context.stack_pop_no_read();
    context.tick();
    // T3
    let address = context.stack_peek() as u16;
    context.stack_pop_no_read();
    context.tick();
    // T4
    let address = address | ((context.stack_peek() as u16) << 8);
    context.tick();
    // T5
    context.mem.read(address);
    context.reg.pc = address + 1;
    context.tick();
}

pub fn return_from_interrupt(context: &mut MOS6502) {
    // T1
    context.stack_peek();
    context.stack_pop_no_read();
    context.tick();
    // T2
    let value = context.stack_peek();
    context.set_proccessor_status(value);
    context.stack_pop_no_read();
    context.tick();
    // T3
    let address = context.stack_peek() as u16;
    context.stack_pop_no_read();
    context.tick();
    // T4
    let address = address | ((context.stack_peek() as u16) << 8);
    context.reg.pc = address;
    context.tick();
}

pub fn break_interrupt(context: &mut MOS6502) {
    // T1
    context.mem.read(context.reg.pc);
    context.reg.pc += 1;
    context.tick();

    // T2
    context.stack_push((context.reg.pc >> 8) as u8);
    context.tick();

    // T3
    context.stack_push(context.reg.pc as u8);
    context.tick();

    // T4
    context.push_processor_status();
    context.flags.break_cmd = true;
    context.flags.int_disable = true;
    context.tick();

    // T5
    let address = context.mem.read(IRQ_VECTOR) as u16;
    context.tick();

    // T6
    let address = address | (context.mem.read(IRQ_VECTOR + 1) as u16) << 8;
    context.reg.pc = address;
    context.tick();
}

pub fn branch(context: &mut MOS6502, relative_offset: u8) {
    // T1
    context.mem.read(context.reg.pc);
    context.tick();

    let old_pc = context.reg.pc;

    let new_pc = if relative_offset & (1 << 7) != 0 {
        let offset = !(relative_offset - 1);
        if offset == 2 {
            panic!("Trapped!");
        }
        unsafe { context.reg.pc.unchecked_sub(offset as u16) }
    } else {
        unsafe { context.reg.pc.unchecked_add(relative_offset as u16) }
    };

    context.reg.pc = new_pc;

    if !same_page(context.reg.pc, new_pc) {
        // T2
        context.mem.read((old_pc & 0xFF00) | (new_pc & 0x00FF));
        context.tick();
    }
}

pub fn branch_if_carry_clear(context: &mut MOS6502, relative_address: u8) {
    if !context.flags.carry {
        branch(context, relative_address);
    }
}

pub fn branch_if_carry_set(context: &mut MOS6502, relative_address: u8) {
    if context.flags.carry {
        branch(context, relative_address);
    }
}

pub fn branch_if_equal(context: &mut MOS6502, relative_address: u8) {
    if context.flags.zero {
        branch(context, relative_address);
    }
}

pub fn branch_if_not_equal(context: &mut MOS6502, relative_address: u8) {
    if !context.flags.zero {
        branch(context, relative_address);
    }
}

pub fn branch_if_minus(context: &mut MOS6502, relative_address: u8) {
    if context.flags.negative {
        branch(context, relative_address);
    }
}

pub fn branch_if_positive(context: &mut MOS6502, relative_address: u8) {
    if !context.flags.negative {
        branch(context, relative_address);
    }
}

pub fn branch_if_overflow_clear(context: &mut MOS6502, relative_address: u8) {
    if !context.flags.overflow {
        branch(context, relative_address);
    }
}

pub fn branch_if_overflow_set(context: &mut MOS6502, relative_address: u8) {
    if context.flags.overflow {
        branch(context, relative_address);
    }
}
