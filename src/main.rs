pub mod compress;
pub mod file_io;

use bincode;

fn main() {
    let file_name = "testFile";
    let mut compressed_vector: Vec<u32> = Vec::new();
    {
        let mut s =
            String::from("this is some random teasdsdfijgahfgasdfhgsalhflsdst. JIAghfbfibsEILYFGSJIAghfbfibsEILYFGSBLAIDFGH");

        compressed_vector = compress::compress(64, &mut s);

        let encoded: Vec<u8> = bincode::serialize(&compressed_vector).unwrap();

        file_io::write_to_file(&encoded, file_name);
    }
    let unencoded = file_io::read_from_file_deserialized(file_name);

    assert_eq!(unencoded, compressed_vector);
}
