use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn sra_b(&mut self) -> isize {
        let value = self.register.b;
        let lsb = value & 0x1;
        let msb = value & 0x80;
        self.register.b = (value >> 1) | msb;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.b == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn sra_c(&mut self) -> isize {
        let value = self.register.c;
        let lsb = value & 0x1;
        let msb = value & 0x80;
        self.register.c = (value >> 1) | msb;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.c == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn sra_d(&mut self) -> isize {
        let value = self.register.d;
        let lsb = value & 0x1;
        let msb = value & 0x80;
        self.register.d = (value >> 1) | msb;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.d == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn sra_e(&mut self) -> isize {
        let value = self.register.e;
        let lsb = value & 0x1;
        let msb = value & 0x80;
        self.register.e = (value >> 1) | msb;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.e == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn sra_h(&mut self) -> isize {
        let value = self.register.h;
        let lsb = value & 0x1;
        let msb = value & 0x80;
        self.register.h = (value >> 1) | msb;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.h == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn sra_l(&mut self) -> isize {
        let value = self.register.l;
        let lsb = value & 0x1;
        let msb = value & 0x80;
        self.register.l = (value >> 1) | msb;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.l == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn sra_hl(&mut self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        let value = mmu.read_byte(address);
        let lsb = value & 0x1;
        let msb = value & 0x80;
        let new_value = (value >> 1) | msb;
        mmu.write_byte(address, new_value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_value == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        16
    }

    pub fn sra_a(&mut self) -> isize {
        let value = self.register.a;
        let lsb = value & 0x1;
        let msb = value & 0x80;
        self.register.a = (value >> 1) | msb;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }
}
