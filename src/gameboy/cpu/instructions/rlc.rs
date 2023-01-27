use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn rl_b(&mut self) -> isize {
        let msb = self.register.b >> 7;
        self.register.b = (self.register.b << 1) | (self.register.is_flag_set(FlagId::C) as u8);

        self.register.set_flag(FlagId::C, msb == 1);
        self.register.set_flag(FlagId::Z, self.register.b == 0);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::N, false);

        8
    }

    pub fn rl_c(&mut self) -> isize {
        let msb = self.register.c >> 7;
        self.register.c = (self.register.c << 1) | (self.register.is_flag_set(FlagId::C) as u8);

        self.register.set_flag(FlagId::C, msb == 1);
        self.register.set_flag(FlagId::Z, self.register.c == 0);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::N, false);

        8
    }

    pub fn rl_d(&mut self) -> isize {
        let msb = self.register.d >> 7;
        self.register.d = (self.register.d << 1) | (self.register.is_flag_set(FlagId::C) as u8);

        self.register.set_flag(FlagId::C, msb == 1);
        self.register.set_flag(FlagId::Z, self.register.d == 0);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::N, false);

        8
    }

    pub fn rl_e(&mut self) -> isize {
        let msb = self.register.e >> 7;
        self.register.e = (self.register.e << 1) | (self.register.is_flag_set(FlagId::C) as u8);

        self.register.set_flag(FlagId::C, msb == 1);
        self.register.set_flag(FlagId::Z, self.register.e == 0);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::N, false);

        8
    }

    pub fn rl_h(&mut self) -> isize {
        let msb = self.register.h >> 7;
        self.register.h = (self.register.h << 1) | (self.register.is_flag_set(FlagId::C) as u8);

        self.register.set_flag(FlagId::C, msb == 1);
        self.register.set_flag(FlagId::Z, self.register.h == 0);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::N, false);

        8
    }

    pub fn rl_l(&mut self) -> isize {
        let msb = self.register.l >> 7;
        self.register.l = (self.register.l << 1) | (self.register.is_flag_set(FlagId::C) as u8);

        self.register.set_flag(FlagId::C, msb == 1);
        self.register.set_flag(FlagId::Z, self.register.l == 0);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::N, false);

        8
    }

    pub fn rl_hl(&mut self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        let old_value = mmu.read_byte(address);
        let msb = old_value >> 7;
        let new_value = (old_value << 1) | (self.register.is_flag_set(FlagId::C) as u8);
        mmu.write_byte(address, new_value);

        self.register.set_flag(FlagId::C, msb == 1);
        self.register.set_flag(FlagId::Z, new_value == 0);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::N, false);

        16
    }

    pub fn rl_a(&mut self) -> isize {
        let msb = self.register.a >> 7;
        self.register.a = (self.register.a << 1) | (self.register.is_flag_set(FlagId::C) as u8);

        self.register.set_flag(FlagId::C, msb == 1);
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::N, false);

        8
    }

    pub fn rlc_a(&mut self) -> isize {
        let a = self.register.a;
        self.register.a = a.rotate_left(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, false);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (a & 0x80) != 0);

        4
    }

    pub fn rlc_b(&mut self) -> isize {
        let b = self.register.b;
        self.register.b = b.rotate_left(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.b == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (b & 0x80) != 0);

        8
    }

    pub fn rlc_c(&mut self) -> isize {
        let c = self.register.c;
        self.register.c = c.rotate_left(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.c == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (c & 0x80) != 0);

        8
    }

    pub fn rlc_d(&mut self) -> isize {
        let d = self.register.d;
        self.register.d = d.rotate_left(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.d == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (d & 0x80) != 0);

        8
    }

    pub fn rlc_e(&mut self) -> isize {
        let e = self.register.e;
        self.register.e = e.rotate_left(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.e == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (e & 0x80) != 0);

        8
    }

    pub fn rlc_h(&mut self) -> isize {
        let h = self.register.h;
        self.register.h = h.rotate_left(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.h == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (h & 0x80) != 0);

        8
    }

    pub fn rlc_l(&mut self) -> isize {
        let l = self.register.l;
        self.register.l = l.rotate_left(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.l == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (l & 0x80) != 0);

        8
    }

    pub fn rlc_hl(&mut self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        let old_value = mmu.read_byte(address);
        let new_value = old_value.rotate_left(1);
        mmu.write_byte(address, new_value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_value == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (old_value & 0x80) != 0);

        16
    }

    pub fn rlc_a2(&mut self) -> isize {
        let a = self.register.a;
        self.register.a = a.rotate_left(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (a & 0x80) != 0);

        8
    }
}
