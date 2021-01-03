pub mod compress;
pub mod decompress;
pub mod file_io;

use std::time::Instant;

fn main() {
    const FILE_NAME: &str = "testFile.hps";
    const BLOCK_SIZE: usize = 4;
    {
        let mut to_compress: Vec<u8> = file_io::read_from_file("EnglishShortened.txt");
        // let mut to_compress: Vec<u8> = String::from("Hello, world!").into_bytes();

        let compressed: Vec<u32> = compress::compress(BLOCK_SIZE, &mut to_compress);

        file_io::write_to_file_serialized(&compressed, FILE_NAME);
    }

    let mut from_file = file_io::read_from_file_deserialized(FILE_NAME);

    let start = Instant::now();
    let decompressed = decompress::decompress(&mut from_file, BLOCK_SIZE);
    println!("Seconds to decompress: {}\n", start.elapsed().as_millis() as f64/1000 as f64);

    println!("{}", String::from_utf8_lossy(&decompressed));
}
