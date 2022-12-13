use std::fmt::{Display, Formatter};

use crate::gameboy::cpu::interrupt::Interrupt;
use crate::gameboy::memory::memory;
use crate::gameboy::memory::memory::Memory;

pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    accumulator: usize,
    divider_accumulator: usize,
}

const CLOCKS_PER_DIVIDER_INC: usize = 256;

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0x18,
            tima: 0x00,
            tma: 0x00,
            tac: 0xF8,
            accumulator: 0,
            divider_accumulator: 0,
        }
    }

    pub fn step(&mut self, if_register: u8) -> Option<u8> {
        self.step_divider();
        self.step_timer(if_register)
    }

    fn step_divider(&mut self) {
        self.divider_accumulator += 4;
        if self.divider_accumulator >= CLOCKS_PER_DIVIDER_INC {
            self.divider_accumulator -= CLOCKS_PER_DIVIDER_INC;
            self.div = self.div.wrapping_add(1);
        }
    }

    fn step_timer(&mut self, if_register: u8) -> Option<u8> {
        if (self.tac & 0b100) > 0 {
            self.accumulator += 4;
            let required_clocks = self.required_clocks();
            if self.accumulator >= required_clocks {
                self.accumulator -= required_clocks;
                let timer = self.tima;
                match timer.checked_add(1) {
                    None => {
                        self.tima = self.tma;
                        return Option::from(if_register | (1 << Interrupt::Timer.bit_number()));
                    }
                    Some(result) => {
                        self.tima = result;
                    }
                }
            }
        }
        None
    }

    fn required_clocks(&self) -> usize {
        match self.tac {
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => 1024,
        }
    }
}

impl Memory for Timer {
    fn accepts_address(&self, address: u16) -> bool {
        address == memory::DIV
            || address == memory::TIMA
            || address == memory::TMA
            || address == memory::TAC
    }

    fn read_byte(&self, address: u16) -> u8 {
        match address {
            memory::DIV => self.div,
            memory::TIMA => self.tima,
            memory::TMA => self.tma,
            memory::TAC => self.tac,
            _ => {
                panic!("Invalid address: {}", address)
            }
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            memory::DIV => {
                self.div = 0;
            }
            memory::TIMA => {
                self.tima = value;
            }
            memory::TMA => {
                self.tma = value;
            }
            memory::TAC => {
                self.tac = value;
            }
            _ => {
                panic!("Invalid address: {}", address)
            }
        }
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timer")
    }
}
