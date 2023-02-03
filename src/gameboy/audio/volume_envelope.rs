use std::fmt::{Display, Formatter};

use crate::gameboy::memory::memory::Memory;

pub struct VolumeEnvelope {
    period: u8,
    period_timer: u8,
    volume: u8,
    upwards: bool,
    register: u8,
    register_address: u16,
}

impl VolumeEnvelope {
    pub fn new(register_address: u16) -> Self {
        Self {
            period: 0,
            period_timer: 0,
            volume: 0,
            upwards: false,
            register: 0,
            register_address,
        }
    }

    pub fn step(&mut self) {
        if self.period != 0 {
            self.period_timer -= 1;

            if self.period_timer == 0 {
                self.period_timer = self.period;
                if self.volume < 0xF && self.upwards || self.volume > 0x0 && !self.upwards {
                    if self.upwards {
                        self.volume += 1;
                    } else {
                        self.volume -= 1;
                    }
                }
            }
        }
    }

    pub fn modify_samples(&self, samples: &mut [f32; 2]) {
        let factor = self.volume as f32 / 0xF as f32;
        samples[0] *= factor;
        samples[1] *= factor;
    }

    pub fn trigger_event(&mut self) {
        self.period_timer = self.register & 0b111;
        self.volume = self.register >> 4;
    }
}

impl Memory for VolumeEnvelope {
    fn accepts_address(&self, address: u16) -> bool {
        address == self.register_address
    }

    fn read_byte(&self, address: u16) -> u8 {
        if address == self.register_address {
            return self.register;
        }
        panic!("invalid address: {address}")
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if address == self.register_address {
            self.register = value;
            self.upwards = (self.register & 0b1000) > 0;
            self.period = self.register & 0b111;
            return;
        }
        panic!("invalid address: {address}")
    }
}

impl Display for VolumeEnvelope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "VolumeEnvelope")
    }
}
