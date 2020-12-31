use crc32fast::Hasher;
use rand::Rng;
use std::{thread::current, vec};

pub fn decompress(compressed_data: &Vec<u32>, block_size: usize) -> Vec<u8> {
    let mut hasher: Hasher;

    let mut decompressed_data: Vec<u8> = vec![0; block_size * compressed_data.len()];

    let mut current_hash: u32 = 0;
    let mut current_bytes: Vec<u8> = vec![0; block_size];

    let mut matches: usize = 0;

    loop {
        hasher = Hasher::new();
        hasher.update(current_bytes.as_slice());
        current_hash = hasher.finalize();
        for (i, item) in compressed_data.iter().enumerate() {
            if current_hash == *item {
                for ii in 0..block_size {
                    decompressed_data[(i * block_size) + ii] = current_bytes[ii];
                }
                // println!("{:?}", current_bytes);
                matches += 1;
            }
        }
        if !increment_byte_vector(&mut current_bytes) {
            break;
        }
    }

    println!(
        "Avg matches per hash: {}",
        matches as f64 / compressed_data.len() as f64
    );

    // let mut current_hash: u32 = 0;
    // let mut current_bytes: Vec<u8>;
    // for i in compressed_data.iter() {
    //     current_bytes = vec![0; block_size];
    //     hasher = Hasher::new();
    //     hasher.update(current_bytes.as_slice());
    //     current_hash = hasher.finalize();
    //     while *i != current_hash {
    //         increment_byte_vector(&mut current_bytes);
    //         hasher = Hasher::default();
    //         hasher.update(current_bytes.as_slice());
    //         current_hash = hasher.finalize();
    //         // println!("Hash: {}", current_hash);
    //         // println!("Bytes: {:?}", current_bytes);
    //     }
    //     println!("{:?}", current_bytes);
    //     decompressed_data.append(&mut current_bytes);
    // }

    decompressed_data
}

fn increment_byte_vector(vector: &mut Vec<u8>) -> bool {
    let mut carry = true;
    let mut i = 0;
    {
        let mut will_overflow: bool = true;
        for i in vector.iter() {
            if *i != 255 {
                will_overflow = false;
            }
        }
        if will_overflow == true {
            return false;
        }
    }

    while carry {
        if (vector[i] == 255) && carry {
            vector[i] = 0;
        } else {
            vector[i] += 1;
            carry = false;
        }
        i += 1;
    }
    true
}

fn generate_random_byte_vector(size: usize) -> Vec<u8> {
    let mut to_ret = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        to_ret.push(rng.gen::<u8>());
    }
    to_ret
}
