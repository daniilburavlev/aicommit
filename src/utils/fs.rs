use std::env::home_dir;
use std::fs::OpenOptions;
use std::io::{Read, Write};

pub fn write_to_home_dir(filename: &str, data: &str) -> bool {
    if let Some(mut home_dir) = home_dir() {
        home_dir.push(filename);
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(home_dir)
        {
            if let Err(_) = file.write_all(data.as_bytes()) {
                return false;
            }
        }
        true
    } else {
        false
    }
}


pub fn read_from_home_dir(filename: &str) -> Option<String> {
    if let Some(mut home_dir) = home_dir() {
        home_dir.push(filename);
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(home_dir)
        {
            let mut data = String::new();
            if let Ok(_) = file.read_to_string(&mut data) {
                return Some(data);
            }
        }
        None
    } else {
        None
    }
}
