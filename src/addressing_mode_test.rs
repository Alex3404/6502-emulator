#[allow(unused_imports)]
use crate::emulator::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn addressing_mode_test() {
        let mut op_codes_to_addressing_mode: HashMap<u8, AddressingMode> = HashMap::new();

        op_codes_to_addressing_mode.insert(0x69, AddressingMode::Immediate);
        op_codes_to_addressing_mode.insert(0x65, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0x75, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0x6D, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0x7D, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0x79, AddressingMode::AbsoluteY);
        op_codes_to_addressing_mode.insert(0x61, AddressingMode::IndirectX);
        op_codes_to_addressing_mode.insert(0x71, AddressingMode::IndirectY);
        op_codes_to_addressing_mode.insert(0x29, AddressingMode::Immediate);
        op_codes_to_addressing_mode.insert(0x25, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0x35, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0x2D, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0x3D, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0x39, AddressingMode::AbsoluteY);
        op_codes_to_addressing_mode.insert(0x21, AddressingMode::IndirectX);
        op_codes_to_addressing_mode.insert(0x31, AddressingMode::IndirectY);
        op_codes_to_addressing_mode.insert(0x0A, AddressingMode::Accumulator);
        op_codes_to_addressing_mode.insert(0x06, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0x16, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0x0E, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0x1E, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0x90, AddressingMode::Relative);
        op_codes_to_addressing_mode.insert(0xB0, AddressingMode::Relative);
        op_codes_to_addressing_mode.insert(0xF0, AddressingMode::Relative);
        op_codes_to_addressing_mode.insert(0x24, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0x2C, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0x30, AddressingMode::Relative);
        op_codes_to_addressing_mode.insert(0xD0, AddressingMode::Relative);
        op_codes_to_addressing_mode.insert(0x10, AddressingMode::Relative);
        op_codes_to_addressing_mode.insert(0x00, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x50, AddressingMode::Relative);
        op_codes_to_addressing_mode.insert(0x70, AddressingMode::Relative);
        op_codes_to_addressing_mode.insert(0x18, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0xD8, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x58, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0xB8, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0xC9, AddressingMode::Immediate);
        op_codes_to_addressing_mode.insert(0xC5, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0xD5, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0xCD, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0xDD, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0xD9, AddressingMode::AbsoluteY);
        op_codes_to_addressing_mode.insert(0xC1, AddressingMode::IndirectX);
        op_codes_to_addressing_mode.insert(0xD1, AddressingMode::IndirectY);
        op_codes_to_addressing_mode.insert(0xE0, AddressingMode::Immediate);
        op_codes_to_addressing_mode.insert(0xE4, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0xEC, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0xC0, AddressingMode::Immediate);
        op_codes_to_addressing_mode.insert(0xC4, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0xCC, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0xC6, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0xD6, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0xCE, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0xDE, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0xCA, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x88, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x49, AddressingMode::Immediate);
        op_codes_to_addressing_mode.insert(0x45, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0x55, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0x4D, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0x5D, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0x59, AddressingMode::AbsoluteY);
        op_codes_to_addressing_mode.insert(0x41, AddressingMode::IndirectX);
        op_codes_to_addressing_mode.insert(0x51, AddressingMode::IndirectY);
        op_codes_to_addressing_mode.insert(0xE6, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0xF6, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0xEE, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0xFE, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0xE8, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0xC8, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x4C, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0x6C, AddressingMode::Indirect);
        op_codes_to_addressing_mode.insert(0x20, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0xA9, AddressingMode::Immediate);
        op_codes_to_addressing_mode.insert(0xA5, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0xB5, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0xAD, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0xBD, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0xB9, AddressingMode::AbsoluteY);
        op_codes_to_addressing_mode.insert(0xA1, AddressingMode::IndirectX);
        op_codes_to_addressing_mode.insert(0xB1, AddressingMode::IndirectY);
        op_codes_to_addressing_mode.insert(0xA2, AddressingMode::Immediate);
        op_codes_to_addressing_mode.insert(0xA6, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0xB6, AddressingMode::ZeroPageY);
        op_codes_to_addressing_mode.insert(0xAE, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0xBE, AddressingMode::AbsoluteY);
        op_codes_to_addressing_mode.insert(0xA0, AddressingMode::Immediate);
        op_codes_to_addressing_mode.insert(0xA4, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0xB4, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0xAC, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0xBC, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0x4A, AddressingMode::Accumulator);
        op_codes_to_addressing_mode.insert(0x46, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0x56, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0x4E, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0x5E, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0xEA, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x09, AddressingMode::Immediate);
        op_codes_to_addressing_mode.insert(0x05, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0x15, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0x0D, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0x1D, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0x19, AddressingMode::AbsoluteY);
        op_codes_to_addressing_mode.insert(0x01, AddressingMode::IndirectX);
        op_codes_to_addressing_mode.insert(0x11, AddressingMode::IndirectY);
        op_codes_to_addressing_mode.insert(0x48, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x08, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x68, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x28, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x2A, AddressingMode::Accumulator);
        op_codes_to_addressing_mode.insert(0x26, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0x36, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0x2E, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0x3E, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0x6A, AddressingMode::Accumulator);
        op_codes_to_addressing_mode.insert(0x66, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0x76, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0x6E, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0x7E, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0x40, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x60, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0xE9, AddressingMode::Immediate);
        op_codes_to_addressing_mode.insert(0xE5, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0xF5, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0xED, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0xFD, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0xF9, AddressingMode::AbsoluteY);
        op_codes_to_addressing_mode.insert(0xE1, AddressingMode::IndirectX);
        op_codes_to_addressing_mode.insert(0xF1, AddressingMode::IndirectY);
        op_codes_to_addressing_mode.insert(0x38, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0xF8, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x78, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x85, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0x95, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0x8D, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0x9D, AddressingMode::AbsoluteX);
        op_codes_to_addressing_mode.insert(0x99, AddressingMode::AbsoluteY);
        op_codes_to_addressing_mode.insert(0x81, AddressingMode::IndirectX);
        op_codes_to_addressing_mode.insert(0x91, AddressingMode::IndirectY);
        op_codes_to_addressing_mode.insert(0x86, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0x96, AddressingMode::ZeroPageY);
        op_codes_to_addressing_mode.insert(0x8E, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0x84, AddressingMode::ZeroPage);
        op_codes_to_addressing_mode.insert(0x94, AddressingMode::ZeroPageX);
        op_codes_to_addressing_mode.insert(0x8C, AddressingMode::Absolute);
        op_codes_to_addressing_mode.insert(0xAA, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0xA8, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0xBA, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x8A, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x9A, AddressingMode::Implied);
        op_codes_to_addressing_mode.insert(0x98, AddressingMode::Implied);

        for (key, value) in op_codes_to_addressing_mode.into_iter() {
            let got = get_addressing_mode(key);
            if got != value {
                println!(
                    "Invalid addressing mode for {:x} Expected: {:?}, Got: {:?}",
                    key, value, got
                );
            }
        }
    }
}
