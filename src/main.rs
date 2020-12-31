pub mod compress;
pub mod decompress;
pub mod file_io;

use bincode;

fn main() {
    let FILE_NAME = "testFile";
    let BLOCK_SIZE = 3;

    let mut compressed_vector: Vec<u32> = Vec::new();
    {
        let mut s: Vec<u8> = "Ben Pratt".as_bytes().to_vec();

        compressed_vector = compress::compress(BLOCK_SIZE, &mut s);
        file_io::write_to_file_serialized(&compressed_vector, FILE_NAME)
    }
    let deserialized = file_io::read_from_file_deserialized(FILE_NAME);

    assert_eq!(deserialized, compressed_vector);

    println!(
        "{}",
        String::from_utf8_lossy(decompress::decompress(&deserialized, BLOCK_SIZE).as_slice())
    );
}
