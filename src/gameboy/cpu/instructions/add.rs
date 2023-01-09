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
        let old_b = self.register.b;
        let old_a = self.register.a;
        let result = old_b.wrapping_add(old_a);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (old_b & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + old_b as u16) > 0xFF);

        4
    }

    pub fn add_a_c(&mut self) -> isize {
        let old_c = self.register.c;
        let old_a = self.register.a;
        let result = old_c.wrapping_add(old_a);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (old_c & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + old_c as u16) > 0xFF);

        4
    }

    pub fn add_a_d(&mut self) -> isize {
        let old_d = self.register.d;
        let old_a = self.register.a;
        let result = old_d.wrapping_add(old_a);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (old_d & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + old_d as u16) > 0xFF);

        4
    }

    pub fn add_a_e(&mut self) -> isize {
        let old_e = self.register.e;
        let old_a = self.register.a;
        let result = old_e.wrapping_add(old_a);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (old_e & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + old_e as u16) > 0xFF);

        4
    }

    pub fn add_a_h(&mut self) -> isize {
        let old_h = self.register.h;
        let old_a = self.register.a;
        let result = old_h.wrapping_add(old_a);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (old_h & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + old_h as u16) > 0xFF);

        4
    }

    pub fn add_a_l(&mut self) -> isize {
        let old_l = self.register.l;
        let old_a = self.register.a;
        let result = old_l.wrapping_add(old_a);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (old_a & 0xF) + (old_l & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (old_a as u16 + old_l as u16) > 0xFF);

        4
    }

    pub fn add_a_hl(&mut self, mmu: &Mmu) -> isize {
        let value = mmu.read_byte(self.register.get_hl());
        let a = self.register.a;
        let result = value.wrapping_add(a);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (a & 0xF) + (value & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (a as u16 + value as u16) > 0xFF);

        8
    }

    pub fn add_a_a(&mut self) -> isize {
        let value = self.register.a;
        let a = self.register.a;
        let result = value.wrapping_add(a);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (a & 0xF) + (value & 0xF) > 0xF);
        self.register
            .set_flag(FlagId::C, (a as u16 + value as u16) > 0xFF);

        4
    }

    pub fn adc_a_a(&mut self) -> isize {
        let a = self.register.a;
        let value = self.register.a;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        let result = value.wrapping_add(a).wrapping_add(carry_bit);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_b(&mut self) -> isize {
        let a = self.register.a;
        let value = self.register.b;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        let result = value.wrapping_add(a).wrapping_add(carry_bit);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_c(&mut self) -> isize {
        let a = self.register.a;
        let value = self.register.c;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        let result = value.wrapping_add(a).wrapping_add(carry_bit);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_d(&mut self) -> isize {
        let a = self.register.a;
        let value = self.register.d;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        let result = value.wrapping_add(a).wrapping_add(carry_bit);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_e(&mut self) -> isize {
        let a = self.register.a;
        let value = self.register.e;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        let result = value.wrapping_add(a).wrapping_add(carry_bit);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_h(&mut self) -> isize {
        let a = self.register.a;
        let value = self.register.h;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        let result = value.wrapping_add(a).wrapping_add(carry_bit);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_l(&mut self) -> isize {
        let a = self.register.a;
        let value = self.register.l;
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        let result = value.wrapping_add(a).wrapping_add(carry_bit);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }

    pub fn adc_a_hl(&mut self, mmu: &Mmu) -> isize {
        let a = self.register.a;
        let value = mmu.read_byte(self.register.get_hl());
        let carry_bit = self.register.is_flag_set(FlagId::C) as u8;
        let result = value.wrapping_add(a).wrapping_add(carry_bit);
        self.register.a = result;

        // SET FLAGS
        self.register.set_flag(FlagId::Z, result == 0);
        self.register.set_flag(FlagId::N, false);
        self.register
            .set_flag(FlagId::H, (a & 0xF) + (value & 0xF) + carry_bit > 0xF);
        self.register.set_flag(
            FlagId::C,
            (value as u16) + (a as u16) + (carry_bit as u16) > 0xFF,
        );

        4
    }
}
