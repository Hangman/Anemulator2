use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::mmu::Mmu;
use crate::gameboy::memory::random_access_memory::RandomAccessMemory;
use crate::gameboy::ppu::Ppu;
use crate::gameboy::ram::memory;
use crate::gameboy::memory::wram::Wram;

pub struct Gameboy {
    cpu: Cpu,
    ppu: Ppu,
    // apu: Apu,
    // mbc: Mbc
    mmu: Mmu,
    // timer: Timer,
    // joypad: Joypad,
}

impl Gameboy {
    pub fn new() -> Self {
        let mut mmu = Mmu::new();

        // ADD MEMORY UNITS
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("VRAM", 0x8000, 0xA000 - 0x8000)));
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("OAM RAM", 0xFE00, 0xFEA0 - 0xFE00)));
        mmu.add_memory_unit(Box::from(Wram::new()));
        // TODO ADD MBC
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("HRAM", 0xFF80, 0xFFFF - 0xFF80)));
        // TODO ADD INTERRUPTS REGISTERS
        // TODO ADD TIMER & DIV
        // TODO ADD JOYPAD
        // TODO ADD DMA
        // TODO ADD PPU
        // TODO ADD SERIAL BUS
        // TODO ADD APU
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("KEY1", memory::KEY1, 1)));
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("VBK", memory::VBK, 1)));
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("HDMA", memory::HDMA1, 5))); // HDMA
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("RP", memory::RP, 1))); // RP
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("BCPS & BCPD & OCPS & OCPD", memory::BCPS, 4))); // BCPS & BCPD & OCPS & OCPD
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("SVBK", memory::SVBK, 1))); // SVBK
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("PROHIBITED AREA", 0xFEA0, 0x60))); // PROHIBITED AREA
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("? 1", 0xFF7F, 1))); // ?
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("? 2", 0xFF03, 1))); // ?
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("? 3", 0xFF08, 7))); // ?
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("? 4", 0xFF4C, 1))); // ?
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("? 5", 0xFF4E, 1))); // ?
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("? 6", 0xFF57, 17))); // ?
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("? 7", 0xFF6C, 4))); // ?
        mmu.add_memory_unit(Box::from(RandomAccessMemory::new("? 8", 0xFF71, 14))); // ?

        Self {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            mmu,
        }
    }
}