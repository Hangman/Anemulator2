use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn dec_a(&mut self) -> isize {
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_sub(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, old_a.trailing_zeros() >= 4);

        4
    }

    pub fn dec_b(&mut self) -> isize {
        let old_b = self.register.b;
        self.register.b = old_b.wrapping_sub(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.b == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, old_b.trailing_zeros() >= 4);

        4
    }

    pub fn dec_c(&mut self) -> isize {
        let old_c = self.register.c;
        self.register.c = old_c.wrapping_sub(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.c == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, old_c.trailing_zeros() >= 4);

        4
    }

    pub fn dec_d(&mut self) -> isize {
        let old_d = self.register.d;
        self.register.d = old_d.wrapping_sub(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.d == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, old_d.trailing_zeros() >= 4);

        4
    }

    pub fn dec_e(&mut self) -> isize {
        let old_e = self.register.e;
        self.register.e = old_e.wrapping_sub(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.e == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, old_e.trailing_zeros() >= 4);

        4
    }

    pub fn dec_h(&mut self) -> isize {
        let old_h = self.register.h;
        self.register.h = old_h.wrapping_sub(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.h == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, old_h.trailing_zeros() >= 4);

        4
    }

    pub fn dec_l(&mut self) -> isize {
        let old_l = self.register.l;
        self.register.l = old_l.wrapping_sub(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.l == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, old_l.trailing_zeros() >= 4);

        4
    }

    pub fn dec_bc(&mut self) -> isize {
        self.register.set_bc(self.register.get_bc().wrapping_sub(1));
        8
    }

    pub fn dec_de(&mut self) -> isize {
        self.register.set_de(self.register.get_de().wrapping_sub(1));
        8
    }

    pub fn dec_hl(&mut self) -> isize {
        self.register.set_hl(self.register.get_hl().wrapping_sub(1));
        8
    }

    pub fn dec_hl_(&mut self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        let new_value = mmu.read_byte(address).wrapping_sub(1);
        mmu.write_byte(address, new_value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_value == 0);
        self.register.set_flag(FlagId::N, true);
        self.register
            .set_flag(FlagId::H, new_value.trailing_zeros() >= 4);

        12
    }

    pub fn dec_sp(&mut self) -> isize {
        self.register.sp = self.register.sp.wrapping_sub(1);
        8
    }
}
