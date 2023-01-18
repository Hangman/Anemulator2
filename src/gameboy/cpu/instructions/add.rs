use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn add_hl_bc(&mut self) -> isize {
        let bc = self.register.get_bc();
        let hl = self.register.get_hl();
        let result = hl.wrapping_add(bc);
        self.register.set_hl(result);

        // SET FLAGS
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (hl & 0xFFF) + (bc & 0xFFF) > 0xFFF);
        self.register
            .set_flag(FlagId::C, (hl as u32) + (bc as u32) > 0xFFFF);

        8
    }

    pub fn add_hl_de(&mut self) -> isize {
        let de = self.register.get_de();
        let hl = self.register.get_hl();
        let result = de.wrapping_add(hl);
        self.register.set_hl(result);

        // SET FLAGS
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (hl & 0xFFF) + (de & 0xFFF) > 0xFFF);
        self.register
            .set_flag(FlagId::C, (hl as u32) + (de as u32) > 0xFFFF);

        8
    }

    pub fn add_hl_hl(&mut self) -> isize {
        let hl = self.register.get_hl();
        let result = hl.wrapping_add(hl);
        self.register.set_hl(result);

        // SET FLAGS
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (hl & 0xFFF) + (hl & 0xFFF) > 0xFFF);
        self.register
            .set_flag(FlagId::C, (hl as u32) + (hl as u32) > 0xFFFF);

        8
    }

    pub fn add_hl_sp(&mut self) -> isize {
        let sp = self.register.sp;
        let hl = self.register.get_hl();
        let result = hl.wrapping_add(sp);
        self.register.set_hl(result);

        // SET FLAGS
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (hl & 0xFFF) + (sp & 0xFFF) > 0xFFF);
        self.register
            .set_flag(FlagId::C, (sp as u32) + (hl as u32) > 0xFFFF);

        8
    }

    pub fn add_a_b(&mut self) -> isize {
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_add(self.register.b);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (self.register.b & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + self.register.b as u16) > 0xFF);

        4
    }

    pub fn add_a_c(&mut self) -> isize {
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_add(self.register.c);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (self.register.c & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + self.register.c as u16) > 0xFF);

        4
    }

    pub fn add_a_d(&mut self) -> isize {
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_add(self.register.d);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (self.register.d & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + self.register.d as u16) > 0xFF);

        4
    }

    pub fn add_a_e(&mut self) -> isize {
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_add(self.register.e);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (self.register.e & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + self.register.e as u16) > 0xFF);

        4
    }

    pub fn add_a_h(&mut self) -> isize {
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_add(self.register.h);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (self.register.h & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + self.register.h as u16) > 0xFF);

        4
    }

    pub fn add_a_l(&mut self) -> isize {
        let old_a = self.register.a;
        self.register.a = old_a.wrapping_add(self.register.l);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (self.register.l & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + self.register.l as u16) > 0xFF);

        4
    }

    pub fn add_a_hl(&mut self, mmu: &Mmu) -> isize {
        let value = mmu.read_byte(self.register.get_hl());
        let old_a = self.register.a;
        self.register.a = value.wrapping_add(old_a);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (value & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + value as u16) > 0xFF);

        8
    }

    pub fn add_a_a(&mut self) -> isize {
        let value = self.register.a;
        let old_a = self.register.a;
        self.register.a = value.wrapping_add(old_a);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (value & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + value as u16) > 0xFF);

        4
    }

    pub fn adc_a_a(&mut self) -> isize {
        let old_a = self.register.a;
        let value = self.register.a;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = value.wrapping_add(old_a).wrapping_add(carry_bit);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (old_a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_b(&mut self) -> isize {
        let old_a = self.register.a;
        let value = self.register.b;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = value.wrapping_add(old_a).wrapping_add(carry_bit);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (old_a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_c(&mut self) -> isize {
        let old_a = self.register.a;
        let value = self.register.c;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = value.wrapping_add(old_a).wrapping_add(carry_bit);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (old_a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_d(&mut self) -> isize {
        let old_a = self.register.a;
        let value = self.register.d;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = value.wrapping_add(old_a).wrapping_add(carry_bit);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (old_a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_e(&mut self) -> isize {
        let old_a = self.register.a;
        let value = self.register.e;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = value.wrapping_add(old_a).wrapping_add(carry_bit);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (old_a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_h(&mut self) -> isize {
        let old_a = self.register.a;
        let value = self.register.h;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = value.wrapping_add(old_a).wrapping_add(carry_bit);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (old_a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_l(&mut self) -> isize {
        let old_a = self.register.a;
        let value = self.register.l;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = value.wrapping_add(old_a).wrapping_add(carry_bit);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (old_a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_d8(&mut self, mmu: &Mmu) -> isize {
        let old_a = self.register.a;
        let value = mmu.read_byte(self.register.pc);
        self.register.pc += 1;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = value.wrapping_add(old_a).wrapping_add(carry_bit);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (old_a as u16) + (carry_bit as u16) > 0xFF,
        );

        8
    }

    pub fn adc_a_hl(&mut self, mmu: &Mmu) -> isize {
        let old_a = self.register.a;
        let value = mmu.read_byte(self.register.get_hl());
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        self.register.a = value.wrapping_add(old_a).wrapping_add(carry_bit);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (old_a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn add_a_d8(&mut self, mmu: &Mmu) -> isize {
        let value = mmu.read_byte(self.register.pc);
        self.register.pc += 1;
        let old_a = self.register.a;
        self.register.a = self.register.a.wrapping_add(value);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, self.register.a == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (value & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + value as u16) > 0xFF);

        8
    }
}
