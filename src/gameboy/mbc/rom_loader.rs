use std::fs;

use crate::gameboy::mbc::mbc::Mbc;
use crate::gameboy::mbc::mbc0::Mbc0;
use crate::gameboy::mbc::mbc1::Mbc1;

pub fn load(path: String) -> Box<dyn Mbc> {
    let path_copy = path.clone();
    let data = fs::read(path)
        .unwrap_or_else(|error| panic!("Error reading file {}: {}", path_copy, error));
    match data[0x147] {
        0x0 => Box::from(Mbc0::new(data.as_slice())),
        0x1 => Box::from(Mbc1::new(data.as_slice())),
        _ => panic!("Unsupported MBC found in ROM: {}", data[0x147]),
    }
}
