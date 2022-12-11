use std::fmt::{Display, Formatter};

use crate::gameboy::memory::mmu::Mmu;
use crate::gameboy::ram::memory;
use crate::gameboy::ram::memory::Memory;

pub struct Ppu {
    pub front_buffer: Box<[[u16; 160]; 144]>,
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
    cycle_accumulator: isize,
    was_off: bool,
    state: PpuMode,
    tile_cache: [u8; 16],
}

const COLORS: [(f32, f32, f32, f32); 4] = [
    (1f32, 1f32, 1f32, 1f32),
    (0.666f32, 0.666f32, 0.666f32, 1f32),
    (0.333f32, 0.333f32, 0.333f32, 1f32),
    (0f32, 0f32, 0f32, 1f32),
];
const TRANSPARENT: (f32, f32, f32, f32) = (1f32, 1f32, 1f32, 0f32);

impl Ppu {
    pub fn new() -> Self {
        Self {
            front_buffer: Box::new([[0; 160]; 144]),
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
            cycle_accumulator: 0,
            state: PpuMode::VBlank,
            was_off: false,
            tile_cache: [0; 16],
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu) -> bool {
        let ppu_on = self.is_bit_set(memory::LCDC, 7);

        if ppu_on {
            self.cycle_accumulator += 4;
            self.was_off = false;

            if self.cycle_accumulator > 0 {
                let old_mode = self.state.clone();
                match old_mode {
                    PpuMode::OamSearch => {
                        self.oam_search(mmu);
                    }
                    PpuMode::PixelTransfer => {
                        self.pixel_transfer(mmu);
                    }
                    PpuMode::HBlank => {
                        self.h_blank(mmu);
                    }
                    PpuMode::VBlank => {
                        self.v_blank(mmu);
                    }
                }
                if self.state == PpuMode::VBlank && old_mode != PpuMode::VBlank {
                    return true;
                }
            }
        } else if !self.was_off {
            self.set_line(mmu, 0);
            self.set_state(mmu, PpuMode::HBlank);
            self.was_off = true;
        }

        false
    }

    fn oam_search(&mut self, mmu: &mut Mmu) {
        self.cycle_accumulator -= 80;
        self.set_state(mmu, PpuMode::PixelTransfer);
    }

    fn pixel_transfer(&mut self, mmu: &mut Mmu) {
        let render_bg = self.is_bit_set(memory::LCDC, 0);
        let render_window = self.is_bit_set(memory::LCDC, 5);
        let render_objects = self.is_bit_set(memory::LCDC, 1);
        let scanline = self.read_byte(memory::LCD_LY);
        let atlas_address_mode = self.is_bit_set(memory::LCDC, 4);

        if render_bg {
            // RENDER BACKGROUND
            self.render_bg(mmu, scanline, atlas_address_mode);

            // RENDER WINDOW
            if render_window {
                self.render_window(scanline, atlas_address_mode);
            }
        }

        // RENDER OBJECTS
        if render_objects {
            self.render_objects(scanline);
        }

        self.cycle_accumulator -= 172;
        self.set_state(mmu, PpuMode::HBlank);
    }

    fn render_bg(&mut self, mmu: &Mmu, scanline: u8, atlas_address_mode: bool) {
        let bg_map_start_address = if self.is_bit_set(memory::LCDC, 3) {
            0x9C00
        } else {
            0x9800
        };
        let scroll_x = self.read_byte(memory::SCROLL_X);
        let mut bg_map_y: u16 = scanline as u16 + self.read_byte(memory::SCROLL_Y) as u16;
        if bg_map_y >= 256 {
            bg_map_y -= 256;
        }
        let bg_map_block_y = bg_map_y / 8;
        let tile_pixel_y: i16 = (bg_map_y % 8) as i16;

        let mut last_tile_address = u16::MAX;

        for pixel_x in 0..160 {
            // FIND TILE ADDRESS
            let mut bg_map_x: i16 = pixel_x as i16 + scroll_x as i16;
            if bg_map_x >= 256 {
                bg_map_x -= 256;
            }
            let bg_map_block_x = bg_map_x / 8;
            let tile_pixel_x = bg_map_x % 8;
            let tile_address =
                bg_map_start_address as u16 + bg_map_block_y as u16 * 32 + bg_map_block_x as u16;

            // READ TILE DATA
            if tile_address != last_tile_address {
                let atlas_tile_address: u16 = if atlas_address_mode {
                    let atlas_tile_index = mmu.read_byte(tile_address);
                    0x8000 + atlas_tile_index as u16 * 16
                } else {
                    let atlas_tile_index = mmu.read_byte(tile_address);
                    0x9000 + atlas_tile_index as u16 * 16
                };

                for i in 0..16 {
                    self.tile_cache[i] = mmu.read_byte(atlas_tile_address as u16 + i as u16);
                }
                last_tile_address = tile_address;
            }

            // FETCH COLOR
            let color_palette_index =
                self.get_color_palette_index_of_tile_pixel(tile_pixel_x, tile_pixel_y);
            let color = self.get_bg_color(color_palette_index);

            // RENDER
            self.front_buffer[pixel_x as usize][scanline as usize] = color_palette_index;
        }
    }

    fn get_color_palette_index_of_tile_pixel(&self, x: i16, y: i16) -> u16 {
        let mut byte1 = self.tile_cache[(y * 2) as usize];
        let mut byte2 = self.tile_cache[(y * 2 + 1) as usize];
        byte1 = byte1 >> 7 - x & 0x1;
        byte2 = byte2 >> 7 - x & 0x1;
        (byte1 as u16) | (byte2 as u16) << 1
    }

    fn render_window(&mut self, scanline: u8, atlas_address_mode: bool) {}

    fn render_objects(&mut self, scanline: u8) {}

    fn is_object_visible(obj_x: i16, obj_y: i16, obj_height: i16, scanline: u8) -> bool {
        let y = obj_y - 16;
        let x = obj_x - 8;
        let line = scanline as i16;
        x >= -8 && x < 160 + 8 && line >= y && line < y + obj_height
    }

    fn get_bg_color(&self, color_index: u16) -> (f32, f32, f32, f32) {
        let palette = self.read_byte(memory::BGP);
        match color_index {
            3 => COLORS[(palette as usize) >> 6],
            2 => COLORS[(palette as usize) >> 4 & 0b11],
            1 => COLORS[(palette as usize) >> 2 & 0b11],
            0 => COLORS[(palette as usize) & 0b11],
            _ => {
                panic!("invalid color_index {}", color_index)
            }
        }
    }

    fn get_object_color(mmu: &Mmu, palette_address: u16, color_index: u16) -> (f32, f32, f32, f32) {
        let palette = mmu.read_byte(palette_address);
        match color_index {
            3 => COLORS[(palette >> 6) as usize],
            2 => COLORS[(palette >> 4 & 0b11) as usize],
            1 => COLORS[(palette >> 2 & 0b11) as usize],
            0 => TRANSPARENT,
            _ => {
                panic!("invalid palette_address")
            }
        }
    }

    fn h_blank(&mut self, mmu: &mut Mmu) {
        let current_line = self.read_byte(memory::LCD_LY);
        self.set_line(mmu, current_line + 1);
        if current_line + 1 > 143 {
            self.set_state(mmu, PpuMode::VBlank);
        } else {
            self.set_state(mmu, PpuMode::OamSearch);
        }
        self.cycle_accumulator -= 204;
    }

    fn v_blank(&mut self, mmu: &mut Mmu) {
        let current_line = self.read_byte(memory::LCD_LY);
        if current_line + 1 > 153 {
            self.set_line(mmu, 0);
            self.set_state(mmu, PpuMode::OamSearch);
        } else {
            self.set_line(mmu, current_line + 1);
        }
        self.cycle_accumulator -= 456;
    }

    fn set_state(&mut self, mmu: &mut Mmu, mode: PpuMode) {
        self.state = mode;

        // SET MODE BITS IN LCD_STAT REGISTER
        self.lcd_stat &= !0b11;
        self.lcd_stat |= self.state.get_flag_bits();

        match self.state {
            PpuMode::OamSearch => {
                // REQUEST INTERRUPT IF OAM SEARCH IS ENABLED AS SOURCE OF INTERRUPTS
                if self.is_bit_set(memory::LCD_STAT, 5) {
                    mmu.set_bit(memory::IF, 1, true);
                }
            }
            PpuMode::PixelTransfer => {}
            PpuMode::HBlank => {
                // REQUEST INTERRUPT IF HBLANK IS ENABLED AS SOURCE OF INTERRUPTS
                if self.is_bit_set(memory::LCD_STAT, 3) {
                    mmu.set_bit(memory::IF, 1, true);
                }
            }
            PpuMode::VBlank => {
                // REQUEST INTERRUPT IF VBLANK IS ENABLED AS SOURCE OF INTERRUPTS
                if self.is_bit_set(memory::LCD_STAT, 4) {
                    mmu.set_bit(memory::IF, 1, true);
                }
                mmu.set_bit(memory::IF, 0, true);
            }
        }
    }

    fn set_line(&mut self, mmu: &mut Mmu, line: u8) {
        self.write_byte(memory::LCD_LY, line);
        let lyc = mmu.read_byte(memory::LCD_LYC);
        self.set_bit(memory::LCD_STAT, 2, line == lyc);

        // REQUEST INTERRUPT IF LYC = LY
        if self.is_bit_set(memory::LCD_STAT, 6) && lyc == line {
            mmu.set_bit(memory::IF, 1, true);
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

#[derive(Clone, PartialEq)]
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
