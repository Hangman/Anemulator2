use crate::gameboy::cpu::registers::{FlagId, Registers};

pub fn rrc_a(register: &mut Registers) -> isize {
    let old_a = register.a;
    register.a = old_a.rotate_right(1);

    // SET FLAGS
    register.set_flag(FlagId::Z, false);
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, false);
    register.set_flag(FlagId::C, (old_a & 0x01) != 0);

    4
}
