pub struct Registers {
    pub a: u8,
    f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
    interrupts_enabled: bool,
    enable_interrupts_delay_counter: i8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0xFFFE,
            interrupts_enabled: false,
            enable_interrupts_delay_counter: 0,
        }
    }

    #[inline]
    pub fn get_f(&self) -> u8 {
        self.f
    }

    #[inline]
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    #[inline]
    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    #[inline]
    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    #[inline]
    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    #[inline]
    pub fn set_f(&mut self, value: u8) {
        self.f = value & 0xF0;
    }

    #[inline]
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value as u8) & 0xF0;
    }

    #[inline]
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    #[inline]
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    #[inline]
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    #[inline]
    pub fn is_flag_set(&self, id: &FlagId) -> bool {
        match id {
            FlagId::Z => self.f & (1 << 7) > 0,
            FlagId::N => self.f & (1 << 6) > 0,
            FlagId::H => self.f & (1 << 5) > 0,
            FlagId::C => self.f & (1 << 4) > 0,
        }
    }

    #[inline]
    pub fn set_flag(&mut self, id: &FlagId, value: bool) {
        let bit_index = match id {
            FlagId::Z => 7,
            FlagId::N => 6,
            FlagId::H => 5,
            FlagId::C => 4,
        };
        if value {
            self.f |= 1 << bit_index;
        } else {
            self.f &= !(1 << bit_index);
        }
    }

    pub fn step(&mut self) {
        if self.enable_interrupts_delay_counter > 0 {
            self.enable_interrupts_delay_counter -= 1;
            if self.enable_interrupts_delay_counter <= 0 {
                self.interrupts_enabled = true;
            }
        }
    }

    #[inline]
    pub fn is_interrupts_enabled(&self) -> bool {
        self.interrupts_enabled
    }

    #[inline]
    pub fn set_interrupts_enabled(&mut self, interrupts_enabled: bool, instant: bool) {
        if interrupts_enabled {
            if instant {
                self.interrupts_enabled = true;
            } else {
                self.enable_interrupts_delay_counter = 2;
            }
        } else {
            self.interrupts_enabled = false;
            self.enable_interrupts_delay_counter = 0;
        }
    }
}

pub enum FlagId {
    Z,
    N,
    H,
    C,
}
