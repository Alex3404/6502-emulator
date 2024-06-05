#[cfg(test)]
mod tests {
    use crate::address_bus;
    use crate::cpu;
    use crate::disassembler;
    use std::collections::VecDeque;
    use std::fs::File;

    #[test]
    fn functional_test() {
        const TEST_FILE_PATH: &str = "tests\\6502_functional_test.bin";
        const TEST_END_PC: u16 = 0x336D;
        const TEST_START_PC: u16 = 0x400;

        let mut file = File::open(TEST_FILE_PATH).unwrap();

        let memory = address_bus::memory_from_file(&mut file, true);
        let mut cpu = cpu::MOS6502::new(Box::new(memory));
        cpu.interrupt(cpu::InterruptType::Reset);
        cpu.set_pc(TEST_START_PC);

        let mut past_registers: VecDeque<cpu::MOS6502Registers> = VecDeque::new();

        while !cpu.is_trapped() {
            cpu.step();
            past_registers.push_back(cpu.reg.clone());
            if past_registers.len() > 64 {
                past_registers.pop_front();
            }
        }

        if cpu.reg.pc != TEST_END_PC {
            for reg in past_registers {
                let disassembly = disassembler::disassemble_instruction(&mut cpu.bus, reg.pc);
                if let Some(str) = disassembly {
                    println!(
                        "A: {:02X} X: {:02X} Y: {:02X} | {}",
                        reg.ac, reg.ix, reg.iy, str
                    )
                }
            }

            panic!("Test Failed!");
        }

        println!("Tested Passed! :D");
    }
}
