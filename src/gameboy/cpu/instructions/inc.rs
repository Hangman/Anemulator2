use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn inc_de(&mut self) -> isize {
        let value = self.register.get_de();
        self.register.set_de(value.wrapping_add(1));
        8
    }

    pub fn inc_bc(&mut self) -> isize {
        let value = self.register.get_bc();
        self.register.set_bc(value.wrapping_add(1));
        8
    }

    pub fn inc_hl(&mut self) -> isize {
        let value = self.register.get_hl();
        self.register.set_hl(value.wrapping_add(1));
        8
    }

    pub fn inc_hl_(&mut self, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        let old_value = mmu.read_byte(address);
        let new_value = old_value.wrapping_add(1);
        mmu.write_byte(address, new_value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_value == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, (old_value & 0xF) == 0xF);

        return 12;
    }

    pub fn inc_sp(&mut self) -> isize {
        self.register.sp = self.register.sp.wrapping_add(1);
        8
    }

    pub fn inc_a(&mut self) -> isize {
        let old_a = self.register.a;
        let new_a = old_a.wrapping_add(1);
        self.register.a = new_a;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, (old_a & 0xF) == 0xF);

        4
    }

    pub fn inc_b(&mut self) -> isize {
        let old_b = self.register.b;
        let new_b = old_b.wrapping_add(1);
        self.register.b = new_b;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_b == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, (old_b & 0xF) == 0xF);

        4
    }

    pub fn inc_c(&mut self) -> isize {
        let old_c = self.register.c;
        let new_c = old_c.wrapping_add(1);
        self.register.c = new_c;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_c == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, (old_c & 0xF) == 0xF);

        4
    }

    pub fn inc_d(&mut self) -> isize {
        let old_d = self.register.d;
        let new_d = old_d.wrapping_add(1);
        self.register.d = new_d;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_d == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, (old_d & 0xF) == 0xF);

        4
    }

    pub fn inc_e(&mut self) -> isize {
        let old_e = self.register.e;
        let new_e = old_e.wrapping_add(1);
        self.register.e = new_e;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_e == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, (old_e & 0xF) == 0xF);

        4
    }

    pub fn inc_h(&mut self) -> isize {
        let old_h = self.register.h;
        let new_h = old_h.wrapping_add(1);
        self.register.h = new_h;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_h == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, (old_h & 0xF) == 0xF);

        4
    }

    pub fn inc_l(&mut self) -> isize {
        let old_l = self.register.l;
        let new_l = old_l.wrapping_add(1);
        self.register.l = new_l;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, new_l == 0);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, (old_l & 0xF) == 0xF);

        4
    }
}
