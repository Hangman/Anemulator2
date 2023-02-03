use std::fmt::{Display, Formatter};

use crate::gameboy::memory::memory;
use crate::gameboy::memory::memory::Memory;

pub struct Sweep {
    shadow_frequency: u16,
    timer: u8,
    period: u8,
    enabled: bool,
    frequency: u16,
    shift: u8,
    decrementing: bool,
    disable_channel: fn(),
}

impl Sweep {
    pub fn new(disable_channel: fn()) -> Self {
        Self {
            shadow_frequency: 0,
            timer: 0,
            period: 0,
            enabled: false,
            frequency: 0,
            shift: 0,
            decrementing: false,
            disable_channel,
        }
    }

    pub fn step(&mut self) {
        self.timer -= 1;

        if self.timer == 0 {
            self.timer = if self.period > 0 { self.period } else { 8 };

            if self.enabled && self.period > 0 {
                let new_frequency = self.calculate_frequency();

                if new_frequency <= 2047 && self.shift > 0 {
                    self.frequency = new_frequency;
                    self.shadow_frequency = new_frequency;
                    self.calculate_frequency();
                }
            }
        }
    }

    fn calculate_frequency(&mut self) -> u16 {
        let mut new_frequency = self.shadow_frequency >> self.shift;

        if self.decrementing {
            new_frequency = self.shadow_frequency - new_frequency;
        } else {
            new_frequency += self.shadow_frequency;
        }

        // OVERFLOW CHECK
        if new_frequency > 2047 {
            (self.disable_channel)();
        }

        new_frequency
    }

    pub fn get_frequency(&self, unsweeped_frequency: u16) -> u16 {
        if self.enabled {
            self.frequency
        } else {
            unsweeped_frequency
        }
    }

    pub fn trigger_event(&mut self, frequency: u16) {
        self.shadow_frequency = frequency;
        self.timer = if self.period > 0 { self.period } else { 8 };
        self.enabled = self.period > 0 || self.shift > 0;
        if self.shift > 0 {
            self.calculate_frequency();
        }
    }
}

impl Memory for Sweep {
    fn accepts_address(&self, address: u16) -> bool {
        address == memory::NR10
    }

    fn read_byte(&self, address: u16) -> u8 {
        if address == memory::NR10 {
            return self.period << 4
                | (if self.decrementing { 0b1000 } else { 0 })
                | self.shift
                | 0b10000000;
        }
        panic!("invalid address: {address}");
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if address != memory::NR10 {
            panic!("invalid address: {address}");
        }

        self.decrementing = (value & 0b1000) > 0;
        self.period = (value & 0b1110000) >> 4;
        self.shift = value & 0b111;
    }
}

impl Display for Sweep {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sweep")
    }
}
