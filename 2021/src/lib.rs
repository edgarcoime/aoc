use std::fs;
use std::path::Path;

pub mod days;

pub fn read_input<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).expect("Failed to read input file")
}

pub fn bits_to_num<I>(bits: I) -> i32
where
    I: IntoIterator<Item = u8>
{
    bits.into_iter().fold(0, |acc, bit| (acc << 1) | bit as i32)
}
