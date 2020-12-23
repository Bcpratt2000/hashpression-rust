pub mod compress;

use bincode;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut s = String::from("sdf");
    let compressed_vector: Vec<u32> = compress::compress(64, &mut s);

    let encoded: Vec<u8> = bincode::serialize(&compressed_vector).unwrap();

    {
        let mut file = File::create("testFile").unwrap();
        file.write_all(&encoded);
        file.sync_all().unwrap();
    }

    for i in compressed_vector.iter() {
        println!("{:b}", i);
    }
}
