use crate::gameboy::cpu::registers::{FlagId, Registers};

pub fn dec_b(register: &mut Registers) -> isize {
    let old_b = register.b;
    register.b = old_b.wrapping_sub(1);

    // SET FLAGS
    register.set_flag(FlagId::Z, register.b == 0);
    register.set_flag(FlagId::N, true);
    register.set_flag(FlagId::H, old_b.trailing_zeros() >= 4);

    4
}

pub fn dec_c(register: &mut Registers) -> isize {
    let old_c = register.c;
    register.c = old_c.wrapping_sub(1);

    // SET FLAGS
    register.set_flag(FlagId::Z, register.c == 0);
    register.set_flag(FlagId::N, true);
    register.set_flag(FlagId::H, old_c.trailing_zeros() >= 4);

    4
}

pub fn dec_d(register: &mut Registers) -> isize {
    let old_d = register.d;
    register.d = old_d.wrapping_sub(1);

    // SET FLAGS
    register.set_flag(FlagId::Z, register.d == 0);
    register.set_flag(FlagId::N, true);
    register.set_flag(FlagId::H, old_d.trailing_zeros() >= 4);

    4
}

pub fn dec_bc(register: &mut Registers) -> isize {
    register.set_bc(register.get_bc().wrapping_sub(1));

    8
}
