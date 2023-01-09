use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn sub_a(&mut self) -> isize {
        self.register.a = 0;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, true);
        self.register.set_flag(FlagId::N, true);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, false);

        4
    }

    pub fn sub_b(&mut self) -> isize {
        let value = self.register.b;
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_sub(value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, old_a < value);

        4
    }

    pub fn sub_c(&mut self) -> isize {
        let value = self.register.c;
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_sub(value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, old_a < value);

        4
    }

    pub fn sub_d(&mut self) -> isize {
        let value = self.register.d;
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_sub(value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, old_a < value);

        4
    }

    pub fn sub_e(&mut self) -> isize {
        let value = self.register.e;
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_sub(value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, old_a < value);

        4
    }

    pub fn sub_h(&mut self) -> isize {
        let value = self.register.h;
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_sub(value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, old_a < value);

        4
    }

    pub fn sub_l(&mut self) -> isize {
        let value = self.register.l;
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_sub(value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, old_a < value);

        4
    }

    pub fn sub_hl(&mut self, mmu: &Mmu) -> isize {
        let value = mmu.read_byte(self.register.get_hl());
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_sub(value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF));
        self.register.set_flag(FlagId::C, old_a < value);

        4
    }

    pub fn sbc_a_a(&mut self) -> isize {
        let old_a = self.register.a;
        let carry = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = 0u8.wrapping_sub(carry);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (old_a & 0xF) + carry);
        self.register.set_flag(FlagId::C, old_a < old_a + carry);

        4
    }

    pub fn sbc_a_b(&mut self) -> isize {
        let value = self.register.b;
        let old_a = self.register.a;
        let carry = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = old_a.wrapping_sub(value).wrapping_sub(carry);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF) + carry);
        self.register.set_flag(FlagId::C, old_a < value + carry);

        4
    }

    pub fn sbc_a_c(&mut self) -> isize {
        let value = self.register.c;
        let old_a = self.register.a;
        let carry = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = old_a.wrapping_sub(value).wrapping_sub(carry);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF) + carry);
        self.register.set_flag(FlagId::C, old_a < value + carry);

        4
    }

    pub fn sbc_a_d(&mut self) -> isize {
        let value = self.register.d;
        let old_a = self.register.a;
        let carry = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = old_a.wrapping_sub(value).wrapping_sub(carry);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF) + carry);
        self.register.set_flag(FlagId::C, old_a < value + carry);

        4
    }

    pub fn sbc_a_e(&mut self) -> isize {
        let value = self.register.e;
        let old_a = self.register.a;
        let carry = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = old_a.wrapping_sub(value).wrapping_sub(carry);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF) + carry);
        self.register.set_flag(FlagId::C, old_a < value + carry);

        4
    }

    pub fn sbc_a_h(&mut self) -> isize {
        let value = self.register.h;
        let old_a = self.register.a;
        let carry = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = old_a.wrapping_sub(value).wrapping_sub(carry);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF) + carry);
        self.register.set_flag(FlagId::C, old_a < value + carry);

        4
    }

    pub fn sbc_a_l(&mut self) -> isize {
        let value = self.register.l;
        let old_a = self.register.a;
        let carry = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = old_a.wrapping_sub(value).wrapping_sub(carry);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF) + carry);
        self.register.set_flag(FlagId::C, old_a < value + carry);

        4
    }

    pub fn sbc_a_hl(&mut self, mmu: &Mmu) -> isize {
        let value = mmu.read_byte(self.register.get_hl());
        let old_a = self.register.a;
        let carry = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = old_a.wrapping_sub(value).wrapping_sub(carry);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) < (value & 0xF) + carry);
        self.register.set_flag(FlagId::C, old_a < value + carry);

        4
    }
}
