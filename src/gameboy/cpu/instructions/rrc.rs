use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn rr_b(&mut self) -> isize {
        let lsb = self.register.b & 0x1;
        let carry = (self.register.is_flag_set(FlagId::C) as u8) << 7;
        self.register.b = (self.register.b >> 1) | carry;

        self.register.set_flag(FlagId::Z, self.register.b == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn rr_c(&mut self) -> isize {
        let lsb = self.register.c & 0x1;
        let carry = (self.register.is_flag_set(FlagId::C) as u8) << 7;
        self.register.c = (self.register.c >> 1) | carry;

        self.register.set_flag(FlagId::Z, self.register.c == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn rr_d(&mut self) -> isize {
        let lsb = self.register.d & 0x1;
        let carry = (self.register.is_flag_set(FlagId::C) as u8) << 7;
        self.register.d = (self.register.d >> 1) | carry;

        self.register.set_flag(FlagId::Z, self.register.d == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn rr_e(&mut self) -> isize {
        let lsb = self.register.e & 0x1;
        let carry = (self.register.is_flag_set(FlagId::C) as u8) << 7;
        self.register.e = (self.register.e >> 1) | carry;

        self.register.set_flag(FlagId::Z, self.register.e == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn rr_h(&mut self) -> isize {
        let lsb = self.register.h & 0x1;
        let carry = (self.register.is_flag_set(FlagId::C) as u8) << 7;
        self.register.h = (self.register.h >> 1) | carry;

        self.register.set_flag(FlagId::Z, self.register.h == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn rr_l(&mut self) -> isize {
        let lsb = self.register.l & 0x1;
        let carry = (self.register.is_flag_set(FlagId::C) as u8) << 7;
        self.register.l = (self.register.l >> 1) | carry;

        self.register.set_flag(FlagId::Z, self.register.l == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn rr_hl(&mut self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        let old_value = mmu.read_byte(address);
        let lsb = old_value & 0x1;
        let carry = (self.register.is_flag_set(FlagId::C) as u8) << 7;
        let new_value = (old_value >> 1) | carry;
        mmu.write_byte(address, new_value);

        self.register.set_flag(FlagId::Z, new_value == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        16
    }

    pub fn rr_a(&mut self) -> isize {
        let lsb = self.register.a & 0x1;
        let carry = (self.register.is_flag_set(FlagId::C) as u8) << 7;
        self.register.a = (self.register.a >> 1) | carry;

        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn rrc_a(&mut self) -> isize {
        let old_a = self.register.a;
        self.register.a = old_a.rotate_right(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, false);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (old_a & 0x01) != 0);

        4
    }

    pub fn rrc_b(&mut self) -> isize {
        let b = self.register.b;
        self.register.b = b.rotate_right(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.b == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (b & 0x01) != 0);

        8
    }

    pub fn rrc_c(&mut self) -> isize {
        let c = self.register.c;
        self.register.c = c.rotate_right(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.c == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (c & 0x01) != 0);

        8
    }

    pub fn rrc_d(&mut self) -> isize {
        let d = self.register.d;
        self.register.d = d.rotate_right(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.d == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (d & 0x01) != 0);

        8
    }

    pub fn rrc_e(&mut self) -> isize {
        let e = self.register.e;
        self.register.e = e.rotate_right(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.e == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (e & 0x01) != 0);

        8
    }

    pub fn rrc_h(&mut self) -> isize {
        let h = self.register.h;
        self.register.h = h.rotate_right(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.h == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (h & 0x01) != 0);

        8
    }

    pub fn rrc_l(&mut self) -> isize {
        let l = self.register.l;
        self.register.l = l.rotate_right(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.l == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (l & 0x01) != 0);

        8
    }

    pub fn rrc_hl(&mut self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        let old_value = mmu.read_byte(address);
        let new_value = old_value.rotate_right(1);
        mmu.write_byte(address, new_value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_value == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (old_value & 0x01) != 0);

        16
    }

    pub fn rrc_a2(&mut self) -> isize {
        let a = self.register.a;
        self.register.a = a.rotate_right(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (a & 0x01) != 0);

        8
    }
}
