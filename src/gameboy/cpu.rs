use strum::IntoEnumIterator;

use crate::gameboy::cpu::instructions::decode;
use crate::gameboy::cpu::interrupt::Interrupt;
use crate::gameboy::cpu::registers::Registers;
use crate::gameboy::memory::memory;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::memory::mmu::Mmu;

mod instructions;
pub mod interrupt;
mod registers;

pub struct Cpu {
    register: Registers,
    cycle_accumulator: isize,
    halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            register: Registers::new(),
            cycle_accumulator: 0,
            halted: false,
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu) {
        self.cycle_accumulator += 4;

        if self.cycle_accumulator > 0 {
            self.register.step();

            if self.handle_interrupts(mmu) {
                return;
            }

            if self.halted {
                self.cycle_accumulator -= 4;
                return;
            }

            let op_code = mmu.read_byte(self.register.pc);
            self.register.pc += 1;

            self.cycle_accumulator -= self.run_instruction(mmu, op_code);
        }
    }

    fn handle_interrupts(&mut self, mmu: &mut Mmu) -> bool {
        let was_halted = self.halted;
        let enabled_register = mmu.read_byte(memory::IE);
        let flag_register = mmu.read_byte(memory::IF);
        if enabled_register > 0 && flag_register > 0 {
            for interrupt in Interrupt::iter() {
                let mask = interrupt.flag_mask();
                if (enabled_register & mask) > 0 && (flag_register & mask) > 0 {
                    self.halted = false;
                    if self.register.is_interrupts_enabled() {
                        // PUSH PC TO THE STACK
                        self.register.sp -= 1;
                        mmu.write_byte(self.register.sp, (self.register.pc >> 8) as u8);
                        self.register.sp -= 1;
                        mmu.write_byte(self.register.sp, self.register.pc as u8);

                        // JUMP
                        self.register.pc = interrupt.jump_address();

                        // CLEAR IME AND IF
                        self.register.set_interrupts_enabled(false, false);
                        mmu.set_bit(memory::IF, interrupt.bit_number(), false);

                        self.cycle_accumulator -= 20;
                        if was_halted {
                            self.cycle_accumulator -= 4;
                        }

                        return true;
                    }
                }
            }
        }

        if was_halted && !self.halted {
            self.cycle_accumulator -= 4;
        }

        false
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }
}
