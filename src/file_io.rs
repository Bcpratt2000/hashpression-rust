use bincode;
use std::fs::File;
use std::io::prelude::*;


pub fn read_from_file(file: &str) -> Vec<u8> {
    let mut file: File = File::open(file).unwrap();
    let mut buf: Vec<u8> = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_end(&mut buf).unwrap();
    buf
}

pub fn read_from_file_deserialized(file: &str) -> Vec<u32> {
    let mut file: File = File::open(file).unwrap();
    let mut buf: Vec<u8> = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_end(&mut buf).unwrap();
    let deserialized: Vec<u32> = bincode::deserialize(&buf).unwrap();
    deserialized
}

pub fn write_vec8_to_file(data: &Vec<u8>, file: &str) {
    let mut file: File = File::create(file).unwrap();
    file.write_all(data).unwrap();
    file.sync_all().unwrap();
}
