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
