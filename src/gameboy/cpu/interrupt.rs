pub enum Interrupt {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    pub fn jump_address(&self) -> u16 {
        match self {
            Interrupt::VBlank => 0x40,
            Interrupt::LcdStat => 0x48,
            Interrupt::Timer => 0x50,
            Interrupt::Serial => 0x58,
            Interrupt::Joypad => 0x60,
        }
    }

    pub fn flag_mask(&self) -> u16 {
        1 << self.bit_number()
    }

    pub fn bit_number(&self) -> u8 {
        match self {
            Interrupt::VBlank => 0,
            Interrupt::LcdStat => 1,
            Interrupt::Timer => 2,
            Interrupt::Serial => 3,
            Interrupt::Joypad => 4,
        }
    }
}
