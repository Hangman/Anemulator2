use crate::gameboy::cpu::instructions::{inc, load};
use crate::gameboy::cpu::registers::Registers;
use crate::gameboy::memory::mmu::Mmu;

pub fn run_instruction(register: &mut Registers, mmu: &mut Mmu, op_code: u8) -> isize {
    match op_code {
        0x00 => 4, // NOP
        0x01 => load::ld_bc_d16(register, mmu),
        0x02 => load::ld_bc_a(register, mmu),
        0x03 => inc::inc_bc(register),
        0x04 => inc::inc_b(register),
        0x05 => todo!(),
        0x06 => todo!(),
        0x07 => todo!(),
        0x08 => todo!(),
        0x09 => todo!(),
        0x0A => todo!(),
        0x0B => todo!(),
        0x0C => todo!(),
        0x0D => todo!(),
        0x0E => todo!(),
        0x0F => todo!(),

        0x10 => todo!(),
        0x11 => todo!(),
        0x12 => todo!(),
        0x13 => todo!(),
        0x14 => todo!(),
        0x15 => todo!(),
        0x16 => todo!(),
        0x17 => todo!(),
        0x18 => todo!(),
        0x19 => todo!(),
        0x1A => todo!(),
        0x1B => todo!(),
        0x1C => todo!(),
        0x1D => todo!(),
        0x1E => todo!(),
        0x1F => todo!(),

        0x20 => todo!(),
        0x21 => todo!(),
        0x22 => todo!(),
        0x23 => todo!(),
        0x24 => todo!(),
        0x25 => todo!(),
        0x26 => todo!(),
        0x27 => todo!(),
        0x28 => todo!(),
        0x29 => todo!(),
        0x2A => todo!(),
        0x2B => todo!(),
        0x2C => todo!(),
        0x2D => todo!(),
        0x2E => todo!(),
        0x2F => todo!(),

        0x30 => todo!(),
        0x31 => todo!(),
        0x32 => todo!(),
        0x33 => todo!(),
        0x34 => todo!(),
        0x35 => todo!(),
        0x36 => todo!(),
        0x37 => todo!(),
        0x38 => todo!(),
        0x39 => todo!(),
        0x3A => todo!(),
        0x3B => todo!(),
        0x3C => todo!(),
        0x3D => todo!(),
        0x3E => todo!(),
        0x3F => todo!(),

        0x40 => load::ld_b_b(),
        0x41 => load::ld_b_c(register),
        0x42 => load::ld_b_d(register),
        0x43 => load::ld_b_e(register),
        0x44 => load::ld_b_h(register),
        0x45 => load::ld_b_l(register),
        0x46 => todo!(),
        0x47 => todo!(),
        0x48 => load::ld_c_b(register),
        0x49 => load::ld_c_c(),
        0x4A => load::ld_c_d(register),
        0x4B => load::ld_c_e(register),
        0x4C => load::ld_c_h(register),
        0x4D => load::ld_c_l(register),
        0x4E => todo!(),
        0x4F => load::ld_c_a(register),

        0x50 => load::ld_d_b(register),
        0x51 => load::ld_d_c(register),
        0x52 => load::ld_d_d(),
        0x53 => load::ld_d_e(register),
        0x54 => load::ld_d_h(register),
        0x55 => load::ld_d_l(register),
        0x56 => todo!(),
        0x57 => load::ld_d_a(register),
        0x58 => load::ld_e_b(register),
        0x59 => load::ld_e_c(register),
        0x5A => load::ld_e_d(register),
        0x5B => load::ld_e_e(),
        0x5C => load::ld_e_h(register),
        0x5D => load::ld_e_l(register),
        0x5E => todo!(),
        0x5F => load::ld_e_a(register),

        0x60 => load::ld_h_b(register),
        0x61 => load::ld_h_c(register),
        0x62 => load::ld_h_d(register),
        0x63 => load::ld_h_e(register),
        0x64 => load::ld_h_h(),
        0x65 => load::ld_h_l(register),
        0x66 => todo!(),
        0x67 => load::ld_h_a(register),
        0x68 => load::ld_l_b(register),
        0x69 => load::ld_l_c(register),
        0x6A => load::ld_l_d(register),
        0x6B => load::ld_l_e(register),
        0x6C => load::ld_l_h(register),
        0x6D => load::ld_l_l(),
        0x6E => todo!(),
        0x6F => load::ld_l_a(register),

        0x70 => todo!(),
        0x71 => todo!(),
        0x72 => todo!(),
        0x73 => todo!(),
        0x74 => todo!(),
        0x75 => todo!(),
        0x76 => todo!(),
        0x77 => todo!(),
        0x78 => load::ld_a_b(register),
        0x79 => load::ld_a_c(register),
        0x7A => load::ld_a_d(register),
        0x7B => load::ld_a_e(register),
        0x7C => load::ld_a_h(register),
        0x7D => load::ld_a_l(register),
        0x7E => todo!(),
        0x7F => load::ld_a_a(),

        0x80 => todo!(),
        0x81 => todo!(),
        0x82 => todo!(),
        0x83 => todo!(),
        0x84 => todo!(),
        0x85 => todo!(),
        0x86 => todo!(),
        0x87 => todo!(),
        0x88 => todo!(),
        0x89 => todo!(),
        0x8A => todo!(),
        0x8B => todo!(),
        0x8C => todo!(),
        0x8D => todo!(),
        0x8E => todo!(),
        0x8F => todo!(),

        0x90 => todo!(),
        0x91 => todo!(),
        0x92 => todo!(),
        0x93 => todo!(),
        0x94 => todo!(),
        0x95 => todo!(),
        0x96 => todo!(),
        0x97 => todo!(),
        0x98 => todo!(),
        0x99 => todo!(),
        0x9A => todo!(),
        0x9B => todo!(),
        0x9C => todo!(),
        0x9D => todo!(),
        0x9E => todo!(),
        0x9F => todo!(),

        0xA0 => todo!(),
        0xA1 => todo!(),
        0xA2 => todo!(),
        0xA3 => todo!(),
        0xA4 => todo!(),
        0xA5 => todo!(),
        0xA6 => todo!(),
        0xA7 => todo!(),
        0xA8 => todo!(),
        0xA9 => todo!(),
        0xAA => todo!(),
        0xAB => todo!(),
        0xAC => todo!(),
        0xAD => todo!(),
        0xAE => todo!(),
        0xAF => todo!(),

        0xB0 => todo!(),
        0xB1 => todo!(),
        0xB2 => todo!(),
        0xB3 => todo!(),
        0xB4 => todo!(),
        0xB5 => todo!(),
        0xB6 => todo!(),
        0xB7 => todo!(),
        0xB8 => todo!(),
        0xB9 => todo!(),
        0xBA => todo!(),
        0xBB => todo!(),
        0xBC => todo!(),
        0xBD => todo!(),
        0xBE => todo!(),
        0xBF => todo!(),

        0xC0 => todo!(),
        0xC1 => todo!(),
        0xC2 => todo!(),
        0xC3 => todo!(),
        0xC4 => todo!(),
        0xC5 => todo!(),
        0xC6 => todo!(),
        0xC7 => todo!(),
        0xC8 => todo!(),
        0xC9 => todo!(),
        0xCA => todo!(),
        0xCB => todo!(),
        0xCC => todo!(),
        0xCD => todo!(),
        0xCE => todo!(),
        0xCF => todo!(),

        0xD0 => todo!(),
        0xD1 => todo!(),
        0xD2 => todo!(),
        0xD4 => todo!(),
        0xD5 => todo!(),
        0xD6 => todo!(),
        0xD7 => todo!(),
        0xD8 => todo!(),
        0xD9 => todo!(),
        0xDA => todo!(),
        0xDC => todo!(),
        0xDE => todo!(),
        0xDF => todo!(),

        0xE0 => todo!(),
        0xE1 => todo!(),
        0xE2 => todo!(),
        0xE5 => todo!(),
        0xE6 => todo!(),
        0xE7 => todo!(),
        0xE8 => todo!(),
        0xE9 => todo!(),
        0xEA => todo!(),
        0xEE => todo!(),
        0xEF => todo!(),

        0xF0 => todo!(),
        0xF1 => todo!(),
        0xF2 => todo!(),
        0xF3 => todo!(),
        0xF5 => todo!(),
        0xF6 => todo!(),
        0xF7 => todo!(),
        0xF8 => todo!(),
        0xF9 => todo!(),
        0xFA => todo!(),
        0xFB => todo!(),
        0xFE => todo!(),
        0xFF => todo!(),

        _ => panic!("unsupported opcode: {}", op_code),
    }
}
