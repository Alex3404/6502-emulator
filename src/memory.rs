use rand::prelude::Rng;
use std::{fs::File, io::Read};

const MEMORY_SIZE: usize = (u16::MAX as usize) + 1;

pub trait MemoryBus {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

struct MemoryBank {
    bytes: [u8; MEMORY_SIZE],
}

impl MemoryBus for MemoryBank {
    fn read(&mut self, address: u16) -> u8 {
        self.bytes[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.bytes[address as usize] = value;
    }
}

impl MemoryBank {
    pub fn randomize_memory(&mut self) {
        for value in self.bytes.iter_mut() {
            *value = rand::thread_rng().gen();
        }
    }

    pub fn new() -> Self {
        let bytes = [0_u8; MEMORY_SIZE];
        Self { bytes }
    }
}

pub fn memory_from_file(file: &mut File, randomize_unfilled_bytes: bool) -> impl MemoryBus {
    let mut memory_bank = MemoryBank::new();
    if randomize_unfilled_bytes {
        memory_bank.randomize_memory();
    }
    let _ = file.read(&mut memory_bank.bytes);
    memory_bank
}
