use crate::gameboy::cpu::registers::{FlagId, Registers};
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

pub fn jr_r8(register: &mut Registers, mmu: &Mmu) -> isize {
    let offset = mmu.read_byte(register.pc) as i8 as i16;
    register.pc += 1;
    register.pc = (register.pc as i16 + offset) as u16;
    12
}

pub fn jr_nz_r8(register: &mut Registers, mmu: &Mmu) -> isize {
    if !register.is_flag_set(FlagId::Z) {
        let offset = mmu.read_byte(register.pc) as i8 as i16;
        register.pc += 1;
        register.pc = (register.pc as i16 + offset) as u16;
        return 12;
    }

    register.pc += 1;
    8
}

pub fn jr_nc_r8(register: &mut Registers, mmu: &Mmu) -> isize {
    if !register.is_flag_set(FlagId::C) {
        let offset = mmu.read_byte(register.pc) as i8 as i16;
        register.pc += 1;
        register.pc = (register.pc as i16 + offset) as u16;
        return 12;
    }

    register.pc += 1;
    8
}

pub fn jr_z_r8(register: &mut Registers, mmu: &Mmu) -> isize {
    if register.is_flag_set(FlagId::Z) {
        let offset = mmu.read_byte(register.pc) as i8 as i16;
        register.pc += 1;
        register.pc = (register.pc as i16 + offset) as u16;
        return 12;
    }

    register.pc += 1;
    8
}

pub fn jr_c_r8(register: &mut Registers, mmu: &Mmu) -> isize {
    if register.is_flag_set(FlagId::C) {
        let offset = mmu.read_byte(register.pc) as i8 as i16;
        register.pc += 1;
        register.pc = (register.pc as i16 + offset) as u16;
        return 12;
    }

    register.pc += 1;
    8
}
