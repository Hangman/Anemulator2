use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn jr_r8(&mut self, mmu: &Mmu) -> isize {
        let offset = mmu.read_byte(self.register.pc) as i8 as i16;
        self.register.pc += 1;
        self.register.pc = (self.register.pc as i16 + offset) as u16;
        12
    }

    pub fn jr_nz_r8(&mut self, mmu: &Mmu) -> isize {
        if !self.register.is_flag_set(FlagId::Z) {
            let offset = mmu.read_byte(self.register.pc) as i8 as i16;
            self.register.pc += 1;
            self.register.pc = (self.register.pc as i16 + offset) as u16;
            return 12;
        }

        self.register.pc += 1;
        8
    }

    pub fn jr_nc_r8(&mut self, mmu: &Mmu) -> isize {
        if !self.register.is_flag_set(FlagId::C) {
            let offset = mmu.read_byte(self.register.pc) as i8 as i16;
            self.register.pc += 1;
            self.register.pc = (self.register.pc as i16 + offset) as u16;
            return 12;
        }

        self.register.pc += 1;
        8
    }

    pub fn jr_z_r8(&mut self, mmu: &Mmu) -> isize {
        if self.register.is_flag_set(FlagId::Z) {
            let offset = mmu.read_byte(self.register.pc) as i8 as i16;
            self.register.pc += 1;
            self.register.pc = (self.register.pc as i16 + offset) as u16;
            return 12;
        }

        self.register.pc += 1;
        8
    }

    pub fn jr_c_r8(&mut self, mmu: &Mmu) -> isize {
        if self.register.is_flag_set(FlagId::C) {
            let offset = mmu.read_byte(self.register.pc) as i8 as i16;
            self.register.pc += 1;
            self.register.pc = (self.register.pc as i16 + offset) as u16;
            return 12;
        }

        self.register.pc += 1;
        8
    }

    pub fn jp_nz_a16(&mut self, mmu: &Mmu) -> isize {
        if !self.register.is_flag_set(FlagId::Z) {
            self.register.pc = mmu.read_word(self.register.pc);
            return 16;
        }

        self.register.pc += 2;
        12
    }

    pub fn jp_a16(&mut self, mmu: &Mmu) -> isize {
        self.register.pc = mmu.read_word(self.register.pc);
        16
    }
}
