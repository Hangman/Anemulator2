pub mod cpu;
pub mod ppu;

pub struct Gameboy {
    cpu: cpu::Cpu,
    ppu: ppu::Ppu,
}

impl Gameboy {
    pub fn new() -> Self {
        Self {
            cpu: cpu::Cpu::new(),
            ppu: ppu::Ppu::new(),
        }
    }
}