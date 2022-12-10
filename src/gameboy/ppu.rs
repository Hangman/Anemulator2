use std::fmt::{Display, Formatter};

use crate::gameboy::ram::memory;
use crate::gameboy::ram::memory::Memory;

pub struct Ppu {
    pub front_buffer: Box<[u8; 160 * 144 * 4]>,
    lcdc: u8,
    lcd_stat: u8,
    lcd_ly: u8,
    lcd_lyc: u8,
    scroll_x: u8,
    scroll_y: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wx: u8,
    wy: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            front_buffer: Box::new([0; 160 * 144 * 4]),
            lcdc: 0,
            lcd_stat: 0,
            lcd_ly: 0,
            lcd_lyc: 0,
            scroll_x: 0,
            scroll_y: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wx: 0,
            wy: 0,
        }
    }
}

impl Display for Ppu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ppu")
    }
}

impl Memory for Ppu {
    fn accepts_address(&self, address: u16) -> bool {
        address == memory::LCDC
            || address == memory::LCD_STAT
            || address == memory::LCD_LY
            || address == memory::LCD_LYC
            || address == memory::SCROLL_X
            || address == memory::SCROLL_Y
            || address == memory::BGP
            || address == memory::OBP0
            || address == memory::OBP1
            || address == memory::WX
            || address == memory::WY
    }

    fn read_byte(&self, address: u16) -> u8 {
        match address {
            memory::LCDC => self.lcdc,
            memory::LCD_STAT => self.lcd_stat,
            memory::LCD_LY => self.lcd_ly,
            memory::LCD_LYC => self.lcd_lyc,
            memory::SCROLL_X => self.scroll_x,
            memory::SCROLL_Y => self.scroll_y,
            memory::BGP => self.bgp,
            memory::OBP0 => self.obp0,
            memory::OBP1 => self.obp1,
            memory::WX => self.wx,
            memory::WY => self.wy,
            _ => panic!("invalid read from address: {}", address),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            memory::LCDC => self.lcdc = value,
            memory::LCD_LYC => self.lcd_lyc = value,
            memory::SCROLL_X => self.scroll_x = value,
            memory::SCROLL_Y => self.scroll_y = value,
            memory::BGP => self.bgp = value,
            memory::OBP0 => self.obp0 = value,
            memory::OBP1 => self.obp1 = value,
            memory::WX => self.wx = value,
            memory::WY => self.wy = value,
            memory::LCD_STAT => {
                let mut old_value = self.lcd_stat;
                old_value &= 0b0000_0111;
                let mut new_value = value;
                new_value &= 0b0111_1000;
                new_value |= old_value | 0b1000_0000;
                self.lcd_stat = new_value;
            }
            memory::LCD_LY => {
                // ignore, writing is not allowed from outside
            }
            _ => panic!("invalid write to address: {}", address),
        }
    }
}

enum PpuMode {
    OamSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

impl PpuMode {
    fn get_flag_bits(&self) -> u8 {
        match self {
            PpuMode::OamSearch => 0b10,
            PpuMode::PixelTransfer => 0b11,
            PpuMode::HBlank => 0b00,
            PpuMode::VBlank => 0b01,
        }
    }
}
