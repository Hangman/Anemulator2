use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn rst_00h(&mut self, mmu: &mut Mmu) -> isize {
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (self.register.pc >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, self.register.pc as u8);
        self.register.pc = 0;
        16
    }

    pub fn rst_08h(&mut self, mmu: &mut Mmu) -> isize {
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (self.register.pc >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, self.register.pc as u8);
        self.register.pc = 0x08;
        16
    }

    pub fn rst_10h(&mut self, mmu: &mut Mmu) -> isize {
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (self.register.pc >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, self.register.pc as u8);
        self.register.pc = 0x10;
        16
    }

    pub fn rst_18h(&mut self, mmu: &mut Mmu) -> isize {
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (self.register.pc >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, self.register.pc as u8);
        self.register.pc = 0x18;
        16
    }

    pub fn rst_20h(&mut self, mmu: &mut Mmu) -> isize {
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (self.register.pc >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, self.register.pc as u8);
        self.register.pc = 0x20;
        16
    }

    pub fn rst_28h(&mut self, mmu: &mut Mmu) -> isize {
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (self.register.pc >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, self.register.pc as u8);
        self.register.pc = 0x28;
        16
    }

    pub fn rst_30h(&mut self, mmu: &mut Mmu) -> isize {
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (self.register.pc >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, self.register.pc as u8);
        self.register.pc = 0x30;
        16
    }

    pub fn rst_38h(&mut self, mmu: &mut Mmu) -> isize {
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (self.register.pc >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, self.register.pc as u8);
        self.register.pc = 0x38;
        16
    }

    pub fn res_b(&mut self, index: u8) -> isize {
        self.register.b &= !(1 << index);
        8
    }

    pub fn res_c(&mut self, index: u8) -> isize {
        self.register.c &= !(1 << index);
        8
    }

    pub fn res_d(&mut self, index: u8) -> isize {
        self.register.d &= !(1 << index);
        8
    }

    pub fn res_e(&mut self, index: u8) -> isize {
        self.register.e &= !(1 << index);
        8
    }

    pub fn res_h(&mut self, index: u8) -> isize {
        self.register.h &= !(1 << index);
        8
    }

    pub fn res_l(&mut self, index: u8) -> isize {
        self.register.l &= !(1 << index);
        8
    }

    pub fn res_hl(&mut self, index: u8, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        let value = mmu.read_byte(address);
        let new_value = value & !(1 << index);
        mmu.write_byte(address, new_value);
        16
    }

    pub fn res_a(&mut self, index: u8) -> isize {
        self.register.a &= !(1 << index);
        8
    }
}
