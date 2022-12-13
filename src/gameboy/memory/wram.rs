use std::fmt::{Display, Formatter};

use crate::gameboy::memory::memory::Memory;

pub struct Wram {
    memory: Box<[u8; 0xFE00 - 0xC000]>,
}

impl Wram {
    pub fn new() -> Self {
        Self {
            memory: Box::new([0; 0xFE00 - 0xC000]),
        }
    }

    fn write_internal(&mut self, address: u16, value: u8) {
        self.memory[(address - 0xC000) as usize] = value
    }
}

impl Display for Wram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WRAM")
    }
}

impl Memory for Wram {
    fn accepts_address(&self, address: u16) -> bool {
        (0xC000..0xFE00).contains(&address)
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.memory[(address - 0xC000) as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.write_internal(address, value);

        // ECHO
        if (0xC000..=0xDDFF).contains(&address) {
            self.write_internal(address + 0x2000, value);
        } else if address >= 0xE000 {
            self.write_internal(address - 0x2000, value);
        }
    }
}
