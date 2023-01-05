use std::fmt::Display;

use crate::gameboy::util::bit_util;

/// Joypad State register
/// Bit 7 = unused
/// Bit 6 = unused
/// Bit 5 = Select action buttons : 0=selected, 1=not selected
/// Bit 4 = Select direction buttons : 0=selected, 1=not selected
/// Bit 3 = Down or Start : 0=pressed, 1=not pressed
/// Bit 2 = Up or Select : 0=pressed, 1=not pressed
/// Bit 1 = Left or B : 0=pressed, 1=not pressed
/// Bit 0 = Right or A : 0=pressed, 1=not pressed
pub const JOYPAD: u16 = 0xFF00;

/// Serial transfer data register
/// Before a transfer, it holds the next byte that will go out.
/// During a transfer, it has a blend of the outgoing and incoming bytes. Each cycle, the leftmost bit is shifted out (and over the wire) and the incoming
/// bit is shifted in from the other side.
pub const SB: u16 = 0xFF01;

/// Serial transfer control register
/// Bit 7 = Transfer Start Flag (0=No transfer is in progress or requested, 1=Transfer in progress, or request
/// Bit 1 = Clock Speed (0=Normal, 1=Fast) ** CGB Mode Only **
/// Bit 0 = Shift Clock (0=External Clock, 1=Internal Clock)
pub const SC: u16 = 0xFF02;

/// This register is incremented at a rate of 16384Hz (~16779Hz on SGB).
/// Writing any value to this register resets it to $00.
/// Additionally, this register is reset when executing the stop instruction, and only begins ticking again once stop mode ends.
pub const DIV: u16 = 0xFF04;

/// Timer Counter register
///
/// This timer is incremented at the clock frequency specified by the TAC register ($FF07).
/// When the value overflows (exceeds $FF) it is reset to the value specified in TMA (FF06) and an pub consterrupt is requested.
pub const TIMA: u16 = 0xFF05;

/// Timer Modulo register
///
/// When TIMA overflows, it is reset to the value in this register and an pub consterrupt is requested. Example of use: if TMA is set to $FF, an pub consterrupt is
/// requested at the clock frequency selected in TAC (because every increment is an overflow). However, if TMA is set to $FE, an pub consterrupt is only requested
/// every two increments, which effectively divides the selected clock by two. Setting TMA to $FD would divide the clock by three, and so on.
///
/// If a TMA write is executed on the same cycle as the content of TMA is transferred to TIMA due to a timer overflow, the old value is transferred to TIMA.
pub const TMA: u16 = 0xFF06;

/// Timer Control register
///
/// Bit 2 = Timer Enable : 0=off, 1=on
/// Bit 1, 0 = Input Clock Select :
/// 00 = CPU Clock / 1024
/// 01 = CPU CLock / 16
/// 10 = CPU Clock / 64
/// 11 = CPU Clock / 256
pub const TAC: u16 = 0xFF07;

/// Interrupt Flag register
/// Bit 4 = Joypad : 0=off, 1=request
/// Bit 3 = Serial : 0=off, 1=request
/// Bit 2 = Timer : 0=off, 1=request
/// Bit 1 = LCD_STAT : 0=off, 1=request
/// Bit 0 = VBlank : 0=off, 1=request
pub const IF: u16 = 0xFF0F;

pub const NR10: u16 = 0xFF10;

pub const NR11: u16 = 0xFF11;

pub const NR12: u16 = 0xFF12;

pub const NR13: u16 = 0xFF13;

pub const NR14: u16 = 0xFF14;

pub const NR20: u16 = 0xFF15;

pub const NR21: u16 = 0xFF16;

pub const NR22: u16 = 0xFF17;

pub const NR23: u16 = 0xFF18;

pub const NR24: u16 = 0xFF19;

pub const NR30: u16 = 0xFF1A;

pub const NR31: u16 = 0xFF1B;

pub const NR32: u16 = 0xFF1C;

pub const NR33: u16 = 0xFF1D;

pub const NR34: u16 = 0xFF1E;

pub const NR40: u16 = 0xFF1F;

pub const NR41: u16 = 0xFF20;

pub const NR42: u16 = 0xFF21;

pub const NR43: u16 = 0xFF22;

pub const NR44: u16 = 0xFF23;

pub const NR50: u16 = 0xFF24;

pub const NR51: u16 = 0xFF25;

pub const NR52: u16 = 0xFF26;

/// LCD Control register
/// Bit 7 = LCD and GPU enable : 0=off, 1=on
/// Bit 6 = Window tile map area : 0=0x9800-0x9BFF, 1=0x9C00-0x9FFF
/// Bit 5 = Window enable : 0=off, 1=on
/// Bit 4 = Background and Window tile data area : 0=0x8800-0x97FF, 1=0x8000-0x8FFF
/// Bit 3 = Background tile map area : 0=0x9800-0x9BFF, 1=0x9C00-0x9FFF
/// Bit 2 = Object size : 0=8x8, 1=8x16
/// Bit 1 = Object enable: 0=off, 1=on
/// Bit 0 = Background and Window enable/priority : 0=off, 1=on
pub const LCDC: u16 = 0xFF40;

/// LCD Status register
/// Bit 7 = unused
/// Bit 6 = LY pub consterrupt source : 0=off, 1=on
/// Bit 5 = Mode 2 OAM pub consterrupt source : 0=off, 1=on
/// Bit 4 = Mode 1 VBlank pub consterrupt source : 0=off, 1=on
/// Bit 3 = Mode 0 HBlank pub consterrupt source : 0=off, 1=on
/// Bit 2 = LY Flag : 0=different, 1=equal
/// Bit 1, 0 = Mode Flag : 00=HBlank, 01=VBlank, 10=OAMSearch, 11=PixelTransfer
pub const LCD_STAT: u16 = 0xFF41;

/// Background scroll Y register
pub const SCROLL_Y: u16 = 0xFF42;

/// Background scroll X register
pub const SCROLL_X: u16 = 0xFF43;

/// LCD Y line register
/// LY indicates the current horizontal line, which might be about to be drawn, being drawn, or just been drawn. LY can hold any value from 0 to 153, with
/// values from 144 to 153 indicating the VBlank period.
pub const LCD_LY: u16 = 0xFF44;

/// LCD Compare Y line register
/// The Game Boy permanently compares the value of the LYC and LY registers. When both values are identical, the “LYC=LY” flag in the STAT register is set,
/// and (if enabled) a STAT pub consterrupt is requested.
pub const LCD_LYC: u16 = 0xFF45;

/// Writing to this register launches a DMA transfer from ROM or RAM to OAM (Object Attribute Memory). The written value specifies the transfer source
/// address divided by $100, that is, source and destination are:
/// Source: $XX00-$XX9F ;XX = $00 to $DF Destination: $FE00-$FE9F
pub const DMA: u16 = 0xFF46;

/// BG Palette Data register
/// Bit 7, 6 = Color for index 3
/// Bit 5, 4 = Color for index 2
/// Bit 3, 2 = Color for index 1
/// Bit 1, 0 = Color for index 0
pub const BGP: u16 = 0xFF47;

pub const OBP0: u16 = 0xFF48;

pub const OBP1: u16 = 0xFF49;

/// Window Y Position
pub const WY: u16 = 0xFF4A;

/// Window X Position + 7
pub const WX: u16 = 0xFF4B;

pub const KEY1: u16 = 0xFF4D;

pub const VBK: u16 = 0xFF4F;

/// Set to non-zero to disable Boot Rom
pub const DISABLE_BOOT_ROM: u16 = 0xFF50;

pub const HDMA1: u16 = 0xFF51;

pub const HDMA2: u16 = 0xFF52;

pub const HDMA3: u16 = 0xFF53;

pub const HDMA4: u16 = 0xFF54;

pub const HDMA5: u16 = 0xFF55;

pub const RP: u16 = 0xFF56;

pub const BCPS: u16 = 0xFF68;

pub const BCPD: u16 = 0xFF69;

pub const OCPS: u16 = 0xFF6A;

pub const OCPD: u16 = 0xFF6B;

pub const SVBK: u16 = 0xFF70;

/// Interrupt Enable register
/// Bit 4 = Joypad : 0=disabled, 1=enabled
/// Bit 3 = Serial : 0=disabled, 1=enabled
/// Bit 2 = Timer : 0=disabled, 1=enabled
/// Bit 1 = LCD STAT : 0=disabled, 1=enabled
/// Bit 0 = VBlank : 0=disabled, 1=enabled
pub const IE: u16 = 0xFFFF;

pub trait Memory: Display {
    fn accepts_address(&self, address: u16) -> bool;

    fn read_byte(&self, address: u16) -> u8;

    fn read_word(&self, address: u16) -> u16 {
        let second_byte_address = address + 1;
        (self.read_byte(address) as u16) | ((self.read_byte(second_byte_address) as u16) << 8)
    }

    fn write_byte(&mut self, address: u16, value: u8);

    fn is_bit_set(&self, address: u16, bitnum: usize) -> bool {
        bit_util::is_bit_set_u8(self.read_byte(address), bitnum)
    }

    fn set_bit(&mut self, address: u16, bitnum: usize, value: bool) {
        let result = bit_util::set_bit_u8(self.read_byte(address), bitnum, value);
        self.write_byte(address, result);
    }
}
