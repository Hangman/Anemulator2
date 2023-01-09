use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;

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
}
