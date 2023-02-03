use std::fmt::{Display, Formatter};

use crate::gameboy::audio::noise_channel::NoiseChannel;
use crate::gameboy::audio::square_channel::SquareChannel;
use crate::gameboy::audio::wave_channel::WaveChannel;
use crate::gameboy::memory::memory;
use crate::gameboy::memory::memory::Memory;

pub struct Apu {
    channel1: SquareChannel,
    channel2: SquareChannel,
    channel3: WaveChannel,
    channel4: NoiseChannel,
    channel_control_register: u8,
    channel_selection_register: u8,
    internal_buffer: [f32; 1024],
    output_buffer: [f32; 1024],
    buffer_position: usize,
    cycle_counter: usize,
    buffer_full: bool,
    frame_sequencer: u8,
    frame_sequencer_cycle_counter: u16,
    enabled: bool,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            channel1: SquareChannel::new(
                memory::NR11,
                memory::NR12,
                true,
                memory::NR13,
                memory::NR14,
            ),
            channel2: SquareChannel::new(
                memory::NR21,
                memory::NR22,
                false,
                memory::NR23,
                memory::NR24,
            ),
            channel3: WaveChannel::new(),
            channel4: NoiseChannel::new(),
            channel_control_register: 0,
            channel_selection_register: 0,
            internal_buffer: [0f32; 1024],
            output_buffer: [0f32; 1024],
            buffer_position: 0,
            cycle_counter: 0,
            buffer_full: false,
            frame_sequencer: 0,
            frame_sequencer_cycle_counter: 0,
            enabled: false,
        }
    }

    pub fn step(&mut self) {
        if self.is_bit_set(memory::NR52, 7) {
            for _i in 0..4 {
                self.step_internal();
            }
        }
    }

    fn step_internal(&mut self) {
        self.cycle_counter += 1;
        self.frame_sequencer_cycle_counter += 1;

        let mut step_length = false;
        let mut step_envelope = false;
        let mut step_sweep = false;
        if self.frame_sequencer_cycle_counter >= 8192 {
            self.frame_sequencer_cycle_counter = 0;
            self.frame_sequencer += 1;
            if self.frame_sequencer > 7 {
                self.frame_sequencer = 0;
            }

            match self.frame_sequencer {
                0 | 4 => step_length = true,
                2 | 6 => {
                    step_length = true;
                    step_sweep = true;
                }
                7 => step_envelope = true,
                _ => {}
            }
        }

        let mut channel_1_samples = self.channel1.step(step_length, step_envelope, step_sweep);
        let mut channel_2_samples = self.channel2.step(step_length, step_envelope, step_sweep);
        let mut channel_3_samples = self.channel3.step(step_length);
        let mut channel_4_samples = self.channel4.step(step_length, step_envelope);

        if self.cycle_counter == 87 {
            self.cycle_counter = 0;

            // CHANNEL PANNING
            if (self.channel_selection_register & 0b1000_0000) == 0 {
                // channel 4 left
                channel_4_samples[0] = 0f32;
            }
            if (self.channel_selection_register & 0b0100_0000) == 0 {
                // channel 3 left
                channel_3_samples[0] = 0f32;
            }
            if (self.channel_selection_register & 0b0010_0000) == 0 {
                // channel 2 left
                channel_2_samples[0] = 0f32;
            }
            if (self.channel_selection_register & 0b0001_0000) == 0 {
                // channel 1 left
                channel_1_samples[0] = 0f32;
            }
            if (self.channel_selection_register & 0b0000_1000) == 0 {
                // channel 4 right
                channel_4_samples[1] = 0f32;
            }
            if (self.channel_selection_register & 0b0000_0100) == 0 {
                // channel 3 right
                channel_3_samples[1] = 0f32;
            }
            if (self.channel_selection_register & 0b0000_0010) == 0 {
                // channel 2 right
                channel_2_samples[1] = 0f32;
            }
            if (self.channel_selection_register & 0b0000_0001) == 0 {
                // channel 1 right
                channel_1_samples[1] = 0f32;
            }

            // MIX CHANNELS
            let mut left_channel_sample = (channel_1_samples[0]
                + channel_2_samples[0]
                + channel_3_samples[0]
                + channel_4_samples[0])
                / 4f32;
            let mut right_channel_sample = (channel_1_samples[1]
                + channel_2_samples[1]
                + channel_3_samples[1]
                + channel_4_samples[1])
                / 4f32;

            // MASTER PANNING
            let left_pan_factor = ((self.channel_control_register >> 4) & 0b111) as f32 / 7f32;
            let right_pan_factor = (self.channel_control_register & 0b111) as f32 / 7f32;
            left_channel_sample *= left_pan_factor;
            right_channel_sample *= right_pan_factor;

            self.internal_buffer[self.buffer_position] = left_channel_sample;
            self.buffer_position += 1;
            self.internal_buffer[self.buffer_position] = right_channel_sample;
            self.buffer_position += 1;
            if self.buffer_position >= 1024 {
                self.buffer_position = 0;
                self.buffer_full = true;
                self.output_buffer.copy_from_slice(&self.internal_buffer);
            }
        }
    }

    pub fn is_buffer_full(&self) -> bool {
        self.buffer_full
    }

    pub fn fetch_samples(&mut self) -> [f32; 1024] {
        self.buffer_full = false;
        self.output_buffer
    }

    fn enabled_apu(&mut self, enabled: bool) {
        if self.enabled && !enabled {
            // TURN OFF
            self.write_byte(memory::NR10, 0);
            self.write_byte(memory::NR11, 0);
            self.write_byte(memory::NR12, 0);
            self.write_byte(memory::NR13, 0);
            self.write_byte(memory::NR14, 0);
            self.write_byte(memory::NR20, 0);
            self.write_byte(memory::NR21, 0);
            self.write_byte(memory::NR22, 0);
            self.write_byte(memory::NR23, 0);
            self.write_byte(memory::NR24, 0);
            self.write_byte(memory::NR30, 0);
            self.write_byte(memory::NR31, 0);
            self.write_byte(memory::NR32, 0);
            self.write_byte(memory::NR33, 0);
            self.write_byte(memory::NR34, 0);
            self.write_byte(memory::NR40, 0);
            self.write_byte(memory::NR41, 0);
            self.write_byte(memory::NR42, 0);
            self.write_byte(memory::NR43, 0);
            self.write_byte(memory::NR44, 0);
            self.write_byte(memory::NR50, 0);
            self.write_byte(memory::NR51, 0);
            for i in 0xFF30..0xFF40 {
                self.write_byte(i, 0);
            }
            self.enabled = false;
        } else if !self.enabled && enabled {
            // TURN ON
            self.enabled = true;
            self.frame_sequencer = 0;
            self.channel1.set_duty_position(0);
            self.channel2.set_duty_position(0);
            self.channel3.set_duty_position(0);
        }
    }
}

impl Memory for Apu {
    fn accepts_address(&self, address: u16) -> bool {
        if self.channel1.accepts_address(address) {
            return true;
        }
        if self.channel2.accepts_address(address) {
            return true;
        }
        if self.channel3.accepts_address(address) {
            return true;
        }
        if self.channel4.accepts_address(address) {
            return true;
        }
        matches!(
            address,
            memory::NR50 | memory::NR51 | memory::NR52 | memory::NR20 | memory::NR40 | 0xFF27
                ..=0xFF2F
        )
    }

    fn read_byte(&self, address: u16) -> u8 {
        if !self.enabled && address != memory::NR52 && !(0xFF30..0xFF40).contains(&address) {
            return 0xFF;
        }
        if self.channel1.accepts_address(address) {
            return self.channel1.read_byte(address);
        }
        if self.channel2.accepts_address(address) {
            return self.channel2.read_byte(address);
        }
        if self.channel3.accepts_address(address) {
            return self.channel3.read_byte(address);
        }
        if self.channel4.accepts_address(address) {
            return self.channel4.read_byte(address);
        }
        if address == memory::NR50 {
            return self.channel_control_register;
        }
        if address == memory::NR51 {
            return self.channel_selection_register;
        }
        if address == memory::NR52 {
            let mut result = if self.enabled { 1 << 7 } else { 0 };
            result |= 0b0111_0000;
            result |= (self.channel4.is_enabled() as u8) << 3;
            result |= (self.channel3.is_enabled() as u8) << 2;
            result |= (self.channel2.is_enabled() as u8) << 1;
            result |= self.channel1.is_enabled() as u8;
            return result;
        }
        if address == memory::NR20 || address == memory::NR40 {
            return 0xFF;
        }
        if (0xFF27..=0xFF2F).contains(&address) {
            return 0xFF;
        }

        panic!("invalid address: {address}")
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if !self.enabled && address != memory::NR52 && !(0xFF30..0xFF40).contains(&address) {
            return;
        }
        if self.channel1.accepts_address(address) {
            self.channel1.write_byte(address, value);
            return;
        }
        if self.channel2.accepts_address(address) {
            self.channel2.write_byte(address, value);
            return;
        }
        if self.channel3.accepts_address(address) {
            self.channel3.write_byte(address, value);
            return;
        }
        if self.channel4.accepts_address(address) {
            self.channel4.write_byte(address, value);
            return;
        }
        if address == memory::NR50 {
            self.channel_control_register = value;
            return;
        }
        if address == memory::NR51 {
            self.channel_selection_register = value;
            return;
        }
        if address == memory::NR52 {
            self.enabled_apu((value & 0b1000_0000) > 0);
            return;
        }
        if address == memory::NR20 || address == memory::NR40 {
            return;
        }
        if (0xFF27..=0xFF2F).contains(&address) {
            return;
        }

        panic!("invalid address: {address}");
    }
}

impl Display for Apu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Apu")
    }
}
