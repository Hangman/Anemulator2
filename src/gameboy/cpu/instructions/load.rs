use crate::gameboy::cpu::registers::Registers;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

pub fn ld_bc_d16(register: &mut Registers, mmu: &mut Mmu) -> isize {
    register.set_bc(mmu.read_word(register.pc));
    register.pc += 2;
    12
}

pub fn ld_bc_a(register: &mut Registers, mmu: &mut Mmu) -> isize {
    mmu.write_byte(register.get_bc(), register.a);
    8
}

pub fn ld_a_a() -> isize {
    4
}

pub fn ld_a_b(register: &mut Registers) -> isize {
    register.a = register.b;
    4
}

pub fn ld_a_c(register: &mut Registers) -> isize {
    register.a = register.c;
    4
}

pub fn ld_a_d(register: &mut Registers) -> isize {
    register.a = register.d;
    4
}

pub fn ld_a_e(register: &mut Registers) -> isize {
    register.a = register.e;
    4
}

pub fn ld_a_h(register: &mut Registers) -> isize {
    register.a = register.h;
    4
}

pub fn ld_a_l(register: &mut Registers) -> isize {
    register.a = register.l;
    4
}

pub fn ld_b_b() -> isize {
    4
}

pub fn ld_b_c(register: &mut Registers) -> isize {
    register.b = register.c;
    4
}

pub fn ld_b_d(register: &mut Registers) -> isize {
    register.b = register.d;
    4
}

pub fn ld_b_e(register: &mut Registers) -> isize {
    register.b = register.e;
    4
}

pub fn ld_b_h(register: &mut Registers) -> isize {
    register.b = register.h;
    4
}

pub fn ld_b_l(register: &mut Registers) -> isize {
    register.b = register.l;
    4
}

pub fn ld_d_a(register: &mut Registers) -> isize {
    register.d = register.a;
    4
}

pub fn ld_d_b(register: &mut Registers) -> isize {
    register.d = register.b;
    4
}

pub fn ld_d_c(register: &mut Registers) -> isize {
    register.d = register.c;
    4
}

pub fn ld_d_d() -> isize {
    4
}

pub fn ld_d_e(register: &mut Registers) -> isize {
    register.d = register.e;
    4
}

pub fn ld_d_h(register: &mut Registers) -> isize {
    register.d = register.h;
    4
}

pub fn ld_d_l(register: &mut Registers) -> isize {
    register.d = register.l;
    4
}

pub fn ld_h_a(register: &mut Registers) -> isize {
    register.h = register.a;
    4
}

pub fn ld_h_b(register: &mut Registers) -> isize {
    register.h = register.b;
    4
}

pub fn ld_h_c(register: &mut Registers) -> isize {
    register.h = register.c;
    4
}

pub fn ld_h_d(register: &mut Registers) -> isize {
    register.h = register.d;
    4
}

pub fn ld_h_e(register: &mut Registers) -> isize {
    register.h = register.e;
    4
}

pub fn ld_h_h() -> isize {
    4
}

pub fn ld_h_l(register: &mut Registers) -> isize {
    register.h = register.l;
    4
}

pub fn ld_l_a(register: &mut Registers) -> isize {
    register.l = register.a;
    4
}

pub fn ld_l_b(register: &mut Registers) -> isize {
    register.l = register.b;
    4
}

pub fn ld_l_c(register: &mut Registers) -> isize {
    register.l = register.c;
    4
}

pub fn ld_l_d(register: &mut Registers) -> isize {
    register.l = register.d;
    4
}

pub fn ld_l_e(register: &mut Registers) -> isize {
    register.l = register.e;
    4
}

pub fn ld_l_h(register: &mut Registers) -> isize {
    register.l = register.h;
    4
}

pub fn ld_l_l() -> isize {
    4
}

pub fn ld_e_a(register: &mut Registers) -> isize {
    register.e = register.a;
    4
}

pub fn ld_e_b(register: &mut Registers) -> isize {
    register.e = register.b;
    4
}

pub fn ld_e_c(register: &mut Registers) -> isize {
    register.e = register.c;
    4
}

pub fn ld_e_d(register: &mut Registers) -> isize {
    register.e = register.d;
    4
}

pub fn ld_e_e() -> isize {
    4
}

pub fn ld_e_h(register: &mut Registers) -> isize {
    register.e = register.h;
    4
}

pub fn ld_e_l(register: &mut Registers) -> isize {
    register.e = register.l;
    4
}

pub fn ld_c_a(register: &mut Registers) -> isize {
    register.c = register.a;
    4
}

pub fn ld_c_b(register: &mut Registers) -> isize {
    register.c = register.b;
    4
}

pub fn ld_c_c() -> isize {
    4
}

pub fn ld_c_d(register: &mut Registers) -> isize {
    register.c = register.d;
    4
}

pub fn ld_c_e(register: &mut Registers) -> isize {
    register.c = register.e;
    4
}

pub fn ld_c_h(register: &mut Registers) -> isize {
    register.c = register.h;
    4
}

pub fn ld_c_l(register: &mut Registers) -> isize {
    register.c = register.l;
    4
}
