use std::fmt::{Display, Formatter};

use crate::gameboy::memory::mmu::Mmu;
use crate::gameboy::ram::memory;
use crate::gameboy::ram::memory::Memory;
use crate::gameboy::util::color::Color;

pub struct Ppu {
    pub front_buffer: Box<[[Color; 160]; 144]>,
    back_buffer: Box<[[u16; 160]; 144]>,
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
    tile_cache: [i32; 16],
}

const WHITE: Color = Color {
    r: 1f32,
    g: 1f32,
    b: 1f32,
    a: 1f32,
};
const LIGHT: Color = Color {
    r: 0.6f32,
    g: 0.6f32,
    b: 0.6f32,
    a: 1f32,
};
const DARK: Color = Color {
    r: 0.3f32,
    g: 0.3f32,
    b: 0.3f32,
    a: 1f32,
};
const BLACK: Color = Color {
    r: 0f32,
    g: 0f32,
    b: 0f32,
    a: 1f32,
};
const TRANSPARENT: Color = Color {
    r: 1f32,
    g: 1f32,
    b: 1f32,
    a: 0f32,
};
const COLORS: [Color; 4] = [WHITE, LIGHT, DARK, BLACK];

impl Ppu {
    pub fn new() -> Self {
        Self {
            front_buffer: Box::new([[TRANSPARENT; 160]; 144]),
            back_buffer: Box::new([[0; 160]; 144]),
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
                    OAM_SEARCH => {
                        self.oam_search(mmu);
                    }
                    PixelTransfer => {
                        self.pixel_transfer(mmu);
                    }
                    HBlank => {
                        self.h_blank(mmu);
                    }
                    VBlank => {
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

        return false;
    }

    fn pixel_transfer(&mut self, mmu: &mut Mmu) {
        let render_bg = self.is_bit_set(memory::LCDC, 0);
        let render_window = self.is_bit_set(memory::LCDC, 5);
        let render_objects = self.is_bit_set(memory::LCDC, 1);
        let scanline = self.read_byte(memory::LCD_LY);
        let atlas_address_mode = self.is_bit_set(memory::LCDC, 4);

        if render_bg {
            self.render_bg(mmu, scanline, atlas_address_mode);

            if render_window {
                self.render_window(mmu, scanline, atlas_address_mode);
            }
        }

        if render_objects {
            self.render_objects(mmu, scanline);
        }

        self.cycle_accumulator -= 172;
        self.set_state(mmu, PpuMode::HBlank);
    }

    fn render_bg(&mut self, mmu: &Mmu, scanline: u8, atlas_address_mode: bool) {
        let bg_map_start_address: u16 = if self.is_bit_set(memory::LCDC, 3) {
            0x9C00
        } else {
            0x9800
        };
        let scroll_x: u8 = self.read_byte(memory::SCROLL_X);
        let bg_map_y: u8 = scanline + self.read_byte(memory::SCROLL_Y);
        let bg_map_block_y: u8 = bg_map_y / 8;
        let tile_pixel_y: u8 = bg_map_y % 8;

        let mut last_tile_address: u16 = u16::MAX;

        for pixel_x in 0u8..160 {
            // FIND TILE ADDRESS
            let mut bg_map_x: u8 = pixel_x + scroll_x;
            if bg_map_x >= 256 {
                bg_map_x -= 256;
            }
            let bg_map_block_x: u8 = bg_map_x / 8;
            let tile_pixel_x: u8 = bg_map_x % 8;
            let tile_address: u16 = bg_map_start_address + bg_map_block_y as u16 * 32 + bg_map_block_x as u16;

            // READ TILE DATA
            if tile_address != last_tile_address {
                let atlas_tile_address: u16 = if atlas_address_mode {
                    let atlas_tile_index = mmu.read_byte(tile_address as u16);
                    0x8000 + atlas_tile_index as u16 * 16
                } else {
                    let atlas_tile_index = mmu.read_byte(tile_address);
                    0x9000 + atlas_tile_index as u16 * 16
                };

                for i in 0..16 {
                    self.tile_cache[i] = mmu.read_byte(atlas_tile_address + i as u16) as i32;
                }
                last_tile_address = tile_address;
            }

            // FETCH COLOR
            let color_palette_index: u16 = self.get_color_palette_index_of_tile_pixel(tile_pixel_x as i16, tile_pixel_y as i16);
            let color: Color = self.get_bg_color(color_palette_index);

            // RENDER
            self.back_buffer[pixel_x as usize][scanline as usize] = color_palette_index;
            self.front_buffer[pixel_x as usize][scanline as usize] = color;
        }
    }

    fn render_window(&mut self, mmu: &Mmu, scanline: u8, atlas_address_mode: bool) {
        let window_map_start_address: u16 = if self.is_bit_set(memory::LCDC, 6) { 0x9C00 } else { 0x9800 };
        let wx: i16 = self.read_byte(memory::WX) as i16 - 7;
        let wy: i16 = self.read_byte(memory::WY) as i16;
        let mut last_tile_address: u16 = u16::MAX;

        if scanline as i16 >= wy {
            for x in wx..160 {
                // FIND TILE ADDRESS
                let window_line = scanline as i16 - wy;
                let pixel_x = x - wx;
                let tile_pixel_x = pixel_x % 8;
                let tile_pixel_y = window_line % 8;
                let tile_address: u16 = (window_map_start_address as i16 + window_line / 8 * 32 + pixel_x / 8) as u16;

                if pixel_x >= 0 && pixel_x < 160 {
                    // READ TILE DATA
                    if tile_address != last_tile_address {
                        let atlas_tile_index = mmu.read_byte(tile_address);
                        let atlas_tile_address: u16 = if atlas_address_mode { 0x8000 + atlas_tile_index as u16 * 16 } else {
                            0x9000 + atlas_tile_index as u16 * 16
                        };

                        for i in 0..16 {
                            self.tile_cache[i as usize] = mmu.read_byte(atlas_tile_address + i) as i32;
                        }
                        last_tile_address = tile_address;
                    }

                    // FETCH COLOR
                    let color_palette_index = self.get_color_palette_index_of_tile_pixel(tile_pixel_x, tile_pixel_y);
                    let color = self.get_bg_color(color_palette_index);

                    // RENDER
                    self.back_buffer[pixel_x as usize][scanline as usize] = color_palette_index;
                    self.front_buffer[pixel_x as usize][scanline as usize] = color;
                }
            }
        }
    }

    fn render_objects(&mut self, mmu: &Mmu, scanline: u8) {
        let obj_height: u8 = if self.is_bit_set(memory::LCDC, 2) {
            16
        } else {
            8
        };
        for i in 0..40 {
            let oam_address: u16 = 0xFE00 + i * 4;
            let obj_y: i16 = mmu.read_byte(oam_address) as i16 - 16;
            let obj_x: i16 = mmu.read_byte(oam_address + 1) as i16 - 8;
            if Ppu::is_object_visible(obj_x, obj_y, obj_height, scanline) {
                let obj_tile_index = mmu.read_byte(oam_address + 2);
                let obj_attributes = mmu.read_byte(oam_address + 3);
                let atlas_tile_address: u16 = 0x8000 + obj_tile_index as u16 * 16;
                let flip_x = (obj_attributes & 0b100000) > 0;
                let flip_y = (obj_attributes & 0b1000000) > 0;
                let priority = (obj_attributes & 0b10000000) == 0;
                let palette_address: u16 = if (obj_attributes & 0b10000) > 0 {
                    0xFF49
                } else {
                    0xFF48
                };
                let y_in_tile = scanline as i16 - obj_y;
                let y_in_first_tile = y_in_tile < 8;
                let atlas_address_modificator = if !y_in_first_tile && !flip_y || y_in_first_tile && obj_height == 16 && flip_y {
                    16
                } else {
                    0
                };
                let tile_pixel_y: i16 = if !y_in_first_tile && !flip_y {
                    y_in_tile - 8
                } else if y_in_first_tile && !flip_y {
                    y_in_tile
                } else if !y_in_first_tile && flip_y {
                    7 - (y_in_tile - 8)
                } else if y_in_first_tile && flip_y {
                    7 - y_in_tile
                } else {
                    y_in_tile
                };

                // FETCH TILE PIXELS FROM ATLAS
                for j in 0..16 {
                    self.tile_cache[j] = mmu.read_byte(atlas_tile_address + atlas_address_modificator + j as u16) as i32;
                }

                self.draw_object_line(
                    mmu,
                    obj_x,
                    tile_pixel_y,
                    flip_x,
                    priority,
                    scanline,
                    palette_address,
                );
            }
        }
    }

    fn is_object_visible(obj_x: i16, obj_y: i16, obj_height: u8, scanline: u8) -> bool {
        let line = scanline as i16;
        (-8..168).contains(&obj_x) && line >= obj_y && line < obj_y + obj_height as i16
    }

    fn draw_object_line(
        &mut self,
        mmu: &Mmu,
        x: i16,
        tile_pixel_y: i16,
        flip_x: bool,
        priority: bool,
        scanline: u8,
        palette_address: u16,
    ) {
        for pixel_x in 0..8 {
            let tile_pixel_x = if flip_x { 7 - pixel_x } else { pixel_x };
            let color_palette_index = self.get_color_palette_index_of_tile_pixel(tile_pixel_x, tile_pixel_y);
            let color = Ppu::get_object_color(mmu, palette_address, color_palette_index);

            let render_x = x + pixel_x;
            if render_x >= 160 {
                return;
            }
            if render_x >= 0 && (priority || self.back_buffer[render_x as usize][scanline as usize] == 0) {
                self.front_buffer[(x + pixel_x) as usize][scanline as usize] = color;
            }
        }
    }

    fn get_object_color(mmu: &Mmu, palette_address: u16, color_index: u16) -> Color {
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

    fn get_bg_color(&self, color_index: u16) -> Color {
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

    fn get_color_palette_index_of_tile_pixel(&self, x: i16, y: i16) -> u16 {
        let mut byte1 = self.tile_cache[(y * 2) as usize];
        let mut byte2 = self.tile_cache[(y * 2 + 1) as usize];
        byte1 = byte1 >> (7 - x) & 0x1;
        byte2 = byte2 >> (7 - x) & 0x1;
        (byte1 as u16) | (byte2 as u16) << 1
    }

    fn oam_search(&mut self, mmu: &mut Mmu) {
        self.cycle_accumulator -= 80;
        self.set_state(mmu, PpuMode::PixelTransfer);
    }

    fn h_blank(&mut self, mmu: &mut Mmu) {
        let scanline = self.read_byte(memory::LCD_LY);
        self.set_line(mmu, scanline + 1);
        if scanline + 1 > 143 {
            self.set_state(mmu, PpuMode::VBlank);
        } else {
            self.set_state(mmu, PpuMode::OamSearch);
        }
        self.cycle_accumulator -= 204;
    }

    fn v_blank(&mut self, mmu: &mut Mmu) {
        let scanline = self.read_byte(memory::LCD_LY);
        if scanline + 1 > 153 {
            self.set_line(mmu, 0);
            self.set_state(mmu, PpuMode::OamSearch);
        } else {
            self.set_line(mmu, scanline + 1);
        }
        self.cycle_accumulator -= 456;
    }

    fn set_line(&mut self, mmu: &mut Mmu, number: u8) {
        self.write_byte(memory::LCD_LY, number);
        let lyc: u8 = mmu.read_byte(memory::LCD_LYC);
        self.set_bit(memory::LCD_STAT, 2, number == lyc);

        // REQUEST INTERRUPT IF LYC = LY
        if self.is_bit_set(memory::LCD_STAT, 6) && number == lyc {
            mmu.set_bit(memory::IF, 1, true);
        }
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
}

impl Memory for Ppu {
    fn accepts_address(&self, address: u16) -> bool {
        address == memory::LCDC || address == memory::LCD_STAT || address == memory::LCD_LY || address == memory::LCD_LYC || address == memory::SCROLL_X || address == memory::SCROLL_Y || address == memory::BGP || address == memory::OBP0 || address == memory::OBP1 || address == memory::WX || address == memory::WY
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

impl Display for Ppu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ppu")
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
