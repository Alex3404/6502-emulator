use crate::{address_bus::AddressBus, cpu::opcode_modes::AddressingMode};
use phf::phf_map;

struct InstructionData<'a> {
    instruction: &'a str,
    mode: AddressingMode,
}

impl InstructionData<'static> {
    const fn new(name: &'static str, mode: AddressingMode) -> Self {
        Self {
            instruction: name,
            mode,
        }
    }
}

static INSTRUCTIONS: phf::Map<u8, InstructionData<'static>> = phf_map! {
    0x69_u8 => InstructionData::new("ADC", AddressingMode::Immediate),
    0x65_u8 => InstructionData::new("ADC", AddressingMode::ZeroPage),
    0x75_u8 => InstructionData::new("ADC", AddressingMode::ZeroPageX),
    0x6D_u8 => InstructionData::new("ADC", AddressingMode::Absolute),
    0x7D_u8 => InstructionData::new("ADC", AddressingMode::AbsoluteX),
    0x79_u8 => InstructionData::new("ADC", AddressingMode::AbsoluteY),
    0x61_u8 => InstructionData::new("ADC", AddressingMode::IndirectX),
    0x71_u8 => InstructionData::new("ADC", AddressingMode::IndirectY),
    0x29_u8 => InstructionData::new("AND", AddressingMode::Immediate),
    0x25_u8 => InstructionData::new("AND", AddressingMode::ZeroPage),
    0x35_u8 => InstructionData::new("AND", AddressingMode::ZeroPageX),
    0x2D_u8 => InstructionData::new("AND", AddressingMode::Absolute),
    0x3D_u8 => InstructionData::new("AND", AddressingMode::AbsoluteX),
    0x39_u8 => InstructionData::new("AND", AddressingMode::AbsoluteY),
    0x21_u8 => InstructionData::new("AND", AddressingMode::IndirectX),
    0x31_u8 => InstructionData::new("AND", AddressingMode::IndirectY),
    0x0A_u8 => InstructionData::new("ASL", AddressingMode::Accumulator),
    0x06_u8 => InstructionData::new("ASL", AddressingMode::ZeroPage),
    0x16_u8 => InstructionData::new("ASL", AddressingMode::ZeroPageX),
    0x0E_u8 => InstructionData::new("ASL", AddressingMode::Absolute),
    0x1E_u8 => InstructionData::new("ASL", AddressingMode::AbsoluteX),
    0x90_u8 => InstructionData::new("BCC", AddressingMode::Relative),
    0xB0_u8 => InstructionData::new("BCS", AddressingMode::Relative),
    0xF0_u8 => InstructionData::new("BEQ", AddressingMode::Relative),
    0x24_u8 => InstructionData::new("BIT", AddressingMode::ZeroPage),
    0x2C_u8 => InstructionData::new("BIT", AddressingMode::Absolute),
    0x30_u8 => InstructionData::new("BMI", AddressingMode::Relative),
    0xD0_u8 => InstructionData::new("BNE", AddressingMode::Relative),
    0x10_u8 => InstructionData::new("BPL", AddressingMode::Relative),
    0x00_u8 => InstructionData::new("BRK", AddressingMode::Implied),
    0x50_u8 => InstructionData::new("BVC", AddressingMode::Relative),
    0x70_u8 => InstructionData::new("BVS", AddressingMode::Relative),
    0x18_u8 => InstructionData::new("CLC", AddressingMode::Implied),
    0xD8_u8 => InstructionData::new("CLD", AddressingMode::Implied),
    0x58_u8 => InstructionData::new("CLI", AddressingMode::Implied),
    0xB8_u8 => InstructionData::new("CLV", AddressingMode::Implied),
    0xC9_u8 => InstructionData::new("CMP", AddressingMode::Immediate),
    0xC5_u8 => InstructionData::new("CMP", AddressingMode::ZeroPage),
    0xD5_u8 => InstructionData::new("CMP", AddressingMode::ZeroPageX),
    0xCD_u8 => InstructionData::new("CMP", AddressingMode::Absolute),
    0xDD_u8 => InstructionData::new("CMP", AddressingMode::AbsoluteX),
    0xD9_u8 => InstructionData::new("CMP", AddressingMode::AbsoluteY),
    0xC1_u8 => InstructionData::new("CMP", AddressingMode::IndirectX),
    0xD1_u8 => InstructionData::new("CMP", AddressingMode::IndirectY),
    0xE0_u8 => InstructionData::new("CPX", AddressingMode::Immediate),
    0xE4_u8 => InstructionData::new("CPX", AddressingMode::ZeroPage),
    0xEC_u8 => InstructionData::new("CPX", AddressingMode::Absolute),
    0xC0_u8 => InstructionData::new("CPY", AddressingMode::Immediate),
    0xC4_u8 => InstructionData::new("CPY", AddressingMode::ZeroPage),
    0xCC_u8 => InstructionData::new("CPY", AddressingMode::Absolute),
    0xC6_u8 => InstructionData::new("DEC", AddressingMode::ZeroPage),
    0xD6_u8 => InstructionData::new("DEC", AddressingMode::ZeroPageX),
    0xCE_u8 => InstructionData::new("DEC", AddressingMode::Absolute),
    0xDE_u8 => InstructionData::new("DEC", AddressingMode::AbsoluteX),
    0xCA_u8 => InstructionData::new("DEX", AddressingMode::Implied),
    0x88_u8 => InstructionData::new("DEY", AddressingMode::Implied),
    0x49_u8 => InstructionData::new("EOR", AddressingMode::Immediate),
    0x45_u8 => InstructionData::new("EOR", AddressingMode::ZeroPage),
    0x55_u8 => InstructionData::new("EOR", AddressingMode::ZeroPageX),
    0x4D_u8 => InstructionData::new("EOR", AddressingMode::Absolute),
    0x5D_u8 => InstructionData::new("EOR", AddressingMode::AbsoluteX),
    0x59_u8 => InstructionData::new("EOR", AddressingMode::AbsoluteY),
    0x41_u8 => InstructionData::new("EOR", AddressingMode::IndirectX),
    0x51_u8 => InstructionData::new("EOR", AddressingMode::IndirectY),
    0xE6_u8 => InstructionData::new("INC", AddressingMode::ZeroPage),
    0xF6_u8 => InstructionData::new("INC", AddressingMode::ZeroPageX),
    0xEE_u8 => InstructionData::new("INC", AddressingMode::Absolute),
    0xFE_u8 => InstructionData::new("INC", AddressingMode::AbsoluteX),
    0xE8_u8 => InstructionData::new("INX", AddressingMode::Implied),
    0xC8_u8 => InstructionData::new("INY", AddressingMode::Implied),
    0x4C_u8 => InstructionData::new("JMP", AddressingMode::Absolute),
    0x6C_u8 => InstructionData::new("JMP", AddressingMode::Indirect),
    0x20_u8 => InstructionData::new("JSR", AddressingMode::Absolute),
    0xA9_u8 => InstructionData::new("LDA", AddressingMode::Immediate),
    0xA5_u8 => InstructionData::new("LDA", AddressingMode::ZeroPage),
    0xB5_u8 => InstructionData::new("LDA", AddressingMode::ZeroPageX),
    0xAD_u8 => InstructionData::new("LDA", AddressingMode::Absolute),
    0xBD_u8 => InstructionData::new("LDA", AddressingMode::AbsoluteX),
    0xB9_u8 => InstructionData::new("LDA", AddressingMode::AbsoluteY),
    0xA1_u8 => InstructionData::new("LDA", AddressingMode::IndirectX),
    0xB1_u8 => InstructionData::new("LDA", AddressingMode::IndirectY),
    0xA2_u8 => InstructionData::new("LDX", AddressingMode::Immediate),
    0xA6_u8 => InstructionData::new("LDX", AddressingMode::ZeroPage),
    0xB6_u8 => InstructionData::new("LDX", AddressingMode::ZeroPageY),
    0xAE_u8 => InstructionData::new("LDX", AddressingMode::Absolute),
    0xBE_u8 => InstructionData::new("LDX", AddressingMode::AbsoluteY),
    0xA0_u8 => InstructionData::new("LDY", AddressingMode::Immediate),
    0xA4_u8 => InstructionData::new("LDY", AddressingMode::ZeroPage),
    0xB4_u8 => InstructionData::new("LDY", AddressingMode::ZeroPageX),
    0xAC_u8 => InstructionData::new("LDY", AddressingMode::Absolute),
    0xBC_u8 => InstructionData::new("LDY", AddressingMode::AbsoluteX),
    0x4A_u8 => InstructionData::new("LSR", AddressingMode::Accumulator),
    0x46_u8 => InstructionData::new("LSR", AddressingMode::ZeroPage),
    0x56_u8 => InstructionData::new("LSR", AddressingMode::ZeroPageX),
    0x4E_u8 => InstructionData::new("LSR", AddressingMode::Absolute),
    0x5E_u8 => InstructionData::new("LSR", AddressingMode::AbsoluteX),
    0xEA_u8 => InstructionData::new("NOP", AddressingMode::Implied),
    0x09_u8 => InstructionData::new("ORA", AddressingMode::Immediate),
    0x05_u8 => InstructionData::new("ORA", AddressingMode::ZeroPage),
    0x15_u8 => InstructionData::new("ORA", AddressingMode::ZeroPageX),
    0x0D_u8 => InstructionData::new("ORA", AddressingMode::Absolute),
    0x1D_u8 => InstructionData::new("ORA", AddressingMode::AbsoluteX),
    0x19_u8 => InstructionData::new("ORA", AddressingMode::AbsoluteY),
    0x01_u8 => InstructionData::new("ORA", AddressingMode::IndirectX),
    0x11_u8 => InstructionData::new("ORA", AddressingMode::IndirectY),
    0x48_u8 => InstructionData::new("PHA", AddressingMode::Implied),
    0x08_u8 => InstructionData::new("PHP", AddressingMode::Implied),
    0x68_u8 => InstructionData::new("PLA", AddressingMode::Implied),
    0x28_u8 => InstructionData::new("PLP", AddressingMode::Implied),
    0x2A_u8 => InstructionData::new("ROL", AddressingMode::Accumulator),
    0x26_u8 => InstructionData::new("ROL", AddressingMode::ZeroPage),
    0x36_u8 => InstructionData::new("ROL", AddressingMode::ZeroPageX),
    0x2E_u8 => InstructionData::new("ROL", AddressingMode::Absolute),
    0x3E_u8 => InstructionData::new("ROL", AddressingMode::AbsoluteX),
    0x6A_u8 => InstructionData::new("ROR", AddressingMode::Accumulator),
    0x66_u8 => InstructionData::new("ROR", AddressingMode::ZeroPage),
    0x76_u8 => InstructionData::new("ROR", AddressingMode::ZeroPageX),
    0x6E_u8 => InstructionData::new("ROR", AddressingMode::Absolute),
    0x7E_u8 => InstructionData::new("ROR", AddressingMode::AbsoluteX),
    0x40_u8 => InstructionData::new("RTI", AddressingMode::Implied),
    0x60_u8 => InstructionData::new("RTS", AddressingMode::Implied),
    0xE9_u8 => InstructionData::new("SBC", AddressingMode::Immediate),
    0xE5_u8 => InstructionData::new("SBC", AddressingMode::ZeroPage),
    0xF5_u8 => InstructionData::new("SBC", AddressingMode::ZeroPageX),
    0xED_u8 => InstructionData::new("SBC", AddressingMode::Absolute),
    0xFD_u8 => InstructionData::new("SBC", AddressingMode::AbsoluteX),
    0xF9_u8 => InstructionData::new("SBC", AddressingMode::AbsoluteY),
    0xE1_u8 => InstructionData::new("SBC", AddressingMode::IndirectX),
    0xF1_u8 => InstructionData::new("SBC", AddressingMode::IndirectY),
    0x38_u8 => InstructionData::new("SEC", AddressingMode::Implied),
    0xF8_u8 => InstructionData::new("SED", AddressingMode::Implied),
    0x78_u8 => InstructionData::new("SEI", AddressingMode::Implied),
    0x85_u8 => InstructionData::new("STA", AddressingMode::ZeroPage),
    0x95_u8 => InstructionData::new("STA", AddressingMode::ZeroPageX),
    0x8D_u8 => InstructionData::new("STA", AddressingMode::Absolute),
    0x9D_u8 => InstructionData::new("STA", AddressingMode::AbsoluteX),
    0x99_u8 => InstructionData::new("STA", AddressingMode::AbsoluteY),
    0x81_u8 => InstructionData::new("STA", AddressingMode::IndirectX),
    0x91_u8 => InstructionData::new("STA", AddressingMode::IndirectY),
    0x86_u8 => InstructionData::new("STX", AddressingMode::ZeroPage),
    0x96_u8 => InstructionData::new("STX", AddressingMode::ZeroPageY),
    0x8E_u8 => InstructionData::new("STX", AddressingMode::Absolute),
    0x84_u8 => InstructionData::new("STY", AddressingMode::ZeroPage),
    0x94_u8 => InstructionData::new("STY", AddressingMode::ZeroPageX),
    0x8C_u8 => InstructionData::new("STY", AddressingMode::Absolute),
    0xAA_u8 => InstructionData::new("TAX", AddressingMode::Implied),
    0xA8_u8 => InstructionData::new("TAY", AddressingMode::Implied),
    0xBA_u8 => InstructionData::new("TSX", AddressingMode::Implied),
    0x8A_u8 => InstructionData::new("TXA", AddressingMode::Implied),
    0x9A_u8 => InstructionData::new("TXS", AddressingMode::Implied),
    0x98_u8 => InstructionData::new("TYA", AddressingMode::Implied),
};

fn instruction_length(mode: AddressingMode) -> usize {
    match mode {
        AddressingMode::Implied => 1,
        AddressingMode::Accumulator => 1,

        AddressingMode::Immediate => 2,
        AddressingMode::IndirectX => 2,
        AddressingMode::IndirectY => 2,
        AddressingMode::Relative => 2,
        AddressingMode::ZeroPage => 2,
        AddressingMode::ZeroPageX => 2,
        AddressingMode::ZeroPageY => 2,

        AddressingMode::Absolute => 3,
        AddressingMode::AbsoluteX => 3,
        AddressingMode::AbsoluteY => 3,
        AddressingMode::Indirect => 3,
    }
}

pub fn decode_paramaters(
    memory: &mut Box<dyn AddressBus>,
    mode: AddressingMode,
    address: u16,
) -> String {
    match mode {
        AddressingMode::Implied => String::from(""),
        AddressingMode::Accumulator => String::from(""),

        AddressingMode::Immediate => {
            format!("#${:02X}", memory.read(address + 1))
        }
        AddressingMode::IndirectX => {
            format!("(${:02X},X)", memory.read(address + 1))
        }
        AddressingMode::IndirectY => {
            format!("(${:02X}),Y", memory.read(address + 1))
        }
        AddressingMode::Relative => {
            let relative_offset = memory.read(address + 1);
            let offset = i16::from(relative_offset as i8) + 2;
            match offset {
                0 => {
                    format!("*    ; ${:04X}", address)
                }
                d if d < 0 => {
                    let value = !(relative_offset - 1);
                    let address = unsafe { address.unchecked_sub(value as u16) };
                    format!("*-${:X}    ; ${:04X}", value, address)
                }
                d if d > 0 => {
                    let address = unsafe { address.unchecked_add(relative_offset as u16) };
                    format!("*+${:X}    ; ${:04X}", relative_offset, address)
                }
                _ => panic!("Unreachable!"),
            }
        }
        AddressingMode::ZeroPage => {
            format!("${:X}", memory.read(address + 1))
        }
        AddressingMode::ZeroPageX => {
            format!("${:X},X", memory.read(address + 1))
        }
        AddressingMode::ZeroPageY => {
            format!("${:X},Y", memory.read(address + 1))
        }

        AddressingMode::Absolute => {
            format!(
                "${:X}",
                memory.read(address + 1) as u16 | ((memory.read(address + 2) as u16) << 8)
            )
        }
        AddressingMode::AbsoluteX => {
            format!(
                "${:X},X",
                memory.read(address + 1) as u16 | ((memory.read(address + 2) as u16) << 8)
            )
        }
        AddressingMode::AbsoluteY => {
            format!(
                "${:X},Y",
                memory.read(address + 1) as u16 | ((memory.read(address + 2) as u16) << 8)
            )
        }
        AddressingMode::Indirect => {
            format!(
                "$({:X})",
                memory.read(address + 1) as u16 | ((memory.read(address + 2) as u16) << 8)
            )
        }
    }
}

pub fn disassemble_instruction(memory: &mut Box<dyn AddressBus>, address: u16) -> Option<String> {
    let opcode = memory.read(address);
    let instruction_data = match INSTRUCTIONS.get(&opcode) {
        Some(data) => data,
        None => {
            return None;
        }
    };

    let mode = instruction_data.mode;
    let mut instruction = String::from(instruction_data.instruction);

    instruction += " ";
    instruction += &decode_paramaters(memory, mode, address);

    let mut byte_column = format!("${:04X} | ", address);
    for i in 0..instruction_length(mode) {
        byte_column += format!("{:02X} ", memory.read(address + i as u16)).as_str();
    }
    while byte_column.len() < 17 {
        byte_column.push(' ');
    }
    byte_column += "| ";
    byte_column += instruction.as_str();

    Some(byte_column)
}
