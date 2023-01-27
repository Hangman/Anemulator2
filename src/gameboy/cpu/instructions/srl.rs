use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn srl_b(&mut self) -> isize {
        let carry = self.register.b & 0x1;
        self.register.b >>= 1;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.b == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, carry > 0);

        8
    }

    pub fn srl_c(&mut self) -> isize {
        let carry = self.register.c & 0x1;
        self.register.c >>= 1;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.c == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, carry > 0);

        8
    }

    pub fn srl_d(&mut self) -> isize {
        let carry = self.register.d & 0x1;
        self.register.d >>= 1;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.d == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, carry > 0);

        8
    }

    pub fn srl_e(&mut self) -> isize {
        let carry = self.register.e & 0x1;
        self.register.e >>= 1;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.e == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, carry > 0);

        8
    }

    pub fn srl_h(&mut self) -> isize {
        let carry = self.register.h & 0x1;
        self.register.h >>= 1;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.h == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, carry > 0);

        8
    }

    pub fn srl_l(&mut self) -> isize {
        let carry = self.register.l & 0x1;
        self.register.l >>= 1;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.l == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, carry > 0);

        8
    }

    pub fn srl_hl(&mut self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        let value = mmu.read_byte(address);
        let carry = value & 0x1;
        let new_value = value >> 1;
        mmu.write_byte(address, new_value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_value == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, carry > 0);

        16
    }

    pub fn srl_a(&mut self) -> isize {
        let carry = self.register.a & 0x1;
        self.register.a >>= 1;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, carry > 0);

        8
    }
}
