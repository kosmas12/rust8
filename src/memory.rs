/*
This file is part of Rust8 - a CHIP-8 emulator made in Rust
Copyright (C) 2021  Kosmas Raptis

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub fn load_rom(path: &Path) -> Vec<u8> {
    let path_display = path.display();

    let mut file = match File::open(&path) {
        Err(error) => panic!("Couldn't open ROM {}: {}", path_display, error),
        Ok(file) => file,
    };

    let mut buffer = [0; 1];
    let mut memory = vec![0; 4096];

    let mut i = 200;
    let mut bytes = file.read(&mut buffer)
        .expect("Couldn't read from ROM");

    while bytes != 0 {
        memory[i] = buffer[0];
        i = i + 1;
        bytes = file.read(&mut buffer)
            .expect("Couldn't read from ROM");
    }

    memory
}
