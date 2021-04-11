use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub fn load_rom(path: &Path) -> Vec<u8> {
    let path_display = path.display();

    let mut file = match File::open(&path) {
        Err(error) => panic!("Couldn't open ROM {}: {}", path_display, error),
        Ok(file) => file,
    };

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("Couldn't read ROM!");

    buffer
}
