use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn cp_a(&mut self) -> isize {
        self.register.set_flag(FlagId::Z, true);
        self.register.set_flag(FlagId::N, true);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);
        4
    }

    pub fn cp_b(&mut self) -> isize {
        let value = self.register.b;
        self.register.set_flag(FlagId::Z, self.register.a == value);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (self.register.a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, self.register.a < value);
        4
    }

    pub fn cp_c(&mut self) -> isize {
        let value = self.register.c;
        self.register.set_flag(FlagId::Z, self.register.a == value);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (self.register.a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, self.register.a < value);
        4
    }

    pub fn cp_d(&mut self) -> isize {
        let value = self.register.d;
        self.register.set_flag(FlagId::Z, self.register.a == value);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (self.register.a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, self.register.a < value);
        4
    }

    pub fn cp_e(&mut self) -> isize {
        let value = self.register.e;
        self.register.set_flag(FlagId::Z, self.register.a == value);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (self.register.a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, self.register.a < value);
        4
    }

    pub fn cp_h(&mut self) -> isize {
        let value = self.register.h;
        self.register.set_flag(FlagId::Z, self.register.a == value);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (self.register.a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, self.register.a < value);
        4
    }

    pub fn cp_l(&mut self) -> isize {
        let value = self.register.l;
        self.register.set_flag(FlagId::Z, self.register.a == value);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (self.register.a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, self.register.a < value);
        4
    }

    pub fn cp_hl(&mut self, mmu: &Mmu) -> isize {
        let value = mmu.read_byte(self.register.get_hl());
        self.register.set_flag(FlagId::Z, self.register.a == value);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (self.register.a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, self.register.a < value);
        4
    }

    pub fn cp_d8(&mut self, mmu: &Mmu) -> isize {
        let value = mmu.read_byte(self.register.pc);
        self.register.pc += 1;
        self.register.set_flag(FlagId::Z, self.register.a == value);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (self.register.a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, self.register.a < value);
        8
    }
}
