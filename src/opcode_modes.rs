use crate::emulator::*;

pub fn implied_1read(context: &mut MOS6502) {
    context.read(context.reg.pc);
    context.tick();
}

pub fn immediate_1read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let value = context.read(context.reg.pc);
    context.reg.pc += 1;
    func(context, value);
    context.tick();
}

pub fn absolute_3read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.read(context.reg.pc) as u16) << 8;
    context.reg.pc += 1;
    context.tick();

    // T3
    let value = context.read(address);
    func(context, value);
    context.tick();
}

pub fn absolute_3write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.read(context.reg.pc) as u16) << 8;
    context.reg.pc += 1;
    context.tick();

    // T3
    let value = func(context);
    context.write(address, value);
    context.tick();
}

pub fn zeropage_2read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let value = context.read(address);
    func(context, value); // Perform the operation
    context.tick();
}

pub fn zeropage_2write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let value = func(context);
    context.write(address, value);
    context.tick();
}

pub fn absolutex_3read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.read(context.reg.pc) as u16) << 8;
    let address_x = address + context.reg.ix as u16;
    context.reg.pc += 1;
    context.tick();

    // T3
    if !same_page(address, address_x) {
        context.read(address_x - 0x100); // Perform buggy read in previous page
        context.tick();
    }

    // T3 or T4
    let value = context.read(address_x);
    func(context, value);
    context.tick();
}

pub fn absolutex_4write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.read(context.reg.pc) as u16) << 8;
    let address_x = address + context.reg.ix as u16;
    context.reg.pc += 1;
    context.tick();

    // T3
    context.read((address & 0xFF00) | (address_x & 0x00FF));
    context.tick();

    // T4
    let value = func(context);
    context.write(address, value);
    context.tick();
}

pub fn absolutey_3read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.read(context.reg.pc) as u16) << 8;
    let address_y = address + context.reg.iy as u16;
    context.reg.pc += 1;
    context.tick();

    // T3
    if !same_page(address, address_y) {
        context.read(address_y - 0x100); // Perform buggy read in previous page
        context.tick();
    }

    // T3 or T4
    let value = context.read(address_y);
    func(context, value);
    context.tick();
}

pub fn absolutey_4write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.read(context.reg.pc) as u16) << 8;
    let address_y = address + context.reg.iy as u16;
    context.reg.pc += 1;
    context.tick();

    // T3
    context.read((address & 0xFF00) | (address_y & 0x00FF));
    context.tick();

    // T4
    let value = func(context);
    context.write(address, value);
    context.tick();
}

pub fn zeropagex_3write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    let address_x = (address + context.reg.ix as u16) & 0xFF;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.read(address);
    context.tick();

    // T3
    let value = func(context);
    context.write(address_x, value);
    context.tick();
}

pub fn zeropagex_3read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    let address_x = (address + context.reg.ix as u16) & 0xFF;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.read(address);
    context.tick();

    // T3
    let value = context.read(address_x);
    func(context, value);
    context.tick();
}

pub fn zeropagey_3write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    let address_y = (address + context.reg.iy as u16) & 0xFF;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.read(address);
    context.tick();

    // T3
    let value = func(context);
    context.write(address_y, value);
    context.tick();
}

pub fn zeropagey_3read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    let address_y = (address + context.reg.iy as u16) & 0xFF;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.read(address);
    context.tick();

    // T3
    let value = context.read(address_y);
    func(context, value);
    context.tick();
}

pub fn indirecty_5read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let indirect_address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = context.read(indirect_address) as u16;
    context.tick();

    // T3
    let address = address | ((context.read(indirect_address + 1) as u16) << 8);
    let address_y = address + context.reg.iy as u16;
    context.tick();

    // T4
    if !same_page(address, address_y) {
        context.read((address & 0xFF00) | (address_y | 0x00FF));
        context.tick();
    }

    // T4 or T5
    let value = context.read(address_y);
    func(context, value);
    context.tick();
}

pub fn indirecty_5write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let indirect_address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = context.read(indirect_address) as u16;
    context.tick();

    // T3
    let address = address | ((context.read(indirect_address + 1) as u16) << 8);
    let address_y = address + context.reg.iy as u16;
    context.tick();

    // T4
    context.read((address & 0xFF00) | (address_y | 0x00FF));
    context.tick();

    // T5
    let value = func(context);
    context.write(address_y, value);
    context.tick();
}

pub fn indirectx_5read(context: &mut MOS6502, func: &ReadInst) {
    // T1
    let indirect_address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.read(indirect_address);
    context.tick();

    // T3
    let address = context.read((indirect_address + context.reg.ix as u16) & 0xFF) as u16;
    context.tick();

    // T4
    let address =
        address | context.read((indirect_address + context.reg.ix as u16 + 1) & 0xFF) as u16;
    context.tick();

    // T5
    let value = context.read(address);
    func(context, value);
    context.tick();
}

pub fn indirectx_5write(context: &mut MOS6502, func: &WriteInst) {
    // T1
    let indirect_address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.read(indirect_address);
    context.tick();

    // T3
    let address = context.read((indirect_address + context.reg.ix as u16) & 0xFF) as u16;
    context.tick();

    // T4
    let address =
        address | context.read((indirect_address + context.reg.ix as u16 + 1) & 0xFF) as u16;
    context.tick();

    // T5
    let value = func(context);
    context.write(address, value);
    context.tick();
}

pub fn ac1_rmw(context: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    context.read(context.reg.pc);
    context.reg.ac = func(context, context.reg.ac);
    context.tick();
}

pub fn zeropage_4rmw(context: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let value = context.read(address);
    context.tick();

    // T3
    context.write(address, value);
    context.tick();

    // T4
    let value = func(context, value);
    context.write(address, value);
    context.tick();
}

pub fn absolute_5rmw(context: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | ((context.read(context.reg.pc) as u16) << 8);
    context.reg.pc += 1;
    context.tick();

    // T3
    let value = context.read(address);
    context.tick();

    // T4
    context.write(address, value);
    context.tick();

    // T5
    let value = func(context, value);
    context.write(address, value);
    context.tick();
}

pub fn zeropagex_5rmw(context: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    let address_x = (address + context.reg.ix as u16) & 0xFF;
    context.reg.pc += 1;
    context.tick();

    // T2
    context.read(address);
    context.tick();

    // T3
    let value = context.read(address_x);
    context.tick();

    // T4
    context.write(address, value);
    context.tick();

    // T5
    let value = func(context, value);
    context.write(address, value);
    context.tick();
}

pub fn absolutex_6rmw(context: &mut MOS6502, func: &ReadWriteInst) {
    // T1
    let address = context.read(context.reg.pc) as u16;
    context.reg.pc += 1;
    context.tick();

    // T2
    let address = address | (context.read(context.reg.pc) as u16) << 8;
    let address = address + context.reg.ix as u16;
    context.reg.pc += 1;
    context.tick();

    // T3
    context.read(address);
    context.tick();

    // T4
    let value = context.read(address);
    context.tick();

    // T5
    context.write(address, value);
    context.tick();

    // T6
    let value = func(context, value);
    context.write(address, value);
    context.tick();
}
