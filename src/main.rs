pub mod compress;
pub mod decompress;
pub mod file_io;

use std::time::Instant;

fn main() {
    const FILE_NAME: &str = "testFile.hps";
    const BLOCK_SIZE: usize = 8;
    {
        let mut to_compress: Vec<u8> = file_io::read_from_file("Worship Backup 12-27-2020.mp4");
        // let mut to_compress: Vec<u8> = String::from("Hello, world!").into_bytes();

        let compressed: Vec<u32> = compress::compress(BLOCK_SIZE, &mut to_compress);

        file_io::write_to_file_serialized(&compressed, FILE_NAME);
    }

    let from_file = file_io::read_from_file_deserialized(FILE_NAME);

    let start = Instant::now();
    let decompressed = decompress::decompress(&from_file, BLOCK_SIZE);
    println!("Seconds to decompress: {}", start.elapsed().as_millis() as f64/1000 as f64);

    println!("{}", String::from_utf8_lossy(&decompressed));

    let mut to_compress: Vec<u8> = String::from("~").into_bytes();

    // let bleh = compress::generate_char_bitmask(&to_compress);
    // println!("{:?}", bleh);
}
