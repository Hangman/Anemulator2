use std::fmt::{Display, Formatter};

use crate::gameboy::audio::sweep::Sweep;
use crate::gameboy::audio::volume_envelope::VolumeEnvelope;
use crate::gameboy::memory::memory::Memory;

const WAVE_DUTY: [[f32; 8]; 4] = [
    [0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 1f32],
    [1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 1f32],
    [1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 1f32, 1f32],
    [0f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 0f32],
];

pub struct SquareChannel {
    volume_address: u16,
    volume_envelope: VolumeEnvelope,
    sweep: Option<Sweep>,
    frequency_timer: u16,
    duty_position: u8,
    length_duty_address: u16,
    length_duty_register: u8,
    freq_low_address: u16,
    freq_high_address: u16,
    freq_low_register: u8,
    freq_high_register: u8,
    samples: [f32; 2],
    enabled: bool,
    length_timer: u8,
}

impl SquareChannel {
    pub fn new(
        length_duty_address: u16,
        volume_address: u16,
        use_sweep: bool,
        freq_low_address: u16,
        freq_high_address: u16,
    ) -> Self {
        Self {
            volume_address,
            volume_envelope: VolumeEnvelope::new(volume_address),
            sweep: if use_sweep {
                Some(Sweep::new(|| println!("Fix my Sweep"))) // FIXME
            } else {
                None
            },
            frequency_timer: 0,
            duty_position: 0,
            length_duty_address,
            length_duty_register: 0,
            freq_low_address,
            freq_high_address,
            freq_low_register: 0,
            freq_high_register: 0,
            samples: [0f32; 2],
            enabled: false,
            length_timer: 0,
        }
    }

    pub fn step(&mut self, step_length: bool, step_envelope: bool, step_sweep: bool) -> [f32; 2] {
        if self.enabled && self.is_dac_on() {
            // STEP SYSTEMS
            if step_length {
                self.step_length();
                if !self.enabled {
                    self.samples[0] = 0f32;
                    self.samples[1] = 0f32;
                    return self.samples;
                }
            }
            if step_envelope {
                self.volume_envelope.step();
            }
            if step_sweep {
                if let Some(sweep) = self.sweep.as_mut() {
                    sweep.step();
                }
            }
            let duty = WAVE_DUTY[(self.length_duty_register >> 6) as usize];
            let mut frequency = self.get_frequency();
            if let Some(sweep) = self.sweep.as_mut() {
                frequency = sweep.get_frequency(frequency);
            }

            self.frequency_timer -= 1;
            if self.frequency_timer == 0 {
                self.frequency_timer = (2048 - frequency) * 4;
                self.duty_position += 1;
                if self.duty_position > 7 {
                    self.duty_position = 0;
                }
            }

            self.samples[0] = duty[self.duty_position as usize];
            self.samples[1] = duty[self.duty_position as usize];
            self.volume_envelope.modify_samples(&mut self.samples);
        } else {
            self.samples[0] = 0f32;
            self.samples[1] = 0f32;
        }

        self.samples
    }

    fn step_length(&mut self) {
        if self.is_length_timer_enabled() {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.enabled = false;
            }
        }
    }

    fn is_length_timer_enabled(&self) -> bool {
        (self.freq_high_register & 0b100_0000) > 0
    }

    fn is_dac_on(&self) -> bool {
        (self.volume_envelope.read_byte(self.volume_address) & 0b1111_1000) > 0
    }

    pub fn set_duty_position(&mut self, value: u8) {
        self.duty_position = value;
    }

    fn trigger_event(&mut self) {
        self.enabled = true;
        self.volume_envelope.trigger_event();
        let freq = self.get_frequency();
        if let Some(sweep) = self.sweep.as_mut() {
            sweep.trigger_event(freq)
        }
    }

    fn get_frequency(&self) -> u16 {
        self.freq_low_register as u16 | (((self.freq_high_register & 0b111) as u16) << 8)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Memory for SquareChannel {
    fn accepts_address(&self, address: u16) -> bool {
        let mut sweep_accepts = false;
        if let Some(sweep) = self.sweep.as_ref() {
            sweep_accepts = sweep.accepts_address(address)
        }
        address == self.length_duty_address
            || address == self.freq_low_address
            || address == self.freq_high_address
            || self.volume_envelope.accepts_address(address)
            || sweep_accepts
    }

    fn read_byte(&self, address: u16) -> u8 {
        if self.volume_envelope.accepts_address(address) {
            return self.volume_envelope.read_byte(address);
        }
        if let Some(sweep) = self.sweep.as_ref() {
            if sweep.accepts_address(address) {
                return sweep.read_byte(address);
            }
        }
        if address == self.length_duty_address {
            return self.length_duty_register | 0b0011_1111;
        }
        if address == self.freq_low_address {
            return self.freq_low_register;
        }
        if address == self.freq_high_address {
            return self.freq_high_register;
        }

        panic!("invalid address: {address}");
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if address == self.length_duty_address {
            self.length_duty_register = value;
            self.length_timer = 64 - (value & 0b11_1111);
            return;
        }
        if address == self.freq_low_address {
            self.freq_low_register = value;
            return;
        }
        if address == self.freq_high_address {
            self.freq_high_register = value;
            if (value & 0b1000_0000) > 0 && self.is_dac_on() {
                self.trigger_event();
            }
            return;
        }
        if self.volume_envelope.accepts_address(address) {
            self.volume_envelope.write_byte(address, value);
            return;
        }
        if let Some(sweep) = self.sweep.as_mut() {
            if sweep.accepts_address(address) {
                sweep.write_byte(address, value);
                return;
            }
        }

        panic!("Invalid address: {address}");
    }
}

impl Display for SquareChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SquareChannel")
    }
}
