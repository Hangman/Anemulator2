use std::cmp::min;
use std::fmt::{Display, Formatter};

use crate::gameboy::mbc::mbc;
use crate::gameboy::mbc::mbc::Mbc;
use crate::gameboy::ram::memory;
use crate::gameboy::ram::memory::Memory;

pub struct Mbc1 {
    rom_banks: Box<[[u8; 128]; 0x4000]>,
    ram_banks: Box<[[u8; 4]; 0x2000]>,
    mode: Mode,
    bank_select_register: usize,
    ram_enabled: bool,
    booted: bool,
    rom_bank0: Box<[u8; 0x4000]>,
    ff50_register: u8,
}

impl Mbc1 {
    pub fn new(cartridge_data: &[u8]) -> Self {
        let mut result = Self {
            rom_banks: Box::new([[0; 128]; 0x4000]),
            ram_banks: Box::new([[0; 4]; 0x2000]),
            mode: Mode::ROM,
            bank_select_register: 1,
            ram_enabled: false,
            booted: false,
            rom_bank0: Box::new([0; 0x4000]),
            ff50_register: 0xFF,
        };

        // COPY ROM BANK 0
        let rom_length = min(result.rom_bank0.len(), cartridge_data.len());
        result.rom_bank0[0..rom_length].copy_from_slice(&cartridge_data[0..rom_length]);

        // COPY OTHER ROM BANKS
        let mut remaining = cartridge_data.len() - rom_length;
        let mut bank_index = 0;
        while remaining > 0 {
            let length = min(remaining, 0x4000);
            let start_index = cartridge_data.len() - remaining;
            let end_index = start_index + length;
            result.rom_banks[bank_index][0..length].copy_from_slice(&cartridge_data[start_index..end_index]);
            remaining -= length;
            bank_index += 1;
        }

        result
    }
}

impl Memory for Mbc1 {
    fn accepts_address(&self, address: u16) -> bool {
        address >= 0 && address < 0x8000 || address == memory::DISABLE_BOOT_ROM || address >= 0xA000 && address < 0xC000
    }

    fn read_byte(&self, address: u16) -> u8 {
        if !self.booted && address >= 0x0000 && address <= 0x00FF {
            return mbc::BOOT_ROM[address as usize];
        }

        if address >= 0x4000 && address < 0x8000 {
            return match self.mode {
                Mode::ROM => {
                    self.rom_banks[self.bank_select_register - 1][(address - 0x4000) as usize]
                }
                Mode::RAM => {
                    self.rom_banks[(self.bank_select_register & 0b11111) - 1][(address - 0x4000) as usize]
                }
            };
        }

        if address >= 0xA000 && address < 0xC000 {
            if self.ram_enabled && self.mode == Mode::RAM {
                return self.ram_banks[(self.bank_select_register & 0b1111111) >> 5][(address - 0xA000) as usize];
            }
            return self.ram_banks[0][(address - 0xA000) as usize];
        }

        if address == memory::DISABLE_BOOT_ROM {
            return self.ff50_register;
        }

        self.rom_bank0[address as usize]
    }

    fn write_byte(&mut self, address: u16, mut value: u8) {
        if address >= 0x0000 && address < 0x2000 {
            // ENABLE/DISABLE RAM
            value &= 0xF;
            self.ram_enabled = value == 0x0A;
        } else if address >= 0x2000 && address < 0x4000 {
            // SELECT ROM BANK NUMBER (LOWER 5 BITS)
            value &= 0b11111;
            if value == 0 {
                value = 1;
            }
            self.bank_select_register = self.bank_select_register & 0b1100000 | (value as usize);
        } else if address >= 0x4000 && address < 0x6000 {
            // SELECT RAM BANK NUMBER OR UPPER BITS OF ROM BANK NUMBER
            self.bank_select_register = self.bank_select_register & 0b11111 | ((value as usize) & 0b11) << 5;
            if self.bank_select_register == 0 {
                self.bank_select_register = 1;
            }
        } else if address >= 0x6000 && address < 0x8000 {
            // SET MODE
            if (value & 0b11) == 0 {
                self.mode = Mode::ROM;
            } else if (value & 0b11) == 1 {
                self.mode = Mode::RAM;
            }
        } else if address >= 0xA000 && address < 0xC000 {
            // WRITE TO EXTERNAL RAM
            if self.mode == Mode::RAM {
                self.ram_banks[(self.bank_select_register & 0b1111111) >> 5][(address as usize) - 0xA000] = value;
            } else {
                self.ram_banks[0][(address as usize) - 0xA000] = value;
            }
        } else if address == memory::DISABLE_BOOT_ROM {
            // DISABLE BOOT ROM
            self.ff50_register = value;
            if value > 0 {
                self.booted = true;
            }
        } else {
            self.rom_bank0[address as usize] = value;
        }
    }
}

impl Display for Mbc1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mbc1 running game {}", self.get_game_name())
    }
}

impl Mbc for Mbc1 {
    fn set_booted(&mut self) {
        self.booted = true;
    }

    fn is_booted(&self) -> bool {
        self.booted
    }

    fn get_game_name(&self) -> String {
        String::from_utf8_lossy(&self.rom_bank0[0x134..=0x0143]).into()
    }
}

#[derive(PartialEq)]
enum Mode {
    ROM,
    RAM,
}