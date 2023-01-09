use crate::gameboy::cpu::registers::FlagId;
use crate::gameboy::cpu::Cpu;

impl Cpu {
    pub fn rlc_a(&mut self) -> isize {
        let a = self.register.a;
        self.register.a = a.rotate_left(1);

        // SET FLAGS
        self.register.set_flag(FlagId::Z, false);
        self.register.set_flag(FlagId::N, false);
        self.register.set_flag(FlagId::H, false);
        self.register.set_flag(FlagId::C, (a & 0x80) != 0);

        return 4;
    }
}
