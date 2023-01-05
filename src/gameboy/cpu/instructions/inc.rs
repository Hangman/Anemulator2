use crate::gameboy::cpu::registers::{FlagId, Registers};

pub fn inc_bc(register: &mut Registers) -> isize {
    let value = register.get_bc();
    register.set_bc(value + 1);
    8
}

pub fn inc_b(register: &mut Registers) -> isize {
    let old_b = register.b;
    let new_b = old_b.wrapping_add(1);
    register.b = new_b;

    // SET FLAGS
    register.set_flag(&FlagId::Z, new_b == 0);
    register.set_flag(&FlagId::N, false);
    register.set_flag(&FlagId::H, (old_b & 0xF) == 0xF);

    4
}
