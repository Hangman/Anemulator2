use crate::gameboy::cpu::registers::{FlagId, Registers};
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

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

pub fn inc_hl(register: &mut Registers) -> isize {
    let value = register.get_hl();
    register.set_hl(value.wrapping_add(1));
    8
}

pub fn inc_hl_(register: &mut Registers, mmu: &mut Mmu) -> isize {
    let address = register.get_hl();
    let old_value = mmu.read_byte(address);
    let new_value = old_value.wrapping_add(1);
    mmu.write_byte(address, new_value);

    // SET FLAGS
    register.set_flag(FlagId::Z, new_value == 0);
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, (old_value & 0xF) == 0xF);

    return 12;
}

pub fn inc_sp(register: &mut Registers) -> isize {
    register.sp = register.sp.wrapping_add(1);
    8
}

pub fn inc_a(register: &mut Registers) -> isize {
    let old_a = register.a;
    let new_a = old_a.wrapping_add(1);
    register.a = new_a;

    // SET FLAGS
    register.set_flag(FlagId::Z, new_a == 0);
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, (old_a & 0xF) == 0xF);

    4
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

pub fn inc_h(register: &mut Registers) -> isize {
    let old_h = register.h;
    let new_h = old_h.wrapping_add(1);
    register.h = new_h;

    // SET FLAGS
    register.set_flag(FlagId::Z, new_h == 0);
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, (old_h & 0xF) == 0xF);

    4
}

pub fn inc_l(register: &mut Registers) -> isize {
    let old_l = register.l;
    let new_l = old_l.wrapping_add(1);
    register.l = new_l;

    // SET FLAGS
    register.set_flag(FlagId::Z, new_l == 0);
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, (old_l & 0xF) == 0xF);

    4
}
