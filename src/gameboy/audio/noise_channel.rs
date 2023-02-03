use std::fmt::{Display, Formatter};

use crate::gameboy::memory::memory;
use crate::gameboy::memory::memory::Memory;

pub struct NoiseChannel {
    polynomial_register: u8,
    length_counter: u8,
    incrementing: bool,
    initial_volume: u8,
    period: u8,
    dac_on: bool,
    enabled: bool,
    length_enabled: bool,
    lfsr: u16,
    period_timer: u8,
    volume: u8,
    frequency_timer: u16,
}

impl NoiseChannel {
    pub fn new() -> Self {
        Self {
            polynomial_register: 0,
            length_counter: 0,
            incrementing: false,
            initial_volume: 0,
            period: 0,
            dac_on: false,
            enabled: false,
            length_enabled: false,
            lfsr: 0,
            period_timer: 0,
            volume: 0,
            frequency_timer: 0,
        }
    }

    pub fn step(&mut self, step_length: bool, step_envelope: bool) -> [f32; 2] {
        if step_length {
            self.step_length();
        }
        if step_envelope {
            self.step_envelope();
        }

        if self.frequency_timer == 0 {
            let divisor_code = (self.polynomial_register & 0x07) as u16;
            let divisor_value = if divisor_code == 0 {
                8
            } else {
                divisor_code << 4
            };
            self.frequency_timer = divisor_value << (self.polynomial_register >> 4);
            let xor_result = (self.lfsr & 0b01) ^ ((self.lfsr & 0b10) >> 1);
            self.lfsr = self.lfsr >> 1 | xor_result << 14;
            if (self.polynomial_register >> 3 & 0b01) != 0 {
                self.lfsr &= !(1 << 6);
                self.lfsr |= xor_result << 6;
            }
        }
        self.frequency_timer -= 1;

        let mut sample = 0f32;
        if self.dac_on && self.enabled {
            let input = (!self.lfsr & 0b1) as f32 * self.volume as f32;
            sample = input / 15f32;
        }

        [sample, sample]
    }

    fn step_length(&mut self) {
        if self.length_enabled {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    fn step_envelope(&mut self) {
        if self.period != 0 {
            self.period_timer -= 1;
            if self.period_timer == 0 {
                self.period_timer = self.period;
                if self.volume < 0xF && self.incrementing {
                    self.volume += 1;
                } else if self.volume > 0 && !self.incrementing {
                    self.volume -= 1;
                }
            }
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Memory for NoiseChannel {
    fn accepts_address(&self, address: u16) -> bool {
        address == 0xFF1F
            || address == memory::NR41
            || address == memory::NR42
            || address == memory::NR43
            || address == memory::NR44
    }

    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF1F => 0xFF,
            memory::NR41 => 0xFF,
            memory::NR42 => {
                self.initial_volume << 4 | (if self.incrementing { 0x08 } else { 0 }) | self.period
            }
            memory::NR43 => self.polynomial_register,
            memory::NR44 => ((if self.length_enabled { 1 } else { 0 }) << 6) | 0b1011_1111,
            _ => panic!("invalid address: {address}"),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if address == 0xFF1F {
            return;
        }
        if address == memory::NR41 {
            self.length_counter = 64 - (value & 0b11_1111);
            return;
        }
        if address == memory::NR42 {
            self.incrementing = (value & 0x08) != 0;
            self.initial_volume = value >> 4;
            self.period = value & 0x07;
            self.dac_on = (value & 0b1111_1000) != 0;

            if !self.dac_on {
                self.enabled = false;
            }
            return;
        }
        if address == memory::NR43 {
            self.polynomial_register = value;
            return;
        }
        if address == memory::NR44 {
            self.length_enabled = (value >> 6 & 0b1) != 0;
            if self.length_counter == 0 {
                self.length_counter = 64;
            }
            let trigger = value >> 7 != 0;
            if trigger && self.dac_on {
                self.enabled = true;
            }
            if trigger {
                self.lfsr = 0x7FFF;
                self.period_timer = self.period;
                self.volume = self.initial_volume;
            }
            return;
        }

        panic!("invalid address: {address}");
    }
}

impl Display for NoiseChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoiseChannel")
    }
}
