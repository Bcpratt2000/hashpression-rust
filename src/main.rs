pub mod compress;
pub mod decompress;
pub mod file_io;

use bincode;

fn main() {
    let FILE_NAME = "testFile.hps";
    let BLOCK_SIZE = 3;
    {
        let mut to_compress: Vec<u8> = file_io::read_from_file("English.txt");

        let compressed: Vec<u32> = compress::compress(BLOCK_SIZE, &mut to_compress);

        file_io::write_to_file_serialized(&compressed, FILE_NAME);
    }

    let from_file = file_io::read_from_file_deserialized(FILE_NAME);

    let decompressed = decompress::decompress(&from_file, BLOCK_SIZE);

    println!("{}", String::from_utf8_lossy(&decompressed));
}
