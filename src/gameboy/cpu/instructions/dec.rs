use crate::gameboy::cpu::registers::{FlagId, Registers};
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

pub fn dec_a(register: &mut Registers) -> isize {
    let old_a = register.a;
    register.a = old_a.wrapping_sub(1);

    // SET FLAGS
    register.set_flag(FlagId::Z, register.a == 0);
    register.set_flag(FlagId::N, true);
    register.set_flag(FlagId::H, old_a.trailing_zeros() >= 4);

    4
}

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

pub fn dec_e(register: &mut Registers) -> isize {
    let old_e = register.e;
    register.e = old_e.wrapping_sub(1);

    // SET FLAGS
    register.set_flag(FlagId::Z, register.e == 0);
    register.set_flag(FlagId::N, true);
    register.set_flag(FlagId::H, old_e.trailing_zeros() >= 4);

    4
}

pub fn dec_h(register: &mut Registers) -> isize {
    let old_h = register.h;
    register.h = old_h.wrapping_sub(1);

    // SET FLAGS
    register.set_flag(FlagId::Z, register.h == 0);
    register.set_flag(FlagId::N, true);
    register.set_flag(FlagId::H, old_h.trailing_zeros() >= 4);

    4
}

pub fn dec_l(register: &mut Registers) -> isize {
    let old_l = register.l;
    register.l = old_l.wrapping_sub(1);

    // SET FLAGS
    register.set_flag(FlagId::Z, register.l == 0);
    register.set_flag(FlagId::N, true);
    register.set_flag(FlagId::H, old_l.trailing_zeros() >= 4);

    4
}

pub fn dec_bc(register: &mut Registers) -> isize {
    register.set_bc(register.get_bc().wrapping_sub(1));
    8
}

pub fn dec_de(register: &mut Registers) -> isize {
    register.set_de(register.get_de().wrapping_sub(1));
    8
}

pub fn dec_hl(register: &mut Registers) -> isize {
    register.set_hl(register.get_hl().wrapping_sub(1));
    8
}

pub fn dec_hl_(register: &mut Registers, mmu: &mut Mmu) -> isize {
    let address = register.get_hl();
    let new_value = mmu.read_byte(address).wrapping_sub(1);
    mmu.write_byte(address, new_value);

    // SET FLAGS
    register.set_flag(FlagId::Z, new_value == 0);
    register.set_flag(FlagId::N, true);
    register.set_flag(FlagId::H, new_value.trailing_zeros() >= 4);

    12
}

pub fn dec_sp(register: &mut Registers) -> isize {
    register.sp = register.sp.wrapping_sub(1);
    8
}
