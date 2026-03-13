use std::fs;
use std::path::Path;

pub mod days;

pub fn read_input<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).expect("Failed to read input file")
}
