use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

impl Cpu {
    pub fn set_b(&mut self, index: u8) -> isize {
        self.register.b |= 1 << index;
        8
    }

    pub fn set_c(&mut self, index: u8) -> isize {
        self.register.c |= 1 << index;
        8
    }

    pub fn set_d(&mut self, index: u8) -> isize {
        self.register.d |= 1 << index;
        8
    }

    pub fn set_e(&mut self, index: u8) -> isize {
        self.register.e |= 1 << index;
        8
    }

    pub fn set_h(&mut self, index: u8) -> isize {
        self.register.h |= 1 << index;
        8
    }

    pub fn set_l(&mut self, index: u8) -> isize {
        self.register.l |= 1 << index;
        8
    }

    pub fn set_hl(&mut self, index: u8, mmu: &mut Mmu) -> isize {
        let address = self.register.get_hl();
        let value = mmu.read_byte(address);
        let new_value = value | (1 << index);
        mmu.write_byte(address, new_value);
        16
    }

    pub fn set_a(&mut self, index: u8) -> isize {
        self.register.b |= 1 << index;
        8
    }
}
