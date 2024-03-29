use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn run_instruction(&mut self, mmu: &mut Mmu, op_code: u8) -> isize {
        match op_code {
            0x00 => 4, // NOP
            0x01 => self.ld_bc_d16(mmu),
            0x02 => self.ld_bc_a(mmu),
            0x03 => self.inc_bc(),
            0x04 => self.inc_b(),
            0x05 => self.dec_b(),
            0x06 => self.ld_b_d8(mmu),
            0x07 => self.rlc_a(),
            0x08 => self.ld_a16_sp(mmu),
            0x09 => self.add_hl_bc(),
            0x0A => self.ld_a_bc(mmu),
            0x0B => self.dec_bc(),
            0x0C => self.inc_c(),
            0x0D => self.dec_c(),
            0x0E => self.ld_c_d8(mmu),
            0x0F => self.rrc_a(),

            0x10 => self.stop(),
            0x11 => self.ld_de_d16(mmu),
            0x12 => self.ld_de_a(mmu),
            0x13 => self.inc_de(),
            0x14 => self.inc_d(),
            0x15 => self.dec_d(),
            0x16 => self.ld_d_d8(mmu),
            0x17 => self.rla(),
            0x18 => self.jr_r8(mmu),
            0x19 => self.add_hl_de(),
            0x1A => self.ld_a_de(mmu),
            0x1B => self.dec_de(),
            0x1C => self.inc_e(),
            0x1D => self.dec_e(),
            0x1E => self.ld_e_d8(mmu),
            0x1F => self.rra(),

            0x20 => self.jr_nz_r8(mmu),
            0x21 => self.ld_hl_d16(mmu),
            0x22 => self.ld_hlplus_a(mmu),
            0x23 => self.inc_hl(),
            0x24 => self.inc_h(),
            0x25 => self.dec_h(),
            0x26 => self.ld_h_d8(mmu),
            0x27 => self.da_a(),
            0x28 => self.jr_z_r8(mmu),
            0x29 => self.add_hl_hl(),
            0x2A => self.ld_a_hplus(mmu),
            0x2B => self.dec_hl(),
            0x2C => self.inc_l(),
            0x2D => self.dec_l(),
            0x2E => self.ld_l_d8(mmu),
            0x2F => self.cpl_a(),

            0x30 => self.jr_nc_r8(mmu),
            0x31 => self.ld_sp_d16(mmu),
            0x32 => self.ld_hlminus_a(mmu),
            0x33 => self.inc_sp(),
            0x34 => self.inc_hl_(mmu),
            0x35 => self.dec_hl_(mmu),
            0x36 => self.ld_hl_d8(mmu),
            0x37 => self.scf(),
            0x38 => self.jr_c_r8(mmu),
            0x39 => self.add_hl_sp(),
            0x3A => self.ld_a_hlminus(mmu),
            0x3B => self.dec_sp(),
            0x3C => self.inc_a(),
            0x3D => self.dec_a(),
            0x3E => self.ld_a_d8(mmu),
            0x3F => self.ccf(),

            0x40 => self.ld_b_b(),
            0x41 => self.ld_b_c(),
            0x42 => self.ld_b_d(),
            0x43 => self.ld_b_e(),
            0x44 => self.ld_b_h(),
            0x45 => self.ld_b_l(),
            0x46 => self.ld_b_hl(mmu),
            0x47 => self.ld_b_a(),
            0x48 => self.ld_c_b(),
            0x49 => self.ld_c_c(),
            0x4A => self.ld_c_d(),
            0x4B => self.ld_c_e(),
            0x4C => self.ld_c_h(),
            0x4D => self.ld_c_l(),
            0x4E => self.ld_c_hl(mmu),
            0x4F => self.ld_c_a(),

            0x50 => self.ld_d_b(),
            0x51 => self.ld_d_c(),
            0x52 => self.ld_d_d(),
            0x53 => self.ld_d_e(),
            0x54 => self.ld_d_h(),
            0x55 => self.ld_d_l(),
            0x56 => self.ld_d_hl(mmu),
            0x57 => self.ld_d_a(),
            0x58 => self.ld_e_b(),
            0x59 => self.ld_e_c(),
            0x5A => self.ld_e_d(),
            0x5B => self.ld_e_e(),
            0x5C => self.ld_e_h(),
            0x5D => self.ld_e_l(),
            0x5E => self.ld_e_hl(mmu),
            0x5F => self.ld_e_a(),

            0x60 => self.ld_h_b(),
            0x61 => self.ld_h_c(),
            0x62 => self.ld_h_d(),
            0x63 => self.ld_h_e(),
            0x64 => 4, // ld_h_h
            0x65 => self.ld_h_l(),
            0x66 => self.ld_h_hl(mmu),
            0x67 => self.ld_h_a(),
            0x68 => self.ld_l_b(),
            0x69 => self.ld_l_c(),
            0x6A => self.ld_l_d(),
            0x6B => self.ld_l_e(),
            0x6C => self.ld_l_h(),
            0x6D => 4, // ld_l_l
            0x6E => self.ld_l_hl(mmu),
            0x6F => self.ld_l_a(),

            0x70 => self.ld_hl_b(mmu),
            0x71 => self.ld_hl_c(mmu),
            0x72 => self.ld_hl_d(mmu),
            0x73 => self.ld_hl_e(mmu),
            0x74 => self.ld_hl_h(mmu),
            0x75 => self.ld_hl_l(mmu),
            0x76 => {
                self.halt();
                4
            }
            0x77 => self.ld_hl_a(mmu),
            0x78 => self.ld_a_b(),
            0x79 => self.ld_a_c(),
            0x7A => self.ld_a_d(),
            0x7B => self.ld_a_e(),
            0x7C => self.ld_a_h(),
            0x7D => self.ld_a_l(),
            0x7E => self.ld_a_hl(mmu),
            0x7F => 4, // ld_a_a

            0x80 => self.add_a_b(),
            0x81 => self.add_a_c(),
            0x82 => self.add_a_d(),
            0x83 => self.add_a_e(),
            0x84 => self.add_a_h(),
            0x85 => self.add_a_l(),
            0x86 => self.add_a_hl(mmu),
            0x87 => self.add_a_a(),
            0x88 => self.adc_a_b(),
            0x89 => self.adc_a_c(),
            0x8A => self.adc_a_d(),
            0x8B => self.adc_a_e(),
            0x8C => self.adc_a_h(),
            0x8D => self.adc_a_l(),
            0x8E => self.adc_a_hl(mmu),
            0x8F => self.adc_a_a(),

            0x90 => self.sub_b(),
            0x91 => self.sub_c(),
            0x92 => self.sub_d(),
            0x93 => self.sub_e(),
            0x94 => self.sub_h(),
            0x95 => self.sub_l(),
            0x96 => self.sub_hl(mmu),
            0x97 => self.sub_a(),
            0x98 => self.sbc_a_b(),
            0x99 => self.sbc_a_c(),
            0x9A => self.sbc_a_d(),
            0x9B => self.sbc_a_e(),
            0x9C => self.sbc_a_h(),
            0x9D => self.sbc_a_l(),
            0x9E => self.sbc_a_hl(mmu),
            0x9F => self.sbc_a_a(),

            0xA0 => self.and_b(),
            0xA1 => self.and_c(),
            0xA2 => self.and_d(),
            0xA3 => self.and_e(),
            0xA4 => self.and_h(),
            0xA5 => self.and_l(),
            0xA6 => self.and_hl(mmu),
            0xA7 => self.and_a(),
            0xA8 => self.xor_b(),
            0xA9 => self.xor_c(),
            0xAA => self.xor_d(),
            0xAB => self.xor_e(),
            0xAC => self.xor_h(),
            0xAD => self.xor_l(),
            0xAE => self.xor_hl(mmu),
            0xAF => self.xor_a(),

            0xB0 => self.or_b(),
            0xB1 => self.or_c(),
            0xB2 => self.or_d(),
            0xB3 => self.or_e(),
            0xB4 => self.or_h(),
            0xB5 => self.or_l(),
            0xB6 => self.or_hl(mmu),
            0xB7 => self.or_a(),
            0xB8 => self.cp_b(),
            0xB9 => self.cp_c(),
            0xBA => self.cp_d(),
            0xBB => self.cp_e(),
            0xBC => self.cp_h(),
            0xBD => self.cp_l(),
            0xBE => self.cp_hl(mmu),
            0xBF => self.cp_a(),

            0xC0 => self.ret_nz(mmu),
            0xC1 => self.pop_bc(mmu),
            0xC2 => self.jp_nz_a16(mmu),
            0xC3 => self.jp_a16(mmu),
            0xC4 => self.call_nz_a16(mmu),
            0xC5 => self.push_bc(mmu),
            0xC6 => self.add_a_d8(mmu),
            0xC7 => self.rst_00h(mmu),
            0xC8 => self.ret_z(mmu),
            0xC9 => self.ret(mmu),
            0xCA => self.jp_z_a16(mmu),
            0xCB => self.run_cb_instruction(mmu),
            0xCC => self.call_z_a16(mmu),
            0xCD => self.call_a16(mmu),
            0xCE => self.adc_a_d8(mmu),
            0xCF => self.rst_08h(mmu),

            0xD0 => self.ret_nc(mmu),
            0xD1 => self.pop_de(mmu),
            0xD2 => self.jp_nc_a16(mmu),
            0xD4 => self.call_nc_a16(mmu),
            0xD5 => self.push_de(mmu),
            0xD6 => self.sub_d8(mmu),
            0xD7 => self.rst_10h(mmu),
            0xD8 => self.ret_c(mmu),
            0xD9 => self.ret_i(mmu),
            0xDA => self.jp_c_a16(mmu),
            0xDC => self.call_c_a16(mmu),
            0xDE => self.sbc_a_d8(mmu),
            0xDF => self.rst_18h(mmu),

            0xE0 => self.ldh_a8_a(mmu),
            0xE1 => self.pop_hl(mmu),
            0xE2 => self.ld__c__a(mmu),
            0xE5 => self.push_hl(mmu),
            0xE6 => self.and_d8(mmu),
            0xE7 => self.rst_20h(mmu),
            0xE8 => self.add_sp_r8(mmu),
            0xE9 => self.jp_hl(),
            0xEA => self.ld_a16_a(mmu),
            0xEE => self.xor_d8(mmu),
            0xEF => self.rst_28h(mmu),

            0xF0 => self.ldh_a_a8(mmu),
            0xF1 => self.pop_af(mmu),
            0xF2 => self.ld_a_c_(mmu),
            0xF3 => self.di(),
            0xF5 => self.push_af(mmu),
            0xF6 => self.or_d8(mmu),
            0xF7 => self.rst_30h(mmu),
            0xF8 => self.ld_hl_spplus_r8(mmu),
            0xF9 => self.ld_sp_hl(),
            0xFA => self.ld_a_a16(mmu),
            0xFB => self.ei(),
            0xFE => self.cp_d8(mmu),
            0xFF => self.rst_38h(mmu),

            _ => panic!("unsupported opcode: {op_code}"),
        }
    }
}
