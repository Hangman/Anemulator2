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
}
