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
