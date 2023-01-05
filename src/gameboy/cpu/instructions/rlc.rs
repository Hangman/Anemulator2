use crate::gameboy::cpu::registers::{FlagId, Registers};

pub fn rlc_a(register: &mut Registers) -> isize {
    let a = register.a;
    register.a = a.rotate_left(1);

    // SET FLAGS
    register.set_flag(FlagId::Z, false);
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, false);
    register.set_flag(FlagId::C, (a & 0x80) != 0);

    return 4;
}
