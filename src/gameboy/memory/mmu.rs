use crate::gameboy::mbc::mbc::Mbc;
use std::fmt::{Display, Formatter};

use crate::gameboy::ram::memory::Memory;

pub struct Mmu {
    mbc: Box<dyn Mbc>,
    unit_lut: Vec<Box<dyn Memory>>,
}

impl Mmu {
    pub fn new(mbc: Box<dyn Mbc>) -> Self {
        Self {
            mbc,
            unit_lut: Vec::new(),
        }
    }

    pub fn add_memory_unit(&mut self, unit: Box<dyn Memory>) {
        self.unit_lut.push(unit);
    }

    fn get_unit(&self, address: u16) -> Option<&Box<dyn Memory>> {
        self.unit_lut
            .iter()
            .find(|&unit| unit.accepts_address(address))
    }

    fn get_mut_unit(&mut self, address: u16) -> Option<&mut Box<dyn Memory>> {
        self.unit_lut
            .iter_mut()
            .find(|unit| unit.accepts_address(address))
    }
}

impl Display for Mmu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mmu")
    }
}

impl Memory for Mmu {
    fn accepts_address(&self, _address: u16) -> bool {
        true
    }

    fn read_byte(&self, address: u16) -> u8 {
        if self.mbc.accepts_address(address) {
            return self.mbc.read_byte(address);
        }
        let unit = self
            .get_unit(address)
            .unwrap_or_else(|| panic!("missing memory unit for address: {}", address));
        unit.read_byte(address)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if self.mbc.accepts_address(address) {
            self.mbc.write_byte(address, value);
            return;
        }
        let unit = self
            .get_mut_unit(address)
            .unwrap_or_else(|| panic!("missing memory unit for address: {}", address));
        unit.write_byte(address, value);
    }
}
