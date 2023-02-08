use crate::gameboy::util::joypad_key::JoypadKey::{Down, Left, Right, Select, Start, Up, A, B};
use crate::gameboy::util::joypad_key::JoypadKeyType::{Action, Direction};

pub const VALUES: [JoypadKey; 8] = [Up, Down, Left, Right, Start, Select, A, B];

pub enum JoypadKey {
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
    A,
    B,
}

#[derive(PartialEq)]
pub enum JoypadKeyType {
    Action,
    Direction,
}

impl JoypadKey {
    pub fn get_index(&self) -> usize {
        match self {
            Up => 0,
            Down => 1,
            Left => 2,
            Right => 3,
            Start => 4,
            Select => 5,
            A => 6,
            B => 7,
        }
    }

    pub fn get_type(&self) -> JoypadKeyType {
        match self {
            Up => Direction,
            Down => Direction,
            Left => Direction,
            Right => Direction,
            Start => Action,
            Select => Action,
            A => Action,
            B => Action,
        }
    }

    pub fn get_bitnum(&self) -> u8 {
        match self {
            Up => 2,
            Down => 3,
            Left => 1,
            Right => 0,
            Start => 3,
            Select => 2,
            A => 0,
            B => 1,
        }
    }
}
