use crate::gameboy::cpu::registers::{FlagId, Registers};

pub fn stop(register: &mut Registers) -> isize {
    // TODO: implement stop properly
    register.pc += 1;
    4
}

pub fn rla(register: &mut Registers) -> isize {
    let old_a = register.a;
    let carry_bit = register.is_flag_set(FlagId::C) as u8;
    register.a = (old_a << 1) | carry_bit;

    register.set_flag(FlagId::Z, false);
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, false);
    register.set_flag(FlagId::C, old_a >> 7 == 1);

    8
}

pub fn rra(register: &mut Registers) -> isize {
    let lsb = register.a & 0b1;
    let carry_bit = register.is_flag_set(FlagId::C) as u8;
    let a = (register.a >> 1) | (carry_bit << 7);
    register.a = a;

    // SET FLAGS
    register.set_flag(FlagId::Z, false);
    register.set_flag(FlagId::H, false);
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::C, lsb != 0);

    8
}

pub fn da_a(register: &mut Registers) -> isize {
    if register.is_flag_set(FlagId::N) {
        if register.is_flag_set(FlagId::C) {
            register.a = register.a.wrapping_add(0xA0);
            register.set_flag(FlagId::C, true);
        }
        if register.is_flag_set(FlagId::H) {
            register.a = register.a.wrapping_add(0xFA);
        }
    } else {
        if register.is_flag_set(FlagId::C) || (register.a > 0x99) {
            register.a = register.a.wrapping_add(0x60);
            register.set_flag(FlagId::C, true);
        }
        if register.is_flag_set(FlagId::H) || ((register.a & 0xF) > 0x9) {
            register.a = register.a.wrapping_add(0x06);
        }
    }

    // SET FLAGS
    register.set_flag(FlagId::Z, register.a == 0);
    register.set_flag(FlagId::H, false);

    4
}

pub fn cpl_a(register: &mut Registers) -> isize {
    register.a = !register.a;

    // SET FLAGS
    register.set_flag(FlagId::N, true);
    register.set_flag(FlagId::H, true);

    4
}

pub fn scf(register: &mut Registers) -> isize {
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, false);
    register.set_flag(FlagId::C, true);
    4
}

pub fn ccf(register: &mut Registers) -> isize {
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, false);
    register.set_flag(FlagId::C, !register.is_flag_set(FlagId::C));
    4
}
