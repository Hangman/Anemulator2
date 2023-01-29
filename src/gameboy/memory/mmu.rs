use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::gameboy::mbc::mbc::Mbc;
use crate::gameboy::memory::memory;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::ppu::Ppu;
use crate::gameboy::timer::Timer;

pub struct Mmu {
    mbc: Box<dyn Mbc>,
    timer: Timer,
    unit_lut: Vec<Box<dyn Memory>>,
    dma: u8,
    pub ppu: Ppu,
    if_register: Rc<RefCell<u8>>,
}

impl Mmu {
    pub fn new(mbc: Box<dyn Mbc>) -> Self {
        let if_reg = Rc::new(RefCell::new(0));
        Self {
            mbc,
            timer: Timer::new(Rc::clone(&if_reg)),
            unit_lut: Vec::new(),
            dma: 0,
            ppu: Ppu::new(Rc::clone(&if_reg)),
            if_register: if_reg,
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

    pub fn step(&mut self) -> bool {
        self.timer.step();
        self.ppu.step()
    }

    fn dma_transfer(&mut self) {
        let source_address = (self.dma as u16) << 8;
        for i in 0..0xA0 {
            self.write_byte(0xFE00 + i, self.read_byte(source_address + i));
        }
    }
}

impl Memory for Mmu {
    fn accepts_address(&self, _address: u16) -> bool {
        true
    }

    fn read_byte(&self, address: u16) -> u8 {
        if address == memory::DMA {
            return self.dma;
        }
        if address == memory::IF {
            return *self.if_register.borrow();
        }
        if self.ppu.accepts_address(address) {
            return self.ppu.read_byte(address);
        }
        if self.mbc.accepts_address(address) {
            return self.mbc.read_byte(address);
        }
        if self.timer.accepts_address(address) {
            return self.timer.read_byte(address);
        }
        let unit = self
            .get_unit(address)
            .unwrap_or_else(|| panic!("missing memory unit for address: {}", address));
        unit.read_byte(address)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if address == memory::DMA {
            self.dma = value;
            self.dma_transfer();
            return;
        }
        if address == memory::IF {
            *(*self.if_register).borrow_mut() = value;
            return;
        }
        if self.ppu.accepts_address(address) {
            self.ppu.write_byte(address, value);
            return;
        }
        if self.mbc.accepts_address(address) {
            self.mbc.write_byte(address, value);
            return;
        }
        if self.timer.accepts_address(address) {
            self.timer.write_byte(address, value);
            return;
        }
        let unit = self
            .get_mut_unit(address)
            .unwrap_or_else(|| panic!("missing memory unit for address: {}", address));
        unit.write_byte(address, value);
    }
}

impl Display for Mmu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mmu")
    }
}
