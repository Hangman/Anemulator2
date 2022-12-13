use std::fmt::{Display, Formatter};

use crate::gameboy::ram::memory;
use crate::gameboy::ram::memory::Memory;

pub struct InterruptRegisters {
    if_reg: u8,
    ie_reg: u8,
}

impl InterruptRegisters {
    pub fn new() -> Self {
        Self {
            if_reg: 0,
            ie_reg: 0,
        }
    }
}

impl Memory for InterruptRegisters {
    fn accepts_address(&self, address: u16) -> bool {
        address == memory::IE || address == memory::IF
    }

    fn read_byte(&self, address: u16) -> u8 {
        match address {
            memory::IE => self.ie_reg,
            memory::IF => self.if_reg,
            _ => {
                panic!("Invalid memory address: {}", address)
            }
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            memory::IE => {
                self.ie_reg = value;
            }
            memory::IF => {
                self.if_reg = value | 0xE0;
            }
            _ => {
                panic!("Invalid memory address: {}", address)
            }
        }
    }
}

impl Display for InterruptRegisters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "InterruptRegisters")
    }
}
