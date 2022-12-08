use std::fmt::{Display, Formatter};

use crate::gameboy::ram::memory::Memory;

pub struct RandomAccessMemory {
    pub name: String,
    pub memory: Vec<u8>,
    pub start_address: usize,
}

impl RandomAccessMemory {
    pub fn create(name: &str, start_address: u16, length: usize) -> Self {
        Self {
            name: name.to_string(),
            memory: vec![0; length],
            start_address: start_address as usize,
        }
    }
}

impl Display for RandomAccessMemory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Memory for RandomAccessMemory {
    fn accepts_address(&self, address: u16) -> bool {
        let address_usize = address as usize;
        address_usize >= self.start_address && address_usize < self.start_address + self.memory.len()
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.memory[(address as usize) - self.start_address]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[(address as usize) - self.start_address] = value;
    }
}