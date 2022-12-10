use std::fs;
use crate::gameboy::mbc::mbc1::Mbc1;

use crate::gameboy::mbc::mbc::Mbc;

pub fn load(path: String) -> Box<dyn Mbc> {
    let data = fs::read(path).expect("Error reading file");
    match data[0x147] {
        0x1 => {
            Box::from(Mbc1::new(data.as_slice()))
        }
        _ => panic!("Unsupported MBC found in ROM: {}", data[0x147])
    }
}