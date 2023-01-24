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
}
