pub mod compress;
pub mod file_io;
pub mod decompress;

use bincode;

fn main() {
    let FILE_NAME = "testFile";
    let BLOCK_SIZE = 3;

    let mut compressed_vector: Vec<u32> = Vec::new();
    {
        let mut s =
            String::from("Hello,\nWorld!");

        compressed_vector = compress::compress(BLOCK_SIZE, &mut s);

        let serialized: Vec<u8> = bincode::serialize(&compressed_vector).unwrap();

        file_io::write_vec8_to_file(&serialized, FILE_NAME);
    }
    let deserialized = file_io::read_from_file_deserialized(FILE_NAME);

    assert_eq!(deserialized, compressed_vector);

    println!("{}", String::from_utf8_lossy(decompress::decompress(&deserialized, BLOCK_SIZE).as_slice()));
}
