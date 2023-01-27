use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn bit_b(&mut self, index: u8) -> isize {
        self.register
            .set_flag(FlagId::Z, (self.register.b & (1 << index)) == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, true);
        8
    }

    pub fn bit_c(&mut self, index: u8) -> isize {
        self.register
            .set_flag(FlagId::Z, (self.register.c & (1 << index)) == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, true);
        8
    }

    pub fn bit_d(&mut self, index: u8) -> isize {
        self.register
            .set_flag(FlagId::Z, (self.register.d & (1 << index)) == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, true);
        8
    }

    pub fn bit_e(&mut self, index: u8) -> isize {
        self.register
            .set_flag(FlagId::Z, (self.register.e & (1 << index)) == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, true);
        8
    }

    pub fn bit_h(&mut self, index: u8) -> isize {
        self.register
            .set_flag(FlagId::Z, (self.register.h & (1 << index)) == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, true);
        8
    }

    pub fn bit_l(&mut self, index: u8) -> isize {
        self.register
            .set_flag(FlagId::Z, (self.register.l & (1 << index)) == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, true);
        8
    }

    pub fn bit_hl(&mut self, index: u8, mmu: &Mmu) -> isize {
        let value = mmu.read_byte(self.register.get_hl());
        self.register
            .set_flag(FlagId::Z, (value & (1 << index)) == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, true);
        16
    }

    pub fn bit_a(&mut self, index: u8) -> isize {
        self.register
            .set_flag(FlagId::Z, (self.register.a & (1 << index)) == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, true);
        8
    }
}
