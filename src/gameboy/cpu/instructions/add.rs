use crate::gameboy::cpu::registers::{FlagId, Registers};

pub fn add_hl_bc(register: &mut Registers) -> isize {
    let bc = register.get_bc();
    let hl = register.get_hl();
    let result = hl.wrapping_add(bc);
    register.set_hl(result);

    // SET FLAGS
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, (hl & 0xFFF) + (bc & 0xFFF) > 0xFFF);
    register.set_flag(FlagId::C, (hl as u32) + (bc as u32) > 0xFFFF);

    8
}

pub fn add_hl_de(register: &mut Registers) -> isize {
    let de = register.get_de();
    let hl = register.get_hl();
    let result = de.wrapping_add(hl);
    register.set_hl(result);

    // SET FLAGS
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, (hl & 0xFFF) + (de & 0xFFF) > 0xFFF);
    register.set_flag(FlagId::C, (hl as u32) + (de as u32) > 0xFFFF);

    8
}

pub fn add_hl_hl(register: &mut Registers) -> isize {
    let hl = register.get_hl();
    let result = hl.wrapping_add(hl);
    register.set_hl(result);

    // SET FLAGS
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, (hl & 0xFFF) + (hl & 0xFFF) > 0xFFF);
    register.set_flag(FlagId::C, (hl as u32) + (hl as u32) > 0xFFFF);

    8
}

pub fn add_hl_sp(register: &mut Registers) -> isize {
    let sp = register.sp;
    let hl = register.get_hl();
    let result = hl.wrapping_add(sp);
    register.set_hl(result);

    // SET FLAGS
    register.set_flag(FlagId::N, false);
    register.set_flag(FlagId::H, (hl & 0xFFF) + (sp & 0xFFF) > 0xFFF);
    register.set_flag(FlagId::C, (sp as u32) + (hl as u32) > 0xFFFF);

    8
}
