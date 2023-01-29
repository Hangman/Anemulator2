use std::cmp::min;
use std::fmt::{Display, Formatter};

use crate::gameboy::mbc::mbc;
use crate::gameboy::mbc::mbc::Mbc;
use crate::gameboy::memory::memory;
use crate::gameboy::memory::memory::Memory;

pub struct Mbc0 {
    rom: Box<[u8; 0x8001]>,
    external_ram: Box<[u8; 0xC000 - 0xA000]>,
    booted: bool,
}

impl Mbc0 {
    pub fn new(cartridge_data: &[u8]) -> Self {
        let mut result = Self {
            rom: Box::new([0; 0x8001]),
            external_ram: Box::new([0; 0xC000 - 0xA000]),
            booted: false,
        };

        // COPY ROM
        let rom_length = min(result.rom.len(), cartridge_data.len());
        result.rom[0..rom_length].copy_from_slice(&cartridge_data[0..rom_length]);

        result
    }
}

impl Memory for Mbc0 {
    fn accepts_address(&self, address: u16) -> bool {
        address < 0x8000
            || address == memory::DISABLE_BOOT_ROM
            || (0xA000..0xC000).contains(&address)
    }

    fn read_byte(&self, address: u16) -> u8 {
        if !self.booted && address <= 0x00FF {
            return mbc::BOOT_ROM[address as usize];
        }

        if (0xA000..0xC000).contains(&address) {
            return self.external_ram[(address - 0xA000) as usize];
        }

        if address == memory::DISABLE_BOOT_ROM {
            return self.rom[0x8000];
        }

        self.rom[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if address == memory::DISABLE_BOOT_ROM {
            if value > 0 {
                self.booted = true;
            }
            self.rom[0x8000] = value;
        }

        if (0xA000..0xC000).contains(&address) {
            self.external_ram[(address - 0xA000) as usize] = value;
        }

        // ignore writes to the ROM
    }
}

impl Display for Mbc0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mbc0 running game {}", self.get_game_name())
    }
}

impl Mbc for Mbc0 {
    fn set_booted(&mut self) {
        self.booted = true;
    }

    fn is_booted(&self) -> bool {
        self.booted
    }

    fn get_game_name(&self) -> String {
        String::from_utf8_lossy(&self.rom[0x134..=0x0143]).into()
    }
}
