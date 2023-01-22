use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn xor_a(&mut self) -> isize {
        self.register.a = 0;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, true);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        4
    }

    pub fn xor_b(&mut self) -> isize {
        self.register.a ^= self.register.b;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        4
    }

    pub fn xor_c(&mut self) -> isize {
        self.register.a ^= self.register.c;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        4
    }

    pub fn xor_d(&mut self) -> isize {
        self.register.a ^= self.register.d;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        4
    }

    pub fn xor_e(&mut self) -> isize {
        self.register.a ^= self.register.e;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        4
    }

    pub fn xor_h(&mut self) -> isize {
        self.register.a ^= self.register.h;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        4
    }

    pub fn xor_l(&mut self) -> isize {
        self.register.a ^= self.register.l;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        4
    }

    pub fn xor_hl(&mut self, mmu: &Mmu) -> isize {
        self.register.a ^= mmu.read_byte(self.register.get_hl());

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        4
    }

    pub fn xor_d8(&mut self, mmu: &Mmu) -> isize {
        self.register.a ^= mmu.read_byte(self.register.pc);
        self.register.pc += 1;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        8
    }
}
