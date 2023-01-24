use std::mem;

use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn ld_hl_l(&self, mmu: &mut Mmu) -> isize {
        mmu.write_byte(self.register.get_hl(), self.register.l);
        8
    }

    pub fn ld_hl_h(&self, mmu: &mut Mmu) -> isize {
        mmu.write_byte(self.register.get_hl(), self.register.h);
        8
    }

    pub fn ld_hl_e(&self, mmu: &mut Mmu) -> isize {
        mmu.write_byte(self.register.get_hl(), self.register.e);
        8
    }

    pub fn ld_hl_d(&self, mmu: &mut Mmu) -> isize {
        mmu.write_byte(self.register.get_hl(), self.register.d);
        8
    }

    pub fn ld_hl_c(&self, mmu: &mut Mmu) -> isize {
        mmu.write_byte(self.register.get_hl(), self.register.c);
        8
    }

    pub fn ld_hl_b(&self, mmu: &mut Mmu) -> isize {
        mmu.write_byte(self.register.get_hl(), self.register.b);
        8
    }

    pub fn ld_hl_a(&self, mmu: &mut Mmu) -> isize {
        mmu.write_byte(self.register.get_hl(), self.register.a);
        8
    }

    pub fn ld_l_hl(&mut self, mmu: &Mmu) -> isize {
        self.register.l = mmu.read_byte(self.register.get_hl());
        4
    }

    pub fn ld_h_hl(&mut self, mmu: &Mmu) -> isize {
        self.register.h = mmu.read_byte(self.register.get_hl());
        4
    }

    pub fn ld_e_hl(&mut self, mmu: &Mmu) -> isize {
        self.register.e = mmu.read_byte(self.register.get_hl());
        4
    }

    pub fn ld_d_hl(&mut self, mmu: &Mmu) -> isize {
        self.register.d = mmu.read_byte(self.register.get_hl());
        4
    }

    pub fn ld_c_hl(&mut self, mmu: &Mmu) -> isize {
        self.register.c = mmu.read_byte(self.register.get_hl());
        4
    }

    pub fn ld_b_a(&mut self) -> isize {
        self.register.b = self.register.a;
        4
    }

    pub fn ld_b_hl(&mut self, mmu: &Mmu) -> isize {
        self.register.b = mmu.read_byte(self.register.get_hl());
        8
    }

    pub fn ld_a_hl(&mut self, mmu: &Mmu) -> isize {
        self.register.a = mmu.read_byte(self.register.get_hl());
        8
    }

    pub fn ldh_a8_a(&mut self, mmu: &mut Mmu) -> isize {
        let address = 0xFF00 + mmu.read_byte(self.register.pc) as u16;
        self.register.pc += 1;
        mmu.write_byte(address, self.register.a);
        12
    }

    pub fn ld_a_d8(&mut self, mmu: &Mmu) -> isize {
        self.register.a = mmu.read_byte(self.register.pc);
        self.register.pc += 1;
        8
    }

    pub fn ld_a_hlminus(&mut self, mmu: &Mmu) -> isize {
        let address = self.register.get_hl();
        self.register.a = mmu.read_byte(address);
        self.register.set_hl(address.wrapping_sub(1));
        8
    }

    pub fn ld_hl_d8(&mut self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        let value = mmu.read_byte(self.register.pc);
        self.register.pc += 1;
        mmu.write_byte(address, value);
        12
    }

    pub fn ld_hlminus_a(&mut self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        mmu.write_byte(address, self.register.a);
        self.register.set_hl(address.wrapping_sub(1));
        8
    }

    pub fn ld_sp_d16(&mut self, mmu: &Mmu) -> isize {
        self.register.sp = mmu.read_word(self.register.pc);
        self.register.pc += 2;
        12
    }

    pub fn ld_l_d8(&mut self, mmu: &Mmu) -> isize {
        self.register.l = mmu.read_byte(self.register.pc);
        self.register.pc += 1;
        8
    }

    pub fn ld_a_hplus(&mut self, mmu: &Mmu) -> isize {
        let address = self.register.get_hl();
        self.register.a = mmu.read_byte(address);
        self.register.set_hl(address.wrapping_add(1));
        8
    }

    pub fn ld_h_d8(&mut self, mmu: &Mmu) -> isize {
        self.register.h = mmu.read_byte(self.register.pc);
        self.register.pc += 1;
        8
    }

    pub fn ld_hlplus_a(&mut self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        mmu.write_byte(address, self.register.a);
        self.register.set_hl(address.wrapping_add(1));
        8
    }

    pub fn ld_hl_d16(&mut self, mmu: &Mmu) -> isize {
        self.register.set_hl(mmu.read_word(self.register.pc));
        self.register.pc += 2;
        12
    }

    pub fn ld_e_d8(&mut self, mmu: &Mmu) -> isize {
        self.register.e = mmu.read_byte(self.register.pc);
        self.register.pc += 1;
        8
    }

    pub fn ld_a_de(&mut self, mmu: &Mmu) -> isize {
        let address = self.register.get_de();
        self.register.a = mmu.read_byte(address);
        8
    }

    pub fn ld_d_d8(&mut self, mmu: &Mmu) -> isize {
        self.register.d = mmu.read_byte(self.register.pc);
        self.register.pc += 1;
        8
    }

    pub fn ld_de_a(&self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_de();
        mmu.write_byte(address, self.register.a);
        8
    }

    pub fn ld_de_d16(&mut self, mmu: &Mmu) -> isize {
        self.register.set_de(mmu.read_word(self.register.pc));
        self.register.pc += 2;
        12
    }

    pub fn ld_c_d8(&mut self, mmu: &Mmu) -> isize {
        self.register.c = mmu.read_byte(self.register.pc);
        self.register.pc += 1;
        8
    }

    pub fn ld_a_bc(&mut self, mmu: &Mmu) -> isize {
        let address = self.register.get_bc();
        self.register.a = mmu.read_byte(address);
        8
    }

    pub fn ld_a16_sp(&mut self, mmu: &mut Mmu) -> isize {
        let address = mmu.read_word(self.register.pc);
        self.register.pc += 2;
        mmu.write_byte(address, self.register.sp as u8);
        mmu.write_byte(address + 1, (self.register.sp >> 8) as u8);
        20
    }

    pub fn ld_bc_d16(&mut self, mmu: &Mmu) -> isize {
        self.register.set_bc(mmu.read_word(self.register.pc));
        self.register.pc += 2;
        12
    }

    pub fn ld_bc_a(&mut self, mmu: &mut Mmu) -> isize {
        mmu.write_byte(self.register.get_bc(), self.register.a);
        8
    }

    pub fn ld_a_b(&mut self) -> isize {
        self.register.a = self.register.b;
        4
    }

    pub fn ld_a_c(&mut self) -> isize {
        self.register.a = self.register.c;
        4
    }

    pub fn ld_a_d(&mut self) -> isize {
        self.register.a = self.register.d;
        4
    }

    pub fn ld_a_e(&mut self) -> isize {
        self.register.a = self.register.e;
        4
    }

    pub fn ld_a_h(&mut self) -> isize {
        self.register.a = self.register.h;
        4
    }

    pub fn ld_a_l(&mut self) -> isize {
        self.register.a = self.register.l;
        4
    }

    pub fn ld_b_b(&self) -> isize {
        4
    }

    pub fn ld_b_c(&mut self) -> isize {
        self.register.b = self.register.c;
        4
    }

    pub fn ld_b_d(&mut self) -> isize {
        self.register.b = self.register.d;
        4
    }

    pub fn ld_b_e(&mut self) -> isize {
        self.register.b = self.register.e;
        4
    }

    pub fn ld_b_h(&mut self) -> isize {
        self.register.b = self.register.h;
        4
    }

    pub fn ld_b_l(&mut self) -> isize {
        self.register.b = self.register.l;
        4
    }

    pub fn ld_d_a(&mut self) -> isize {
        self.register.d = self.register.a;
        4
    }

    pub fn ld_d_b(&mut self) -> isize {
        self.register.d = self.register.b;
        4
    }

    pub fn ld_d_c(&mut self) -> isize {
        self.register.d = self.register.c;
        4
    }

    pub fn ld_d_d(&self) -> isize {
        4
    }

    pub fn ld_d_e(&mut self) -> isize {
        self.register.d = self.register.e;
        4
    }

    pub fn ld_d_h(&mut self) -> isize {
        self.register.d = self.register.h;
        4
    }

    pub fn ld_d_l(&mut self) -> isize {
        self.register.d = self.register.l;
        4
    }

    pub fn ld_h_a(&mut self) -> isize {
        self.register.h = self.register.a;
        4
    }

    pub fn ld_h_b(&mut self) -> isize {
        self.register.h = self.register.b;
        4
    }

    pub fn ld_h_c(&mut self) -> isize {
        self.register.h = self.register.c;
        4
    }

    pub fn ld_h_d(&mut self) -> isize {
        self.register.h = self.register.d;
        4
    }

    pub fn ld_h_e(&mut self) -> isize {
        self.register.h = self.register.e;
        4
    }

    pub fn ld_h_l(&mut self) -> isize {
        self.register.h = self.register.l;
        4
    }

    pub fn ld_l_a(&mut self) -> isize {
        self.register.l = self.register.a;
        4
    }

    pub fn ld_l_b(&mut self) -> isize {
        self.register.l = self.register.b;
        4
    }

    pub fn ld_l_c(&mut self) -> isize {
        self.register.l = self.register.c;
        4
    }

    pub fn ld_l_d(&mut self) -> isize {
        self.register.l = self.register.d;
        4
    }

    pub fn ld_l_e(&mut self) -> isize {
        self.register.l = self.register.e;
        4
    }

    pub fn ld_l_h(&mut self) -> isize {
        self.register.l = self.register.h;
        4
    }

    pub fn ld_e_a(&mut self) -> isize {
        self.register.e = self.register.a;
        4
    }

    pub fn ld_e_b(&mut self) -> isize {
        self.register.e = self.register.b;
        4
    }

    pub fn ld_e_c(&mut self) -> isize {
        self.register.e = self.register.c;
        4
    }

    pub fn ld_e_d(&mut self) -> isize {
        self.register.e = self.register.d;
        4
    }

    pub fn ld_e_e(&self) -> isize {
        4
    }

    pub fn ld_e_h(&mut self) -> isize {
        self.register.e = self.register.h;
        4
    }

    pub fn ld_e_l(&mut self) -> isize {
        self.register.e = self.register.l;
        4
    }

    pub fn ld_c_a(&mut self) -> isize {
        self.register.c = self.register.a;
        4
    }

    pub fn ld__c__a(&mut self, mmu: &mut Mmu) -> isize {
        let address = 0xFF00 + self.register.c as u16;
        mmu.write_byte(address, self.register.a);
        8
    }

    pub fn ld_a16_a(&mut self, mmu: &mut Mmu) -> isize {
        let address = mmu.read_word(self.register.pc);
        self.register.pc += 2;
        mmu.write_byte(address, self.register.a);
        16
    }

    pub fn ld_c_b(&mut self) -> isize {
        self.register.c = self.register.b;
        4
    }

    pub fn ld_c_c(&self) -> isize {
        4
    }

    pub fn ld_c_d(&mut self) -> isize {
        self.register.c = self.register.d;
        4
    }

    pub fn ld_c_e(&mut self) -> isize {
        self.register.c = self.register.e;
        4
    }

    pub fn ld_c_h(&mut self) -> isize {
        self.register.c = self.register.h;
        4
    }

    pub fn ld_c_l(&mut self) -> isize {
        self.register.c = self.register.l;
        4
    }

    pub fn ld_b_d8(&mut self, mmu: &Mmu) -> isize {
        self.register.b = mmu.read_byte(self.register.pc);
        self.register.pc += 1;
        8
    }

    pub fn ldh_a_a8(&mut self, mmu: &Mmu) -> isize {
        let offset = mmu.read_byte(self.register.pc) as u16;
        self.register.pc += 1;
        self.register.a = mmu.read_byte(0xFF00 + offset);
        12
    }

    pub fn ld_a_c_(&mut self, mmu: &Mmu) -> isize {
        let address = 0xFF00 + self.register.c as u16;
        self.register.a = mmu.read_byte(address);
        8
    }

    pub fn ld_hl_spplus_r8(&mut self, mmu: &Mmu) -> isize {
        let offset = mmu.read_byte(self.register.pc) as i8 as i16 as u16;
        self.register.pc += 1;
        self.register.set_hl(self.register.sp.wrapping_add(offset));

        // SET FLAGS
        self.register.set_flag(FlagId::Z, false);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (self.register.sp & 0xF) + (offset & 0xF) > 0xF);
        self.register.set_flag(
            FlagId::C,
            (self.register.sp & 0xFF) + (offset & 0xFF) > 0xFF,
        );

        12
    }

    pub fn ld_sp_hl(&mut self) -> isize {
        self.register.sp = self.register.get_hl();
        8
    }

    pub fn ld_a_a16(&mut self, mmu: &Mmu) -> isize {
        let address = mmu.read_word(self.register.pc);
        self.register.pc += 2;
        self.register.a = mmu.read_byte(address);
        16
    }
}
