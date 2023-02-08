use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::gameboy::cpu::interrupt::Interrupt;
use crate::gameboy::memory::memory;
use crate::gameboy::memory::memory::Memory;
use crate::gameboy::util::bit_util::set_bit;
use crate::gameboy::util::joypad_key;
use crate::gameboy::util::joypad_key::{JoypadKey, JoypadKeyType};

pub struct Joypad {
    register: u8,
    buttons_pressed: [bool; 8],
    if_register: Rc<RefCell<u8>>,
}

impl Joypad {
    pub fn new(if_register: Rc<RefCell<u8>>) -> Self {
        Self {
            register: 0,
            buttons_pressed: [false; 8],
            if_register,
        }
    }

    pub fn on_joypad_state_change(&mut self, key: JoypadKey, pressed: bool) {
        self.buttons_pressed[key.get_index()] = pressed;
        self.update_input();

        // TODO: ONLY TRIGGER INTERRUPT IF THE CORRESPONDING BUTTON TYPE IS SELECTED
        set_bit!(
            self.if_register.borrow_mut(),
            Interrupt::Joypad.bit_number()
        );
    }

    fn update_input(&mut self) {
        let mut ff00 = self.register | 0b11000000;
        let type_selected = if ff00 & (1 << 5) == 0 {
            JoypadKeyType::Action
        } else {
            JoypadKeyType::Direction
        };

        for key in joypad_key::VALUES {
            if key.get_type() == type_selected {
                let pressed = self.buttons_pressed[key.get_index()];
                if pressed {
                    ff00 &= !(1 << key.get_bitnum());
                } else {
                    ff00 |= 1 << key.get_bitnum();
                }
            }
        }
        self.register = ff00;
    }
}

impl Memory for Joypad {
    fn accepts_address(&self, address: u16) -> bool {
        address == memory::JOYPAD
    }

    fn read_byte(&self, address: u16) -> u8 {
        if address == memory::JOYPAD {
            return self.register;
        }
        panic!("invalid address: {address}");
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if address == memory::JOYPAD {
            self.register = value;
            self.update_input();
            return;
        }
        panic!("invalid address: {address}");
    }
}

impl Display for Joypad {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Joypad")
    }
}
