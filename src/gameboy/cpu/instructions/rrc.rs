use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;

impl Cpu {
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
}
