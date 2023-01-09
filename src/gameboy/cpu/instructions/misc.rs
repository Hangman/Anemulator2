use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;

impl Cpu {
    pub fn stop(&mut self) -> isize {
        // TODO: implement stop properly
        self.register.pc += 1;
        4
    }

    pub fn rla(&mut self) -> isize {
        let old_a = self.register.a;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = (old_a << 1) | carry_bit;

        self.register.set_flag(FlagId::Z, false);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, old_a >> 7 == 1);

        8
    }

    pub fn rra(&mut self) -> isize {
        let lsb = self.register.a & 0b1;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        let a = (self.register.a >> 1) | (carry_bit << 7);
        self.register.a = a;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::C, lsb != 0);

        8
    }

    pub fn da_a(&mut self) -> isize {
        if self.register.is_flag_set(FlagId::N) {
            if self.register.is_flag_set(FlagId::C) {
                self.register.a = self.register.a.wrapping_add(0xA0);
                self.register.set_flag(FlagId::C, true);
            }
            if self.register.is_flag_set(FlagId::H) {
                self.register.a = self.register.a.wrapping_add(0xFA);
            }
        } else {
            if self.register.is_flag_set(FlagId::C) || (self.register.a > 0x99) {
                self.register.a = self.register.a.wrapping_add(0x60);
                self.register.set_flag(FlagId::C, true);
            }
            if self.register.is_flag_set(FlagId::H) || ((self.register.a & 0xF) > 0x9) {
                self.register.a = self.register.a.wrapping_add(0x06);
            }
        }

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::H, false);

        4
    }

    pub fn cpl_a(&mut self) -> isize {
        self.register.a = !self.register.a;

        // SET FLAGS
        self.register.set_flag(FlagId::N, true);
        self.register.set_flag(FlagId::H, true);

        4
    }

    pub fn scf(&mut self) -> isize {
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, true);
        4
    }

    pub fn ccf(&mut self) -> isize {
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register
            .set_flag(FlagId::C, !self.register.is_flag_set(FlagId::C));
        4
    }
}
