use std::fmt::{Display, Formatter};

use crate::gameboy::memory::memory;
use crate::gameboy::memory::memory::Memory;

pub struct WaveChannel {
    wave_pattern: [u8; 0xFF40 - 0xFF30],
    samples: [f32; 2],
    frequency_timer: u16,
    frequency: u16,
    duty_position: u8,
    dac_on: bool,
    length_enabled: bool,
    length_counter: u8,
    enabled: bool,
    output_level: u8,
    volume_shift: u8,
}

impl WaveChannel {
    pub fn new() -> Self {
        Self {
            wave_pattern: [0; 0xFF40 - 0xFF30],
            samples: [0f32; 2],
            frequency_timer: 0,
            frequency: 0,
            duty_position: 0,
            dac_on: false,
            length_enabled: false,
            length_counter: 0,
            enabled: false,
            output_level: 0,
            volume_shift: 0,
        }
    }

    pub fn step(&mut self, step_length: bool) -> [f32; 2] {
        if step_length {
            self.step_length();
        }

        if self.frequency_timer == 0 {
            self.frequency_timer = (2048 - self.frequency) * 2;
            self.duty_position = (self.duty_position + 1) & 31;
        }
        self.frequency_timer -= 1;

        let mut float_sample = 0f32;
        if self.dac_on && self.enabled {
            let sample_bit_shift = if (self.duty_position & 1) != 0 { 4 } else { 0 };
            let pattern_sample = self.wave_pattern[(self.duty_position as f32 / 2f32) as usize];
            let sample = (pattern_sample >> sample_bit_shift) & 0x0F;
            float_sample = (sample >> self.volume_shift) as f32 / 15f32;
        }
        self.samples[0] = float_sample;
        self.samples[1] = float_sample;

        self.samples
    }

    fn step_length(&mut self) {
        if self.length_enabled && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn set_duty_position(&mut self, value: u8) {
        self.duty_position = value;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Memory for WaveChannel {
    fn accepts_address(&self, address: u16) -> bool {
        address == memory::NR30
            || address == memory::NR31
            || address == memory::NR32
            || address == memory::NR33
            || address == memory::NR34
            || (0xFF30..0xFF40).contains(&address)
    }

    fn read_byte(&self, address: u16) -> u8 {
        match address {
            memory::NR30 => {
                if self.dac_on {
                    (1 << 7) | 0x7F
                } else {
                    0x7F
                }
            }
            memory::NR31 => 0xFF,
            memory::NR32 => self.output_level << 5 | 0x9F,
            memory::NR33 => 0xFF,
            memory::NR34 => {
                if self.length_enabled {
                    (1 << 6) | 0b1011_1111
                } else {
                    0b1011_1111
                }
            }
            0xFF30..=0xFF3F => self.wave_pattern[(address - 0xFF30) as usize],
            _ => {
                panic!("invalid address: {address}")
            }
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if address == memory::NR30 {
            self.dac_on = value >> 7 != 0;
            if !self.dac_on {
                self.enabled = false;
            }
            return;
        }
        if address == memory::NR31 {
            self.length_counter = 255 - value;
            return;
        }
        if address == memory::NR32 {
            self.output_level = (value >> 5) & 0b11;
            match self.output_level {
                0b00 => self.volume_shift = 4,
                0b01 => self.volume_shift = 0,
                0b10 => self.volume_shift = 1,
                0b11 => self.volume_shift = 2,
                _ => println!("nothing"),
            }
            return;
        }
        if address == memory::NR33 {
            self.frequency = (self.frequency & 0x700) | value as u16;
            return;
        }
        if address == memory::NR34 {
            self.frequency = (self.frequency & 0xFF) | ((value as u16 & 0x7) << 8);
            self.length_enabled = (value >> 6 & 0b1) != 0;
            if self.length_counter == 0 {
                self.length_counter = 255;
            }
            let trigger = value >> 7 != 0;
            if trigger && self.dac_on {
                self.enabled = true;
            }
            return;
        }
        if (0xFF30..0xFF40).contains(&address) {
            // If aiming for 100% accuracy, the write must be offset by the current duty position when the channel is enabled. We ignore this fact.
            self.wave_pattern[(address - 0xFF30) as usize] = value;
            return;
        }

        panic!("invalid address: {address}");
    }
}

impl Display for WaveChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WaveChannel")
    }
}
