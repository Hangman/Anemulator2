use crate::gameboy::cpu::registers::Registers;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

pub fn jr_r8(register: &mut Registers, mmu: &Mmu) -> isize {
    let offset = mmu.read_byte(register.pc);
    register.pc += 1;
    register.pc += offset as u16;
    12
}
