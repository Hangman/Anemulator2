use crate::gameboy::cpu::registers::{FlagId, Registers};

pub fn inc_de(register: &mut Registers) -> isize {
    let value = register.get_de();
    register.set_de(value.wrapping_add(1));
    8
}

pub fn inc_bc(register: &mut Registers) -> isize {
    let value = register.get_bc();
    register.set_bc(value.wrapping_add(1));
    8
}

pub fn inc_b(register: &mut Registers) -> isize {
    let old_b = register.b;
    let new_b = old_b.wrapping_add(1);
    register.b = new_b;

    // SET FLAGS
    register.set_flag(FlagId::Z, new_b == 0);
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, (old_b & 0xF) == 0xF);

    4
}

pub fn inc_c(register: &mut Registers) -> isize {
    let old_c = register.c;
    let new_c = old_c.wrapping_add(1);
    register.c = new_c;

    // SET FLAGS
    register.set_flag(FlagId::Z, new_c == 0);
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, (old_c & 0xF) == 0xF);

    4
}

pub fn inc_d(register: &mut Registers) -> isize {
    let old_d = register.d;
    let new_d = old_d.wrapping_add(1);
    register.d = new_d;

    // SET FLAGS
    register.set_flag(FlagId::Z, new_d == 0);
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, (old_d & 0xF) == 0xF);

    4
}

pub fn inc_e(register: &mut Registers) -> isize {
    let old_e = register.e;
    let new_e = old_e.wrapping_add(1);
    register.e = new_e;

    // SET FLAGS
    register.set_flag(FlagId::Z, new_e == 0);
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, (old_e & 0xF) == 0xF);

    4
}
