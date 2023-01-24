use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

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

    pub fn ret_nz(&mut self, mmu: &Mmu) -> isize {
        if !self.register.is_flag_set(FlagId::Z) {
            self.register.pc = mmu.read_word(self.register.sp);
            self.register.sp += 2;
            return 20;
        }
        8
    }

    pub fn pop_af(&mut self, mmu: &Mmu) -> isize {
        let data = mmu.read_word(self.register.sp);
        self.register.sp += 2;
        self.register.set_af(data);
        12
    }

    pub fn pop_bc(&mut self, mmu: &Mmu) -> isize {
        let data = mmu.read_word(self.register.sp);
        self.register.sp += 2;
        self.register.set_bc(data);
        12
    }

    pub fn pop_de(&mut self, mmu: &Mmu) -> isize {
        let data = mmu.read_word(self.register.sp);
        self.register.sp += 2;
        self.register.set_de(data);
        12
    }

    pub fn pop_hl(&mut self, mmu: &Mmu) -> isize {
        let data = mmu.read_word(self.register.sp);
        self.register.sp += 2;
        self.register.set_hl(data);
        12
    }

    pub fn call_z_a16(&mut self, mmu: &mut Mmu) -> isize {
        let address = mmu.read_word(self.register.pc);
        self.register.pc += 2;
        if self.register.is_flag_set(FlagId::Z) {
            self.register.sp -= 1;
            mmu.write_byte(self.register.sp, ((self.register.pc & 0xFF00) >> 8) as u8);
            self.register.sp -= 1;
            mmu.write_byte(self.register.sp, self.register.pc as u8);
            self.register.pc = address;
            return 16;
        }

        12
    }

    pub fn call_nz_a16(&mut self, mmu: &mut Mmu) -> isize {
        let address = mmu.read_word(self.register.pc);
        self.register.pc += 2;
        if !self.register.is_flag_set(FlagId::Z) {
            self.register.sp -= 1;
            mmu.write_byte(self.register.sp, ((self.register.pc & 0xFF00) >> 8) as u8);
            self.register.sp -= 1;
            mmu.write_byte(self.register.sp, self.register.pc as u8);
            self.register.pc = address;
            return 16;
        }

        12
    }

    pub fn call_nc_a16(&mut self, mmu: &mut Mmu) -> isize {
        let address = mmu.read_word(self.register.pc);
        self.register.pc += 2;
        if !self.register.is_flag_set(FlagId::C) {
            self.register.sp -= 1;
            mmu.write_byte(self.register.sp, ((self.register.pc & 0xFF00) >> 8) as u8);
            self.register.sp -= 1;
            mmu.write_byte(self.register.sp, self.register.pc as u8);
            self.register.pc = address;
            return 16;
        }

        12
    }

    pub fn call_c_a16(&mut self, mmu: &mut Mmu) -> isize {
        let address = mmu.read_word(self.register.pc);
        self.register.pc += 2;
        if self.register.is_flag_set(FlagId::C) {
            self.register.sp -= 1;
            mmu.write_byte(self.register.sp, ((self.register.pc & 0xFF00) >> 8) as u8);
            self.register.sp -= 1;
            mmu.write_byte(self.register.sp, self.register.pc as u8);
            self.register.pc = address;
            return 16;
        }

        12
    }

    pub fn push_bc(&mut self, mmu: &mut Mmu) -> isize {
        let bc = self.register.get_bc();
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (bc >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, bc as u8);
        16
    }

    pub fn push_af(&mut self, mmu: &mut Mmu) -> isize {
        let af = self.register.get_af();
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (af >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, af as u8);
        16
    }

    pub fn push_de(&mut self, mmu: &mut Mmu) -> isize {
        let de = self.register.get_de();
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (de >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, de as u8);
        16
    }

    pub fn push_hl(&mut self, mmu: &mut Mmu) -> isize {
        let hl = self.register.get_hl();
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (hl >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, hl as u8);
        16
    }

    pub fn ret_z(&mut self, mmu: &Mmu) -> isize {
        if self.register.is_flag_set(FlagId::Z) {
            self.register.pc = mmu.read_word(self.register.sp);
            self.register.sp += 2;
            return 20;
        }

        8
    }

    pub fn ret_c(&mut self, mmu: &Mmu) -> isize {
        if self.register.is_flag_set(FlagId::C) {
            self.register.pc = mmu.read_word(self.register.sp);
            self.register.sp += 2;
            return 20;
        }

        8
    }

    pub fn ret_i(&mut self, mmu: &Mmu) -> isize {
        self.register.pc = mmu.read_word(self.register.sp);
        self.register.sp += 2;
        self.register.set_interrupts_enabled(true, true);
        16
    }

    pub fn ret_nc(&mut self, mmu: &Mmu) -> isize {
        if !self.register.is_flag_set(FlagId::C) {
            self.register.pc = mmu.read_word(self.register.sp);
            self.register.sp += 2;
            return 20;
        }

        8
    }

    pub fn ret(&mut self, mmu: &Mmu) -> isize {
        self.register.pc = mmu.read_word(self.register.sp);
        self.register.sp += 2;
        16
    }

    pub fn call_a16(&mut self, mmu: &mut Mmu) -> isize {
        let address = mmu.read_word(self.register.pc);
        self.register.pc += 2;
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, (self.register.pc >> 8) as u8);
        self.register.sp -= 1;
        mmu.write_byte(self.register.sp, self.register.pc as u8);
        self.register.pc = address;

        24
    }

    pub fn di(&mut self) -> isize {
        self.register.set_interrupts_enabled(false, false);
        4
    }

    pub fn ei(&mut self) -> isize {
        self.register.set_interrupts_enabled(true, false);
        4
    }
}
