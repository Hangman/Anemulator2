pub fn set_bit_u8(byte: u8, bitnum: usize, value: bool) -> u8 {
    let mut data = byte;
    if value {
        data |= 1 << bitnum;
    } else {
        data &= !(1 << bitnum);
    }
    data
}

/// Get a bit of a number.
macro_rules! get_bit {
    ($num:expr, $bit:expr) => {
        (*($num) & (1 << $bit)) != 0
    };
}

/// Set a bit of a number.
macro_rules! set_bit {
    ($num:expr, $bit:expr) => {
        *($num) |= (1 << $bit);
    };
}

/// Reset a bit of a number.
macro_rules! reset_bit {
    ($num:expr, $bit:expr) => {
        *($num) &= !(1 << $bit);
    };
}

pub fn is_bit_set_u8(byte: u8, bitnum: usize) -> bool {
    (byte & (1 << bitnum)) > 0
}

pub(crate) use {get_bit, reset_bit, set_bit};
