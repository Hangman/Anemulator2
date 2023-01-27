use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn swap_b(&mut self) -> isize {
        let low_nibble = self.register.b & 0x0F;
        let high_nibble = self.register.b >> 4;
        self.register.b = high_nibble | (low_nibble << 4);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.b == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        8
    }

    pub fn swap_c(&mut self) -> isize {
        let low_nibble = self.register.c & 0x0F;
        let high_nibble = self.register.c >> 4;
        self.register.c = high_nibble | (low_nibble << 4);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.c == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        8
    }

    pub fn swap_d(&mut self) -> isize {
        let low_nibble = self.register.d & 0x0F;
        let high_nibble = self.register.d >> 4;
        self.register.d = high_nibble | (low_nibble << 4);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.d == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        8
    }

    pub fn swap_e(&mut self) -> isize {
        let low_nibble = self.register.e & 0x0F;
        let high_nibble = self.register.e >> 4;
        self.register.e = high_nibble | (low_nibble << 4);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.e == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        8
    }

    pub fn swap_h(&mut self) -> isize {
        let low_nibble = self.register.h & 0x0F;
        let high_nibble = self.register.h >> 4;
        self.register.h = high_nibble | (low_nibble << 4);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.h == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        8
    }

    pub fn swap_l(&mut self) -> isize {
        let low_nibble = self.register.l & 0x0F;
        let high_nibble = self.register.l >> 4;
        self.register.l = high_nibble | (low_nibble << 4);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.l == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        8
    }

    pub fn swap_hl(&mut self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        let value = mmu.read_byte(address);
        let low_nibble = value & 0x0F;
        let high_nibble = value >> 4;
        let new_value = high_nibble | (low_nibble << 4);
        mmu.write_byte(address, new_value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_value == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        16
    }

    pub fn swap_a(&mut self) -> isize {
        let low_nibble = self.register.a & 0x0F;
        let high_nibble = self.register.a >> 4;
        self.register.a = high_nibble | (low_nibble << 4);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        8
    }
}
